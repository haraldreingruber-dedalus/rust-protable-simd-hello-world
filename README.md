# Rust Portable SIMD "Hello World" Example

This is a basic example of how to get started with [Rust's standard library portable SIMD API](https://github.com/rust-lang/stdsimd).

Currently, this is tested for the following targets:
* `x86_64-pc-windows-msvc`
* `wasm32-unknown-unknown`

*Note:* The API's are all **very expiermental** and you have to use Rust's nightly toolchain. 
Also, WebAssembly SIMD is very experimental and is deactivated by default if supported at all. 
See [Implementation Status](https://github.com/WebAssembly/simd/blob/master/proposals/simd/ImplementationStatus.md) of the Browsers.
