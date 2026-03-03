from struct import pack

from scapy.all import Raw, rdpcap, sniff, sr1, srp
from scapy.layers.inet import ICMP, IP, TCP, UDP
from scapy.layers.l2 import ARP, Ether

packet = IP(dst="192.168.1.1") / UDP(dport=80) / b"I use Nixos btw"
# packet.show()
packet = IP(dst="192.168.1.1") / TCP(dport=80, flags="S") / b"I use Arch btw"
# packet.show()

# sr(IP(dst="172.20.29.5/30")/TCP(dport=[21,22,23]),inter=0.5,retry=-2,timeout=1)

# SYN Scans
# sr1(IP(dst="72.14.207.99")/TCP(dport=80,flags="S"))
# sr(IP(dst="192.168.1.1")/TCP(sport=666,dport=(440,443),flags="S"))
# sr(IP(dst="192.168.1.1")/TCP(sport=RandShort(),dport=[440,441,442,443],flags="S"))
# ans.summary( lambda s,r: r.sprintf("%TCP.sport% \t %TCP.flags%") )
# srloop(IP(dst="www.target.com/30")/TCP())


def myread():
    packets = rdpcap("./error_reporting.pcap")

    for packet in packets:
        if packet.haslayer(TCP) and packet.haslayer(Raw):
            payload = packet[Raw].load
            if b"flag{" in payload:
                print(payload)


# 发送刚才构造的 packet 并等待 1 个响应 (sr1 = send and receive 1)
response = sr1(packet, timeout=2)


# 嗅探特定网卡上的 ICMP 流量
def packet_callback(pkt):
    print(pkt.summary())


sniff(iface="enp7s0", filter="icmp", prn=packet_callback, count=10)
