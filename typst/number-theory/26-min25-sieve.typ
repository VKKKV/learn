#import "../template.typ": *
#show: note-template

#set document(title: "Min_25 筛", author: "Arch User")

= Min_25 筛

Min_25 筛是一类用于求质数相关前缀和的高级算法，常见于
$sum f(p)$
以及积性函数前缀和问题。

#remark[
  它通常利用：

  - 小于等于 $sqrt(n)$ 的素数预处理。
  - 对 $floor(n / i)$ 的不同取值建状态。
  - 把“筛掉前若干个素数”的过程写成递推。
]
