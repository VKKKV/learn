#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "欧拉函数", author: "Arch User")

= 欧拉函数

欧拉函数 $phi(n)$ 统计 $1$ 到 $n$ 中与 $n$ 互素的整数个数。
它刻画了模 $n$ 乘法群的大小，也是欧拉定理、原根理论、积性函数和筛法中的基础对象。

== 定义

#definition[
  $phi(n)$ 定义为满足
  $1 <= k <= n$
  且
  $upright("gcd")(k, n) = 1$
  的整数 $k$ 的个数。
]

#example[
  在 $1$ 到 $12$ 中，与 $12$ 互素的数是
  $1, 5, 7, 11$
  因而
  $ phi(12) = 4 $
]

== 素数幂情形

#theorem[
  若 $p$ 是素数，$k >= 1$，则
  $ phi(p^k) = p^k - p^(k - 1) = p^k (1 - 1/p) $ <phi-prime-power>
]

#proof[
  在 $1$ 到 $p^k$ 中，不能与 $p^k$ 互素的整数恰好是 $p$ 的倍数。
  这样的数共有
  $ p^(k - 1) $
  个，所以剩余可与 $p^k$ 互素的数有
  $ p^k - p^(k - 1) $
  个。
]

== 一般公式

#theorem[
  若
  $ n = p_1^(a_1) p_2^(a_2) dots.c p_k^(a_k) $
  是 $n$ 的质因数分解，则
  $ phi(n) = n (1 - 1/p_1) (1 - 1/p_2) dots.c (1 - 1/p_k) $ <phi-formula>
]

#proof[
  对每个素因子 $p_i$，从 $1$ 到 $n$ 中有 $1 / p_i$ 的数会被它整除。
  只有那些不被任何一个 $p_i$ 整除的数才与 $n$ 互素。
  对这些素因子逐层筛除后，就得到 @phi-formula。

  也可以从素数幂公式 @phi-prime-power 和互素乘法性出发推出该式。
]

== 积性

#proposition[
  若
  $ upright("gcd")(m, n) = 1 $
  则
  $ phi(m n) = phi(m) phi(n) $ <phi-multiplicative>
]

#remark[
  这说明欧拉函数是积性函数。
  一旦知道素数幂处的值，再结合 @phi-multiplicative，就能求任意正整数的值。
]

== 例题

#example[
  计算
  $ phi(36) $

  因为
  $ 36 = 2^2 3^2 $
  所以
  $ phi(36) = 36 (1 - 1/2)(1 - 1/3) = 36 1/2 2/3 = 12 $
]

#example[
  计算
  $ phi(1000) $

  因为
  $ 1000 = 2^3 5^3 $
  所以
  $ phi(1000) = 1000 (1 - 1/2)(1 - 1/5) = 400 $
]

== 与欧拉定理的关系

#remark[
  欧拉函数的最重要应用之一是欧拉定理：
  若
  $ upright("gcd")(a, n) = 1 $
  则
  $ a^(phi(n)) equiv 1 pmod(n) $

  因而在模幂问题中，$phi(n)$ 往往给出一个天然周期上界。
]

#exercise[
  计算
  $ phi(84) $
]

#exercise[
  证明：若 $p$ 为素数，则
  $ phi(p) = p - 1 $
]
