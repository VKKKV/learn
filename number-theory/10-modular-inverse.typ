#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "模逆元", author: "vkkkv")

= 模逆元

模逆元让我们在模意义下做“除法”。
它在线性同余方程、组合数取模、中国剩余定理和很多算法实现中都会反复出现。

== 定义

#definition[
  若
  $ a x equiv 1 pmod(n) $
  则称 $x$ 是 $a$ 在模 $n$ 下的逆元，记作
  $ a^(-1) pmod(n) $
]

#remark[
  逆元不是普通实数意义上的倒数，而是“乘起来在模 $n$ 下等于 $1$”的元素。
]

== 存在条件

#theorem[
  $a$ 在模 $n$ 下存在逆元，当且仅当
  $ upright("gcd")(a, n) = 1 $ <inverse-exists>
]

#proof[
  若逆元存在，即存在 $x$ 使得
  $ a x equiv 1 pmod(n) $
  则
  $ a x - 1 = k n $
  也就是
  $ a x + n(-k) = 1 $
  因而 $upright("gcd")(a, n)$ 必须整除 $1$，所以它只能等于 $1$。

  反过来，若
  $ upright("gcd")(a, n) = 1 $
  则由裴蜀定理，存在整数 $x, y$ 使得
  $ a x + n y = 1 $
  两边模 $n$ 就得到
  $ a x equiv 1 pmod(n) $
  所以逆元存在。
]

== 求法

#proposition[
  求逆元的常见方法有：

  - 扩展欧几里得算法：适用于一般互素模数。
  - 费马小定理：若模数是素数 $p$，则
    $ a^(-1) equiv a^(p - 2) pmod(p) $
  - 递推法：当需要批量求 $1$ 到 $n$ 的逆元时可线性预处理。
]

== 例题

#example[
  在模 $7$ 下，$3$ 的逆元是 $5$，因为
  $ 3 5 = 15 equiv 1 pmod(7) $
]

#example[
  求 $8$ 在模 $15$ 下的逆元。

  由于
  $ upright("gcd")(8, 15) = 1 $
  所以逆元存在。

  试算可得
  $ 8 2 = 16 equiv 1 pmod(15) $
  因而
  $ 8^(-1) equiv 2 pmod(15) $
]

#example[
  例 1：求 $11$ 在模 $26$ 下的逆元。

  先算
  $ upright("gcd")(11, 26) = 1 $
  所以逆元存在。

  用试算可得
  $ 11 19 = 209 
     equiv 1 pmod(26) $
  因而
  $ 11^(-1) equiv 19 pmod(26) $
]

#example[
  例 2：求 $7$ 在模 $17$ 下的逆元。

  因为 $17$ 是素数，且 $7 in.not 17$，可用费马小定理：
  $ 7^(-1) equiv 7^(15) pmod(17) $

  但直接试算更快：
  $ 7 5 = 35 equiv 1 pmod(17) $
  所以
  $ 7^(-1) equiv 5 pmod(17) $
]

#example[
  例 3：判断 $6$ 在模 $15$ 下是否有逆元。

  计算
  $ upright("gcd")(6, 15) = 3 != 1 $
  因而由 @inverse-exists，$6$ 在模 $15$ 下没有逆元。

  这意味着方程
  $ 6 x equiv 1 pmod(15) $
  无解。
]

#example[
  例 4：利用逆元计算
  $ 3 / 5 pmod(7) $

  在模 $7$ 下，$5$ 的逆元是 $3$，因为
  $ 5 3 = 15 equiv 1 pmod(7) $

  因而
  $ 3 / 5 equiv 3 5^(-1) equiv 3 3 = 9 equiv 2 pmod(7) $
]

== 与线性同余方程的关系

#remark[
  当
  $ upright("gcd")(a, n) = 1 $
  时，方程
  $ a x equiv b pmod(n) $
  可以直接两边同乘 $a^(-1)$，得到
  $ x equiv a^(-1) b pmod(n) $
]

#exercise[
  求 $11$ 在模 $26$ 下的逆元。
]
