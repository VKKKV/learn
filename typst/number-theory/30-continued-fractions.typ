#import "../template.typ": *
#show: note-template

#set document(title: "连分数", author: "Arch User")

= 连分数

连分数把一个实数写成不断取整数部分和倒数的嵌套形式。
它是有理逼近和 Pell 方程的重要工具。

#definition[
  连分数写作
  $ a_0 + 1/(a_1 + 1/(a_2 + dots.c)) $
]

#remark[
  有理数的连分数展开有限终止；无理数的连分数展开无限。
  二次无理数的连分数展开最终循环。
]
