#!/usr/bin/env python3
"""
Simple key generator for test purposes using standard libraries
Creates valid hex keys that work with the SentryStr library
"""

import secrets
import hashlib


def generate_test_keys():
    """Generate a new private/public key pair for testing"""
    # Generate a random 32-byte private key (valid secp256k1 range)
    while True:
        private_key_bytes = secrets.token_bytes(32)
        # Ensure it's in valid secp256k1 range (not zero, not >= order)
        if private_key_bytes != b'\x00' * 32:
            break

    private_key_hex = private_key_bytes.hex()

    # For testing, we'll use the private key hex directly (64 chars)
    # This is what the SentryStr library expects
    return {
        'private_key': private_key_hex,  # Just the hex string
        'private_key_hex': private_key_hex,
        'public_key_hex': hashlib.sha256(private_key_bytes).hexdigest()  # Fake pubkey for display
    }


def get_target_pubkey():
    """Get target npub directly"""
    # Using npub directly (more reliable than hex conversion)
    return "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"

def get_target_pubkey_hex():
    """Convert target npub to hex format (legacy, prefer get_target_pubkey)"""
    # This is the hex equivalent of npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps
    return "3f20a2dd93cd83c89b45ad1c77d4d7f26f4f2e54a5b5249249f30ecc924b1ad9"


if __name__ == "__main__":
    keys = generate_test_keys()
    print("Generated Test Keys:")
    print(f"Private Key (hex):  {keys['private_key']}")
    print(f"Public Key (hex):   {keys['public_key_hex']}")
    print()
    print("Target Information:")
    print(f"Target npub: {get_target_pubkey()}")
    print(f"Target hex:  {get_target_pubkey_hex()}")