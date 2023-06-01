# Meenle-Noonle
[Meenle-Noonle](https://kooshnoo.github.io/meenle-noonle/) is a 3D wireframe software renderer, written in Rust. 
It runs entirely on the CPU, and does not use the GPU. 
The web demo runs with no dependencies at all, 
but the ports for desktop and Nintendo Wii use platform libraries `RVL_SDK` and `sdl`.

## Directory structure
The rust source for the main library business logic is in `src/lib.rs`. 
`rs-ppc-support` is a helper crate for running Meenle-Noonle on the Wii, 
since the rust standard library is not available for that target. 
`meenle-noonle-v1` is the previous version of the project, reliant on the web Canvas 2D api.
`meenle-noonle-sdl` is the crate for the desktop ports.

| File Name             | Description             |
|-----------------------|-------------------------|
| index.html            | Website source          |
| index.js              | Website JS              |
| meenle-noonle.wasm    | WASM binary             |
| meenle-noonle-rvl.cpp | Wii source code         |
| meenle-noonle-rvl.elf | Wii debug executable    |
| meenle-noonle-rvl.dol | Wii homebrew executable |
| meenle-noonle-sdl     | Mac executable          |
| meenle-noonle-sdl.exe | Windows executable      |

## Building
Install rust, then: `cargo r`