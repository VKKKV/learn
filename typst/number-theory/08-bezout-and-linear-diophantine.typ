#import "../template.typ": *
#show: note-template

#set document(title: "裴蜀定理与一次不定方程", author: "Arch User")

= 裴蜀定理与一次不定方程

裴蜀定理把最大公约数与整数线性组合联系起来，是模逆元、线性同余方程和不定方程求解的基础。

== 裴蜀定理

#theorem[
  对任意不全为零的整数 $a, b$，存在整数 $x, y$ 使得
  $ a x + b y = upright("gcd")(a, b) $ <bezout>
]

#remark[
  这说明最大公约数一定能写成 $a$ 和 $b$ 的整数线性组合。
  同时也说明所有形如
  $ a x + b y $
  的整数集合，正好是 $upright("gcd")(a, b)$ 的所有倍数。
]

== 一次不定方程的可解条件

#theorem[
  一次不定方程
  $ a x + b y = c $ <linear-diophantine>
  有整数解，当且仅当
  $ upright("gcd")(a, b) divides c $ <linear-diophantine-solvable>
]

#proof[
  设 $d = upright("gcd")(a, b)$。

  若方程有解，则左边是 $a$ 与 $b$ 的整数线性组合，因此一定是 $d$ 的倍数，所以 $d divides c$。

  反过来，若 $d divides c$，设
  $ c = d t $
  又由裴蜀定理，存在整数 $x_0, y_0$ 使得
  $ a x_0 + b y_0 = d $
  两边同乘 $t$，得到
  $ a (t x_0) + b (t y_0) = c $
  从而方程有整数解。
]

== 通解形式

#proposition[
  若 $(x_0, y_0)$ 是方程 @linear-diophantine 的一组特解，记
  $ d = upright("gcd")(a, b) $
  则它的全部整数解为
  $ x = x_0 + (b / d) t $
  $ y = y_0 - (a / d) t $
  其中 $t in ZZ$。
]

#proof[
  将上式代回可验证它始终成立。
  反过来，若 $(x, y)$ 与 $(x_0, y_0)$ 都是解，则两式相减得到
  $ a(x - x_0) + b(y - y_0) = 0 $
  即
  $ a(x - x_0) = -b(y - y_0) $
  再利用 $d = upright("gcd")(a, b)$ 对 $a, b$ 做约分，就能得到上述参数化表示。
]

== 例题

#example[
  求方程
  $ 18 x + 30 y = 42 $
  的整数解。

  因为
  $ upright("gcd")(18, 30) = 6 $
  且 $6 divides 42$，所以有解。

  两边同除以 $6$，得到
  $ 3 x + 5 y = 7 $
  一组特解是
  $ x_0 = -1,
     y_0 = 2 $

  因而通解为
  $ x = -1 + 5 t,
     y = 2 - 3 t,
     t in ZZ $
]

== 与同余的联系

#remark[
  方程
  $ a x + b y = c $
  与线性同余方程
  $ a x equiv c pmod(b) $
  本质上是同一个问题的两个视角。
  一个在整数方程里讨论，一个在模意义下讨论。
]

#exercise[
  求方程
  $ 24 x + 36 y = 60 $
  的一组特解与通解。
]
