#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}
