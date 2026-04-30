#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "数论基础", author: "vkkkv")

= 数论基础

数论研究整数的性质，核心对象包括整除、素数、最大公约数、同余、算术函数与丢番图方程。
它既有古典味道，也与算法、密码学、组合数学紧密相连。

== 基本对象

#definition[
  记 $ZZ$ 为整数集合，$NN$ 为正整数集合。
  若存在整数 $k$ 使得 $a = b k$，则称 $b$ 整除 $a$，记作
  $ b divides a $
]

#proposition[
  整除关系满足：

  - 若 $a divides b$ 且 $b divides c$，则 $a divides c$。
  - 若 $a divides b$ 且 $a divides c$，则对任意整数 $x, y$，有
    $ a divides (b x + c y) $
]

#remark[
  第二条是整个初等数论中最常用的结构之一。
  裴蜀定理、欧几里得算法、线性同余方程都在利用“整除对整数线性组合封闭”。
]

#example[
  因为 $6 divides 18$ 且 $6 divides 30$，所以
  $ 6 divides (2 18 - 30) = 6 $
]

== 常见主题

#remark[
  一条自然的学习路径通常是：

  - 先掌握整除、素数、最大公约数。
  - 再学习同余、模逆元、线性同余、中国剩余定理。
  - 然后进入欧拉函数、费马小定理、原根、离散对数。
  - 最后再接触筛法、算术函数、莫比乌斯反演和更高级的解析或算法工具。
]
