#import "@local/math-note:0.1.0": *
#show: note-template

#set document(
  title: "同余",
  author: "Arch User",
)

= 同余（Congruence Modulo）

同余是数论中最基础也最重要的关系之一。
它把“两个整数除以同一个正整数后余数相同”这件事抽象成了一个等价关系，
从而允许我们像操作普通等式一样操作模意义下的表达式。
后续的模逆元、线性同余方程、中国剩余定理、费马小定理、欧拉定理等内容，
都建立在这套语言之上。

== 定义与直观理解

#definition[
  设 $a, b in ZZ$，$n in NN$。若
  $ a equiv b pmod(n) $ <def-congruence>
  则称 $a$ 与 $b$ 模 $n$ 同余。

  这等价于
  $ n divides (a - b) $ <criterion-congruence>

  也就是说，存在某个整数 $k$，使得
  $ a - b = k n $
]

#remark[
  从余数角度看，$a equiv b pmod(n)$ 表示 $a$ 和 $b$ 除以 $n$ 的余数相同。
  从整除角度看，@criterion-congruence 表示它们之差是 $n$ 的倍数。
  这两个表述完全等价，但在证明题中通常“差能被模整除”更好用。
]

#example[
  $ 35 equiv 11 pmod(12) $
  因为 $12 divides (35 - 11) = 24$。

  $ -2 equiv 10 pmod(12) $
  因为 $12 divides (-2 - 10) = -12$。

  $ 1001 equiv 1 pmod(10) $
  因为二者相差 $1000$，它能被 $10$ 整除。
]

== 同余是一种等价关系

#theorem[
  对固定的正整数 $n$，模 $n$ 同余满足：

  - 自反性：$a equiv a pmod(n)$。
  - 对称性：若 $a equiv b pmod(n)$，则 $b equiv a pmod(n)$。
  - 传递性：若 $a equiv b pmod(n)$ 且 $b equiv c pmod(n)$，则 $a equiv c pmod(n)$。
]

#proof[
  自反性来自 $a - a = 0$，而 $n divides 0$。

  对称性来自：若 $n divides (a - b)$，则也有
  $ n divides -(a - b) = b - a $

  传递性来自：若 $n divides (a - b)$ 且 $n divides (b - c)$，则
  $ n divides ((a - b) + (b - c)) = a - c $
]

#remark[
  因为它是等价关系，所以整数集合 $ZZ$ 会被划分成若干个互不相交的同余类。
  模 $n$ 时恰好有 $n$ 个不同的同余类，可分别用
  $0, 1, 2, dots.c, n - 1$
  作为代表元。
]

== 代数性质

#theorem[
  若 $a equiv b pmod(n)$ 且 $c equiv d pmod(n)$，则
  $ a + c equiv b + d pmod(n) $ <cong-add>
  $ a - c equiv b - d pmod(n) $ <cong-sub>
  $ a c equiv b d pmod(n) $ <cong-mul>

  更一般地，若 $a equiv b pmod(n)$，则对任意多项式 $f(x) in ZZ[x]$ 都有
  $ f(a) equiv f(b) pmod(n) $ <poly-compatible>
]

#proof[
  由 $a equiv b pmod(n)$ 与 $c equiv d pmod(n)$，得
  $ n divides (a - b) $
  $ n divides (c - d) $

  因而
  $ n divides ((a + c) - (b + d)) $
  $ n divides ((a - c) - (b - d)) $

  又因为
  $ a c - b d = a(c - d) + d(a - b) $
  右边两项都能被 $n$ 整除，所以得到 @cong-mul。

  多项式的结论来自反复使用加法与乘法相容性。
]

#example[
  因为 $17 equiv 2 pmod(5)$，所以
  $ 17^2 + 3 17 + 1 equiv 2^2 + 3 2 + 1 equiv 11 equiv 1 pmod(5) $

  这说明在做模运算时，可以先把大数替换成更简单的同余代表元，再继续计算。
]

== 消去与等价变形

#proposition[
  若 $a c equiv b c pmod(n)$，则一般不能直接推出
  $ a equiv b pmod(n) $

  只有在
  $ upright("gcd")(c, n) = 1 $
  时，才可以消去公共因子 $c$，得到
  $ a equiv b pmod(n) $ <cancel-law>
]

#proof[
  由 $a c equiv b c pmod(n)$ 可知
  $ n divides c(a - b) $

  当 $upright("gcd")(c, n) = 1$ 时，由互素性可推出
  $ n divides (a - b) $
  因而得到 @cancel-law。
]

#example[
  在模 $6$ 下，
  $ 2 1 equiv 2 4 pmod(6) $
  因为 $2 equiv 8 pmod(6)$。

  但不能推出 $1 equiv 4 pmod(6)$。
  这里失败的原因是 $upright("gcd")(2, 6) != 1$。
]

== 同余类与剩余系

#definition[
  固定模数 $n$，所有与 $a$ 模 $n$ 同余的整数构成的集合称为 $a$ 的同余类，记作
  $ [a]_n = {x in ZZ mid x equiv a pmod(n)} $

  若一组整数中恰好从每个同余类中选出一个代表元，就称它构成模 $n$ 的一个完全剩余系。
]

#example[
  模 $5$ 的完全剩余系可以取
  $ {0, 1, 2, 3, 4} $

  也可以取
  $ {-2, -1, 0, 1, 2} $

  它们表示的是同样的五个同余类，只是代表元不同。
]

#definition[
  若把模 $n$ 下与 $n$ 互素的代表元单独挑出来，就得到一个简化剩余系。
  它的元素个数为
  $ phi(n) $
  这正是欧拉函数的含义之一。
]

== 与整除、最大公约数的关系

