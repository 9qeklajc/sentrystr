#!/usr/bin/env python3

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python"))

try:
    import sentrystr
    from key_generator import generate_test_keys
except ImportError as e:
    print(f"❌ Failed to import: {e}")
    sys.exit(1)

TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
RELAYS = ["wss://relay.damus.io", "wss://nos.lol", "wss://nostr.chaima.info"]


def run_combined_example():
    keys = generate_test_keys()
    sender_private_key = keys["private_key"]

    try:
        config = sentrystr.Config(sender_private_key, RELAYS)
        client = sentrystr.NostrSentryClient(config)
        client.setup_direct_messaging(TARGET_NPUB)

        info_event = sentrystr.Event()
        info_event.with_message("Application started")
        info_event.with_level(sentrystr.Level("info"))
        client.capture_event(info_event)

        warning_event = sentrystr.Event()
        warning_event.with_message("High memory usage")
        warning_event.with_level(sentrystr.Level("warning"))
        client.capture_event(warning_event)

        client.send_direct_message("System maintenance required")

        error_event = sentrystr.Event()
        error_event.with_message("hello from python")
        error_event.with_level(sentrystr.Level("error"))
        client.capture_event(error_event)

        print("✅ Combined example completed")
        return True

    except Exception as e:
        print(f"❌ Failed: {e}")
        return False


if __name__ == "__main__":
    success = run_combined_example()
    sys.exit(0 if success else 1)
