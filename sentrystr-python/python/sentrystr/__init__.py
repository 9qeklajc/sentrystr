"""SentryStr Python Bindings"""

from ._sentrystr import (
    Config,
    Event,
    Exception,
    Frame,
    Level,
    NostrSentryClient,
    Request,
    SentryStrError,
    Stacktrace,
    User,
)
from .handler import (
    SentryStrHandler,
    SentryStrLoggingHandle,
    install_sentrystr_logging,
)

__all__ = [
    "NostrSentryClient",
    "Config",
    "Event",
    "Level",
    "Exception",
    "Stacktrace",
    "Frame",
    "User",
    "Request",
    "SentryStrError",
    "SentryStrHandler",
    "SentryStrLoggingHandle",
    "install_sentrystr_logging",
]

__version__ = "0.2.0"
