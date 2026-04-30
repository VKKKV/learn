#import "../template.typ": *
#show: note-template

#set document(title: "Meissel-Lehmer 算法", author: "Arch User")

= Meissel-Lehmer 算法

Meissel-Lehmer 算法用于计算素数计数函数
$upright("pi")(n)$。
它比朴素筛法更适合大范围计数。

#remark[
  核心是把“不超过 $n$ 的素数个数”拆成若干递归子问题，
  并用记忆化和预处理避免重复计算。
]
