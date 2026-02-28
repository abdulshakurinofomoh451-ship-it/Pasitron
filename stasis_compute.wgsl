struct Params {
    dt: f32,
    magnetic_strength: f32,
};

@group(0) @binding(0) var<storage, read_write> positions : array<vec4<f32>>;
@group(0) @binding(1) var<uniform> params : Params;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let i = gid.x;
    if (i >= arrayLength(&positions)) {
        return;
    }

    var p = positions[i];

    // Simple circular motion as Lorentz-force analogue
    let angle = params.dt * params.magnetic_strength;
    let cosA = cos(angle);
    let sinA = sin(angle);

    let x = p.x * cosA - p.y * sinA;
    let y = p.x * sinA + p.y * cosA;

    positions[i] = vec4<f32>(x, y, p.z, 1.0);
}
