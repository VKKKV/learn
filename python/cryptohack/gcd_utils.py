def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

def extended_gcd(q, p):
    if q == 0:
        return p, 0, 1
    else:
        g, u, v = extended_gcd(p % q, q)
        return g, v - (p // q) * u, u

