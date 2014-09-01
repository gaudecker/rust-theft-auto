#version 140

in vec3 pos;
in vec2 uv;
in vec3 color;

uniform mat4 projection, view;

out vec3 v_color;
out vec2 v_uv;

void main() {
    v_color = color;
    v_uv = uv;
    gl_Position = projection * view * vec4(pos, 1.0);
}
