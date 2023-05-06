use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

mod logs;
mod shells;

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
    /// Do something
    #[clap(alias = "d")]
    Do { something: String },
    /// Print version information
    Version,
    /// Generate shell completion scripts
    Completion {
        #[clap(subcommand)]
        shell: shells::Shell,
    },
}

fn main() -> Result<()> {
    logs::init_tracing();
    let opt = Opt::parse();
    if opt.color {
        std::env::set_var("CLICOLOR_FORCE", "1");
    }
    match opt.cmd {
        Command::Do { something } => unimplemented!("{}", something),
        Command::Version => tracing::info!("{}", VERSION_INFO),
        Command::Completion { shell } => {
            let clap_shell = shells::shell_to_clap_shell(shell);
            let mut cmd = Opt::command();
            generate(clap_shell, &mut cmd, BIN_NAME, &mut std::io::stdout());
        }
    }
    Ok(())
}
