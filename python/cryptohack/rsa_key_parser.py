from Crypto.PublicKey import RSA

"""
:Keywords:
  n : integer
    The modulus.
  e : integer
    The public exponent.
  d : integer
    The private exponent. Only required for private keys.
  p : integer
    The first factor of the modulus. Only required for private keys.
  q : integer
    The second factor of the modulus. Only required for private keys.
  u : integer
    The CRT coefficient (inverse of p modulo q). Only required for
    private keys.
"""


with open("./privacy_enhanced_mail.pem", "rb") as f:
    flag = RSA.import_key(f.read())

print(flag.d)

with open("./2048b-rsa-example-cert.der", "rb") as f:
    flag = RSA.import_key(f.read())

print(flag.n)

with open("./bruce_rsa.pub", "rb") as f:
    flag = RSA.import_key(f.read())

print(flag.n)


