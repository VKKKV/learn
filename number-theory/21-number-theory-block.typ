#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "数论分块", author: "vkkkv")

= 数论分块

数论分块利用商值
$ floor(n / l) $
在一段区间内保持不变这一性质，把求和从 $O(n)$ 降到大约 $O(sqrt(n))$。
它是很多整除分块、前缀和和杜教筛推导的基础技巧。

== 核心观察

#proposition[
  对固定的 $n$，值
  $ floor(n / i) $
  的不同取值个数只有 $O(sqrt(n))$ 级别。
]

#proof[
  当
  $ i <= sqrt(n) $
  时，指标本身不超过 $sqrt(n)$ 个；
  当
  $ i > sqrt(n) $
  时，商值
  $ floor(n / i) < sqrt(n) $
  也至多产生大约 $sqrt(n)$ 种不同结果。
]

== 分块方式

#proposition[
  若当前处理到左端点 $l$，记
  $ t = floor(n / l) $
  则在区间
  $ l <= i <= floor(n / t) $
  上，
  $ floor(n / i) $
  都等于同一个值 $t$。
]

#remark[
  所以算法里常令
  $ r = floor(n / floor(n / l)) $
  然后整段处理区间 $[l, r]$。
]

== 典型应用

#example[
  计算
  $ sum_(i=1)^n floor(n / i) $
  时，若直接枚举是 $O(n)$；
  若按相同商值分块，则可降到大约 $O(sqrt(n))$。
]

#remark[
  只要求和式中反复出现
  $ floor(n / i) $
  或
  $ floor(m / i) $
  这样的结构，就应第一时间考虑数论分块。
]

#exercise[
  试手动列出 $n = 20$ 时，
  $ floor(20 / i) $
  的不同分块区间。
]
