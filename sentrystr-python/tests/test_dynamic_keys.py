#!/usr/bin/env python3
"""
Dynamic key generation test for SentryStr Python bindings
Generates new keys for each test run
"""

import sys
import os
import time
from datetime import datetime

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))
sys.path.insert(0, os.path.dirname(__file__))

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
    "wss://relay.snort.social",
    "wss://nostr.wine",
    "wss://relay.nostr.band"
]

def run_dynamic_key_test():
    """Run test with dynamically generated keys"""

    # Generate new keys for this test run
    keys = generate_test_keys()
    sender_private_key = keys['private_key']  # Hex format for SentryStr
    sender_public_key = keys['public_key_hex']  # For display

    print("🔐 DYNAMIC KEY GENERATION TEST")
    print("=" * 60)
    print(f"🎯 Target npub: {TARGET_NPUB}")
    print(f"🆔 Generated sender pubkey: {sender_public_key[:20]}...")
    print(f"🔑 Generated sender private key: {sender_private_key[:20]}...")
    print(f"📡 Using relays: {', '.join(RELAYS)}")
    print(f"⏰ Test started at: {datetime.now().isoformat()}")
    print("=" * 60)

    try:
        # Test 1: Create encrypted client for target
        print("\n📝 Test 1: Encrypted messaging to target npub")
        encrypted_config = sentrystr.Config(sender_private_key, RELAYS)
        encrypted_config.with_encryption(TARGET_PUBKEY)
        encrypted_client = sentrystr.NostrSentryClient(encrypted_config)

        encrypted_client.capture_message(f"🔐 Encrypted test message from dynamically generated key: {sender_public_key[:16]}...")
        print("✅ Sent encrypted message to target")
        time.sleep(2)

        # Test 2: Create public client
        print("\n📝 Test 2: Public messaging")
        public_config = sentrystr.Config(sender_private_key, RELAYS)
        public_client = sentrystr.NostrSentryClient(public_config)

        public_client.capture_message(f"🌍 Public test message from new key: {sender_public_key[:16]}...")
        print("✅ Sent public message")
        time.sleep(2)

        # Test 3: Detailed error with context
        print("\n📝 Test 3: Detailed error event")
        event = sentrystr.Event()
        event.with_message("Dynamic key test - Database connection failure")
        event.with_level(sentrystr.Level("error"))

        # Add user context
        user = sentrystr.User()
        user.with_id(f"dynamic_user_{int(time.time())}")
        user.with_email("dynamic.test@sentrystr.dev")
        user.with_username("dynamic_tester")
        event.with_user(user)

        # Add tags with dynamic data
        event.with_tag("test_type", "dynamic_keys")
        event.with_tag("sender_pubkey", sender_public_key[:20] + "...")  # Truncated for privacy
        event.with_tag("timestamp", datetime.now().isoformat())
        event.with_tag("test_run_id", str(int(time.time())))

        # Add extra data
        event.add_extra("sender_pubkey", sender_public_key)
        event.add_extra("key_generation_time", datetime.now().isoformat())
        event.add_extra("test_framework", "dynamic_sentrystr")
        event.add_extra("encryption_target", TARGET_NPUB)

        encrypted_client.capture_event(event)
        print("✅ Sent detailed error event with dynamic context")
        time.sleep(2)

        # Test 4: Exception with stack trace
        print("\n📝 Test 4: Exception with stack trace")

        # Create frames
        frame1 = sentrystr.Frame("test_dynamic_keys.py")
        frame1.with_function("run_dynamic_key_test")
        frame1.with_lineno(95)

        frame2 = sentrystr.Frame("dynamic_auth.py")
        frame2.with_function("authenticate_with_new_key")
        frame2.with_lineno(42)

        frame3 = sentrystr.Frame("key_manager.py")
        frame3.with_function("validate_key_pair")
        frame3.with_lineno(178)

        stacktrace = sentrystr.Stacktrace([frame1, frame2, frame3])

        exception = sentrystr.Exception("KeyValidationError", f"Invalid key format for {sender_public_key[:20]}...")
        exception.with_stacktrace(stacktrace)

        exception_event = sentrystr.Event()
        exception_event.with_message("Dynamic key validation failed")
        exception_event.with_level(sentrystr.Level("error"))
        exception_event.with_exception(exception)
        exception_event.with_tag("error_category", "key_management")
        exception_event.with_tag("dynamic_key", "true")

        public_client.capture_event(exception_event)
        print("✅ Sent exception with stack trace")
        time.sleep(2)

        # Test 5: Multiple severity levels with dynamic data
        print("\n📝 Test 5: Multiple severity levels")

        severity_tests = [
            ("debug", f"Debug: Key pair generated successfully - {sender_public_key[:16]}..."),
            ("info", f"Info: Connection established from {sender_public_key[:16]}..."),
            ("warning", f"Warning: Rate limiting applied to {sender_public_key[:16]}..."),
            ("error", f"Error: Authentication failed for {sender_public_key[:16]}..."),
            ("fatal", f"Fatal: System shutdown initiated by {sender_public_key[:16]}...")
        ]

        for level, message in severity_tests:
            event = sentrystr.Event()
            event.with_message(message)
            event.with_level(sentrystr.Level(level))
            event.with_tag("severity_test", "dynamic_keys")
            event.with_tag("original_level", level)
            event.with_tag("sender_key", sender_public_key[:20] + "...")
            event.add_extra("test_sequence", f"severity_{level}")
            event.add_extra("sender_pubkey", sender_public_key)
            event.add_extra("generation_timestamp", datetime.now().isoformat())

            # Alternate between encrypted and public
            client = encrypted_client if level in ["error", "fatal"] else public_client
            client.capture_event(event)
            print(f"✅ Sent {level.upper()} level event ({'encrypted' if level in ['error', 'fatal'] else 'public'})")
            time.sleep(1)

        # Test 6: Performance monitoring with dynamic data
        print("\n📝 Test 6: Performance monitoring")

        perf_event = sentrystr.Event()
        perf_event.with_message(f"Performance: Key generation took 0.{int(time.time()) % 1000}ms")
        perf_event.with_level(sentrystr.Level("info"))
        perf_event.with_tag("performance", "key_generation")
        perf_event.with_tag("operation", "dynamic_key_creation")
        perf_event.add_extra("key_generation_duration_ms", int(time.time()) % 1000)
        perf_event.add_extra("generated_key", sender_public_key)
        perf_event.add_extra("key_entropy_bits", 256)
        perf_event.add_extra("algorithm", "custom_test_generation")

        public_client.capture_event(perf_event)
        print("✅ Sent performance monitoring event")

        print("\n" + "="*60)
        print("🎉 Dynamic key test completed successfully!")
        print(f"📱 Check Nostr clients for events from: {sender_public_key[:20]}...")
        print(f"🔐 Encrypted events sent to: {TARGET_NPUB}")
        print("🌍 Public events visible to all")
        print("⏰ Events may take a moment to propagate")
        print("🔄 Run this test again for different keys each time!")

        return True

    except Exception as e:
        print(f"❌ Dynamic key test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = run_dynamic_key_test()
    if success:
        print("\n✅ Dynamic key test completed successfully!")
    else:
        print("\n❌ Dynamic key test failed!")

    sys.exit(0 if success else 1)