# SentryStr Python Bindings

Python bindings for SentryStr - a decentralized error tracking and alerting system using Nostr.

## Installation

```bash
pip install sentrystr
```

## Quick Start

```python
import sentrystr

# Basic setup
config = sentrystr.Config("your_private_key", ["wss://relay.damus.io"])
client = sentrystr.NostrSentryClient(config)

# Send events
event = sentrystr.Event().with_message("Something went wrong").with_level(sentrystr.Level("error"))
client.capture_event_sync(event)

# Or use convenience methods
client.capture_error_sync("Database connection failed")
client.capture_message_sync("System started")
```

## Async Support

```python
import asyncio
import sentrystr

async def main():
    config = sentrystr.Config("your_private_key", ["wss://relay.damus.io"])
    client = await sentrystr.NostrSentryClient.create(config)

    await client.capture_error("Async error occurred")

asyncio.run(main())
```

## Logging Integration (Background, Non-Blocking)

Announcing an event to Nostr relays is a network round-trip and **blocks** the
calling thread. To plug SentryStr into Python's standard `logging` without
stalling your request handlers or event loop, use `install_sentrystr_logging`.
It attaches a `QueueHandler` to your logger(s) and drains the queue from a
dedicated background worker thread (built on `logging.handlers.QueueListener`),
so the slow relay publish never happens on the hot path.

```python
import logging
from sentrystr import install_sentrystr_logging

# Forward WARNING and above from the "myapp" logger to the relays.
handle = install_sentrystr_logging(
    relays=["wss://relay.damus.io", "wss://nos.lol"],
    private_key="nsec1...",          # optional; an ephemeral key is generated if omitted
    level=logging.WARNING,           # int or name, e.g. "ERROR"
    loggers=["myapp"],               # one logger, or pass several at once
    # recipient_pubkey="npub1...",   # optional: NIP-44 encrypt events to this pubkey
)

logging.getLogger("myapp").error("Something broke")  # returns immediately

# On shutdown, flush queued events and stop the worker thread:
handle.close()
```

`install_sentrystr_logging` accepts `relays`/`private_key`, or a prebuilt
`config=`, or an existing `client=`. Records are enriched with the logger name,
level, module/function, and `request_id` (if present on the record) as tags. The
handle is also a context manager.

For full control you can use `SentryStrHandler` directly — it is a plain
`logging.Handler` (and therefore blocking); wrap it in your own
`QueueListener` if you need a custom worker setup.

## Advanced Usage

### Creating Custom Events

```python
import sentrystr

# Create a detailed event
event = (sentrystr.Event()
    .with_message("User authentication failed")
    .with_level(sentrystr.Level("error"))
    .with_logger("auth.login")
    .with_tag("user_id", "12345")
    .with_tag("ip_address", "192.168.1.1"))

# Add user information
user = (sentrystr.User()
    .with_id("user123")
    .with_email("user@example.com")
    .with_username("john_doe"))

event = event.with_user(user)

# Add request information
request = (sentrystr.Request()
    .with_url("https://example.com/login")
    .with_method("POST")
    .with_query_string("redirect=/dashboard"))

event = event.with_request(request)

client.capture_event_sync(event)
```

### Exception Handling

```python
import sentrystr

try:
    # Some code that might raise an exception
    raise ValueError("Something went wrong")
except Exception as e:
    # Create exception object
    exception = sentrystr.Exception(type(e).__name__, str(e))

    # Create event with exception
    event = sentrystr.Event().with_exception(exception)
    client.capture_event_sync(event)
```

### Configuration Options

```python
import sentrystr

config = sentrystr.Config("your_private_key", ["wss://relay.damus.io"])

# Set optional configuration
config.set_platform("python")
config.set_server_name("web-server-01")
config.set_release("1.0.0")
config.set_environment("production")

# Add encryption (optional)
config.with_encryption_keys("public_key", "private_key")
config.with_encryption_version("nip44")

client = sentrystr.NostrSentryClient(config)
```

## Configuration from Environment

You can also configure SentryStr using environment variables:

```bash
export SENTRYSTR_PRIVATE_KEY="your_private_key"
export SENTRYSTR_RELAYS="wss://relay.damus.io,wss://relay.nostr.band"
export SENTRYSTR_PLATFORM="python"
export SENTRYSTR_SERVER_NAME="web-server-01"
export SENTRYSTR_RELEASE="1.0.0"
export SENTRYSTR_ENVIRONMENT="production"
```

```python
import sentrystr

# Load configuration from environment
config = sentrystr.Config.from_env()
client = sentrystr.NostrSentryClient(config)
```

## API Reference

### NostrSentryClient

- `NostrSentryClient(config)` - Create a new client
- `NostrSentryClient.create(config)` - Async factory method
- `capture_event(event)` - Async method to capture an event
- `capture_message(message)` - Async method to capture a message
- `capture_error(error)` - Async method to capture an error
- `capture_exception(exception, message=None)` - Async method to capture an exception
- `capture_event_sync(event)` - Sync method to capture an event
- `capture_message_sync(message)` - Sync method to capture a message
- `capture_error_sync(error)` - Sync method to capture an error
- `capture_exception_sync(exception, message=None)` - Sync method to capture an exception

### Config

- `Config(private_key, relays)` - Create a new configuration
- `Config.from_env()` - Load configuration from environment variables
- Various setters for platform, server_name, release, environment
- `with_encryption_keys(public_key, private_key)` - Add encryption
- `with_encryption_version(version)` - Set encryption version ("nip04" or "nip44")

### Event

- `Event()` - Create a new event
- `with_message(message)` - Set the message
- `with_level(level)` - Set the level
- `with_logger(logger)` - Set the logger
- `with_user(user)` - Set user information
- `with_request(request)` - Set request information
- `with_exception(exception)` - Set exception information
- `with_tag(key, value)` - Add a tag
- `with_extra(key, value)` - Add extra data

### Level

- `Level(level_string)` - Create a level ("debug", "info", "warning", "error", "fatal")

### Exception, User, Request, Frame, Stacktrace

Various data structures for capturing detailed error information.

## License

MIT