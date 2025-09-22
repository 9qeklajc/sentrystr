#!/usr/bin/env python3
"""
Combined example demonstrating both event capture and encrypted messaging
Based on sentrystr/src/combined_example.rs
"""

import sys
import os
import time
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

# Configuration matching the Rust example
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()  # Use npub directly

RELAYS = [
    "wss://relay.damus.io",
    "wss://nos.lol"
]

def run_combined_example():
    """
    Python equivalent of the Rust combined_example.rs::run_combined_example()

    This demonstrates:
    1. Regular event capture (public)
    2. Encrypted event capture (to specific recipient)
    3. Different severity levels
    4. Event filtering by level
    """
    print("🚀 Combined Example: Event Capture + Encryption")
    print("=" * 60)

    # Generate keys for this session (like Keys::generate() in Rust)
    keys = generate_test_keys()
    sender_private_key = keys['private_key']
    sender_public_key = keys['public_key_hex']

    print(f"🔑 Generated sender key: {sender_public_key[:20]}...")
    print(f"🎯 Target recipient: {TARGET_NPUB}")
    print(f"📡 Using relays: {', '.join(RELAYS)}")
    print()

    try:
        # Create the main client (like NostrSentryClient::new() in Rust)
        main_config = sentrystr.Config(sender_private_key, RELAYS)
        main_client = sentrystr.NostrSentryClient(main_config)
        print("✓ Created main NostrSentryClient")

        # Create encrypted client for sensitive events (like DirectMessageSender in Rust)
        encrypted_config = sentrystr.Config(sender_private_key, RELAYS)
        encrypted_config.with_encryption(TARGET_PUBKEY)
        encrypted_client = sentrystr.NostrSentryClient(encrypted_config)
        print("✓ Created encrypted client for sensitive events")
        print()

        # 1. Capture an info event - public (equivalent to info event in Rust)
        print("📝 Test 1: Info event (public) - equivalent to Level::Info in Rust")
        info_event = sentrystr.Event()
        info_event.with_message("Application started successfully")
        info_event.with_level(sentrystr.Level("info"))
        info_event.with_tag("component", "startup")
        info_event.with_tag("environment", "production")
        info_event.add_extra("startup_time", datetime.now().isoformat())
        info_event.add_extra("python_version", "3.x")
        info_event.add_extra("sender_key", sender_public_key)

        main_client.capture_event(info_event)
        print("✅ Info event captured (public)")
        time.sleep(2)

        # 2. Capture a warning event - encrypted (equivalent to Level::Warning in Rust)
        print("\n📝 Test 2: Warning event (encrypted) - equivalent to Level::Warning with DM in Rust")
        warning_event = sentrystr.Event()
        warning_event.with_message("High memory usage detected")
        warning_event.with_level(sentrystr.Level("warning"))
        warning_event.with_tag("category", "performance")
        warning_event.with_tag("alert_level", "warning")
        warning_event.add_extra("memory_usage_percent", 85.7)
        warning_event.add_extra("threshold", 80.0)
        warning_event.add_extra("process_count", 124)
        warning_event.add_extra("timestamp", datetime.now().isoformat())

        encrypted_client.capture_event(warning_event)
        print("✅ Warning event captured with encryption (like DM in Rust)")
        time.sleep(2)

        # 3. Send a standalone message - encrypted (equivalent to send_direct_message in Rust)
        print("\n📝 Test 3: Standalone encrypted message - equivalent to send_direct_message in Rust")
        standalone_message = "Manual alert: System maintenance required"
        encrypted_client.capture_message(standalone_message)
        print("✅ Standalone encrypted message sent")
        time.sleep(2)

        # 4. Capture an error - encrypted (equivalent to Level::Error in Rust)
        print("\n📝 Test 4: Error event (encrypted) - equivalent to Level::Error with DM in Rust")
        error_event = sentrystr.Event()
        error_event.with_message("Database connection failed")
        error_event.with_level(sentrystr.Level("error"))

        # Add user context (like the Rust example would have)
        user = sentrystr.User()
        user.with_id(f"user_{int(time.time())}")
        user.with_email("system@example.com")
        user.with_username("system_user")
        error_event.with_user(user)

        # Add detailed error context
        error_event.with_tag("database", "postgresql")
        error_event.with_tag("severity", "critical")
        error_event.with_tag("component", "database")
        error_event.add_extra("connection_string", "postgresql://db:5432/prod")
        error_event.add_extra("retry_count", 3)
        error_event.add_extra("last_success", "2025-09-22T20:30:00Z")
        error_event.add_extra("error_code", "CONNECTION_TIMEOUT")

        encrypted_client.capture_event(error_event)
        print("✅ Error event captured with encryption")
        time.sleep(2)

        # 5. Demonstrate event filtering by level (like min_level in DirectMessageBuilder)
        print("\n📝 Test 5: Multiple events with level filtering simulation")

        # Debug event (would be filtered out in Rust with min_level: Warning)
        debug_event = sentrystr.Event()
        debug_event.with_message("Debug: Processing user authentication token")
        debug_event.with_level(sentrystr.Level("debug"))
        debug_event.with_tag("filtered", "would_not_send_dm")
        main_client.capture_event(debug_event)  # Send as public (no encryption)
        print("✅ Debug event (public only - would be filtered in DM)")

        # Fatal event (would definitely send DM in Rust)
        fatal_event = sentrystr.Event()
        fatal_event.with_message("Fatal: System shutdown initiated")
        fatal_event.with_level(sentrystr.Level("fatal"))
        fatal_event.with_tag("priority", "immediate")
        fatal_event.add_extra("shutdown_reason", "memory_exhausted")
        fatal_event.add_extra("uptime", "72h 15m")
        encrypted_client.capture_event(fatal_event)  # Send encrypted (like DM)
        print("✅ Fatal event (encrypted - would send DM)")
        time.sleep(2)

        print("\n" + "=" * 60)
        print("🎉 Combined example completed successfully!")
        print("\n📊 Summary of what was demonstrated:")
        print("✅ Event capture to public relays (like regular events in Rust)")
        print("✅ Encrypted event capture (like DirectMessageSender in Rust)")
        print("✅ Different severity levels (matching Rust Level enum)")
        print("✅ Event filtering by level (simulating min_level behavior)")
        print("✅ Rich event context (tags, extras, user info)")
        print("✅ Dynamic key generation (like Keys::generate())")
        print()
        print("📱 Events sent to:")
        print(f"   • Public events: visible to all monitoring sender key")
        print(f"   • Encrypted events: only visible to {TARGET_NPUB}")
        print(f"   • Sender key: {sender_public_key[:20]}...")
        print("⏰ Events may take a moment to propagate across relays")

        return True

    except Exception as e:
        print(f"❌ Combined example failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def demonstrate_configuration_switching():
    """
    Python equivalent of switch_dm_configurations from Rust example
    Shows how to switch between different client configurations
    """
    print("\n🔄 Configuration Switching Example")
    print("=" * 60)

    try:
        # Generate different keys for different configurations
        config1_keys = generate_test_keys()
        config2_keys = generate_test_keys()

        print(f"🔑 Configuration 1 key: {config1_keys['public_key_hex'][:20]}...")
        print(f"🔑 Configuration 2 key: {config2_keys['public_key_hex'][:20]}...")

        # Configuration 1: High-security encrypted events (like NIP-17 in Rust)
        print("\n📝 Configuration 1: High-security encrypted events")
        secure_config = sentrystr.Config(config1_keys['private_key'], RELAYS)
        secure_config.with_encryption(TARGET_PUBKEY)
        secure_client = sentrystr.NostrSentryClient(secure_config)

        secure_event = sentrystr.Event()
        secure_event.with_message("Error with high-security configuration")
        secure_event.with_level(sentrystr.Level("error"))
        secure_event.with_tag("config", "high_security")
        secure_event.add_extra("encryption", "nip44_v2")
        secure_event.add_extra("client_id", "config_1")

        secure_client.capture_event(secure_event)
        print("✅ High-security encrypted event sent")
        time.sleep(2)

        # Configuration 2: Public events for non-sensitive data
        print("\n📝 Configuration 2: Public events for monitoring")
        public_config = sentrystr.Config(config2_keys['private_key'], RELAYS)
        public_client = sentrystr.NostrSentryClient(public_config)

        public_event = sentrystr.Event()
        public_event.with_message("System monitoring update")
        public_event.with_level(sentrystr.Level("info"))
        public_event.with_tag("config", "public_monitoring")
        public_event.add_extra("encryption", "none")
        public_event.add_extra("client_id", "config_2")

        public_client.capture_event(public_event)
        print("✅ Public monitoring event sent")

        print("\n✅ Configuration switching demonstrated")
        print("💡 This shows how to use different clients for different security needs")

        return True

    except Exception as e:
        print(f"❌ Configuration switching failed: {e}")
        return False

def main():
    """Main function running all examples"""
    print("🚀 SentryStr Python Combined Example")
    print("Based on sentrystr/src/combined_example.rs")
    print("=" * 80)

    # Run the main combined example
    success1 = run_combined_example()

    if success1:
        # Run the configuration switching example
        success2 = demonstrate_configuration_switching()

        if success1 and success2:
            print("\n" + "=" * 80)
            print("🎉 All combined examples completed successfully!")
            print("📚 This demonstrates the same functionality as the Rust combined_example.rs")
            print("🔄 Run this script multiple times to see different keys each time")
        else:
            print("\n❌ Some examples failed")
    else:
        print("\n❌ Main example failed")

if __name__ == "__main__":
    main()