#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "狄利克雷卷积", author: "vkkkv")

= 狄利克雷卷积

狄利克雷卷积是算术函数上的一种乘法结构。
很多经典恒等式一旦写成卷积形式，就会变得非常清晰，也更方便做反演与构造。

== 定义

#definition[
  对算术函数 $f, g$，定义它们的狄利克雷卷积为
  $ (f * g)(n) = sum_(d divides n) f(d) g(n / d) $
]

#remark[
  它和普通序列卷积不同，求和指标不是“和为 $n$”，而是“约数 $d$ 与商 $n / d$ 的配对”。
]

== 单位元

#proposition[
  单位函数 $epsilon$ 满足
  $ epsilon(1) = 1 $
  且当 $n > 1$ 时
  $ epsilon(n) = 0 $
  它是狄利克雷卷积下的单位元。
]

#proof[
  因为
  $ (f * epsilon)(n) = sum_(d divides n) f(d) epsilon(n / d) $
  只有当
  $ n / d = 1 $
  时项才不为零，即只有 $d = n$ 这一项保留，所以
  $ (f * epsilon)(n) = f(n) $
]

== 常见恒等式

#proposition[
  记常值函数 $1(n) = 1$，则：

  - 约数个数函数满足
    $ d = 1 * 1 $
  - 约数和函数满足
    $ sigma = id * 1 $

  这里 $id(n) = n$。
]

#remark[
  所以很多算术函数并不是“凭空定义”，而是能由更基本的函数通过卷积构造出来。
]

== 卷积与积性

#proposition[
  若 $f$ 与 $g$ 都是积性函数，则
  $ f * g $
  仍然是积性函数。
]

#remark[
  这条性质非常重要，因为它解释了为什么很多通过“按约数求和”定义出来的函数仍然保有良好的乘法结构。
]

== 例子

#example[
  对于 $n = 6$，有约数
  $ 1, 2, 3, 6 $
  所以
  $ (1 * 1)(6) = 1(1)1(6) + 1(2)1(3) + 1(3)1(2) + 1(6)1(1) = 4 $

  这正是 $6$ 的约数个数。
]

#exercise[
  验证：
  $ (id * 1)(4) = sigma(4) $
]
