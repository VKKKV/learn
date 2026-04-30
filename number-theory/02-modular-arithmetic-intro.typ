#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "模算术简介", author: "Arch User")

= 模算术简介

模算术把整数“按余数分类”。在模 $n$ 的意义下，只关心一个数除以 $n$ 以后剩下什么。
它是同余理论的语言层，也是程序竞赛和密码学里最常用的计算模型之一。

#definition[
  若
  $ a equiv b pmod(n) $
  则称 $a$ 与 $b$ 模 $n$ 同余。
  这等价于 $n divides (a - b)$。
]

#example[
  在模 $7$ 下，$15$ 与 $1$ 是等价的，因为
  $ 15 equiv 1 pmod(7) $
  所以计算 $15^3 mod 7$ 时，可以先把 $15$ 换成 $1$。
]

#proposition[
  模运算中可安全进行：

  - 先取模再加减乘。
  - 把大数替换成更小的同余代表元。
  - 做幂运算时利用周期性降幂。

  但除法不能直接做，除非除数在模意义下可逆。
]
