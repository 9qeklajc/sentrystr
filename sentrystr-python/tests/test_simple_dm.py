#!/usr/bin/env python3
"""
Simple direct messaging test that exactly matches the Rust combined example
"""

import os
import sys
import time

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python"))

try:
    import sentrystr
    from key_generator import generate_test_keys, get_target_pubkey

    print("✓ Successfully imported sentrystr and key generator")
except ImportError as e:
    print(f"❌ Failed to import: {e}")
    print("Make sure to build the Python bindings first with: maturin develop")
    sys.exit(1)

# Configuration matching Rust example exactly
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()

RELAYS = ["wss://relay.damus.io", "wss://nos.lol", "wss://nostr.chaima.info"]


def main():
    """Python equivalent of Rust run_combined_example()"""
    print("🚀 Simple Direct Messaging Test (matches Rust exactly)")
    print(f"🎯 Target: {TARGET_NPUB}")
    print("=" * 60)

    # Generate keys (like Keys::generate() in Rust)
    keys = generate_test_keys()
    sender_private_key = keys["private_key"]
    sender_public_key = keys["public_key_hex"]

    print(f"🔑 Generated sender key: {sender_public_key[:20]}...")

    try:
        # 1. Create main client (public events)
        main_config = sentrystr.Config(sender_private_key, RELAYS)
        main_client = sentrystr.NostrSentryClient(main_config)
        print("✓ Created main client")

        # 2. Create encrypted client (DirectMessage equivalent)
        encrypted_config = sentrystr.Config(sender_private_key, RELAYS)
        encrypted_config.with_encryption(TARGET_PUBKEY)
        encrypted_client = sentrystr.NostrSentryClient(encrypted_config)
        print("✓ Created encrypted client (DM equivalent)")

        # 3. Info event - public (like Rust example)
        info_event = sentrystr.Event()
        info_event.with_message("Application started successfully")
        info_event.with_level(sentrystr.Level("info"))

        main_client.capture_event(info_event)
        print("✅ Info event captured (public)")
        time.sleep(2)

        # 4. Warning event - encrypted DM (like Rust with min_level: Warning)
        warning_event = sentrystr.Event()
        warning_event.with_message("High memory usage detected")
        warning_event.with_level(sentrystr.Level("warning"))

        encrypted_client.capture_event(warning_event)
        print("✅ Warning event captured with DM")
        time.sleep(2)

        # 5. Standalone DM (like send_direct_message in Rust)
        encrypted_client.capture_message("Manual alert: System maintenance required")
        print("✅ Standalone DM sent")
        time.sleep(2)

        # 6. Error event - encrypted DM (like Rust error with DM)
        error_event = sentrystr.Event()
        error_event.with_message("hello world")  # Matches Rust exactly
        error_event.with_level(sentrystr.Level("error"))

        encrypted_client.capture_event(error_event)
        print("✅ Error event captured with DM")

        print("\n" + "=" * 60)
        print("🎉 Simple DM test completed successfully!")
        print("📱 This exactly matches the Rust combined_example.rs flow:")
        print("   • Info event (public)")
        print("   • Warning event (encrypted/DM)")
        print("   • Standalone message (encrypted/DM)")
        print("   • Error event (encrypted/DM)")
        print(f"🔐 Encrypted events sent to: {TARGET_NPUB}")

        return True

    except Exception as e:
        print(f"❌ Simple DM test failed: {e}")
        import traceback

        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
