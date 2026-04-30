#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "同余方程", author: "vkkkv")

= 同余方程

同余方程是模意义下的方程。
线性同余方程只是最简单的一类，更一般的同余方程可能涉及高次幂、指数、剩余类结构以及模分解。

== 定义

#definition[
  形如
  $ f(x) equiv 0 pmod(n) $
  的问题称为同余方程。

  更一般地，
  $ f(x) equiv g(x) pmod(n) $
  也可转化为
  $ f(x) - g(x) equiv 0 pmod(n) $
]

== 基本思路

#remark[
  求解同余方程时，通常优先检查：

  - 模数是否是素数或素数幂
  - 方程是否可以因式分解
  - 是否能借助 CRT 分模求解
  - 是否涉及二次剩余、原根或离散对数
]

== 从整数方程视角理解

#proposition[
  同余方程
  $ f(x) equiv 0 pmod(n) $
  等价于存在整数 $k$ 使得
  $ f(x) = k n $
]

#remark[
  这个视角在证明可解性时很有用；
  但在真正求解时，通常还是直接在剩余类里思考更方便。
]

== 常见分类

#proposition[
  常见的同余方程类型有：

  - 线性同余方程：$a x equiv b pmod(n)$
  - 二次同余方程：$x^2 equiv a pmod(n)$
  - 指数同余方程：$a^x equiv b pmod(n)$
  - 多项式同余方程：$f(x) equiv 0 pmod(n)$
]

== 例题

#example[
  解同余方程
  $ x^2 - 1 equiv 0 pmod(8) $

  因式分解得
  $ (x - 1)(x + 1) equiv 0 pmod(8) $

  逐个代入模 $8$ 的代表元可知，解为
  $ x equiv 1, 3, 5, 7 pmod(8) $
]

#example[
  解同余方程
  $ x^2 equiv 1 pmod(5) $

  直接枚举模 $5$ 的代表元，有
  $ 1^2 equiv 1,
     4^2 equiv 1 pmod(5) $
  所以解为
  $ x equiv 1, 4 pmod(5) $
]

== 与分模求解的关系

#remark[
  当模数可分解成互素因子的乘积时，很多同余方程都可以先分别在各个小模数下求解，再用中国剩余定理拼回去。
  这是处理中等复杂度同余方程的标准套路。
]

#exercise[
  求解
  $ x^2 equiv 4 pmod(7) $
]
