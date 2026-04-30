#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "Stern-Brocot 树与 Farey 序列", author: "vkkkv")

= Stern-Brocot 树与 Farey 序列

Stern-Brocot 树与 Farey 序列都在组织有理数。
一个偏树结构，一个偏有序序列结构，但它们都围绕中项
$ (a + c) / (b + d) $
展开。

== 中项

#definition[
  若两个分数分别是
  $ a / b $
  和
  $ c / d $
  则它们的中项为
  $ (a + c) / (b + d) $
]

#remark[
  中项会落在这两个分数之间，因此非常适合用来递归细分区间。
]

== Stern-Brocot 树

#proposition[
  Stern-Brocot 树可以无重复地枚举所有正有理数。
  每个正有理数恰好出现一次，且始终保持最简分数形式。
]

#remark[
  这棵树本质上是“不断取中项并细分”的结果。
  在构造有理逼近或求分数路径时很常见。
]

== Farey 序列

#definition[
  Farey 序列 $F_n$ 是所有分母不超过 $n$ 的最简分数，按从小到大的顺序排列得到的序列。
]

#proposition[
  Farey 序列中相邻两项
  $ a / b < c / d $
  满足
  $ b c - a d = 1 $
]

#remark[
  这条关系说明 Farey 序列中的相邻分数非常“紧密”，也是很多数论构造与计数的起点。
]

== 联系

#remark[
  Stern-Brocot 树强调“如何生成全部正有理数”；
  Farey 序列强调“在分母上界限制下，如何按大小组织有理数”。
  两者都与中项和互素结构密切相关。
]

#exercise[
  写出 Farey 序列 $F_4$。
]
