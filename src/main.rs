use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::{generate, shells};

const BIN_NAME: &str = include_str!(concat!(env!("OUT_DIR"), "/bin-name.txt"));
const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    include_str!(concat!(env!("OUT_DIR"), "/commit-info.txt"))
);

#[derive(Parser)]
struct Opt {
    #[clap(long)]
    color: bool,
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum Command {
    /// Copy
    #[clap(alias = "d")]
    Do { something: String },
    /// Version
    Version,
    /// Completion
    Completion {
        #[clap(subcommand)]
        shell: Shell,
    },
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum Shell {
    Bash,
    Fish,
    Zsh,
    PowerShell,
    Elvish,
}

fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .with_level(false)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn main() -> Result<()> {
    init_tracing();
    let opt = Opt::parse();
    if opt.color {
        std::env::set_var("CLICOLOR_FORCE", "1");
    }
    match opt.cmd {
        Command::Do { something } => unimplemented!("{}", something),
        Command::Version => println!("{}", VERSION_INFO),
        Command::Completion { shell } => {
            let shell = match shell {
                Shell::Bash => shells::Shell::Bash,
                Shell::Fish => shells::Shell::Fish,
                Shell::Zsh => shells::Shell::Zsh,
                Shell::PowerShell => shells::Shell::PowerShell,
                Shell::Elvish => shells::Shell::Elvish,
            };
            let mut cmd = Opt::command();
            generate(shell, &mut cmd, BIN_NAME, &mut std::io::stdout());
        }
    }
    Ok(())
}
