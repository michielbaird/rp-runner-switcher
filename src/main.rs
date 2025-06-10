use clap::{Parser, ValueEnum};
use std::process::{exit, Command};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Chip {
    #[default]
    RP2040,
    RP2350A
}

#[derive(Parser)]
struct Args {
    #[arg(long, value_enum)]
    chip: Chip,

    /// ELF binary produced by cargo
    elf_path: String,

    #[arg(long, default_value_t = false )]
    use_probe: bool,


}

fn main() {
    let args = Args::parse();
    let status = match (args.chip, args.use_probe) {
        (Chip::RP2040, false) => {
            Command::new("elf2uf2-rs")
                .args(["--deploy", "--serial", "--verbose"])
                .arg(&args.elf_path)
                .status()
        }
        (Chip::RP2040, true) => {
            Command::new("probe-rs")
                .arg("run")
                .arg("--chip")
                .arg("RP2040")
                .arg("--")
                .arg(&args.elf_path)
                .status()
        },
        (Chip::RP2350A, true) => {
            Command::new("probe-rs")
                .arg("run")
                .arg("--chip")
                .arg("RP2350A")
                .arg("--")
                .arg(&args.elf_path)
                .status()
        }
        (Chip::RP2350A, false) => {
            Command::new("elf2uf2-rs")
                .args(["--deploy", "--family", "0xe48bff59",  "--serial", "--verbose"])
                .arg(&args.elf_path)
                .status()
        },
    }
    .expect("failed to run external tool");
    

    exit(status.code().unwrap_or(1));
}
