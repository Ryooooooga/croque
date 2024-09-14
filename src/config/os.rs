use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[cfg(target_os = "linux")]
#[derive(Debug, Default, Deserialize)]
pub struct OsConfig {
    #[serde(default)]
    pub alpine: AlpineOsConfig,

    #[serde(default)]
    pub amazon: AmazonOsConfig,

    #[serde(default)]
    pub arch: ArchOsConfig,

    #[serde(default)]
    pub centos: CentOSOsConfig,

    #[serde(default)]
    pub debian: DebianOsConfig,

    #[serde(default)]
    pub gentoo: GentooOsConfig,

    #[serde(default)]
    pub nix: NixOsConfig,

    #[serde(default)]
    pub raspbian: RaspbianOsConfig,

    #[serde(default)]
    pub ubuntu: UbuntuOsConfig,

    #[serde(default)]
    pub linux: LinuxOsConfig,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Default, Deserialize)]
pub struct OsConfig {
    #[serde(default)]
    pub mac: MacOsConfig,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Default, Deserialize)]
pub struct OsConfig {
    #[serde(default)]
    pub windows: WindowsOsConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AlpineOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "AlpineOsConfig::default_content")]
    pub content: String,
}

impl AlpineOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for AlpineOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AmazonOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "AmazonOsConfig::default_content")]
    pub content: String,
}

impl AmazonOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for AmazonOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ArchOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "ArchOsConfig::default_content")]
    pub content: String,
}

impl ArchOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for ArchOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CentOSOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "CentOSOsConfig::default_content")]
    pub content: String,
}

impl CentOSOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for CentOSOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GentooOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "GentooOsConfig::default_content")]
    pub content: String,
}

impl GentooOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for GentooOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NixOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "NixOsConfig::default_content")]
    pub content: String,
}

impl NixOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for NixOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RaspbianOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "RaspbianOsConfig::default_content")]
    pub content: String,
}

impl RaspbianOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for RaspbianOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DebianOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "DebianOsConfig::default_content")]
    pub content: String,
}

impl DebianOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for DebianOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UbuntuOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "UbuntuOsConfig::default_content")]
    pub content: String,
}

impl UbuntuOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for UbuntuOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LinuxOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "LinuxOsConfig::default_content")]
    pub content: String,
}

impl LinuxOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for LinuxOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MacOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "MacOsConfig::default_content")]
    pub content: String,
}

impl MacOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for MacOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WindowsOsConfig {
    #[serde(default = "default_style")]
    pub style: Style,

    #[serde(default = "WindowsOsConfig::default_content")]
    pub content: String,
}

impl WindowsOsConfig {
    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for WindowsOsConfig {
    fn default() -> Self {
        Self {
            style: default_style(),
            content: Self::default_content(),
        }
    }
}

fn default_style() -> Style {
    Style {
        foreground: Color::Named(NamedColor::White),
        background: Color::Fixed(33),
        decoration: vec![],
    }
}
