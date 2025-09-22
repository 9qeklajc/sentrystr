#!/usr/bin/env python3

import hashlib
import secrets


def generate_test_keys():
    while True:
        private_key_bytes = secrets.token_bytes(32)
        if private_key_bytes != b"\x00" * 32:
            break

    private_key_hex = private_key_bytes.hex()

    return {
        "private_key": private_key_hex,
        "private_key_hex": private_key_hex,
        "public_key_hex": hashlib.sha256(private_key_bytes).hexdigest(),
    }


def get_target_pubkey():
    return "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"


def get_target_pubkey_hex():
    return "3f20a2dd93cd83c89b45ad1c77d4d7f26f4f2e54a5b5249249f30ecc924b1ad9"


if __name__ == "__main__":
    keys = generate_test_keys()
    print(f"Private Key: {keys['private_key']}")
    print(f"Public Key: {keys['public_key_hex']}")
    print(f"Target: {get_target_pubkey()}")
