import matplotlib.pyplot as plt
import numpy as np
from matplotlib import font_manager

# 设置中文字体
font_path = "C:/Windows/Fonts/simsun.ttc"  # 替换为你的中文字体路径
font_prop = font_manager.FontProperties(fname=font_path)

# 数据准备
firewall_brands = ["Cisco", "Fortinet", "Palo Alto"]
firewall_prices = [1500, 1000, 1200]

ids_brands = ["Cisco", "Check Point", "Fortinet"]
ids_prices = [3000, 2500, 3200]

ips_brands = ["Cisco", "Palo Alto", "Fortinet"]
ips_prices = [4500, 4000, 3800]

# 分组数据
labels = ["防火墙", "入侵检测系统", "入侵防御系统"]
prices = [firewall_prices, ids_prices, ips_prices]

# 设置柱状图的宽度和位置
bar_width = 0.25
x = np.arange(len(labels))

# 绘制柱状图
fig, ax = plt.subplots(figsize=(10, 6))

# 绘制每组数据
for i, (brand_prices, brand_labels) in enumerate(
    zip(prices, [firewall_brands, ids_brands, ips_brands])
):
    ax.bar(x + i * bar_width, brand_prices, width=bar_width, label=brand_labels)

# 添加标签和标题
ax.set_xlabel("产品类型", fontproperties=font_prop)
ax.set_ylabel("售价 ($)", fontproperties=font_prop)
ax.set_title("各产品价格对比", fontproperties=font_prop)
ax.set_xticks(x + bar_width)
ax.set_xticklabels(labels, fontproperties=font_prop)
ax.legend(prop=font_prop)

# 显示图形
plt.tight_layout()
plt.show()
