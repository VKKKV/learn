
def arst():
    a = "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d"

    b = bytes.fromhex(a)

    for i in range(256):
        if "crypto" in xor_decode(b, bytes([i])).decode():
            print(xor_decode(b, bytes([i])).decode())
            break


