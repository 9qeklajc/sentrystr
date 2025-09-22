#!/usr/bin/env python3
"""
Test the new send_direct_message method
"""

import os
import sys

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python"))

try:
    import sentrystr
    from key_generator import generate_test_keys, get_target_pubkey

    print("✓ Successfully imported sentrystr and key generator")
except ImportError as e:
    print(f"❌ Failed to import: {e}")
    sys.exit(1)

# Configuration
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()

RELAYS = ["wss://relay.damus.io", "wss://nos.lol", "wss://nostr.chaima.info"]


def test_send_direct_message():
    """Test the new send_direct_message method"""
    print("🔐 Testing send_direct_message method")
    print("=" * 60)

    # Generate keys
    keys = generate_test_keys()
    sender_private_key = keys["private_key"]
    sender_public_key = keys["public_key_hex"]

    print(f"🔑 Generated sender key: {sender_public_key[:20]}...")
    print(f"🎯 Target recipient: {TARGET_NPUB}")

    try:
        # Create main client (no encryption needed for DM setup)
        config = sentrystr.Config(sender_private_key, RELAYS)
        client = sentrystr.NostrSentryClient(config)
        print("✓ Created client")

        # Test if methods exist
        if hasattr(client, "send_direct_message") and hasattr(
            client, "setup_direct_messaging"
        ):
            print(
                "✓ send_direct_message and setup_direct_messaging methods are available"
            )

            # Setup direct messaging (like DirectMessageBuilder in Rust)
            client.setup_direct_messaging(TARGET_NPUB)
            print("✓ Set up direct messaging")

            # Test sending a direct message
            message = "🔐 Test direct message using new send_direct_message method"
            client.send_direct_message(message)
            print("✅ Direct message sent successfully!")

            return True
        else:
            print("❌ Required methods not available")
            return False

    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback

        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = test_send_direct_message()
    if success:
        print("\n✅ send_direct_message test completed successfully!")
    else:
        print("\n❌ send_direct_message test failed!")

    sys.exit(0 if success else 1)
