"""Non-blocking Python :mod:`logging` integration for SentryStr.

:class:`SentryStrHandler` is a standard :class:`logging.Handler` that turns log
records into SentryStr events and announces them to Nostr relays. Announcing to
relays is a network round-trip and *blocks*, so calling it directly from a
request handler or an asyncio event loop would stall the caller.

:func:`install_sentrystr_logging` wires the handler up behind a
:class:`logging.handlers.QueueHandler` / :class:`~logging.handlers.QueueListener`
pair so that:

* the calling thread only pays the cost of putting a record on an in-memory
  queue (fast, non-blocking), and
* a dedicated background thread drains the queue and performs the blocking
  relay announcement.

This is the canonical standard-library pattern for keeping slow handlers off the
hot path, which makes SentryStr a drop-in "turn it on" logging sink for any
Python application.
"""

from __future__ import annotations

import contextlib
import logging
import logging.handlers
import queue
import secrets
import time
from typing import Any, Sequence, Union

from ._sentrystr import (
    Config,
    Event,
    Level,
    NostrSentryClient,
)
from ._sentrystr import (
    Exception as SentryStrException,
)

__all__ = [
    "SentryStrHandler",
    "SentryStrLoggingHandle",
    "install_sentrystr_logging",
]

LoggerLike = Union[str, logging.Logger]


def _level_to_sentrystr(levelno: int) -> str:
    """Map a stdlib numeric log level to a SentryStr level string."""
    if levelno >= logging.CRITICAL:
        return "fatal"
    if levelno >= logging.ERROR:
        return "error"
    if levelno >= logging.WARNING:
        return "warning"
    if levelno >= logging.INFO:
        return "info"
    return "debug"


class SentryStrHandler(logging.Handler):
    """A :class:`logging.Handler` that announces records to Nostr via SentryStr.

    The underlying :class:`NostrSentryClient` blocks while publishing, so this
    handler is intended to run on a background thread (see
    :func:`install_sentrystr_logging`). It can also be used directly when
    blocking is acceptable.

    The :class:`NostrSentryClient` is created lazily on the first emitted record
    so that the (blocking) relay connection happens on whichever thread drains
    records, not at install time. If client construction fails, records are
    dropped quietly and construction is retried with exponential backoff, so a
    relay that is briefly unreachable does not disable logging for the whole
    process lifetime nor spin in a tight retry loop.
    """

    # Backoff bounds (seconds) for retrying a failed client construction.
    _INITIAL_RETRY_BACKOFF = 5.0
    _MAX_RETRY_BACKOFF = 300.0

    def __init__(
        self,
        client: Any | None = None,
        *,
        config: Any | None = None,
        private_key: str | None = None,
        relays: Sequence[str] | None = None,
        recipient_pubkey: str | None = None,
        platform: str = "python",
        level: int = logging.NOTSET,
    ) -> None:
        super().__init__(level=level)
        self._client = client
        self._config = config
        self._private_key = private_key
        self._relays = list(relays) if relays is not None else None
        self._recipient_pubkey = recipient_pubkey
        self._platform = platform
        self._init_error: BaseException | None = None
        self._retry_after = 0.0
        self._retry_backoff = self._INITIAL_RETRY_BACKOFF

    def _ensure_client(self) -> Any:
        if self._client is not None:
            return self._client
        # While a previous construction is still in its backoff window, fail fast
        # without re-attempting the (blocking) relay connection.
        if self._init_error is not None and time.monotonic() < self._retry_after:
            raise self._init_error
        try:
            config = self._config
            if config is None:
                if not self._relays:
                    raise ValueError(
                        "SentryStrHandler requires `relays`, a `config`, or a "
                        "`client`."
                    )
                # A bare hex key is accepted by SentryStr; generate an ephemeral
                # identity when the caller does not supply one.
                config = Config(
                    self._private_key or secrets.token_hex(32), self._relays
                )
                if self._recipient_pubkey:
                    config.with_nip44_encryption(self._recipient_pubkey)
            self._client = NostrSentryClient(config)
        except BaseException as exc:
            # Schedule the next attempt with exponential backoff so an
            # unreachable relay neither disables logging permanently nor spins.
            self._init_error = exc
            self._retry_after = time.monotonic() + self._retry_backoff
            self._retry_backoff = min(
                self._retry_backoff * 2, self._MAX_RETRY_BACKOFF
            )
            raise
        # Success: clear the error latch and reset the backoff.
        self._init_error = None
        self._retry_backoff = self._INITIAL_RETRY_BACKOFF
        return self._client

    def _build_event(self, record: logging.LogRecord) -> Any:
        event = Event()
        event.platform = self._platform
        event.level = Level(_level_to_sentrystr(record.levelno))
        event.message = self.format(record)
        event.logger = record.name

        event.add_tag("level", record.levelname)
        event.add_tag("logger_name", record.name)
        if record.module:
            event.add_tag("module", record.module)
        if record.funcName:
            event.add_tag("function", record.funcName)

        request_id = getattr(record, "request_id", None)
        if request_id:
            event.add_tag("request_id", str(request_id))

        event.add_extra("pathname", record.pathname)
        event.add_extra("lineno", record.lineno)

        # When a QueueHandler has already flattened the record, exc_info is gone
        # and the traceback lives in the formatted message. When present, also
        # attach it as a structured exception.
        if record.exc_info and record.exc_info[0] is not None:
            exc_type, exc_value, _ = record.exc_info
            type_name = getattr(exc_type, "__name__", str(exc_type))
            event.with_exception(SentryStrException(type_name, str(exc_value)))

        return event

    def emit(self, record: logging.LogRecord) -> None:
        try:
            client = self._ensure_client()
        except Exception:
            # Client unavailable (or within the retry backoff window). Drop
            # quietly; _ensure_client retries the connection automatically once
            # the backoff elapses.
            return
        try:
            client.capture_event(self._build_event(record))
        except Exception:
            self.handleError(record)


