// 设置页面布局
#set page(
  paper: "a4",
  margin: (left: 2.5cm, right: 2.5cm, top: 2.5cm, bottom: 2.5cm)
)

#set text(font: "Noto Sans CJK SC", lang: "zh", region: "cn", size: 12pt)

#set heading(numbering: "1. ")
#let numbered_eq(content) = math.equation(
    block: true,
    numbering: "(1)",
    content,
    )

= 我的第一个 Typst 文档
#lorem(10)

// 数学公式示例
$ x^2 + y^2 = z^2 $

restaurant

文字默认是左对齐的。

#align(center)[但我想临时居中！]

而不影响其他文字。

$E = m c^2 $

#set align(left)
这行字左对齐

#set align(right)
这行字右对齐

#set align(center)
这行字居中

+ hello
  - hello
+ hello
4. hello
+ hello


#import "@preview/numbly:0.1.0": numbly
#set heading(numbering: numbly(
  "{1:一}、",
  "{2:1}.",
  "{3:a}.",
  "{4:i}.",
))
= 一级标题
我走了。
== 二级标题
我来了。
== 二级标题
我来了吗？
=== 三级标题
我走了又来了。

$

$


webapp forever
