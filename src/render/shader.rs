pub const CRT_VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    color = color0;
    uv = texcoord;
}
"#;

pub const CRT_FRAGMENT: &str = r#"#version 100
precision lowp float;

varying vec2 uv;
varying vec4 color;
uniform sampler2D Texture;

void main() {
    vec2 crt_uv = uv - vec2(0.5);
    // bend screen
    crt_uv *= 1.0 + pow((abs(crt_uv.yx) / 4.0), vec2(2.5));
    crt_uv += vec2(0.5);

    if (crt_uv.x < 0.0 || crt_uv.x > 1.0 || crt_uv.y < 0.0 || crt_uv.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        vec4 tex_color = texture2D(Texture, crt_uv);
        float scanline = sin(crt_uv.y * 800.0) * 0.04;
        tex_color.rgb -= scanline;
        
        float dist = distance(uv, vec2(0.5));
        tex_color.rgb *= smoothstep(0.9, 0.4, dist);
        
        gl_FragColor = tex_color * color;
    }
}
"#;
