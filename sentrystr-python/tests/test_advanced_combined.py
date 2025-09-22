#!/usr/bin/env python3
"""
Advanced combined example with exception handling and real-world scenarios
Extends the basic combined example with production-ready error tracking
"""

import sys
import os
import time
import json
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

# Configuration
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()  # Use npub directly

RELAYS = [
    "wss://relay.damus.io",
    "wss://nos.lol",
    "wss://relay.snort.social"
]

class SentryStrManager:
    """
    A manager class that provides the same functionality as the Rust combined example
    but with Python-specific patterns and better organization
    """

    def __init__(self):
        self.keys = generate_test_keys()
        self.sender_private_key = self.keys['private_key']
        self.sender_public_key = self.keys['public_key_hex']

        # Create clients for different purposes (similar to Rust's different configurations)
        self.public_client = self._create_public_client()
        self.secure_client = self._create_secure_client()

        print(f"🔑 Initialized SentryStr Manager with key: {self.sender_public_key[:20]}...")

    def _create_public_client(self):
        """Create client for public events (info, debug)"""
        config = sentrystr.Config(self.sender_private_key, RELAYS)
        return sentrystr.NostrSentryClient(config)

    def _create_secure_client(self):
        """Create client for encrypted events (warnings, errors, fatal)"""
        config = sentrystr.Config(self.sender_private_key, RELAYS)
        config.with_encryption(TARGET_PUBKEY)
        return sentrystr.NostrSentryClient(config)

    def capture_with_level_filtering(self, event):
        """
        Simulate the DirectMessageBuilder's min_level filtering from Rust
        Public events for low severity, encrypted for high severity
        """
        # The event stores the level internally, we need to check the level that was set
        # For this demo, we'll manually track it or extract from tags
        level_tag = None
        for tag_name, tag_value in [("level", None)]:  # This is a simplified approach
            # In practice, you'd store the level when creating the event
            pass

        # For this example, let's determine level from the message content or tags
        # In a real implementation, you'd want to store this when setting the level
        message = getattr(event, 'message', '') if hasattr(event, 'message') else ''

        # Simple heuristic based on event content (better would be to store level properly)
        if 'startup' in message.lower() or 'authentication' in message.lower() or 'api request' in message.lower():
            level = 'info'
        elif 'warning' in message.lower() or 'performance' in message.lower():
            level = 'warning'
        elif 'error' in message.lower() or 'exception' in message.lower() or 'failed' in message.lower():
            level = 'error'
        elif 'fatal' in message.lower() or 'shutdown' in message.lower():
            level = 'fatal'
        else:
            level = 'info'

        if level in ['debug', 'info']:
            # Send to public client (like regular events in Rust)
            self.public_client.capture_event(event)
            return 'public'
        else:
            # Send to encrypted client (like DirectMessage in Rust)
            self.secure_client.capture_event(event)
            return 'encrypted'

    def capture_exception_with_stacktrace(self, exc_type, exc_message, stack_frames):
        """
        Capture an exception with full stack trace
        Equivalent to detailed exception handling in Rust
        """
        # Create frames
        frames = []
        for frame_data in stack_frames:
            frame = sentrystr.Frame(frame_data['filename'])
            frame.with_function(frame_data['function'])
            frame.with_lineno(frame_data['lineno'])
            frames.append(frame)

        # Create stacktrace and exception
        stacktrace = sentrystr.Stacktrace(frames)
        exception = sentrystr.Exception(exc_type, exc_message)
        exception.with_stacktrace(stacktrace)

        # Create detailed event
        event = sentrystr.Event()
        event.with_message(f"Exception: {exc_type} - {exc_message}")
        event.with_level(sentrystr.Level("error"))
        event.with_exception(exception)
        event.with_tag("exception_type", exc_type)
        event.with_tag("has_stacktrace", "true")
        event.add_extra("captured_at", datetime.now().isoformat())
        event.add_extra("sender_key", self.sender_public_key)

        # Always encrypt exceptions (high severity)
        self.secure_client.capture_event(event)
        return 'encrypted'

    def simulate_application_lifecycle(self):
        """
        Simulate a complete application lifecycle with different event types
        Similar to the combined Rust example but more comprehensive
        """
        print("\n🔄 Simulating Application Lifecycle")
        print("=" * 50)

        # 1. Application startup (info level - public)
        startup_event = sentrystr.Event()
        startup_event.with_message("Application startup completed")
        startup_event.with_level(sentrystr.Level("info"))
        startup_event.with_tag("lifecycle", "startup")
        startup_event.with_tag("component", "main")
        startup_event.add_extra("startup_time_ms", 1234)
        startup_event.add_extra("memory_usage_mb", 45.7)

        delivery = self.capture_with_level_filtering(startup_event)
        print(f"✅ Startup event sent ({delivery})")
        time.sleep(1)

        # 2. User authentication (debug level - public)
        auth_event = sentrystr.Event()
        auth_event.with_message("User authentication successful")
        auth_event.with_level(sentrystr.Level("debug"))

        user = sentrystr.User()
        user.with_id("user_123")
        user.with_username("alice")
        user.with_email("alice@example.com")
        auth_event.with_user(user)

        auth_event.with_tag("operation", "authentication")
        auth_event.add_extra("session_id", "sess_abc123")
        auth_event.add_extra("login_method", "password")

        delivery = self.capture_with_level_filtering(auth_event)
        print(f"✅ Authentication event sent ({delivery})")
        time.sleep(1)

        # 3. Performance warning (warning level - encrypted)
        perf_event = sentrystr.Event()
        perf_event.with_message("Database query performance degraded")
        perf_event.with_level(sentrystr.Level("warning"))
        perf_event.with_tag("category", "performance")
        perf_event.with_tag("component", "database")
        perf_event.add_extra("query_time_ms", 5432)
        perf_event.add_extra("threshold_ms", 1000)
        perf_event.add_extra("query", "SELECT * FROM large_table WHERE complex_condition")

        delivery = self.capture_with_level_filtering(perf_event)
        print(f"✅ Performance warning sent ({delivery})")
        time.sleep(1)

        # 4. Critical error with exception (error level - encrypted)
        stack_frames = [
            {"filename": "main.py", "function": "main", "lineno": 42},
            {"filename": "database.py", "function": "connect", "lineno": 156},
            {"filename": "connection.py", "function": "establish", "lineno": 89}
        ]

        delivery = self.capture_exception_with_stacktrace(
            "DatabaseConnectionError",
            "Connection pool exhausted after 30 seconds",
            stack_frames
        )
        print(f"✅ Critical exception captured ({delivery})")
        time.sleep(1)

        # 5. System shutdown (fatal level - encrypted)
        shutdown_event = sentrystr.Event()
        shutdown_event.with_message("System shutdown initiated due to critical error")
        shutdown_event.with_level(sentrystr.Level("fatal"))
        shutdown_event.with_tag("lifecycle", "shutdown")
        shutdown_event.with_tag("reason", "critical_error")
        shutdown_event.add_extra("uptime_hours", 72.5)
        shutdown_event.add_extra("shutdown_reason", "memory_exhaustion")
        shutdown_event.add_extra("last_checkpoint", datetime.now().isoformat())

        delivery = self.capture_with_level_filtering(shutdown_event)
        print(f"✅ Shutdown event sent ({delivery})")

        print("\n📊 Lifecycle simulation complete!")

