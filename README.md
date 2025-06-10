# rp-runner-switcher

> A Cargo runner for RP2040 and RP2350 boards — auto-selects between `probe-rs` and `elf2uf2-rs` so you don’t have to.

## ✨ Features

- 📦 Acts as a drop-in runner for `.cargo/config.toml`
- 🔌 Detects chip via `--chip rp2040` / `--chip rp2350[a]`
- 🔁 Switches between `probe-rs` and `elf2uf2-rs`
- 🔧 Supports `--use-probe` to force probe use
- 🪛 Cross-platform and cargo-friendly

## 🛠️ Installation

Install globally, with it's dependencies (no need to include in your project’s `Cargo.toml`):

```sh
cargo install rp-runner-switcher
cargo install --git https://github.com/ninjasource/elf2uf2-rs --branch pico2-support
cargo install probe-rs
```

## ⚠️ Features & Dependencies Setup

To use `rp-runner-switcher` effectively, your embedded project should define **target-specific features** and set up dependencies accordingly.

### Example `Cargo.toml` (in your embedded project, using embassy)

```toml
[features]
rp2040 = ["embassy-rp/rp2040",]
rp2350 = ["embassy-rp/rp235xa", "embassy-rp/binary-info"]

[dependencies]
embassy-rp = { version = "...", optional = true, features = [..] }
```


## 🧱 Memory Layout Warning: `build.rs` Required

RP2040 and RP2350 use **different memory layouts**, so you’ll need to dynamically select the correct linker script in your project’s `build.rs`.

### Example logic:

```rust
// build.rs
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let memory_x = match target.as_str() {
        "thumbv6m-none-eabi" => "memory_rp2040.x",
        "thumbv8m.main-none-eabihf" => "memory_rp2350.x",
        _ => panic!("Unsupported target"),
    };

    fs::copy(memory_x, out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
```

This ensures the correct memory layout is used during linking based on your target triple. Don't forget to include both memory_rp2040.x and memory_rp2350.x in your project root!

### Additional Note for RP2040 Users

When targeting **RP2040**, you'll also need to explicitly set the linker script via `cargo:rustc-link-arg-bins`:

```rust
if target == "thumbv6m-none-eabi" {
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
}
```

This ensures the correct link-rp.x file is used when linking your binary. Some HALs require this explicitly instead of relying on memory.x.

Make sure link-rp.x is included in your project and correctly describes the RP2040 memory layout.

## Example

For a project setup example. See blinky: https://github.com/michielbaird/rp-runner-switcher/tree/main/examples/blinky_embassy