#import "../template.typ": *
#show: note-template

#set document(title: "Stern-Brocot 树与 Farey 序列", author: "Arch User")

= Stern-Brocot 树与 Farey 序列

这两个对象都在组织有理数。
一个偏树结构，一个偏序列结构，但都与中项
$ (a + c) / (b + d) $
密切相关。

#definition[
  若两个分数分别是
  $ a / b $
  和
  $ c / d $
  则它们的中项为
  $ (a + c) / (b + d) $
]

#remark[
  Stern-Brocot 树可以无重复地枚举所有正有理数；Farey 序列则按大小顺序排列分母不超过某个上界的最简分数。
]
