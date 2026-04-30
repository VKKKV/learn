#import "../template.typ": *
#show: note-template

#set document(title: "最大公约数", author: "Arch User")

= 最大公约数

最大公约数是初等数论中最基础的概念之一。
它刻画两个整数共享因子的程度，也是欧几里得算法、裴蜀定理、模逆元和线性同余方程的共同起点。

== 定义

#definition[
  对不全为零的整数 $a, b$，同时整除 $a$ 与 $b$ 的最大正整数称为它们的最大公约数，记作
  $ upright("gcd")(a, b) $
]

#remark[
  按约定，$upright("gcd")(a, b)$ 总是取正值。
  例如
  $ upright("gcd")(-12, 18) = 6 $
  因为公约数与符号无关，只和绝对值有关。
]

== 基本性质

#proposition[
  设 $d = upright("gcd")(a, b)$，则

  - $d divides a$ 且 $d divides b$。
  - 若 $c divides a$ 且 $c divides b$，则 $c divides d$。

  换句话说，$d$ 是“所有公约数中的最大者”，同时也是“被所有公约数整除的那个数”。
]

#proposition[
  对任意整数 $k$，有
  $ upright("gcd")(a, b) = upright("gcd")(a, b + k a) $ <gcd-invariant>
]

#proof[
  一个整数同时整除 $a$ 和 $b$，当且仅当它同时整除 $a$ 和 $b + k a$。
  因而这两对数拥有完全相同的公约数集合，所以最大公约数相同。
]

== 欧几里得算法

#theorem[
  欧几里得算法说明：
  $ upright("gcd")(a, b) = upright("gcd")(b, a mod b) $ <euclid>
]

#proof[
  设
  $ a = b q + r $
  其中 $r = a mod b$。

  若某个整数同时整除 $a$ 与 $b$，则它也整除
  $ a - b q = r $

  反过来，若某个整数同时整除 $b$ 与 $r$，则它也整除
  $ b q + r = a $

  所以 $(a, b)$ 与 $(b, r)$ 拥有完全相同的公约数集合，于是得到 @euclid。
]

#example[
  计算
  $ upright("gcd")(84, 30) $

  由欧几里得算法，
  $ upright("gcd")(84, 30) = upright("gcd")(30, 24) = upright("gcd")(24, 6) = 6 $
]

== 线性组合刻画

#theorem[
  $upright("gcd")(a, b)$ 恰好是所有整数线性组合
  $ a x + b y $
  所能取到的最小正值。特别地，存在整数 $x, y$ 使得
  $ a x + b y = upright("gcd")(a, b) $ <gcd-linear-combination>
]

#remark[
  这就是裴蜀定理的核心内容。
  它说明最大公约数不仅仅是一个“最大因子”，也是整数线性组合结构中的最小正元。
]

#example[
  对于 $84$ 和 $30$，我们已经知道最大公约数是 $6$。
  进一步回代可得
  $ 6 = 84 - 2 30 + 24 - 24 $
  更系统地做扩展欧几里得算法，可以求出一组具体系数。
]

== 互素

#definition[
  若
  $ upright("gcd")(a, b) = 1 $
  则称 $a$ 与 $b$ 互素。
]

#proposition[
  若 $a$ 与 $b$ 互素，且 $a divides b c$，则
  $ a divides c $
]

#remark[
  这条性质常被称为“欧几里得引理”的一个常用形式。
  它在证明分数约分、模逆元存在性和同余消去律时都会出现。
]

== 常见技巧

#remark[
  关于最大公约数，最常见的变形技巧有：

  - 用 $upright("gcd")(a, b) = upright("gcd")(b, a mod b)$ 递归下降。
  - 用 $upright("gcd")(a, b) = upright("gcd")(a, b + k a)$ 替换复杂表达式。
  - 若已知分解质因数，则按各素因子的最小指数直接求最大公约数。
]

#exercise[
  求
  $ upright("gcd")(252, 198) $
]

#exercise[
  证明：若 $upright("gcd")(a, b) = d$，则
  $ upright("gcd")(a / d, b / d) = 1 $
]
