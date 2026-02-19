#version 450

layout(location = 0) out vec4 outColor;

void main() {
    // 输出纯红色 RGBA (1.0, 0.0, 0.0, 1.0)
    outColor = vec4(1.0,1.0, 0.4, 1.0);
}
