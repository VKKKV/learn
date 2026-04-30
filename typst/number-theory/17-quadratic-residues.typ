#import "../template.typ": *
#show: note-template

#set document(title: "二次剩余", author: "Arch User")

= 二次剩余

二次剩余研究方程
$ x^2 equiv a pmod(n) $
何时有解。
它是高次同余方程、平方根取模和更深层 reciprocity 理论的入口。

== 定义

#definition[
  若存在整数 $x$ 使得
  $ x^2 equiv a pmod(p) $ <qr-def>
  则称 $a$ 是模奇素数 $p$ 的二次剩余；否则称为二次非剩余。
]

#remark[
  在模奇素数 $p$ 下，非零剩余类一共有 $p - 1$ 个，其中恰好一半是二次剩余，一半是二次非剩余。
]

== Euler 判别法

#theorem[
  Euler 判别法：若 $upright("gcd")(a, p) = 1$，则
  $ a^((p - 1)/2) equiv 1 pmod(p) $
  当且仅当 $a$ 是模 $p$ 的二次剩余。
]

#remark[
  这一定理给出了一个“判定有没有平方根”的可计算标准。
  在算法上，先算一次快速幂，就能判断一个数是否是二次剩余。
]

== 例子

#example[
  在模 $7$ 下，平方数有：
  $ 1^2 equiv 1,
     2^2 equiv 4,
     3^2 equiv 2 pmod(7) $

  因而模 $7$ 的非零二次剩余是
  $ {1, 2, 4} $
]

#example[
  判断 $3$ 是否是模 $7$ 的二次剩余。

  由 Euler 判别法，计算
  $ 3^((7 - 1)/2) = 3^3 = 27 equiv -1 pmod(7) $
  所以 $3$ 不是模 $7$ 的二次剩余。
]

== 与原根的联系

#proposition[
  若 $g$ 是模奇素数 $p$ 的原根，则模 $p$ 的所有非零二次剩余恰好是
  $ g^(2k) $
  这些偶次幂所构成的集合。
]

#remark[
  这说明二次剩余本质上是“指数是偶数”的那一半元素。
  所以一旦掌握原根和阶的语言，二次剩余会变得很自然。
]

#exercise[
  求模 $11$ 的所有非零二次剩余。
]