#proposition[
  若 $d divides a$ 且 $d divides b$，并且 $a equiv b pmod(n)$，则
  $ a / d equiv b / d pmod(n / d) $
  只要右侧表达式有意义。

  另外，若 $a equiv b pmod(n)$，则
  $ upright("gcd")(a, n) = upright("gcd")(b, n) $ <gcd-invariant>
]

#proof[
  对于第二条，由 $a = b + k n$ 可知
  $ upright("gcd")(a, n) = upright("gcd")(b + k n, n) = upright("gcd")(b, n) $
  这是欧几里得算法不变性的直接应用。
]

#example[
  因为 $26 equiv 5 pmod(7)$，所以
  $ upright("gcd")(26, 7) = upright("gcd")(5, 7) = 1 $

  这在判断某个数在模 $n$ 下是否可逆时特别有用。
]

== 幂与周期现象

#theorem[
  固定整数 $a$ 与模数 $n$ 后，数列
  $ a, a^2, a^3, dots.c $
  在模 $n$ 意义下最终一定会出现循环。

  特别地，当 $upright("gcd")(a, n) = 1$ 时，存在正整数 $t$ 使得
  $ a^t equiv 1 pmod(n) $
  这个最小的正整数 $t$ 称为 $a$ 模 $n$ 的阶。
]

#remark[
  这就是为什么模幂运算经常可以降幂。
  后面的费马小定理、欧拉定理、原根、离散对数，都是在研究这种周期结构。
]

#example[
  在模 $7$ 下，
  $ 3^1 equiv 3,
     3^2 equiv 2,
     3^3 equiv 6,
     3^4 equiv 4,
     3^5 equiv 5,
     3^6 equiv 1 pmod(7) $

  因而 $3$ 在模 $7$ 下的阶是 $6$。
]

== 线性同余方程

#theorem[
  线性同余方程
  $ a x equiv b pmod(n) $ <linear-congruence>
  有解，当且仅当
  $ upright("gcd")(a, n) divides b $ <linear-solvable>

  若设 $d = upright("gcd")(a, n)$，则它一旦有解，就恰好有 $d$ 个模 $n$ 不同的解。
]

#proof[
  方程 @linear-congruence 等价于
  $ a x - b = k n $
  也就是整数方程
  $ a x + n(-k) = b $

  因而它可解的充要条件就是 $upright("gcd")(a, n)$ 整除 $b$，这与裴蜀定理完全一致。
]

#example[
  求
  $ 6 x equiv 9 pmod(15) $

  由于 $upright("gcd")(6, 15) = 3$，且 $3 divides 9$，所以有解。
  两边同除以 $3$，得到
  $ 2 x equiv 3 pmod(5) $
  再乘以 $2$ 在模 $5$ 下的逆元 $3$，得到
  $ x equiv 4 pmod(5) $

  因而模 $15$ 的解为
  $ x equiv 4, 9, 14 pmod(15) $
]

== 题解化例题

#example[
  例 1：计算
  $ 2^{100} mod 7 $

  因为
  $ 2^3 = 8 equiv 1 pmod(7) $
  所以 $2$ 在模 $7$ 下的周期是 $3$。
  将 $100$ 除以 $3$，有
  $ 100 = 3 33 + 1 $
  因而
  $ 2^{100} = (2^3)^{33} 2 equiv 1^{33} 2 equiv 2 pmod(7) $
]

#example[
  例 2：判断
  $ 1234567 $
  是否能被 $9$ 整除。

  因为模 $9$ 下有
  $ 10 equiv 1 pmod(9) $
  所以
  $ 1234567 equiv 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28 equiv 1 pmod(9) $
  因而它不能被 $9$ 整除。
]

#example[
  例 3：求
  $ 37^2 + 5 37 + 8 mod 6 $

  因为
  $ 37 equiv 1 pmod(6) $
  所以
  $ 37^2 + 5 37 + 8 equiv 1^2 + 5 1 + 8 = 14 equiv 2 pmod(6) $
]

#example[
  例 4：证明若 $a$ 是奇整数，则
  $ a^2 equiv 1 pmod(8) $

  设
  $ a = 2k + 1 $
  则
  $ a^2 = 4k^2 + 4k + 1 = 4k(k + 1) + 1 $

  因为相邻两个整数中必有一个偶数，所以 $k(k+1)$ 为偶数，设
  $ k(k+1) = 2t $
  则
  $ a^2 = 8t + 1 $
  从而
  $ a^2 equiv 1 pmod(8) $
]

== 与后续内容的连接

#remark[
  同余是后续各章的公共语言：

  - 模逆元研究的是何时能把乘法“除掉”。
  - 线性同余方程研究的是 $a x equiv b pmod(n)$ 的求解。
  - 中国剩余定理研究的是多个同余条件如何合并。
  - 费马小定理与欧拉定理研究的是幂在模意义下的周期。
  - 二次剩余、原根、离散对数则进一步研究乘法群结构。
]

== 常见结论速记

#proposition[
  在做题时最常用的几条结论是：

  - $a equiv b pmod(n)$ 当且仅当 $n divides (a - b)$。
  - 同余可做加、减、乘，也可代入整系数多项式。
  - 消去乘法因子时要检查该因子与模数是否互素。
  - 方程 $a x equiv b pmod(n)$ 可解当且仅当 $upright("gcd")(a, n) divides b$。
  - 若 $upright("gcd")(a, n) = 1$，则 $a$ 在模 $n$ 下存在逆元。
]

#exercise[
  证明：若 $a equiv b pmod(n)$，则对任意正整数 $m$，有
  $ a^m equiv b^m pmod(n) $
]

#exercise[
  求解同余方程
  $ 14 x equiv 8 pmod(20) $
]
