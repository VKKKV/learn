#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "中国剩余定理", author: "Arch User")

= 中国剩余定理

中国剩余定理研究多个模条件如何合并成一个模条件。
它在理论上说明“互素模数下的信息可以独立编码”，在算法上则允许把大模数问题拆成若干小模数问题。

== 基本结论

#theorem[
  若 $n_1, n_2, dots.c, n_k$ 两两互素，则同余方程组
  $ x equiv a_i pmod(n_i) $ <crt-system>
  有唯一的模
  $ N = n_1 n_2 dots.c n_k $ <crt-modulus>
  的解。
]

#remark[
  “唯一”不是说只有一个整数解，而是说所有解模 $N$ 同余。
  也就是说，全部解构成一个同余类。
]

== 两个模数的情形

#proposition[
  若
  $ upright("gcd")(m, n) = 1 $
  则方程组
  $ x equiv a pmod(m) $
  $ x equiv b pmod(n) $
  有唯一的模 $m n$ 的解。
]

#proof[
  设
  $ M = m n $
  再设
  $ M_1 = n,
     M_2 = m $

  因为 $upright("gcd")(M_1, m) = 1$，$upright("gcd")(M_2, n) = 1$，所以它们分别在对应模数下存在逆元。
  于是可以构造出只在某个模数下取 $1$、在另一个模数下取 $0$ 的“基底元”，再线性组合得到解。
]

== 构造公式

#proposition[
  设
  $ N = n_1 n_2 dots.c n_k $
  且
  $ N_i = N / n_i $
  取 $e_i$ 满足
  $ N_i e_i equiv 1 pmod(n_i) $
  则方程组 @crt-system 的一个解可写为
  $ x equiv sum_(i=1)^k a_i N_i e_i pmod(N) $
]

== 例题

#example[
  解方程组
  $ x equiv 2 pmod(3) $
  $ x equiv 3 pmod(5) $

  设
  $ N = 15,
     N_1 = 5,
     N_2 = 3 $

  在模 $3$ 下，$5 equiv 2$，其逆元是 $2$；
  在模 $5$ 下，$3$ 的逆元也是 $2$。

  因而
  $ x equiv 2 5 2 + 3 3 2 = 38 equiv 8 pmod(15) $
]

#example[
  例 1：解方程组
  $ x equiv 1 pmod(4) $
  $ x equiv 2 pmod(5) $

  先枚举第一式的解：
  $ 1, 5, 9, 13, dots.c $
  其中
  $ 17 equiv 2 pmod(5) $
  所以
  $ x equiv 17 pmod(20) $
]

#example[
  例 2：解方程组
  $ x equiv 2 pmod(3) $
  $ x equiv 2 pmod(5) $

  两个模条件右侧相同，所以显然
  $ x equiv 2 pmod(15) $
]

#example[
  例 3：解方程组
  $ x equiv 1 pmod(3) $
  $ x equiv 2 pmod(4) $
  $ x equiv 3 pmod(5) $

  先合并前两式。
  满足第一式的数是
  $ 1, 4, 7, 10, dots.c $
  其中
  $ 10 equiv 2 pmod(4) $
  所以前两式合并为
  $ x equiv 10 pmod(12) $

  再与第三式联立。
  序列
  $ 10, 22, 34, 46, dots.c $
  中有
  $ 58 equiv 3 pmod(5) $
  所以最终解为
  $ x equiv 58 pmod(60) $
]

#example[
  例 4：判断方程组
  $ x equiv 1 pmod(4) $
  $ x equiv 3 pmod(6) $
  是否有解。

  因为
  $ upright("gcd")(4, 6) = 2 $
  且
  $ 1 equiv 3 pmod(2) $
  成立，所以有解。

  实际上枚举可得
  $ x = 9 $
  是一组解，因此解集为
  $ x equiv 9 pmod(12) $
]

== 不互素时

#remark[
  若模数不互素，CRT 仍然可以讨论，但要额外检查兼容性条件。
  例如方程组
  $ x equiv a pmod(m) $
  $ x equiv b pmod(n) $
  有解，当且仅当
  $ a equiv b pmod(upright("gcd")(m, n)) $
]

#exercise[
  解方程组
  $ x equiv 1 pmod(4) $
  $ x equiv 2 pmod(5) $
  $ x equiv 3 pmod(7) $
]
