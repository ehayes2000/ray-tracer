// useful docs
// wgpu: https://docs.rs/wgpu/latest
// wgsl: https://www.w3.org/TR/WGSL
// tour of wgsl: https://google.github.io/tour-of-wgsl
// learn wgpu: https://sotrh.github.io/learn-wgpu
// wgsl fundementals: https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl-function-reference.html
// WGSL doesn't use Rust or C layout. This package provides a trait to align
// structs for WGSL and some types to create, read, and write buffers

pub mod app;
mod bvh;
pub mod mesh;
pub mod scene;
pub mod state;
pub mod types;