class SentryStrLoggingHandle:
    """Handle for a background SentryStr logging worker.

    Returned by :func:`install_sentrystr_logging`. Call :meth:`close` (also
    aliased as :meth:`stop`) on application shutdown to detach the handler and
    flush any queued records to the relays.
    """

    def __init__(
        self,
        listener: logging.handlers.QueueListener,
        queue_handler: logging.handlers.QueueHandler,
        handler: SentryStrHandler,
        loggers: Sequence[logging.Logger],
    ) -> None:
        self._listener = listener
        self.queue_handler = queue_handler
        self.handler = handler
        self._loggers = list(loggers)
        self._closed = False

    def close(self, timeout: float | None = None) -> None:
        if self._closed:
            return
        self._closed = True
        for lg in self._loggers:
            with contextlib.suppress(Exception):
                lg.removeHandler(self.queue_handler)
        try:
            if timeout is None:
                # Drain the queue (flushing all pending events) before returning.
                self._listener.stop()
            else:
                self._stop_within(timeout)
        finally:
            self.handler.close()

    stop = close

    def _stop_within(self, timeout: float) -> None:
        """Signal the worker to stop and wait at most ``timeout`` seconds for the
        backlog to flush. Any remaining events are abandoned -- the
        :class:`~logging.handlers.QueueListener` worker is a daemon thread, so it
        cannot keep the interpreter alive -- so shutdown can never hang on a slow
        or unreachable relay.
        """
        listener = self._listener
        thread = getattr(listener, "_thread", None)
        if thread is None:
            return
        # The listener stops once it dequeues its sentinel. Make room first if
        # the queue is full so the sentinel is guaranteed to be enqueued.
        for _ in range(3):
            try:
                listener.enqueue_sentinel()
                break
            except queue.Full:
                with contextlib.suppress(queue.Empty):
                    listener.queue.get_nowait()
        thread.join(timeout)
        if not thread.is_alive():
            listener._thread = None

    def __enter__(self) -> SentryStrLoggingHandle:
        return self

    def __exit__(self, *exc: object) -> bool:
        self.close()
        return False


