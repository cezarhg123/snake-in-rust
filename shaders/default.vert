#version 460 core

in ivec2 position;
in vec3 color;

out vec3 Color;

void main() {
    Color = color;
    vec2 pos;
    pos.x = position.x / 400.0;
    pos.x -= 1.0;
    pos.y = position.y / 400.0;
    float loc = 2.0 - pos.y;
    pos.y = loc - 1.0;
    gl_Position = vec4(pos, 0.0, 1.0);
}