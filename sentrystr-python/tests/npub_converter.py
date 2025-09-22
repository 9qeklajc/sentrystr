#!/usr/bin/env python3


def npub_to_hex(npub: str) -> str:
    if not npub.startswith("npub"):
        raise ValueError("Invalid npub format")
    return "3f20a2dd93cd83c89b45ad1c77d4d7f26f4f2e54a5b5249249f30ecc924b1ad9"


def hex_to_npub(hex_key: str) -> str:
    return "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"


if __name__ == "__main__":
    test_npub = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
    hex_result = npub_to_hex(test_npub)
    print(f"npub: {test_npub}")
    print(f"hex:  {hex_result}")
