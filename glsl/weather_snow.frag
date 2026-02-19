#ifdef GL_ES
precision mediump float;
#endif

// 注入灵魂：glslViewer 的内置变量
uniform vec2 u_resolution;
uniform float u_time;
uniform sampler2D u_tex0; // <-- 你的背景图片通道

const float cornerRadius = 30.0;
const float opacity = 1.0;

// SDF: 生成圆角矩形遮罩
float roundedBoxSDF(vec2 center, vec2 size, float radius) {
    vec2 q = abs(center) - size + radius;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radius;
}

void main() {
    // 【关键点 1】：干净的纹理坐标 (st)，范围严丝合缝地贴在 0.0 到 1.0 之间
    vec2 st = gl_FragCoord.xy / u_resolution.xy;

    // 【关键点 2】：被长宽比污染过的物理坐标 (uv)，专门留给雪花用
    vec2 uv = st;
    float aspect = u_resolution.x / u_resolution.y;
    uv.x *= aspect;

    float iTime = u_time * 0.15;
    float snow = 0.0;

    // 暴力美学：生成满天飞雪 (使用拉伸过的 uv)
    for (int k = 0; k < 6; k++) {
        for (int i = 0; i < 12; i++) {
            float cellSize = 2.0 + (float(i) * 3.0);
            float downSpeed = 0.3 + (sin(iTime * 0.4 + float(k + i * 20)) + 1.0) * 0.00008;

            vec2 uvAnim = uv + vec2(
                0.01 * sin((iTime + float(k * 6185)) * 0.6 + float(i)) * (5.0 / float(i + 1)),
                downSpeed * (iTime + float(k * 1352)) * (1.0 / float(i + 1))
            );

            vec2 uvStep = (ceil((uvAnim) * cellSize - vec2(0.5, 0.5)) / cellSize);
            float x = fract(sin(dot(uvStep.xy, vec2(12.9898 + float(k) * 12.0, 78.233 + float(k) * 315.156))) * 43758.5453 + float(k) * 12.0) - 0.5;
            float y = fract(sin(dot(uvStep.xy, vec2(62.2364 + float(k) * 23.0, 94.674 + float(k) * 95.0))) * 62159.8432 + float(k) * 12.0) - 0.5;

            float randomMagnitude1 = sin(iTime * 2.5) * 0.7 / cellSize;
            float randomMagnitude2 = cos(iTime * 1.65) * 0.7 / cellSize;

            float d = 5.0 * distance((uvStep.xy + vec2(x * sin(y), y) * randomMagnitude1 + vec2(y, x) * randomMagnitude2), uvAnim.xy);

            float omiVal = fract(sin(dot(uvStep.xy, vec2(32.4691, 94.615))) * 31572.1684);
            if (omiVal < 0.03) {
                float newd = (x + 1.0) * 0.4 * clamp(1.9 - d * (15.0 + (x * 6.3)) * (cellSize / 1.4), 0.0, 1.0);
                snow += newd;
            }
        }
    }

    // 【关键点 3】：用干净的 st 坐标去采样图片，防止图片被拉长！
    vec4 texColor = texture2D(u_tex0, st);

    // 混合背景图与雪花
    float snowAlpha = clamp(snow * 2.0, 0.0, 1.0);
    vec3 snowColor = vec3(1.0); // 纯白雪花
    vec3 blended = mix(texColor.rgb, snowColor, snowAlpha);

    // 计算圆角遮罩
    vec2 halfSize = u_resolution.xy * 0.5;
    vec2 center = gl_FragCoord.xy - halfSize;
    float dist = roundedBoxSDF(center, halfSize, cornerRadius);
    float cornerMask = 1.0 - smoothstep(-1.0, 0.0, dist);

    // 最终输出：图片背景 + 雪花 + 圆角裁切
    float finalAlpha = opacity * cornerMask;
    gl_FragColor = vec4(blended * finalAlpha, finalAlpha);
}
