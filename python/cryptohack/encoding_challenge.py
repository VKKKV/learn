import base64
import codecs
import json

from Crypto.Util.number import long_to_bytes
from pwn import log, remote


def base64_decode(encoded: str) -> str:
    """Decodes a base64 encoded string."""
    return base64.b64decode(encoded).decode()


def rot13_decode(encoded: str) -> str:
    """Decodes a ROT13 encoded string."""
    return codecs.decode(encoded, "rot13")


def hex_decode(encoded: str) -> str:
    """Decodes a hex-encoded string."""
    return bytes.fromhex(encoded).decode()


def bigint_decode(encoded: str) -> str:
    """Decodes a big integer represented as a hex string (e.g., '0x...')."""
    # Convert hex string to integer, then to bytes, then decode to string
    return long_to_bytes(int(encoded, 16)).decode()


def utf8_decode(encoded: list) -> str:
    """Decodes a list of UTF-8 byte values."""
    return bytes(encoded).decode()


def decode_data(data_type: str, encoded_value) -> str:
    """Routes the encoded value to the appropriate decoder based on type."""
    match data_type:
        case "base64":
            return base64_decode(encoded_value)
        case "rot13":
            return rot13_decode(encoded_value)
        case "hex":
            return hex_decode(encoded_value)
        case "bigint":
            return bigint_decode(encoded_value)
        case "utf-8":
            return utf8_decode(encoded_value)
        case _:
            log.error(f"Unknown encoding type: {data_type}")
            return ""


def solve_challenge():
    """Connects to the challenge server and solves the encoding levels."""
    host, port = "socket.cryptohack.org", 13377
    r = remote(host, port, level="debug")

    def json_recv():
        return json.loads(r.recvline().decode())

    def json_send(data):
        r.sendline(json.dumps(data).encode())

    try:
        while True:
            received = json_recv()

            # The flag is provided in the 'crypto' field once all levels are completed
            if "crypto" in received:
                log.success(f"Flag found: {received['crypto']}")
                break

            log.info(f"Decoding {received['type']}: {received['encoded']}")

            decoded_val = decode_data(received["type"], received["encoded"])
            json_send({"decoded": decoded_val})

    except EOFError:
        log.warning("Connection closed by remote host.")
    except Exception as e:
        log.error(f"An error occurred: {e}")
    finally:
        r.close()


if __name__ == "__main__":
    solve_challenge()
