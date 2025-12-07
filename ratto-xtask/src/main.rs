use std::{env, fmt::Display, path::PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about, long_about = None, bin_name = "cargo xtask")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build part or all of the operating system
    Build {
        /// Target config
        #[arg(short, long)]
        config_path: PathBuf,

        // Build profile
        #[arg(long, value_enum, default_value_t = Profile::Release)]
        profile: Profile,

        /// Virtualization provider to use
        #[arg(long)]
        virtualization: Option<VirtualizationProvider>,
    },
    /// Build and run the OS in an emulator
    Run {
        /// Target config
        #[arg(short, long)]
        config_path: PathBuf,

        // Build profile
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,

        /// Virtualization provider to use
        #[arg(short, long, value_enum, default_value_t = VirtualizationProvider::Qemu)]
        provider: VirtualizationProvider,

        /// Additional args to pass to the virtualization provider
        #[arg(last = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(ValueEnum, Debug, PartialEq, Clone, Copy)]
enum Profile {
    Debug,
    Release,
}

impl Profile {
    fn as_str(&self) -> &'static str {
        match self {
            Profile::Debug => "debug",
            Profile::Release => "release",
        }
    }
}

#[derive(ValueEnum, Debug, PartialEq, Clone, Copy)]
enum VirtualizationProvider {
    Qemu,
}

impl VirtualizationProvider {
    fn run_with_kernel(
        &self,
        kernel_image: &PathBuf,
        config: &Config,
        args: &[String],
    ) -> anyhow::Result<()> {
        match self {
            VirtualizationProvider::Qemu => {
                let mut cmd = match config.arch {
                    Architecture::AArch64 => {
                        let mut cmd = std::process::Command::new("qemu-system-aarch64");
                        if let Some(machine_type) = &config.machine_type {
                            cmd.args(&["-M", machine_type]);
                        }

                        // -semihosting -semihosting-config enable=on,target=native
                        cmd.args(&[
                            "-semihosting",
                            "-semihosting-config",
                            "enable=on,target=native",
                        ]);

                        cmd
                    }
                    Architecture::X86_64 => {
                        todo!();
                    }
                };

                // Add any additional args from the CLI
                cmd.args(args);

                cmd.arg("-kernel")
                    .arg(kernel_image)
                    .arg("-nographic")
                    .args(&["-serial", "mon:stdio"])
                    .args(&["-audio", "none"])
                    .status()
                    .context("Failed to run QEMU")?;
            }
        }

        Ok(())
    }
}

impl Display for VirtualizationProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_possible_value() {
            Some(val) => write!(f, "{}", val.get_name()),
            None => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
enum Architecture {
    AArch64,
    X86_64,
}

impl Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::AArch64 => write!(f, "aarch64"),
            Architecture::X86_64 => write!(f, "x86_64"),
        }
    }
}

impl Architecture {
    fn target_triple(&self) -> &str {
        match self {
            Architecture::AArch64 => "aarch64-unknown-none-softfloat",
            Architecture::X86_64 => "x86_64-unknown-none",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
struct Config {
    arch: Architecture,
    ld_path: PathBuf,
    machine_type: Option<String>,
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("[Error] {}", e);
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build {
            config_path,
            profile,
            virtualization,
        }) => {
            let config = load_config(config_path)?;
            build_kernel(&config, *profile, *virtualization)?;
        }
        Some(Commands::Run {
            config_path,
            provider,
            args,
            ..
        }) => {
            let config = load_config(config_path)?;
            let kernel_image = build_kernel(&config, Profile::Debug, Some(*provider))?;

            provider.run_with_kernel(&kernel_image, &config, args)?;
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }

    Ok(())
}

fn load_config(path: &PathBuf) -> anyhow::Result<Config> {
    let contents = std::fs::read_to_string(path).context(format!(
        "Unable to read configuration file: {}",
        path.display()
    ))?;

    let config = ron::from_str::<Config>(&contents).context(format!(
        "Unable to parse configuration file: {}",
        path.display()
    ))?;

    Ok(config)
}

fn build_kernel(
    config: &Config,
    profile: Profile,
    virtualization: Option<VirtualizationProvider>,
) -> anyhow::Result<PathBuf> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let workspace_root = {
        let root_from_env =
            env::var("CARGO_WORKSPACE_DIR").context("Unable to determine cargo workspace root")?;
        PathBuf::from(root_from_env)
    };

    let (ld_path, ld_script) = {
        let qualified_path = if config.ld_path.is_absolute() {
            config.ld_path.clone()
        } else {
            PathBuf::from(&workspace_root).join(&config.ld_path)
        };

        let ld_dir = qualified_path
            .parent()
            .context("Unable to determine linker script directory")?
            .to_owned();

        let ld_script = qualified_path
            .file_name()
            .context("Unable to determine linker script file name")?
            .to_owned();

        (ld_dir, ld_script)
    };

    let arch_triple = config.arch.target_triple();

    match config.arch {
        Architecture::AArch64 => {
            let mut cmd = std::process::Command::new(cargo);

            cmd.arg("build")
                .args(&["--package", "ratto-entry"])
                .arg("--target")
                .arg(arch_triple);

            if profile == Profile::Release {
                cmd.arg("--release");
            }

            if let Some(provider) = virtualization {
                cmd.arg("--features");
                cmd.arg(provider.to_string());
            }

            // no_std binaries require panic=abort and a panic handler
            let rust_flags = format!(
                "-C panic=abort -C link-arg=--library-path={} -C link-arg=--script={}",
                ld_path.display(),
                ld_script.display()
            );

            cmd.env("RUSTFLAGS", &rust_flags);
            if !cmd.status().unwrap().success() {
                return Err(anyhow::anyhow!("Kernel build failed"));
            }
        }
        Architecture::X86_64 => {
            todo!();
        }
    }

    let kernel_elf = workspace_root
        .join("target")
        .join(arch_triple)
        .join(profile.as_str())
        .join("ratto-entry");

    let kernel_bin = kernel_elf
        .parent()
        .context("Unable to determine kernel.bin path")?
        .join("ratto-kernel.bin");

    let status = std::process::Command::new("rust-objcopy")
        .arg("--strip-all")
        .args(&["-O", "binary"])
        .arg(&kernel_elf)
        .arg(&kernel_bin)
        .status()
        .unwrap();

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to generate kernel binary"));
    }

    Ok(kernel_bin)
}
