#!/usr/bin/env sage

# TODO
# 1. 将原方程映射为 Weierstrass 标准形的椭圆曲线: Y^2 = X^3 + 109X + 224
E = EllipticCurve([0, 0, 0, 109, 224])
print(f"[*] Base Curve Loaded: {E}")

# 2. 自动获取曲线的有理点生成元 (Generator)
# 在 Arch 里我们不硬编码猜测，让底层算法 (2-descent) 找出来
gens = E.gens()
P = gens[0]
print(f"[*] Found Generator Point P: {P}")

# 3. 核心计算：寻找正整数解需要计算 9P
print("[*] Computing 9*P... (Prepare for exponential integer growth)")
P9 = 9 * P
X = P9[0]
Y = P9[1]

# 4. 逆映射 (Inverse Birational Map)
# 将椭圆曲线上的 (X, Y) 映射回原始方程的 (x, y, z) 齐次坐标
# 这是通过代数几何推导出的逆映射公式
x_frac = 56 - X + Y
y_frac = 56 - X - Y
z_frac = -28 + X

# 5. 提取纯净的整数解
# 因为它们共享相同的分母，我们只需要提取它们的分子并消除负号
# 真正的系统级控制：精确控制每一个位，丢弃浮点数带来的 bloat
lcm_denom = lcm([x_frac.denominator(), y_frac.denominator(), z_frac.denominator()])

x_int = abs(x_frac.numerator() * (lcm_denom / x_frac.denominator()))
y_int = abs(y_frac.numerator() * (lcm_denom / y_frac.denominator()))
z_int = abs(z_frac.numerator() * (lcm_denom / z_frac.denominator()))

print("\n[+] Calculation Complete. Payload generated:")
print(f"x = {x_int}")
print(f"y = {y_int}")
print(f"z = {z_int}")
