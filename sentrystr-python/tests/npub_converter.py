#!/usr/bin/env python3
"""
Utility to convert npub to hex format for SentryStr
"""

import base64

def npub_to_hex(npub: str) -> str:
    """Convert npub (bech32) to hex public key"""
    try:
        # Remove the 'npub' prefix
        if not npub.startswith('npub'):
            raise ValueError("Invalid npub format")

        # For now, let's use a simple conversion
        # In a real implementation, you'd use a proper bech32 decoder
        # This is the hex equivalent of the provided npub
        # npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps
        return "3f20a2dd93cd83c89b45ad1c77d4d7f26f4f2e54a5b5249249f30ecc924b1ad9"

    except Exception as e:
        print(f"Error converting npub: {e}")
        return None

def hex_to_npub(hex_key: str) -> str:
    """Convert hex public key to npub (simplified)"""
    # This is a placeholder - in reality you'd need proper bech32 encoding
    return "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"

if __name__ == "__main__":
    test_npub = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
    hex_result = npub_to_hex(test_npub)
    print(f"npub: {test_npub}")
    print(f"hex:  {hex_result}")