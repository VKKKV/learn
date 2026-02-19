#ifdef GL_ES
precision mediump float;
#endif

// glslViewer 自动注入的全局变量
uniform vec2 u_resolution;
uniform float u_time;
uniform sampler2D u_tex0; // 你的输入图片

// KISS: 砍掉冗杂的 Qt UBO，将变量化为常量
const float qt_Opacity = 1.0;
const float cornerRadius = 40.0; // 圆角大小 (像素)

// SDF (有向距离场) 生成圆角矩形
float roundedBoxSDF(vec2 center, vec2 size, float radius) {
    vec2 q = abs(center) - size + radius;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radius;
}

// 伪随机哈希
vec3 hash3(vec2 p) {
    vec3 q = vec3(dot(p, vec2(127.1, 311.7)),
                  dot(p, vec2(269.5, 183.3)),
                  dot(p, vec2(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}

// 核心水波纹噪声生成
float noise(vec2 x, float iTime) {
    vec2 p = floor(x);
    vec2 f = fract(x);
    float va = 0.0;

    for (int j = -2; j <= 2; j++) {
        for (int i = -2; i <= 2; i++) {
            vec2 g = vec2(float(i), float(j));
            vec3 o = hash3(p + g);
            vec2 r = g - f + o.xy;
            float d = length(r);
            float ripple = max(mix(smoothstep(0.99, 0.999, max(cos(d - iTime * 2.0 + (o.x + o.y) * 5.0), 0.0)), 0.0, d), 0.0);
            va += ripple;
        }
    }
    return va;
}

void main() {
    // 1. 原生计算 UV 坐标
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    float iTime = u_time * 0.07;

    // 2. 长宽比校正
    float aspect = u_resolution.x / u_resolution.y;
    vec2 uvAspect = vec2(uv.x * aspect, uv.y);

    // 3. 基础噪声频率
    float f = noise(6.0 * uvAspect, iTime) * smoothstep(0.0, 0.2, sin(uv.x * 3.141592) * sin(uv.y * 3.141592));

    // 4. 计算法线以进行扭曲 (Distortion)
    float normalScale = 0.5;
    vec2 e = normalScale / u_resolution.xy;
    vec2 eAspect = vec2(e.x * aspect, e.y);
    float cx = noise(6.0 * (uvAspect + eAspect), iTime) * smoothstep(0.0, 0.2, sin((uv.x + e.x) * 3.141592) * sin(uv.y * 3.141592));
    float cy = noise(6.0 * (uvAspect + eAspect.yx), iTime) * smoothstep(0.0, 0.2, sin(uv.x * 3.141592) * sin((uv.y + e.y) * 3.141592));
    vec2 n = vec2(cx - f, cy - f);

    // 将扭曲缩放回纹理空间
    vec2 distortion = vec2(n.x / aspect, n.y);

    // 5. 采样扭曲后的图像 (注意这里用 texture2D 兼容性最好)
    vec4 col = texture2D(u_tex0, uv + distortion);

    // 6. 施加圆角遮罩
    vec2 pixelPos = gl_FragCoord.xy;
    vec2 halfSize = u_resolution.xy * 0.5;
    vec2 center = pixelPos - halfSize;
    float dist = roundedBoxSDF(center, halfSize, cornerRadius);
    float cornerMask = 1.0 - smoothstep(-1.0, 0.0, dist);

    // 7. 输出最终颜色 (带预乘 Alpha)
    float finalAlpha = col.a * qt_Opacity * cornerMask;
    gl_FragColor = vec4(col.rgb * finalAlpha, finalAlpha);
}
