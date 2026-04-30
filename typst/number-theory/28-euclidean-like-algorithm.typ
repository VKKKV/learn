#import "../template.typ": *
#show: note-template

#set document(title: "类欧几里德算法", author: "Arch User")

= 类欧几里德算法

类欧几里德算法常用于快速求和形如
$sum floor((a i + b) / c)$
的表达式。

#remark[
  它的精神与欧几里得算法类似：
  通过交换参数、取整分解和递归降维，让问题规模快速下降。
]
