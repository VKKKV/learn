#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "莫比乌斯反演", author: "Arch User")

= 莫比乌斯反演

莫比乌斯反演是数论与组合中最常用的“从按约数求和恢复原函数”的工具之一。
它的语言核心就是狄利克雷卷积与莫比乌斯函数。

== 莫比乌斯函数

#definition[
  莫比乌斯函数 $mu(n)$ 定义为：

  - $mu(1) = 1$
  - 若 $n$ 含平方因子，则 $mu(n) = 0$
  - 若 $n$ 是 $k$ 个互异素数的乘积，则 $mu(n) = (-1)^k$
]

#example[
  $ mu(1) = 1 $
  $ mu(2) = -1 $
  $ mu(6) = 1 $
  $ mu(12) = 0 $
]

== 关键恒等式

#proposition[
  莫比乌斯函数满足
  $ mu * 1 = epsilon $
]

#remark[
  这意味着 $mu$ 是常值函数 $1$ 在狄利克雷卷积下的逆元。
  莫比乌斯反演本质上就是对两边同时“卷积上 $mu$”。
]

== 反演公式

#theorem[
  若
  $ g(n) = sum_(d divides n) f(d) $ <mobius-forward>
  则
  $ f(n) = sum_(d divides n) mu(d) g(n / d) $ <mobius-inversion>
]

#proof[
  式 @mobius-forward 可以写成
  $ g = f * 1 $
  两边再与 $mu$ 做卷积，得到
  $ g * mu = f * 1 * mu = f * epsilon = f $
  这正是 @mobius-inversion。
]

== 例子

#example[
  若已知
  $ g(n) = sum_(d divides n) 1 = d(n) $
  即约数个数函数，
  那么按反演可恢复出
  $ 1(n) = sum_(d divides n) mu(d) d(n / d) $
]

== 使用场景

#remark[
  莫比乌斯反演常出现在：

  - 按约数求和转原函数
  - 统计互素对数
  - 计数中去重
  - 杜教筛和其他积性函数前缀和算法
]

#exercise[
  证明：若
  $ F(n) = sum_(d divides n) f(d) $
  则
  $ f(n) = sum_(d divides n) mu(n / d) F(d) $
]
