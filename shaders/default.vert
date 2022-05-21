#version 460 core

in ivec2 position;
in vec3 color;

out vec3 Color;

void main() {
    Color = color;
    vec2 pos;
    pos.x = position.x / 640.0;
    pos.x -= 1.0;
    pos.y = position.y / 360.0;
    pos.y -= 0.0;
    gl_Position = vec4(pos, 0.0, 1.0);
}