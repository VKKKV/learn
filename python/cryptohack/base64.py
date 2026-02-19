import base64

h = 0x72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf

b = bytes.fromhex(hex(h)[2:])

result = base64.b64encode(b).decode()

print(result)
