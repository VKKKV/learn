#import "../template.typ": *
#show: note-template

#set document(title: "离散对数", author: "Arch User")

= 离散对数

离散对数是连续对数在有限乘法群里的对应物。
它在理论上依赖原根与群结构，在算法上与 BSGS、Pollard rho 等方法相连，也是现代密码学中的核心难题之一。

== 定义

#definition[
  若模 $n$ 的乘法群由 $g$ 生成，且
  $ g^x equiv a pmod(n) $ <dlog-def>
  则称 $x$ 是 $a$ 关于底 $g$ 的离散对数。
]

#remark[
  离散对数存在的前提是 $a$ 必须落在由 $g$ 生成的循环子群中。
  若 $g$ 是原根，则所有与 $n$ 互素的剩余类都可表示为某个 $g^x$。
]

== 与普通对数的类比

#proposition[
  在形式上，离散对数把乘法问题转成加法问题：

  - 若 $a equiv g^u pmod(n)$，$b equiv g^v pmod(n)$，则
    $ a b equiv g^(u + v) pmod(n) $
  - 因而乘法同余有时可以转成指数的线性同余
]

#remark[
  这就是为什么原根和离散对数在研究指数同余方程时特别有用。
]

== 例子

#example[
  在模 $7$ 下，$3$ 是原根，因为
  $ 3^1 equiv 3,
     3^2 equiv 2,
     3^3 equiv 6,
     3^4 equiv 4,
     3^5 equiv 5,
     3^6 equiv 1 pmod(7) $

  所以
  $ log_3 5 equiv 5 mod 6 $
  因为
  $ 3^5 equiv 5 pmod(7) $
]

== 典型问题

#proposition[
  离散对数常用于处理：

  - 指数同余方程 $a^x equiv b pmod(p)$
  - 原根和阶相关问题
  - 椭圆曲线和有限域上的密码系统安全分析
]

== 算法思路

#remark[
  理论上“存在”并不代表“容易求”。
  朴素枚举需要线性时间，而 BSGS 可以降到大约 $O(sqrt(n))$ 级别。
  Pollard rho for log 则进一步优化了空间或随机性能。
]

#remark[
  离散对数难题之所以重要，是因为在大模数或大有限域上，目前没有已知的高效通用算法能像普通实数对数那样轻松求出答案。
]

#exercise[
  在模 $11$ 下验证 $2$ 是原根，并求出
  $ log_2 7 $
]
