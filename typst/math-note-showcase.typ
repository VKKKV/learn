#import "template.typ": *

#show: note-template

#set document(
  title: "综合数学笔记示例",
  author: "Arch Btw",
)

= 综合数学笔记示例

这是一份基于 `template.typ` 的数学笔记示例。
模板层内置了常用数学语义块与若干记号宏，
这里直接展示不同学科中的典型公式、符号与交叉引用写法。

== 使用说明

#remark[
  这份文件本身就是一个可直接复用的起点：

  - `#show: note-template` 负责继承统一页面模板。
  - `definition / theorem / proposition / lemma / corollary / example / proof / remark / exercise / solution` 负责语义化区块。
  - `E`、`P`、`Var`、`Cov`、`Normal`、`ket`、`bra`、`braket` 等宏由模板统一提供。
  - 块级公式已经开启自动编号，因此可以直接使用标签和 `@引用`。
]

== 数论

#definition[
  设 $a, b in ZZ$，$n in NN$。若
  $ a equiv b pmod(n) $ <def-congruence>
  则称 $a$ 与 $b$ 模 $n$ 同余。

  这等价于
  $ n divides (a - b) $ <criterion-congruence>
]

#theorem[
  若 $a equiv b pmod(n)$ 且 $c equiv d pmod(n)$，则
  $ a + c equiv b + d pmod(n) $ <cong-add>
  且
  $ a c equiv b d pmod(n) $ <cong-mul>
]

#proof[
  由 $a equiv b pmod(n)$ 与 $c equiv d pmod(n)$，根据 @criterion-congruence 可知
  $ n divides (a - b) $
  且
  $ n divides (c - d) $

  两式相加得到
  $ n divides ((a + c) - (b + d)) $
  从而得到 @cong-add。

  另一方面，
  $ a c - b d = a(c - d) + d(a - b) $
  右侧两项都能被 $n$ 整除，因此可得 @cong-mul。
]
