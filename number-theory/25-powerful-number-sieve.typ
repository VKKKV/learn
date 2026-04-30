#import "@local/math-note:0.1.0": *
#show: note-template

#set document(title: "Powerful Number 筛", author: "Arch User")

= Powerful Number 筛

Powerful Number 指的是每个质因子的指数都至少为 $2$ 的数。
围绕这类数构造的筛法常用于更快地求某些积性函数前缀和，因为它能把“真正需要枚举的结构”压缩到更稀疏的集合上。

== 定义

#definition[
  若对每个素数 $p$，只要 $p divides n$ 就有
  $ p^2 divides n $
  则称 $n$ 为 powerful number。
]

#example[
  $1, 4, 8, 9, 16, 25, 27, 36$ 都是 powerful number；
  $12 = 2^2 3$ 不是，因为素因子 $3$ 的指数只有 $1$。
]

== 为什么会出现

#remark[
  许多积性函数在拆成“平方自由部分”和“高次部分”后，会自然把主贡献集中到 powerful number 上。
  由于这类数比全部正整数稀疏得多，算法就有机会更快。
]

== 基本结构

#proposition[
  每个 powerful number 都可以写成
  $ a^2 b^3 $
  的形式，其中 $a, b$ 为正整数。
]

#remark[
  这说明 powerful number 的结构并不随意，它本质上由“每个素因子的指数至少为 $2$”这一限制强力约束。
  算法里常利用这点做有界枚举。
]

== 与筛法的关系

#remark[
  Powerful Number 筛通常不是“从零开始”的第一种筛法，而是在已经熟悉线性筛、数论分块、卷积和前缀和技巧之后，为了进一步压缩状态数而使用。
]

#proposition[
  这类筛法的核心通常是：

  - 先将函数贡献按素因子指数分类
  - 再把“低指数难处理”的部分吸收到一个更稀疏的枚举对象中
  - 最后对剩余部分配合前缀和或分块处理
]

== 学习建议

#remark[
  学这部分时，不要先记具体模板代码。
  更重要的是先理解：为什么把问题限制到 powerful number 上之后，枚举规模会明显下降。
]

#exercise[
  列出不超过 $50$ 的所有 powerful number。
]
