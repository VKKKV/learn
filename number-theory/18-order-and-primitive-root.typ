#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "阶与原根", author: "Arch User")

= 阶与原根

当 $upright("gcd")(a, n) = 1$ 时，$a$ 在模 $n$ 下的乘法行为具有周期。
这个周期长度就是阶；若某个元素的阶达到最大值，它就成为原根。

== 阶

#definition[
  设 $upright("gcd")(a, n) = 1$。
  最小正整数 $t$ 满足
  $ a^t equiv 1 pmod(n) $
  时，称 $t$ 为 $a$ 模 $n$ 的阶，记作
  $ upright("ord")_n(a) $
]

#proposition[
  若 $upright("ord")_n(a) = t$，且
  $ a^k equiv 1 pmod(n) $
  则必有
  $ t divides k $
]

#proof[
  将 $k$ 除以 $t$，写成
  $ k = q t + r,
     0 <= r < t $
  则
  $ a^k = (a^t)^q a^r equiv a^r pmod(n) $
  又由于 $a^k equiv 1 pmod(n)$，得 $a^r equiv 1 pmod(n)$。
  由 $t$ 的最小性可知只能有 $r = 0$，所以 $t divides k$。
]

== 阶与欧拉函数

#theorem[
  若 $upright("gcd")(a, n) = 1$，则
  $ upright("ord")_n(a) divides phi(n) $ <order-divides-phi>
]

#remark[
  这是欧拉定理
  $ a^(phi(n)) equiv 1 pmod(n) $
  的直接推论。
  因为阶是满足该式的最小正整数，所以它一定整除任何其他可行指数，特别整除 $phi(n)$。
]

== 原根

#definition[
  若
  $ upright("ord")_n(g) = phi(n) $
  则称 $g$ 是模 $n$ 的原根。
]

#remark[
  原根把模乘法群变成了“由一个元素生成的循环群”。
  一旦找到原根，很多乘法问题都可以转成指数问题。
]

== 例子

#example[
  在模 $7$ 下，
  $ 3^1 equiv 3,
     3^2 equiv 2,
     3^3 equiv 6,
     3^4 equiv 4,
     3^5 equiv 5,
     3^6 equiv 1 pmod(7) $

  因而
  $ upright("ord")_7(3) = 6 = phi(7) $
  所以 $3$ 是模 $7$ 的一个原根。
]

#example[
  在模 $7$ 下，
  $ 2^1 equiv 2,
     2^2 equiv 4,
     2^3 equiv 1 pmod(7) $
  所以
  $ upright("ord")_7(2) = 3 $
  它不是原根。
]

#example[
  例 1：求 $2$ 在模 $9$ 下的阶。

  依次计算：
  $ 2^1 equiv 2,
     2^2 equiv 4,
     2^3 equiv 8,
     2^4 equiv 16 equiv 7,
     2^5 equiv 14 equiv 5,
     2^6 equiv 10 equiv 1 pmod(9) $

  所以
  $ upright("ord")_9(2) = 6 $
]

#example[
  例 2：判断 $2$ 是否是模 $9$ 的原根。

  因为
  $ phi(9) = 6 $
  而上题求得
  $ upright("ord")_9(2) = 6 $
  所以 $2$ 是模 $9$ 的原根。
]

#example[
  例 3：求 $3$ 在模 $10$ 下的阶。

  因为
  $ upright("gcd")(3, 10) = 1 $
  可讨论其阶。

  依次算：
  $ 3^1 equiv 3,
     3^2 equiv 9,
     3^3 equiv 27 equiv 7,
     3^4 equiv 21 equiv 1 pmod(10) $

  所以
  $ upright("ord")_(10)(3) = 4 $
]

#example[
  例 4：判断 $2$ 是否是模 $7$ 的原根。

  因为
  $ phi(7) = 6 $
  但
  $ 2^3 equiv 1 pmod(7) $
  所以
  $ upright("ord")_7(2) = 3 != 6 $
  因而 $2$ 不是模 $7$ 的原根。
]

== 与离散对数的关系

#remark[
  若 $g$ 是模 $n$ 的原根，则任意与 $n$ 互素的剩余类都能表示成
  $ g^k $
  的形式。
  因而求解乘法同余方程时，常可先取“离散对数”，把问题转成指数同余。
]

#exercise[
  求 $2$ 在模 $9$ 下的阶。
]
