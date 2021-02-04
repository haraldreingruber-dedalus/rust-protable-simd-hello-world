![SIMD Hello World](https://github.com/haraldreingruber-dedalus/rust-protable-simd-hello-world/workflows/SIMD%20Hello%20World/badge.svg)

# Rust Portable SIMD "Hello World" Example

This is a basic example of how to get started with [Rust's standard library portable SIMD API](https://github.com/rust-lang/stdsimd).

Currently, this is tested for the following targets:
* `x86_64-unknown-linux-gnu`
* `x86_64-pc-windows-msvc`
* `x86_64-pc-windows-gnu` (cross-compiled from Linux)
* `x86_64-apple-darwin`
* `wasm32-unknown-unknown`

But it's likely to work on other platforms as well.

Only for `wasm32-unknown-unknown` it is verified that the disassembly contains the SIMD instructions.

*Note:* The API's are all **very expiermental** and you have to use Rust's nightly toolchain. 
Also, WebAssembly SIMD is very experimental and is deactivated by default if supported at all. 
See [Implementation Status](https://github.com/WebAssembly/simd/blob/master/proposals/simd/ImplementationStatus.md) of the Browsers.
