#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "高次剩余与单位根", author: "Arch User")

= 高次剩余与单位根

高次剩余把二次剩余推广到
$ x^k equiv a pmod(n) $
的情形；单位根研究
$ x^k equiv 1 pmod(n) $
的所有解。
这两个主题本质上都在研究模乘法群里“幂映射”的像与核。

== 高次剩余

#definition[
  若方程
  $ x^k equiv a pmod(n) $
  有解，则称 $a$ 是模 $n$ 的 $k$ 次剩余。
]

#example[
  二次剩余就是 $k = 2$ 的特例。
  因此高次剩余可以看成对二次剩余理论的直接推广。
]

== 单位根

#definition[
  满足
  $ x^k equiv 1 pmod(n) $
  的解称为模 $n$ 下的 $k$ 次单位根。
]

#remark[
  单位根研究的是幂映射的核；
  高次剩余研究的是幂映射的像。
  这两个对象是同一个群论结构的两面。
]

== 与循环群的关系

#proposition[
  若模乘法群是循环群，设原根为 $g$，则方程
  $ x^k equiv a pmod(n) $
  常可转化为指数同余：
  $ k t equiv b pmod(phi(n)) $
]

#remark[
  这说明一旦能把乘法结构转成指数结构，很多高次剩余问题就会退化为线性同余问题。
]

== 例子

#example[
  在模 $7$ 下，求解
  $ x^3 equiv 1 pmod(7) $

  直接代入可得
  $ 1^3 equiv 1,
     2^3 equiv 1,
     4^3 equiv 1 pmod(7) $
  所以有三个三次单位根：
  $ 1, 2, 4 $
]

== 学习位置

#remark[
  这个主题通常建立在二次剩余、原根、离散对数之后。
  若前面三部分已经熟悉，高次剩余本质上就是把“平方”换成“$k$ 次幂”。
]

#exercise[
  在模 $9$ 下尝试求解
  $ x^2 equiv 1 pmod(9) $
]
