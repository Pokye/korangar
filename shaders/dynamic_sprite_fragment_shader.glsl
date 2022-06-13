#version 450

layout(location = 0) in vec2 texture_coordinates;

layout(location = 0) out vec4 fragment_color;

layout (set = 0, binding = 0) uniform sampler2D sprite_texture;

layout(push_constant) uniform Constants {
    vec2 screen_position;
    vec2 screen_size;
    vec2 texture_position;
    vec2 texture_size;
    vec3 color;
} constants;

void main() {
    fragment_color = texture(sprite_texture, texture_coordinates) * vec4(constants.color, 1.0);

    //fragment_color.r = pow(fragment_color.r, 1.75);
    //fragment_color.g = pow(fragment_color.g, 1.75);
    //fragment_color.b = pow(fragment_color.b, 1.75);
}
