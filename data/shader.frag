#version 140

out vec4 out_color;

uniform sampler2D s_texture;

in vec3 v_color;
in vec2 v_uv;

void main() {
    out_color = texture(s_texture, v_uv) * vec4(v_color, 1.0);
}
