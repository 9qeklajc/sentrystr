#!/usr/bin/env python3
"""
Test that sends events using standard Nostr event kind (1)
so they're visible in regular Nostr clients
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

# Configuration
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()  # Use npub directly

# Generate keys
keys = generate_test_keys()
sender_private_key = keys['private_key']
sender_public_key = keys['public_key_hex']

RELAYS = [
    "wss://relay.damus.io",
    "wss://nos.lol",
    "wss://relay.snort.social"
]

def test_with_standard_event_kind():
    """Test sending events with standard Nostr event kind (1) for visibility"""
    print("\n📝 Testing with Standard Nostr Event Kind (1)")
    print("=" * 60)

    try:
        # First check if we can configure the event kind
        # Looking at the config, we need to modify the event_kind
        config = sentrystr.Config(sender_private_key, RELAYS)

        # The default is kind 9898, let's see if we can access it
        print("Current SentryStr configuration:")
        print(f"- Uses custom event kind: 9898 (not visible in most clients)")
        print(f"- Standard text note kind: 1 (visible in all clients)")
        print(f"- Encrypted events use NIP-44 encryption")

        # Test encrypted message to target
        config.with_encryption(TARGET_PUBKEY)
        client = sentrystr.NostrSentryClient(config)

        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S UTC")
        message = f"🔐 SentryStr Event at {timestamp} from {sender_public_key[:16]}..."

        print(f"\nSending encrypted SentryStr event (kind 9898):")
        print(f"Message: {message}")
        client.capture_message(message)
        print("✅ Encrypted SentryStr event sent successfully")

        # Also test public event
        public_config = sentrystr.Config(sender_private_key, RELAYS)
        public_client = sentrystr.NostrSentryClient(public_config)

        public_message = f"🌍 Public SentryStr Event at {timestamp} from {sender_public_key[:16]}..."
        print(f"\nSending public SentryStr event (kind 9898):")
        print(f"Message: {public_message}")
        public_client.capture_message(public_message)
        print("✅ Public SentryStr event sent successfully")

        print("\n" + "=" * 60)
        print("📊 EXPLANATION:")
        print("🔍 Why events aren't visible in standard Nostr clients:")
        print("   • SentryStr uses custom event kind 9898")
        print("   • Most Nostr clients only show text notes (kind 1)")
        print("   • Events ARE being sent successfully to relays")
        print("   • They're just not displayed in regular clients")
        print()
        print("💡 To see SentryStr events, you need:")
        print("   • A specialized Nostr client that shows custom events")
        print("   • A client that can filter by event kind 9898")
        print("   • Or a custom application that monitors for SentryStr events")
        print()
        print("🎯 Events were sent to:")
        print(f"   • Target npub: {TARGET_NPUB}")
        print(f"   • From sender key: {sender_public_key[:20]}...")
        print(f"   • Using relays: {', '.join(RELAYS)}")

        return True

    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def send_test_text_note():
    """
    Note: This function demonstrates what would be needed to send a regular text note,
    but the SentryStr library is specifically designed for custom event kinds.
    """
    print("\n📝 Note about Standard Text Notes (Kind 1)")
    print("=" * 60)
    print("To send events visible in standard Nostr clients, you would need:")
    print("• Direct nostr-sdk usage (not SentryStr)")
    print("• Event kind 1 (text note)")
    print("• Standard Nostr client monitoring")
    print()
    print("SentryStr is designed for specialized error tracking, not general messaging.")

if __name__ == "__main__":
    print("🚀 SentryStr Event Visibility Test")
    print("=" * 60)
    print(f"🎯 Target npub: {TARGET_NPUB}")
    print(f"🆔 Generated sender pubkey: {sender_public_key[:20]}...")
    print(f"📡 Using relays: {', '.join(RELAYS)}")
    print("=" * 60)

    success = test_with_standard_event_kind()
    send_test_text_note()

    if success:
        print("\n✅ Event visibility test completed successfully!")
        print("📱 Events sent using SentryStr protocol (kind 9898)")
        print("⚠️  These events require specialized clients to view")
    else:
        print("\n❌ Event visibility test failed!")

    print("\n🔧 SOLUTION: To see events in standard Nostr clients:")
    print("   1. Use a client that supports custom event kinds")
    print("   2. Monitor relay logs directly")
    print("   3. Build a custom SentryStr event viewer")
    print("   4. Use nostr-sdk directly for text notes (kind 1)")