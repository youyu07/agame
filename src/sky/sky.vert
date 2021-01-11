#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 0) out vec3 v_pos;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    vec4 pos = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_pos = pos.xyz;
    gl_Position = pos.xyww;
}