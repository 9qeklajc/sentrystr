#!/usr/bin/env python3
"""
Test sending events to specific npub using SentryStr Python bindings
"""

import os
import sys
import time
from datetime import datetime

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python"))

try:
    import sentrystr

    print("✓ Successfully imported sentrystr")
except ImportError as e:
    print(f"❌ Failed to import sentrystr: {e}")
    print("Make sure to build the Python bindings first with: maturin develop")
    sys.exit(1)

# Configuration
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
# Convert npub to hex format for the library
TARGET_PUBKEY = "3f20a2dd93cd83c89b45ad1c77d4d7f26f4f2e54a5b5249249f30ecc924b1ad9"

# Test sender key (this is safe to share - it's just for testing)
SENDER_PRIVATE_KEY = "nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99"

RELAYS = [
    "wss://relay.damus.io",
    "wss://nos.lol",
    "wss://relay.snort.social",
    "wss://nostr.wine",
]


def send_test_events():
    """Send various test events to the target npub"""
    print(f"🎯 Sending test events to npub: {TARGET_NPUB}")
    print("🔐 Using encrypted messaging to target")
    print(f"📡 Using relays: {', '.join(RELAYS)}")
    print("=" * 60)

    try:
        # Create configuration with encryption to target npub
        config = sentrystr.Config(SENDER_PRIVATE_KEY, RELAYS)
        config.with_encryption(TARGET_PUBKEY)
        print("✓ Created encrypted configuration")

        # Create client
        client = sentrystr.NostrSentryClient(config)
        print("✓ Created NostrSentryClient")

        # Test 1: Basic error message
        print("\n📝 Test 1: Basic Error Message")
        client.capture_error(
            "Hello from SentryStr Python bindings! This is a test error message."
        )
        print("✓ Sent basic error message")
        time.sleep(2)

        # Test 2: Info message
        print("\n📝 Test 2: Info Message")
        client.capture_message("SentryStr Python bindings test - System operational ✅")
        print("✓ Sent info message")
        time.sleep(2)

        # Test 3: Detailed event with context
        print("\n📝 Test 3: Detailed Event with User Context")
        event = sentrystr.Event()
        event.with_message("Test event with full context from Python bindings")
        event.with_level(sentrystr.Level("warning"))

        # Add user information
        user = sentrystr.User()
        user.with_id("test_user_123")
        user.with_email("tester@sentrystr.dev")
        user.with_username("python_tester")
        event.with_user(user)

        # Add tags
        event.with_tag("source", "python_bindings")
        event.with_tag("test_type", "integration")
        event.with_tag("timestamp", datetime.now().isoformat())
        event.add_tag("environment", "test")

        # Add extra data
        event.add_extra("test_framework", "custom")
        event.add_extra(
            "python_version", f"{sys.version_info.major}.{sys.version_info.minor}"
        )
        event.add_extra("event_number", 3)

        client.capture_event(event)
        print("✓ Sent detailed event with context")
        time.sleep(2)

        # Test 4: Exception with stack trace
        print("\n📝 Test 4: Exception with Stack Trace")

        # Create frames
        frame1 = sentrystr.Frame("test_send_to_npub.py")
        frame1.with_function("send_test_events")
        frame1.with_lineno(87)

        frame2 = sentrystr.Frame("sentrystr_client.py")
        frame2.with_function("process_request")
        frame2.with_lineno(42)

        frame3 = sentrystr.Frame("database.py")
        frame3.with_function("execute_query")
        frame3.with_lineno(156)

        # Create stacktrace
        stacktrace = sentrystr.Stacktrace([frame1, frame2, frame3])

        # Create exception
        exception = sentrystr.Exception(
            "DatabaseError", "Connection timeout after 30 seconds"
        )
        exception.with_stacktrace(stacktrace)

        # Create event
        event = sentrystr.Event()
        event.with_message("Simulated database connection failure")
        event.with_level(sentrystr.Level("error"))
        event.with_exception(exception)
        event.with_tag("error_type", "database")
        event.with_tag("severity", "high")

        client.capture_event(event)
        print("✓ Sent exception with stack trace")
        time.sleep(2)

        # Test 5: Performance monitoring event
        print("\n📝 Test 5: Performance Monitoring")
        perf_event = sentrystr.Event()
        perf_event.with_message("Slow API response detected")
        perf_event.with_level(sentrystr.Level("warning"))
        perf_event.with_tag("performance", "api_latency")
        perf_event.with_tag("endpoint", "/api/v1/users")
        perf_event.add_extra("response_time_ms", 2500)
        perf_event.add_extra("expected_time_ms", 200)
        perf_event.add_extra("slowdown_factor", 12.5)

        client.capture_event(perf_event)
        print("✓ Sent performance monitoring event")
        time.sleep(2)

        # Test 6: Different severity levels
        print("\n📝 Test 6: Multiple Severity Levels")
        severity_tests = [
            ("debug", "Debug message: Processing user authentication"),
            ("info", "Info: User successfully logged in"),
            ("warning", "Warning: API rate limit approaching"),
            ("error", "Error: Failed to send notification email"),
            ("fatal", "Fatal: System critical error detected"),
        ]

        for level, message in severity_tests:
            event = sentrystr.Event()
            event.with_message(f"Severity test - {message}")
            event.with_level(sentrystr.Level(level))
            event.with_tag("test_category", "severity_levels")
            event.with_tag("original_level", level)
            event.add_extra("test_sequence", f"severity_{level}")

            client.capture_event(event)
            print(f"✓ Sent {level.upper()} level event")
            time.sleep(1)

        print("\n" + "=" * 60)
        print("🎉 All test events sent successfully!")
        print(f"📱 Check your Nostr client for events sent to {TARGET_NPUB}")
        print("🔐 Events are encrypted and should only be visible to the target npub")
        print(
            "⏰ Note: It may take a few moments for events to propagate across relays"
        )
        print(
            "💡 Look for events from npub: npub1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qg4jsaxr"
        )

        return True

    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback

        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = send_test_events()
    if success:
        print("\n✅ Test completed successfully!")
    else:
        print("\n❌ Test failed!")

    sys.exit(0 if success else 1)
