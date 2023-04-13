#version 450

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 matrix;
    vec4 ambient_light_color;
    vec3 light_pos;
    vec4 light_color;
} ubo;

layout(location = 0)  in vec4 frag_color;
layout(location = 1)  in vec3 frag_pos;
layout(location = 2)  in vec3 frag_normal;

layout(location = 0) out vec4 out_color;

void main() {
    vec3 direction = ubo.light_pos - frag_pos;
    float attenuation = 1.0 / dot(direction, direction);

    float diff = max(dot(normalize(frag_normal), normalize(direction)), 0.0);

    vec4 light_color = ubo.light_color * attenuation;
    vec4 diffuse = light_color * diff;

    vec4 color = diffuse + ubo.ambient_light_color;

    out_color = vec4(color.xyz, 1.0) * color.w * frag_color;
}