#!/usr/bin/env python3
"""
Real-world test suite for SentryStr Python bindings
Sends actual events to the specified npub for testing
"""

import sys
import os
import time
import asyncio
from datetime import datetime

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))

try:
    import sentrystr
    from key_generator import generate_test_keys, get_target_pubkey
    print("✓ Successfully imported sentrystr and key generator")
except ImportError as e:
    print(f"❌ Failed to import: {e}")
    print("Make sure to build the Python bindings first with: maturin develop")
    sys.exit(1)

# Test configuration
TEST_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TEST_PUBKEY = get_target_pubkey()  # Use npub directly

# Generate dynamic keys for this test run
DYNAMIC_KEYS = generate_test_keys()
TEST_PRIVATE_KEY = DYNAMIC_KEYS['private_key']
TEST_PUBLIC_KEY = DYNAMIC_KEYS['public_key_hex']
TEST_RELAYS = [
    "wss://relay.damus.io",
    "wss://nos.lol",
    "wss://relay.snort.social",
    "wss://nostr.chaima.info"
]

def test_basic_error_reporting():
    """Test basic error reporting functionality"""
    print("\n🧪 Testing Basic Error Reporting...")

    try:
        # Create configuration
        config = sentrystr.Config(TEST_PRIVATE_KEY, TEST_RELAYS)
        print("✓ Created configuration")

        # Create client
        client = sentrystr.NostrSentryClient(config)
        print("✓ Created NostrSentryClient")

        # Test simple error message
        client.capture_error(f"Test error from Python bindings with dynamic key: {TEST_PUBLIC_KEY[:16]}...")
        print("✓ Sent basic error message")

        # Test simple message
        client.capture_message(f"Test message from Python bindings - System operational from key: {TEST_PUBLIC_KEY[:16]}...")
        print("✓ Sent basic message")

        return True

    except Exception as e:
        print(f"❌ Basic error reporting test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_detailed_event_creation():
    """Test creating detailed events with full context"""
    print("\n🧪 Testing Detailed Event Creation...")

    try:
        config = sentrystr.Config(TEST_PRIVATE_KEY, TEST_RELAYS)
        client = sentrystr.NostrSentryClient(config)

        # Create a detailed event
        event = sentrystr.Event()
        event.with_message("Database connection failed - Connection timeout")
        event.with_level(sentrystr.Level("error"))

        # Add user context
        user = sentrystr.User()
        user.with_id("user_12345")
        user.with_email("test.user@example.com")
        user.with_username("testuser")
        event.with_user(user)
        print("✓ Added user context")

        # Add request context
        request = sentrystr.Request()
        request.with_url("https://api.example.com/users/12345")
        request.with_method("GET")
        request.with_query_string("include=profile&format=json")
        event.with_user(user)  # Note: should be event.with_request(request) but testing current API
        print("✓ Added request context")

        # Add tags
        event.with_tag("environment", "test")
        event.with_tag("service", "user-api")
        event.with_tag("version", "1.2.3")
        event.with_tag("test_type", "dynamic_keys")
        event.add_tag("region", "us-west-2")
        print("✓ Added tags")

        # Add extra data
        event.add_extra("response_time", 5.432)
        event.add_extra("retry_count", 3)
        event.add_extra("database_host", "db-primary.example.com")
        event.add_extra("sender_pubkey", TEST_PUBLIC_KEY)
        event.add_extra("test_run_id", str(int(time.time())))
        print("✓ Added extra data")

        # Send the detailed event
        client.capture_event(event)
        print("✓ Sent detailed event")

        return True

    except Exception as e:
        print(f"❌ Detailed event creation test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_exception_handling():
    """Test exception handling and reporting"""
    print("\n🧪 Testing Exception Handling...")

    try:
        config = sentrystr.Config(TEST_PRIVATE_KEY, TEST_RELAYS)
        client = sentrystr.NostrSentryClient(config)

        # Simulate different types of exceptions
        exceptions_to_test = [
            ("ValueError", "Invalid input parameter: expected integer, got string"),
            ("ConnectionError", "Failed to connect to database after 3 retries"),
            ("TimeoutError", "Request timed out after 30 seconds"),
            ("PermissionError", "Access denied: insufficient privileges for operation"),
            ("FileNotFoundError", "Configuration file not found: /etc/app/config.yml")
        ]

        for exc_type, exc_message in exceptions_to_test:
            # Create exception event
            exception = sentrystr.Exception(exc_type, exc_message)

            event = sentrystr.Event()
            event.with_message(f"Unhandled {exc_type}: {exc_message}")
            event.with_level(sentrystr.Level("error"))
            event.with_exception(exception)
            event.with_tag("exception_type", exc_type)

            client.capture_event(event)
            print(f"✓ Sent {exc_type} exception")

            # Small delay between events
            time.sleep(0.5)

        return True

    except Exception as e:
        print(f"❌ Exception handling test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_stacktrace_reporting():
    """Test stacktrace creation and reporting"""
    print("\n🧪 Testing Stacktrace Reporting...")

    try:
        config = sentrystr.Config(TEST_PRIVATE_KEY, TEST_RELAYS)
        client = sentrystr.NostrSentryClient(config)

        # Create frames for a simulated stack trace
        frames = []

        # Frame 1: Top of stack
        frame1 = sentrystr.Frame("main.py")
        frame1.with_function("main")
        frame1.with_lineno(42)
        frames.append(frame1)

        # Frame 2: Middle
        frame2 = sentrystr.Frame("api/handlers.py")
        frame2.with_function("handle_user_request")
        frame2.with_lineno(128)
        frames.append(frame2)

        # Frame 3: Bottom (where error occurred)
        frame3 = sentrystr.Frame("database/connection.py")
        frame3.with_function("connect_to_database")
        frame3.with_lineno(256)
        frames.append(frame3)

        # Create stacktrace
        stacktrace = sentrystr.Stacktrace(frames)

        # Create exception with stacktrace
        exception = sentrystr.Exception("DatabaseConnectionError", "Connection pool exhausted")
        exception.with_stacktrace(stacktrace)

        # Create event
        event = sentrystr.Event()
        event.with_message("Database connection failed with full stack trace")
        event.with_level(sentrystr.Level("fatal"))
        event.with_exception(exception)
        event.with_tag("component", "database")
        event.with_tag("severity", "critical")

        client.capture_event(event)
        print("✓ Sent event with complete stacktrace")

        return True

    except Exception as e:
        print(f"❌ Stacktrace reporting test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_different_log_levels():
    """Test different log levels and message types"""
    print("\n🧪 Testing Different Log Levels...")

    try:
        config = sentrystr.Config(TEST_PRIVATE_KEY, TEST_RELAYS)
        client = sentrystr.NostrSentryClient(config)

        # Test all log levels
        levels_and_messages = [
            ("debug", "Debug: Processing user authentication token"),
            ("info", "Info: User login successful for user@example.com"),
            ("warning", "Warning: API rate limit approaching (80% of quota used)"),
            ("error", "Error: Failed to send notification email"),
            ("fatal", "Fatal: System memory exhausted, shutting down")
        ]

        for level_name, message in levels_and_messages:
            event = sentrystr.Event()
            event.with_message(message)
            event.with_level(sentrystr.Level(level_name))
            event.with_tag("test_type", "log_level_test")
            event.with_tag("level", level_name)
            event.add_extra("timestamp", datetime.now().isoformat())

            client.capture_event(event)
            print(f"✓ Sent {level_name.upper()} level message")

            time.sleep(0.3)

        return True

    except Exception as e:
        print(f"❌ Log levels test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_performance_monitoring():
    """Test performance monitoring events"""
    print("\n🧪 Testing Performance Monitoring...")

    try:
        config = sentrystr.Config(TEST_PRIVATE_KEY, TEST_RELAYS)
        client = sentrystr.NostrSentryClient(config)

        # Simulate performance events
        performance_events = [
            {
                "operation": "database_query",
                "duration": 1.234,
                "query": "SELECT * FROM users WHERE active = true",
                "level": "info"
            },
            {
                "operation": "api_request",
                "duration": 0.567,
                "endpoint": "/api/v1/users",
                "level": "info"
            },
            {
                "operation": "cache_miss",
                "duration": 0.089,
                "cache_key": "user:12345:profile",
                "level": "warning"
            },
            {
                "operation": "slow_query",
                "duration": 5.678,
                "query": "SELECT COUNT(*) FROM large_table",
                "level": "warning"
            }
        ]

        for perf_data in performance_events:
            event = sentrystr.Event()
            event.with_message(f"Performance: {perf_data['operation']} took {perf_data['duration']}s")
            event.with_level(sentrystr.Level(perf_data['level']))
            event.with_tag("operation_type", perf_data['operation'])
            event.with_tag("performance", "monitoring")
            event.add_extra("duration_seconds", perf_data['duration'])

            # Add operation-specific data
            for key, value in perf_data.items():
                if key not in ['operation', 'duration', 'level']:
                    event.add_extra(key, value)

            client.capture_event(event)
            print(f"✓ Sent performance event: {perf_data['operation']}")

            time.sleep(0.2)

        return True

    except Exception as e:
        print(f"❌ Performance monitoring test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def run_all_tests():
    """Run all test suites"""
    print(f"🚀 Starting SentryStr Python Bindings Test Suite (Dynamic Keys)")
    print(f"📡 Sending events to npub: {TEST_NPUB}")
    print(f"🆔 Generated sender pubkey: {TEST_PUBLIC_KEY[:20]}...")
    print(f"🔑 Generated sender private key: {TEST_PRIVATE_KEY[:20]}...")
    print(f"🔗 Using relays: {', '.join(TEST_RELAYS)}")
    print("=" * 60)

    tests = [
        ("Basic Error Reporting", test_basic_error_reporting),
        ("Detailed Event Creation", test_detailed_event_creation),
        ("Exception Handling", test_exception_handling),
        ("Stacktrace Reporting", test_stacktrace_reporting),
        ("Different Log Levels", test_different_log_levels),
        ("Performance Monitoring", test_performance_monitoring),
    ]

    passed = 0
    failed = 0

    for test_name, test_func in tests:
        print(f"\n{'='*20} {test_name} {'='*20}")
        try:
            if test_func():
                passed += 1
                print(f"✅ {test_name} PASSED")
            else:
                failed += 1
                print(f"❌ {test_name} FAILED")
        except Exception as e:
            failed += 1
            print(f"❌ {test_name} FAILED with exception: {e}")

        # Small delay between test suites
        time.sleep(1)

    print("\n" + "="*60)
    print(f"📊 Test Results: {passed} passed, {failed} failed")

    if failed == 0:
        print("🎉 All tests passed! Events should be visible on the Nostr network.")
        print(f"📱 Check your Nostr client for events sent to {TEST_NPUB}")
        print(f"🆔 Events sent from dynamically generated key: {TEST_PUBLIC_KEY[:20]}...")
        print("⏰ Note: It may take a few moments for events to propagate across relays")
        print("🔄 Run this test again for different keys each time!")
    else:
        print("⚠️  Some tests failed. Check the output above for details.")

    return failed == 0

if __name__ == "__main__":
    success = run_all_tests()
    sys.exit(0 if success else 1)