#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "升幂引理", author: "Arch User")

= 升幂引理

升幂引理通常记作 LTE，用于计算某个素数在差幂表达式中的指数。
它特别适合处理
$a^n - b^n$
和
$a^n + b^n$
这类式子，在竞赛数论和整除证明中非常高频。

== 记号

#definition[
  对素数 $p$ 与非零整数 $m$，记
  $ v_p(m) $
  为 $p$ 在 $m$ 的质因数分解中出现的指数。

  也就是说，$p^(v_p(m)) divides m$，但
  $ p^(v_p(m) + 1) in.not m $
]

== 典型版本

#theorem[
  若奇素数 $p divides (a - b)$，且
  $ p in.not a,
     p in.not b $
  则对任意正整数 $n$，有
  $ v_p(a^n - b^n) = v_p(a - b) + v_p(n) $ <lte-minus>
]

#remark[
  使用 LTE 的核心不是背公式，而是先确认：

  - 用的是 $a^n - b^n$ 还是 $a^n + b^n$
  - 素数 $p$ 是否整除 $a - b$ 或 $a + b$
  - $p$ 是否整除 $a$ 或 $b$
  - 指数 $n$ 的奇偶性是否满足条件
]

== 直观理解

#remark[
  @lte-minus 的含义是：
  当 $p$ 已经整除 $a - b$ 时，幂次差里来自 $p$ 的额外指数，恰好再加上一份 $v_p(n)$。
  所以指数本身的质因子结构会直接进入答案。
]

== 例题

#example[
  求
  $ v_3(2^6 - 1) $

  将它写成
  $ 2^6 - 1^6 $
  因为
  $ 3 divides (2 - 1) $
  且 $3 in.not 2, 1$，所以由 @lte-minus，
  $ v_3(2^6 - 1) = v_3(2 - 1) + v_3(6) = 0 + 1 = 1 $
]

#example[
  求
  $ v_5(6^10 - 1) $

  因为
  $ 5 divides (6 - 1) $
  所以
  $ v_5(6^10 - 1) = v_5(5) + v_5(10) = 1 + 1 = 2 $
]

== 常见用途

#proposition[
  LTE 常见于以下场景：

  - 判断某个幂差能被多高次的素数整除
  - 证明某些整除关系
  - 估计阶乘、组合数或递推式中的素因子指数
  - 将“巨大表达式的整除次数”转化为较小数的指数和
]

#exercise[
  求
  $ v_3(10^9 - 1) $
]
