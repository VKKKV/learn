#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "线性同余方程", author: "Arch User")

= 线性同余方程

线性同余方程的标准形式是
$ a x equiv b pmod(n) $。
它和一次不定方程本质等价，是模逆元和 CRT 之前最重要的一类问题。

== 可解条件

#theorem[
  方程
  $ a x equiv b pmod(n) $ <linear-cong>
  有解，当且仅当
  $ upright("gcd")(a, n) divides b $ <linear-cong-solvable>
]

#proof[
  方程 @linear-cong 等价于
  $ a x - b = k n $
  即
  $ a x + n(-k) = b $
  这正是一次不定方程。
  所以它可解的充要条件与裴蜀定理给出的结论完全一致，也就是 @linear-cong-solvable。
]

== 解的个数

#proposition[
  若设
  $ d = upright("gcd")(a, n) $
  则一旦有解，模 $n$ 意义下恰好有 $d$ 个不同解。
]

#remark[
  当 $d = 1$ 时，解唯一；
  当 $d > 1$ 时，可以先把方程整体除以 $d$，转成一个模数更小、首项与模数互素的方程，再恢复全部解。
]

== 求解步骤

#proposition[
  解
  $ a x equiv b pmod(n) $
  的标准流程是：

  - 先算 $d = upright("gcd")(a, n)$。
  - 若 $d in.not b$，则无解。
  - 若 $d divides b$，则将方程除以 $d$。
  - 此时新的首项与模数互素，可以借助模逆元求出一个标准解。
]

== 例题

#example[
  求解
  $ 4 x equiv 2 pmod(6) $

  因为
  $ upright("gcd")(4, 6) = 2 $
  且 $2 divides 2$，所以有解。

  两边同除以 $2$，得到
  $ 2 x equiv 1 pmod(3) $

  在模 $3$ 下，$2$ 的逆元是 $2$，所以
  $ x equiv 2 pmod(3) $

  恢复到模 $6$，得到两个解：
  $ x equiv 2, 5 pmod(6) $
]

#example[
  例 1：求解
  $ 3 x equiv 4 pmod(7) $

  因为
  $ upright("gcd")(3, 7) = 1 $
  所以解唯一。

  在模 $7$ 下，$3$ 的逆元是 $5$，故
  $ x equiv 5 4 = 20 equiv 6 pmod(7) $
]

#example[
  例 2：求解
  $ 6 x equiv 9 pmod(15) $

  因为
  $ upright("gcd")(6, 15) = 3 $
  且 $3 divides 9$，所以有解。

  两边除以 $3$，得到
  $ 2 x equiv 3 pmod(5) $

  模 $5$ 下，$2^(-1) equiv 3$，因此
  $ x equiv 3 3 = 9 equiv 4 pmod(5) $

  恢复到模 $15$，得到三个解：
  $ x equiv 4, 9, 14 pmod(15) $
]

#example[
  例 3：判断
  $ 8 x equiv 5 pmod(14) $
  是否有解。

  因为
  $ upright("gcd")(8, 14) = 2 $
  但 $2 in.not 5$，所以无解。
]

#example[
  例 4：求解
  $ 10 x equiv 20 pmod(35) $

  因为
  $ upright("gcd")(10, 35) = 5 $
  且 $5 divides 20$，所以有解。

  两边除以 $5$，得
  $ 2 x equiv 4 pmod(7) $
  模 $7$ 下 $2^(-1) equiv 4$，故
  $ x equiv 4 4 = 16 equiv 2 pmod(7) $

  恢复到模 $35$，有 $5$ 个解：
  $ x equiv 2, 9, 16, 23, 30 pmod(35) $
]

== 与模逆元的关系

#remark[
  当
  $ upright("gcd")(a, n) = 1 $
  时，线性同余方程就是“乘一个逆元”就能解的最简单情形。
  所以模逆元可以视为线性同余方程的特殊情况。
]

#exercise[
  求解
  $ 6 x equiv 9 pmod(15) $
]