class _DroppingQueueHandler(logging.handlers.QueueHandler):
    """:class:`~logging.handlers.QueueHandler` that drops records when the queue
    is full instead of blocking the caller or noisily reporting the error.

    This keeps the logging hot path non-blocking and bounds memory: if the
    background worker cannot keep up with a slow or unreachable relay, new
    records are discarded rather than accumulating without limit.
    """

    def enqueue(self, record: logging.LogRecord) -> None:
        try:
            self.queue.put_nowait(record)
        except queue.Full:
            pass


def _resolve_logger(logger: LoggerLike | None) -> logging.Logger:
    if logger is None:
        return logging.getLogger()
    if isinstance(logger, str):
        return logging.getLogger(logger)
    return logger


def install_sentrystr_logging(
    *,
    relays: Sequence[str] | None = None,
    private_key: str | None = None,
    level: int | str = logging.ERROR,
    logger: LoggerLike | None = None,
    loggers: Sequence[LoggerLike] | None = None,
    recipient_pubkey: str | None = None,
    platform: str = "python",
    formatter: logging.Formatter | None = None,
    filters: Sequence[logging.Filter] | None = None,
    queue_size: int = -1,
    client: Any | None = None,
    config: Any | None = None,
    respect_handler_level: bool = True,
) -> SentryStrLoggingHandle:
    """Attach SentryStr to Python logging, announcing on a background thread.

    Records at or above ``level`` on the targeted logger(s) are pushed onto an
    in-memory queue (non-blocking) and announced to the relays by a dedicated
    background worker thread, so the slow relay round-trip never blocks the
    caller.

    Parameters
    ----------
    relays:
        Relay URLs to announce to. Required unless ``config`` or ``client`` is
        given.
    private_key:
        Secret key (nsec or 64-char hex). Defaults to an ephemeral random key.
    level:
        Minimum level to forward (int or level name such as ``"ERROR"``).
    logger / loggers:
        Target logger(s) to attach to. Defaults to the root logger. ``loggers``
        accepts several at once (useful when application loggers set
        ``propagate=False``).
    recipient_pubkey:
        If set, events are NIP-44 encrypted to this pubkey.
    formatter:
        Optional formatter applied to the announced message.
    filters:
        Optional filters attached to the queue handler before it goes live, so
        they run on the calling thread (e.g. to enrich or scrub records, or to
        read a request-scoped ``ContextVar``) before records are handed off to
        the background worker. Attaching them here -- rather than after install
        returns -- avoids a startup window where records bypass them.
    queue_size:
        Bound on the in-memory queue (``<= 0`` means unbounded). When bounded,
        records that arrive while the queue is full are dropped rather than
        blocking the caller.

    Returns
    -------
    SentryStrLoggingHandle
        Call ``.close()`` on shutdown to flush and detach.
    """
    if isinstance(level, str):
        resolved = logging.getLevelName(level.upper())
        level = resolved if isinstance(resolved, int) else logging.ERROR

    handler = SentryStrHandler(
        client=client,
        config=config,
        private_key=private_key,
        relays=relays,
        recipient_pubkey=recipient_pubkey,
        platform=platform,
        level=level,
    )
    if formatter is not None:
        handler.setFormatter(formatter)

    log_queue: queue.Queue[Any] = queue.Queue(queue_size)
    queue_handler = _DroppingQueueHandler(log_queue)
    queue_handler.setLevel(level)
    if filters:
        for record_filter in filters:
            queue_handler.addFilter(record_filter)

    listener = logging.handlers.QueueListener(
        log_queue, handler, respect_handler_level=respect_handler_level
    )
    listener.start()

    targets: list[logging.Logger] = []
    if loggers:
        targets.extend(_resolve_logger(item) for item in loggers)
    if logger is not None or not loggers:
        targets.append(_resolve_logger(logger))

    # De-duplicate while preserving order.
    unique_targets: list[logging.Logger] = []
    seen: set[int] = set()
    for target in targets:
        if id(target) not in seen:
            seen.add(id(target))
            unique_targets.append(target)

    for target in unique_targets:
        target.addHandler(queue_handler)

    return SentryStrLoggingHandle(listener, queue_handler, handler, unique_targets)
