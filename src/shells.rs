#[allow(clippy::enum_variant_names)]
#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
    PowerShell,
    Elvish,
}

pub fn shell_to_clap_shell(shell: Shell) -> clap_complete::shells::Shell {
    match shell {
        Shell::Bash => clap_complete::shells::Shell::Bash,
        Shell::Fish => clap_complete::shells::Shell::Fish,
        Shell::Zsh => clap_complete::shells::Shell::Zsh,
        Shell::PowerShell => clap_complete::shells::Shell::PowerShell,
        Shell::Elvish => clap_complete::shells::Shell::Elvish,
    }
}
