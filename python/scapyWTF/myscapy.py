from struct import pack

from scapy.all import sr1, rdpcap, sniff, Raw
from scapy.all import sniff, sr1, srp
from scapy.layers.inet import ICMP, IP, UDP, TCP
from scapy.layers.l2 import ARP, Ether

packet = IP(dst="192.168.1.1") / UDP(dport=80) / b"I use Nixos btw"
# packet.show()
packet = IP(dst="192.168.1.1") / TCP(dport=80, flags="S") / b"I use Arch btw"
# packet.show()


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
