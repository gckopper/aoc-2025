// A read-only storage buffer that stores and array of unsigned 32bit integers
@group(0) @binding(0) var<storage, read> input: array<u32>;
// This storage buffer can be read from and written to
@group(0) @binding(1) var<storage, read_write> output: array<u32>;

// Tells wgpu that this function is a valid compute pipeline entry_point
@compute
// Specifies the "dimension" of this work group
@workgroup_size(1, 1, 1)
fn main(
    // global_invocation_id specifies our position in the invocation grid
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>,
    @builtin(num_workgroups) num_workgroups: vec3<u32>
) {
    let line = global_invocation_id.x;
    let column = global_invocation_id.y;
    let total = arrayLength(&input);

    let left_wall = global_invocation_id.x > 0;
    let right_wall = global_invocation_id.x + 1 < num_workgroups.x;
    let ceiling = global_invocation_id.y > 0;
    let floor = global_invocation_id.y + 1 < num_workgroups.y;
    
    let v_0_0 = u32(left_wall && ceiling &&
        64 == input[(global_invocation_id.x - 1) + (global_invocation_id.y - 1) * num_workgroups.x]);

    let v_0_1 = u32(ceiling &&
        64 == input[global_invocation_id.x       + (global_invocation_id.y - 1) * num_workgroups.x]);
        
    let v_0_2 = u32(right_wall && ceiling &&
        64 == input[(global_invocation_id.x + 1) + (global_invocation_id.y - 1) * num_workgroups.x]);

    let v_1_0 = u32(left_wall &&
        64 == input[(global_invocation_id.x - 1) +  global_invocation_id.y      * num_workgroups.x]);

    let v_1_1 = input[ global_invocation_id.x      +  global_invocation_id.y      * num_workgroups.x];

    let v_1_2 = u32(right_wall &&
        64 == input[(global_invocation_id.x + 1) +  global_invocation_id.y      * num_workgroups.x]);

    let v_2_0 = u32(left_wall && floor &&
        64 == input[(global_invocation_id.x - 1) + (global_invocation_id.y + 1) * num_workgroups.x]);

    let v_2_1 = u32(floor &&
        64 == input[ global_invocation_id.x      + (global_invocation_id.y + 1) * num_workgroups.x]);

    let v_2_2 = u32(right_wall && floor &&
        64 == input[(global_invocation_id.x + 1) + (global_invocation_id.y + 1) * num_workgroups.x]);

    // a simple copy operation
    output[global_invocation_id.x + global_invocation_id.y * num_workgroups.x] = 64 * u32(v_1_1 == 64 && (v_0_0 + v_0_1 + v_0_2 + v_1_0 + v_1_2 + v_2_0 + v_2_1 + v_2_2) < 4);
}

