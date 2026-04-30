#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "阶乘取模", author: "vkkkv")

= 阶乘取模

阶乘取模问题常出现在组合数、Lucas 定理、Wilson 定理、模素数算法和排列计数中。
它看起来只是“把很多数乘起来再取模”，但真正难点常在于 $n$ 很大、模数特殊，或者需要重复查询。

== 基本观察

#proposition[
  直接算
  $ n! mod p $
  只在 $n$ 较小时可行。
  若模数是素数，往往结合 Wilson 定理、分块预处理、Lucas 定理一起使用。
]

#remark[
  当 $n >= p$ 且模数是素数 $p$ 时，
  $ n! $
  一定含有因子 $p$，因此
  $ n! equiv 0 pmod(p) $
  这在很多题里是第一步判断。
]

== Wilson 定理

#theorem[
  素数 $p$ 满足
  $ (p - 1)! equiv -1 pmod(p) $ <wilson>
]

#remark[
  Wilson 定理说明：模素数时，除了 $1$ 和 $-1$ 之外，其他非零元素都能两两配对成逆元，乘积最终只剩下 $-1$。
]

== 例子

#example[
  在模 $7$ 下，
  $ 6! = 720 equiv -1 pmod(7) $
  这正是 @wilson 的具体体现。
]

#example[
  计算
  $ 10! mod 7 $

  因为 $10!$ 含因子 $7$，所以
  $ 10! equiv 0 pmod(7) $
]

== 常见场景

#remark[
  阶乘取模最常出现于：

  - 组合数公式
    $ binom(n, k) = n! / (k!(n-k)!) $
  - Lucas 定理中的小组合数预处理
  - 模素数或模素数幂的计数问题
]

== 技巧提示

#remark[
  当题目需要大量查询
  $ 1!, 2!, dots.c, n! $
  的模值时，通常会先做前缀预处理；
  当模数是素数时，还常配合逆元数组一起维护组合数。
]

#exercise[
  计算
  $ 6! mod 11 $
]
