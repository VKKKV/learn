#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "二次域", author: "vkkkv")

= 二次域

二次域是形如
$ QQ(sqrt(d)) $
的数域，其中 $d$ 不是平方数。
它是代数数论的入门对象，也和连分数、Pell 方程、单位元结构密切相关。

== 定义

#definition[
  二次域由所有形如
  $ a + b sqrt(d) $
  的数构成，其中 $a, b in QQ$，且 $d$ 不是平方数。
]

#remark[
  因为加入了 $sqrt(d)$，所以这个域严格大于有理数域 $QQ$，但又只“扩张了一层”。
]

== 共轭与范数

#definition[
  对元素
  $ alpha = a + b sqrt(d) $
  定义它的共轭为
  $ bar(alpha) = a - b sqrt(d) $
  范数定义为
  $ N(alpha) = alpha bar(alpha) = a^2 - d b^2 $
]

#proposition[
  范数具有乘法性：
  $ N(alpha beta) = N(alpha) N(beta) $
]

#remark[
  这条性质在研究单位元、分解和 Pell 方程时极其重要。
]

== 为什么重要

#remark[
  在二次域中，整数分解、单位元和范数结构会比普通整数环更丰富。
  很多看似初等的问题，一旦放到二次域中，就会显露出更清晰的代数结构。
]

== 与 Pell 方程的关系

#remark[
  Pell 方程
  $ x^2 - d y^2 = 1 $
  正好可以写成
  $ N(x + y sqrt(d)) = 1 $
  因而它本质上是在研究二次域中范数为 $1$ 的单位元。
]

#exercise[
  计算
  $ N(3 + 2 sqrt(2)) $
]
