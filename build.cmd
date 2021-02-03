REM Use --debug if you want to verify the simd instructions in the disassembly. In the release version the const computation is optimized quite a bit :)
wasm-pack build . --target web --debug