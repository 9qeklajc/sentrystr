#!/usr/bin/env python3
"""
Debug test to investigate why events aren't being received
"""

import os
import sys
import time
from datetime import datetime

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

# Configuration
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()  # Use npub directly

# Generate keys
keys = generate_test_keys()
sender_private_key = keys["private_key"]
sender_public_key = keys["public_key_hex"]

# Test with fewer relays for better debugging
TEST_RELAYS = ["wss://relay.damus.io", "wss://nos.lol"]


def debug_public_event():
    """Send a simple public event"""
    print("\n🔍 DEBUG: Testing public event...")

    try:
        config = sentrystr.Config(sender_private_key, TEST_RELAYS)
        client = sentrystr.NostrSentryClient(config)

        timestamp = datetime.now().isoformat()
        message = f"🌍 DEBUG: Public test message at {timestamp} from key {sender_public_key[:16]}..."

        print(f"Sending public message: {message}")
        client.capture_message(message)
        print("✅ Public message sent successfully")

        return True
    except Exception as e:
        print(f"❌ Public event failed: {e}")
        import traceback

        traceback.print_exc()
        return False


def debug_encrypted_event():
    """Send a simple encrypted event to target npub"""
    print("\n🔍 DEBUG: Testing encrypted event...")

    try:
        config = sentrystr.Config(sender_private_key, TEST_RELAYS)
        print(f"Setting encryption target: {TARGET_PUBKEY}")
        config.with_encryption(TARGET_PUBKEY)
        client = sentrystr.NostrSentryClient(config)

        timestamp = datetime.now().isoformat()
        message = f"🔐 DEBUG: Encrypted test message at {timestamp} for {TARGET_NPUB}"

        print(f"Sending encrypted message: {message}")
        client.capture_message(message)
        print("✅ Encrypted message sent successfully")

        return True
    except Exception as e:
        print(f"❌ Encrypted event failed: {e}")
        import traceback

        traceback.print_exc()
        return False


def debug_with_original_key():
    """Test with the original hardcoded key to see if dynamic keys are the issue"""
    print("\n🔍 DEBUG: Testing with original hardcoded key...")

    # Original key from the tests
    original_key = "nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99"

    try:
        config = sentrystr.Config(original_key, TEST_RELAYS)
        config.with_encryption(TARGET_PUBKEY)
        client = sentrystr.NostrSentryClient(config)

        timestamp = datetime.now().isoformat()
        message = f"🔐 DEBUG: Message from original hardcoded key at {timestamp}"

        print(f"Sending message with original key: {message}")
        client.capture_message(message)
        print("✅ Original key message sent successfully")

        return True
    except Exception as e:
        print(f"❌ Original key test failed: {e}")
        import traceback

        traceback.print_exc()
        return False


def main():
    print("🚀 DEBUG: Event Reception Investigation")
    print("=" * 60)
    print(f"🎯 Target npub: {TARGET_NPUB}")
    print(f"🎯 Target pubkey (hex): {TARGET_PUBKEY}")
    print(f"🆔 Generated sender pubkey: {sender_public_key}")
    print(f"🔑 Generated sender private key: {sender_private_key}")
    print(f"📡 Using relays: {', '.join(TEST_RELAYS)}")
    print("=" * 60)

    # Test 1: Public event
    success1 = debug_public_event()
    time.sleep(3)

    # Test 2: Encrypted event
    success2 = debug_encrypted_event()
    time.sleep(3)

    # Test 3: Original key
    success3 = debug_with_original_key()

    print("\n" + "=" * 60)
    print("📊 DEBUG RESULTS:")
    print(f"Public event: {'✅ SUCCESS' if success1 else '❌ FAILED'}")
    print(f"Encrypted event: {'✅ SUCCESS' if success2 else '❌ FAILED'}")
    print(f"Original key test: {'✅ SUCCESS' if success3 else '❌ FAILED'}")

    if all([success1, success2, success3]):
        print("\n🎉 All debug tests passed!")
        print("📱 Events should now be visible in your Nostr client")
        print("⏰ Wait a few moments for relay propagation")
        print("\n💡 Possible reasons for not seeing events:")
        print("   1. Relay propagation delay (wait 1-2 minutes)")
        print("   2. Client not monitoring the correct npub")
        print("   3. Encrypted events require proper decryption")
        print("   4. Some relays might be down or slow")
    else:
        print("\n❌ Some tests failed - check errors above")


if __name__ == "__main__":
    main()
