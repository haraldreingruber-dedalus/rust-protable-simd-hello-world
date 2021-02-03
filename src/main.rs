use core_simd::*;

// These libs would also be interesting, not sure if they support WASM:
// https://github.com/AdamNiederer/faster
// https://github.com/Lokathor/wide

// This example is inspired by https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

fn main() {
    let result = simd_computation();

    println!("{:?}", result);
}

fn simd_computation() -> [f32; 4] {
// checkout the API docs here: https://rust-lang.github.io/stdsimd/core_simd/

    // initialize vectors
    let x = f32x4::from_array([1f32, 2f32, 3f32, 4f32]);
    let y = f32x4::from_array([2f32, 3f32, 4f32, 5f32]);

    // sum
    let z = x + y;

    // return result
    return z.to_array();
}