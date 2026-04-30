#import "../template.typ": *
#show: note-template

#set document(title: "卢卡斯定理", author: "Arch User")

= 卢卡斯定理

卢卡斯定理用于计算大组合数模素数。
核心思想是把 $n$ 和 $m$ 写成 $p$ 进制，从而把一个大组合数拆成若干个小组合数。

== 定理陈述

#theorem[
  若 $p$ 为素数，且
  $ n = n_k dots.c n_1 n_0 $,
  $ m = m_k dots.c m_1 m_0 $
  是它们的 $p$ 进制表示，则
  $ binom(n, m) equiv binom(n_0, m_0) binom(n_1, m_1) dots.c binom(n_k, m_k) pmod(p) $ <lucas>
]

#remark[
  所以问题会被拆成若干个“小组合数模 $p$”的乘积。
  只要能高效求出
  $ binom(a, b) mod p $
  对于 $0 <= a, b < p$ 的值，就能递归解决大范围情形。
]

== 直接推论

#corollary[
  若某一位满足
  $ m_i > n_i $
  则
  $ binom(n, m) equiv 0 pmod(p) $
]

#remark[
  这条推论非常好用，因为它意味着只要在 $p$ 进制下某一位“选太多了”，组合数模 $p$ 就直接为零。
]

== 例题

#example[
  计算
  $ binom(10, 3) mod 2 $

  将它们写成二进制：
  $ 10 = 1010_2,
     3 = 0011_2 $

  所以由 @lucas，
  $ binom(10, 3) equiv binom(0, 1) binom(1, 1) binom(0, 0) binom(1, 0) pmod(2) $

  由于
  $ binom(0, 1) = 0 $
  因而结果直接为
  $ 0 pmod(2) $
]

#example[
  计算
  $ binom(8, 2) mod 3 $

  因为
  $ 8 = 22_3,
     2 = 02_3 $
  所以
  $ binom(8, 2) equiv binom(2, 2) binom(2, 0) equiv 1 pmod(3) $
]

== 适用范围

#remark[
  卢卡斯定理原始形式只适用于“模素数”。
  若模数是素数幂或一般合数，则通常需要 Lucas 扩展、CRT 和更复杂的组合数取模算法。
]

#exercise[
  计算
  $ binom(100, 20) mod 5 $
]
