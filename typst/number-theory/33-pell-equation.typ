#import "../template.typ": *
#show: note-template

#set document(title: "Pell 方程", author: "Arch User")

= Pell 方程

Pell 方程的标准形式是
$ x^2 - d y^2 = 1 $
其中 $d$ 为非平方正整数。

#theorem[
  当 $d$ 不是平方数时，Pell 方程有无穷多组整数解。
  其基本解通常可由 $sqrt(d)$ 的连分数展开求出。
]

#remark[
  一旦找到最小正解，其他解通常可以通过二次域中的乘法结构递推得到。
]
