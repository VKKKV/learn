#ifdef GL_ES
precision mediump float;
#endif

// 这些是 glslViewer 自动为你注入的魔法变量 (Uniforms)
uniform vec2 u_resolution; // 窗口的宽高
uniform float u_time;      // 程序运行的时间（秒）

void main() {
    // 将像素坐标归一化到 0.0 到 1.0 之间
    vec2 st = gl_FragCoord.xy / u_resolution.xy;

    // 用坐标计算 RG (红绿)，用时间的正弦值计算 B (蓝)
    float r = st.x;
    float g = st.y;
    float b = abs(sin(u_time * 2.0)); // 蓝色会随着时间呼吸闪烁
    // float b = 1.0 // stop

    // 输出颜色
    gl_FragColor = vec4(r, g, b, 1.0);
}
