#import "../template.typ": *
#show: note-template

#set document(title: "Powerful Number 筛", author: "Arch User")

= Powerful Number 筛

Powerful Number 指的是每个质因子的指数都至少为 $2$ 的数。
围绕这类数构造的筛法常用于更快地求某些积性函数和。

#definition[
  若对每个素数 $p$，只要 $p divides n$ 就有
  $ p^2 divides n $
  则称 $n$ 为 powerful number。
]

#remark[
  这类筛法的关键是把整数分解成“平方核”和“剩余部分”，从而缩小枚举规模。
]
