#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "费马小定理与欧拉定理", author: "Arch User")

= 费马小定理与欧拉定理

这两个定理描述了模幂运算中的周期现象，是快速幂降幂、模逆元推导和密码学的核心工具。

== 费马小定理

#theorem[
  若 $p$ 为素数且 $upright("gcd")(a, p) = 1$，则
  $ a^(p - 1) equiv 1 pmod(p) $ <flt>
]

#corollary[
  对任意整数 $a$，都有
  $ a^p equiv a pmod(p) $
]

#remark[
  第一种形式适合在 $a$ 与 $p$ 互素时降幂；第二种形式常用于整式恒等式与计数题。
]

== 欧拉定理

#theorem[
  若 $upright("gcd")(a, n) = 1$，则
  $ a^(phi(n)) equiv 1 pmod(n) $ <euler-theorem>
]

#proof[
  设 $r_1, r_2, dots.c, r_(phi(n))$ 是模 $n$ 的一个简化剩余系。
  因为 $a$ 与 $n$ 互素，所以
  $ a r_1, a r_2, dots.c, a r_(phi(n)) $
  模 $n$ 之后仍然构成同一个简化剩余系。

  因而它们的乘积同余：
  $ a^(phi(n)) r_1 r_2 dots.c r_(phi(n)) equiv r_1 r_2 dots.c r_(phi(n)) pmod(n) $
  由于右边乘积与 $n$ 互素，可以消去，从而得到 @euler-theorem。
]

== 两者关系

#remark[
  欧拉定理是费马小定理的推广。
  当 $n = p$ 为素数时，
  $ phi(p) = p - 1 $
  所以 @euler-theorem 就退化为 @flt。
]

== 应用

#proposition[
  若要计算
  $ a^k mod n $
  且已知 $upright("gcd")(a, n) = 1$，则可先把指数 $k$ 按 $phi(n)$ 或更小的周期做缩减。
]

#example[
  计算
  $ 3^100 mod 7 $

  因为 $phi(7) = 6$，所以
  $ 3^100 = 3^(96 + 4) equiv (3^6)^16 3^4 equiv 1^16 3^4 pmod(7) $

  而
  $ 3^2 = 9 equiv 2 pmod(7) $
  所以
  $ 3^4 equiv 4 pmod(7) $

  最终得到
  $ 3^100 equiv 4 pmod(7) $
]

#exercise[
  计算
  $ 2^100 mod 15 $
]
