#version 140

in vec3 pos;
in vec2 uv;
in vec3 color;

//uniform vec3 model;
uniform mat4 projection, view;

out vec3 v_color;

void main() {
    v_color = color;
    gl_Position = projection * view * vec4(pos, 1.0);
}
