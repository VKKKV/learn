# Flag

## TCP Flags (控制位)

| Flag | 名称 | 作用 |
| ---- | ---- | ---- |
| **SYN** | Synchronize | 同步序列号，TCP 三次握手的第一步 |
| **ACK** | Acknowledge | 确认收到数据 |
| **FIN** | Finish | 正常关闭连接 |
| **RST** | Reset | 强制重置连接 |
| **PSH** | Push | 强制推送数据到应用层 |
| **URG** | Urgent | 紧急数据指针 |

### PSH 标志详解

- **内核机制**：默认情况下，Linux 网络栈使用缓冲区提高效率，攒够数据或超时才发送
- **PSH 作用**：强制命令"立刻把数据推给应用层"，不做缓冲
- **实战意义**：交互式程序（SSH、反弹 Shell）中每个数据包都带 PSH，CTF 流量分析中看到连续 PSH 包通常意味着实时命令交互

---

## UDP 报头

只有四个字段：`Source Port`, `Destination Port`, `Length`, `Checksum`

- 不保证到达，不重传，不排序
- 无连接，不需要 TCP 那种 Flag 同步机制

---

## Scapy 字段说明

### IP Layer
- `src`: 源地址
- `dst`: 目标地址（通常是网关）
- `proto`: 协议类型 (tcp/udp)
- `ttl`: 生存时间

### TCP/UDP Layer
- `sport`: 源端口
- `dport`: 目标端口
- `flags`: TCP 标志（如 `S` 表示 SYN）
- `seq`: 序列号

---

## Raw Layer

`Raw` = 原始未解析的二进制数据

Scapy 解析流量包时按层剥离：`Ethernet -> IP -> TCP/UDP`。如果无法解析上层协议（如无对应解析器或自定义数据），剩余字节会放入 `Raw` 层。

使用 `pkt[Raw].load` 可直接提取负载进行二进制匹配，比 Wireshark 追踪 TCP 流更高效。
