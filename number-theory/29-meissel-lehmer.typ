#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "Meissel-Lehmer 算法", author: "Arch User")

= Meissel-Lehmer 算法

Meissel-Lehmer 算法用于计算素数计数函数
$ upright("pi")(n) $。
它比朴素筛法更适合大范围素数计数，是“先理解结构，再写实现”的典型数论算法。

== 素数计数函数

#definition[
  记
  $ upright("pi")(n) $
  为不超过 $n$ 的素数个数。
]

#example[
  $ upright("pi")(10) = 4 $
  因为不超过 $10$ 的素数是
  $ 2, 3, 5, 7 $
]

== 为什么不用朴素筛

#remark[
  当 $n$ 很大时，直接筛到 $n$ 的空间和时间成本会快速上升。
  这时更自然的思路是：不要真的把每个数都筛一遍，而是通过递归地剔除小素数贡献来统计答案。
]

== 核心思路

#remark[
  Meissel-Lehmer 的核心是把“不超过 $n$ 的素数个数”拆成若干递归子问题，
  并用预处理和记忆化避免重复计算。
]

#proposition[
  这类算法通常会构造一个辅助函数，表示“去掉前若干个素数后，仍然留下多少候选数”，再通过递推恢复
  $ upright("pi")(n) $
]

== 学习建议

#remark[
  学它之前，最好已经熟悉：

  - 埃氏筛和线性筛
  - 数论分块
  - 递归计数函数的写法

  否则只看公式会非常抽象。
]

#exercise[
  思考：为什么求
  $ upright("pi")(n) $
  时，很多方法都会特别关注小于等于 $sqrt(n)$ 的素数？
]
