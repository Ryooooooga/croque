use core::fmt;
use std::borrow::Cow;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl Shell {
    pub fn control<C: fmt::Display>(&self, c: C) -> PromptControl<'_, C> {
        PromptControl::new(self, c)
    }

    fn controls(&self) -> (&'static str, &'static str) {
        match self {
            Shell::Bash => (r"\[", r"\]"),
            Shell::Fish => (r"", r""),
            Shell::Zsh => (r"%{", r"%}"),
        }
    }

    pub fn escape<'a>(&self, s: &'a str) -> Cow<'a, str> {
        match self {
            Shell::Bash => Cow::from(s.replace('\\', r"\\")),
            Shell::Fish => Cow::from(s),
            Shell::Zsh => Cow::from(s.replace('%', "%%")),
        }
    }
}

#[derive(Debug)]
pub struct PromptControl<'a, C: fmt::Display> {
    shell: &'a Shell,
    control: C,
}

impl<'a, C: fmt::Display> PromptControl<'a, C> {
    fn new(shell: &'a Shell, control: C) -> Self {
        Self { shell, control }
    }
}

impl<C: fmt::Display> fmt::Display for PromptControl<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (prefix, suffix) = self.shell.controls();
        write!(f, "{}{}{}", prefix, self.control, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control() {
        assert_eq!(
            &format!("{}", Shell::Bash.control("\x1b[m")),
            "\\[\x1b[m\\]"
        );
        assert_eq!(&format!("{}", Shell::Fish.control("\x1b[m")), "\x1b[m");
        assert_eq!(&format!("{}", Shell::Zsh.control("\x1b[m")), "%{\x1b[m%}");
    }
}
