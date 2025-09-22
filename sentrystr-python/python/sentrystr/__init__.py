"""
SentryStr Python Bindings

A Python library for SentryStr - a decentralized error tracking and alerting system using Nostr.

This package provides Python bindings for the Rust SentryStr library, allowing you to:
- Send error events to Nostr relays
- Capture exceptions, messages, and custom events
- Configure encryption and direct messaging
- Track application errors in a decentralized manner

Example:
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
"""

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
]

__version__ = "0.1.0"
