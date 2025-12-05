import numpy as np
import matplotlib.pyplot as plt

# 定义 x 的范围和步长
x = np.linspace(0, 2 * np.pi, 1000)

# 计算 cos(x) 和 sin(x) 的值
y_cos = np.cos(x)
y_sin = np.sin(x)


# 绘制 cos(x) 和 sin(x) 的图像
plt.figure(figsize=(10, 5))
plt.plot(x, y_cos, label="cos(x)")
plt.plot(x, y_sin, label="sin(x)")
plt.title("Graph of cos(x) and sin(x)")
plt.xlabel("x")
plt.ylabel("y")
plt.axhline(0, color="black", linewidth=0.5)
plt.axvline(0, color="black", linewidth=0.5)
plt.grid(color="gray", linestyle="--", linewidth=0.5)
plt.legend()
plt.show()
