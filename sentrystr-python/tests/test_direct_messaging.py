#!/usr/bin/env python3
"""
Focused test for direct messaging functionality
Equivalent to DirectMessageSender in Rust combined example
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

RELAYS = ["wss://relay.damus.io", "wss://nos.lol", "wss://nostr.chaima.info"]


def test_direct_messaging():
    """
    Test direct messaging functionality
    Python equivalent of DirectMessageSender in Rust
    """
    print("🔐 DIRECT MESSAGING TEST")
    print("=" * 60)

    # Generate keys for this session
    keys = generate_test_keys()
    sender_private_key = keys["private_key"]
    sender_public_key = keys["public_key_hex"]

    print(f"🔑 Generated sender key: {sender_public_key[:20]}...")
    print(f"🎯 Target recipient: {TARGET_NPUB}")
    print(f"📡 Using relays: {', '.join(RELAYS)}")
    print()

    try:
        # Create encrypted client (equivalent to DirectMessageSender)
        print("📝 Creating encrypted client (DirectMessage equivalent)")
        encrypted_config = sentrystr.Config(sender_private_key, RELAYS)
        encrypted_config.with_encryption(TARGET_PUBKEY)
        encrypted_client = sentrystr.NostrSentryClient(encrypted_config)
        print("✓ Created encrypted client")
        time.sleep(1)

        # Test 1: Direct message equivalent (standalone message)
        print("\n🔗 Test 1: Standalone Direct Message")
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S UTC")
        standalone_message = f"🔐 Direct Message from Python SentryStr at {timestamp}"

        encrypted_client.capture_message(standalone_message)
        print("✅ Sent standalone direct message")
        time.sleep(2)

        # Test 2: Error with direct messaging (equivalent to min_level filtering)
        print("\n🔗 Test 2: Error Event with Direct Messaging")
        error_event = sentrystr.Event()
        error_event.with_message("Critical system error - requires immediate attention")
        error_event.with_level(sentrystr.Level("error"))

        # Add context that would trigger a DM in Rust
        error_event.with_tag("priority", "critical")
        error_event.with_tag("alert", "immediate")
        error_event.with_tag("dm_trigger", "error_level")
        error_event.add_extra("error_code", "SYS_CRITICAL_001")
        error_event.add_extra("affected_systems", ["database", "api", "auth"])
        error_event.add_extra("escalation_required", True)
        error_event.add_extra("on_call_engineer", "alice@company.com")

        encrypted_client.capture_event(error_event)
        print("✅ Sent error event via direct message")
        time.sleep(2)

        # Test 3: Warning with direct messaging
        print("\n🔗 Test 3: Warning Event with Direct Messaging")
        warning_event = sentrystr.Event()
        warning_event.with_message("High memory usage detected - monitoring required")
        warning_event.with_level(sentrystr.Level("warning"))
        warning_event.with_tag("category", "performance")
        warning_event.with_tag("dm_trigger", "warning_level")
        warning_event.add_extra("memory_usage_percent", 87.5)
        warning_event.add_extra("threshold_percent", 80.0)
        warning_event.add_extra("trend", "increasing")

        encrypted_client.capture_event(warning_event)
        print("✅ Sent warning event via direct message")
        time.sleep(2)

        # Test 4: Fatal event with direct messaging
        print("\n🔗 Test 4: Fatal Event with Direct Messaging")
        fatal_event = sentrystr.Event()
        fatal_event.with_message("System shutdown imminent - all services affected")
        fatal_event.with_level(sentrystr.Level("fatal"))
        fatal_event.with_tag("severity", "fatal")
        fatal_event.with_tag("dm_trigger", "fatal_level")
        fatal_event.with_tag("immediate_action", "required")
        fatal_event.add_extra("shutdown_in_seconds", 60)
        fatal_event.add_extra("reason", "memory_exhaustion")
        fatal_event.add_extra("last_backup", "2025-09-22T20:00:00Z")
        fatal_event.add_extra("emergency_contact", "+1-555-ONCALL")

        encrypted_client.capture_event(fatal_event)
        print("✅ Sent fatal event via direct message")
        time.sleep(2)

        # Test 5: Business-critical alert
        print("\n🔗 Test 5: Business-Critical Alert")
        business_event = sentrystr.Event()
        business_event.with_message(
            "Payment processing system failure - revenue impact"
        )
        business_event.with_level(sentrystr.Level("error"))

        # Add user context for business impact
        user = sentrystr.User()
        user.with_id("system_monitor")
        user.with_email("alerts@company.com")
        user.with_username("payment_monitor")
        business_event.with_user(user)

        business_event.with_tag("business_impact", "high")
        business_event.with_tag("revenue_affected", "true")
        business_event.with_tag("dm_trigger", "business_critical")
        business_event.add_extra("failed_transactions", 1247)
        business_event.add_extra("estimated_loss_usd", 156789.50)
        business_event.add_extra("affected_customers", 890)
        business_event.add_extra("recovery_eta", "15 minutes")

        encrypted_client.capture_event(business_event)
        print("✅ Sent business-critical alert via direct message")

        print("\n" + "=" * 60)
        print("🎉 Direct messaging test completed successfully!")
        print()
        print("📊 Summary - Events sent via encrypted messaging:")
        print("✅ Standalone direct message")
        print("✅ Error event (critical system)")
        print("✅ Warning event (performance)")
        print("✅ Fatal event (system shutdown)")
        print("✅ Business-critical alert (payment failure)")
        print()
        print(f"📱 All messages sent to: {TARGET_NPUB}")
        print(f"🔑 From sender key: {sender_public_key[:20]}...")
        print("🔐 All events encrypted with NIP-44")
        print("⏰ Messages may take a moment to propagate")
        print()
        print("💡 This demonstrates the Python equivalent of:")
        print("   • Rust DirectMessageSender functionality")
        print("   • Level-based message filtering")
        print("   • Encrypted direct messaging")
        print("   • Rich context for critical alerts")

        return True

    except Exception as e:
        print(f"❌ Direct messaging test failed: {e}")
        import traceback

        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = test_direct_messaging()
    if success:
        print("\n✅ Direct messaging test completed successfully!")
    else:
        print("\n❌ Direct messaging test failed!")

    sys.exit(0 if success else 1)
