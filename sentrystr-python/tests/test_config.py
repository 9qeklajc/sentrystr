"""
Configuration for SentryStr Python binding tests
"""

# Target npub for receiving test events
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"

# Test private key (generated for testing purposes - safe to use publicly)
# This corresponds to npub: npub1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qg4jsaxr
TEST_PRIVATE_KEY = "nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99"

# Reliable Nostr relays for testing
TEST_RELAYS = [
    "wss://relay.damus.io",
    "wss://nos.lol",
    "wss://relay.snort.social",
    "wss://nostr.wine",
    "wss://relay.nostr.band",
]

# Test scenarios configuration
TEST_SCENARIOS = {
    "basic_errors": {"enabled": True, "count": 3, "delay": 1.0},
    "detailed_events": {"enabled": True, "count": 2, "delay": 2.0},
    "exception_types": {
        "enabled": True,
        "types": [
            "ValueError",
            "ConnectionError",
            "TimeoutError",
            "PermissionError",
            "RuntimeError",
        ],
    },
    "log_levels": {
        "enabled": True,
        "levels": ["debug", "info", "warning", "error", "fatal"],
    },
    "performance": {"enabled": True, "simulate_slow_operations": True},
}
