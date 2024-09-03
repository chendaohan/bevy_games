#import bevy_pbr::forward_io::VertexOutput

@fragment fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let middle = vec3f(0.03983, 0.25513, 0.09821);
    var color = vec3f(0.);
    if mesh.uv.y < 0.577 {
        color = mix(vec3f(0.1095, 0.98535, 0.45923), middle, mesh.uv.y / 0.577);
    } else {
        color = mix(middle, vec3f(0.1083, 0.15125, 0.01605), (mesh.uv.y - 0.577) / 0.423);
    }
    return vec4f(color, 1.);
}