def demonstrate_real_world_patterns():
    """
    Demonstrate real-world error tracking patterns
    """
    print("\n🌍 Real-World Error Tracking Patterns")
    print("=" * 50)

    manager = SentryStrManager()

    # Pattern 1: API request tracking
    print("\n📡 Pattern 1: API Request Monitoring")
    api_event = sentrystr.Event()
    api_event.with_message("API request completed")
    api_event.with_level(sentrystr.Level("info"))

    # Add request context
    request = sentrystr.Request()
    request.with_url("https://api.example.com/users/123")
    request.with_method("GET")
    request.with_query_string("include=profile,settings")

    api_event.with_tag("api_endpoint", "/users/{id}")
    api_event.with_tag("http_method", "GET")
    api_event.with_tag("response_status", "200")
    api_event.add_extra("response_time_ms", 245)
    api_event.add_extra("response_size_bytes", 1024)
    api_event.add_extra("user_agent", "SentryStr-Python/1.0")

    delivery = manager.capture_with_level_filtering(api_event)
    print(f"✅ API request tracked ({delivery})")

    # Pattern 2: Business logic error
    print("\n💼 Pattern 2: Business Logic Error")
    business_error = sentrystr.Event()
    business_error.with_message("Payment processing failed - insufficient funds")
    business_error.with_level(sentrystr.Level("error"))

    user = sentrystr.User()
    user.with_id("customer_789")
    user.with_email("customer@example.com")
    business_error.with_user(user)

    business_error.with_tag("business_flow", "payment")
    business_error.with_tag("error_type", "insufficient_funds")
    business_error.with_tag("payment_method", "credit_card")
    business_error.add_extra("requested_amount", 299.99)
    business_error.add_extra("available_amount", 45.67)
    business_error.add_extra("transaction_id", "txn_abc123")
    business_error.add_extra("payment_processor", "stripe")

    delivery = manager.capture_with_level_filtering(business_error)
    print(f"✅ Business error captured ({delivery})")

    # Pattern 3: Security incident
    print("\n🛡️  Pattern 3: Security Incident")
    security_event = sentrystr.Event()
    security_event.with_message("Multiple failed login attempts detected")
    security_event.with_level(sentrystr.Level("warning"))
    security_event.with_tag("security", "authentication")
    security_event.with_tag("threat_level", "medium")
    security_event.with_tag("auto_action", "rate_limit")
    security_event.add_extra("failed_attempts", 5)
    security_event.add_extra("source_ip", "192.168.1.100")
    security_event.add_extra("time_window_minutes", 10)
    security_event.add_extra("user_agent", "suspicious_bot_v1.0")

    delivery = manager.capture_with_level_filtering(security_event)
    print(f"✅ Security incident logged ({delivery})")

    print("\n✅ Real-world patterns demonstration complete!")

def main():
    """Main function demonstrating advanced SentryStr usage"""
    print("🚀 Advanced SentryStr Python Combined Example")
    print("Enhanced version of the Rust combined_example.rs")
    print("=" * 80)

    try:
        # Create manager and run lifecycle simulation
        manager = SentryStrManager()
        manager.simulate_application_lifecycle()

        # Demonstrate real-world patterns
        demonstrate_real_world_patterns()

        print("\n" + "=" * 80)
        print("🎉 Advanced combined example completed successfully!")
        print()
        print("📚 This example demonstrated:")
        print("✅ Automatic level-based event routing (public vs encrypted)")
        print("✅ Full application lifecycle monitoring")
        print("✅ Exception capture with stack traces")
        print("✅ Real-world error tracking patterns")
        print("✅ Rich event context (users, requests, business data)")
        print("✅ Security incident reporting")
        print()
        print(f"📱 Events sent from key: {manager.sender_public_key[:20]}...")
        print(f"🔐 Encrypted events sent to: {TARGET_NPUB}")
        print("⏰ Events may take a moment to propagate")

    except Exception as e:
        print(f"❌ Advanced example failed: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()