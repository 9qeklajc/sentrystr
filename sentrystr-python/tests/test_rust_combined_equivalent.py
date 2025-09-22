#!/usr/bin/env python3
"""
Python test that exactly matches the Rust combined_example.rs functionality
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
    sys.exit(1)

# Configuration matching Rust exactly
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()

RELAYS = ["wss://relay.damus.io", "wss://nos.lol", "wss://nostr.chaima.info"]


def run_combined_example():
    """Python equivalent of Rust run_combined_example()"""
    print("🚀 Python Combined Example (matches Rust exactly)")
    print(f"🎯 Target: {TARGET_NPUB}")
    print("=" * 60)

    # Generate keys (like Keys::generate() in Rust)
    keys = generate_test_keys()
    sender_private_key = keys["private_key"]
    sender_public_key = keys["public_key_hex"]

    print(f"🔑 Generated sender key: {sender_public_key[:20]}...")

    try:
        # 1. Setup configuration for the main client (like Rust)
        config = sentrystr.Config(sender_private_key, RELAYS)
        client = sentrystr.NostrSentryClient(config)
        print("✓ Created main NostrSentryClient")

        # 2. Setup direct messaging (like DirectMessageBuilder in Rust)
        client.setup_direct_messaging(TARGET_NPUB)
        print("✓ Set up direct messaging (equivalent to DirectMessageBuilder)")

        # Now both functionalities work together:

        # 3. Capture an info event - this will be logged but no DM sent (below min level)
        info_event = sentrystr.Event()
        info_event.with_message("Application started successfully")
        info_event.with_level(sentrystr.Level("info"))

        client.capture_event(info_event)
        print("✅ Info event captured (no DM - below min level)")
        time.sleep(2)

        # 4. Capture a warning event - this will be logged AND a DM will be sent
        warning_event = sentrystr.Event()
        warning_event.with_message("High memory usage detected")
        warning_event.with_level(sentrystr.Level("warning"))

        client.capture_event(warning_event)
        print("✅ Warning event captured with DM")
        time.sleep(2)

        # 5. Send a standalone direct message
        client.send_direct_message("Manual alert: System maintenance required")
        print("✅ Standalone DM sent")
        time.sleep(2)

        # 6. Capture an error - this will also trigger a DM
        error_event = sentrystr.Event()
        error_event.with_message("hello from python")  # Matches Rust exactly
        error_event.with_level(sentrystr.Level("error"))

        client.capture_event(error_event)
        print("✅ Error event captured with DM")

        print("\n" + "=" * 60)
        print("🎉 Python Combined example completed successfully!")
        print("📱 This exactly matches the Rust combined_example.rs flow:")
        print("   • Info event (logged, no DM - below min_level)")
        print("   • Warning event (logged + DM - at min_level)")
        print("   • Standalone message (DM)")
        print("   • Error event (logged + DM - above min_level)")
        print(f"🔐 Direct messages sent to: {TARGET_NPUB}")
        print("📊 Uses NIP-17 for privacy (like Rust)")
        print("🔄 Min level filtering: Warning and above get DMs")

        return True

    except Exception as e:
        print(f"❌ Combined example failed: {e}")
        import traceback

        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = run_combined_example()
    if success:
        print("\n✅ Python combined example matches Rust functionality!")
    else:
        print("\n❌ Python combined example failed!")

    sys.exit(0 if success else 1)
