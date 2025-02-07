use nu_ansi_term::{Color as ANSIColor, Style as ANSIStyle};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Style {
    #[serde(default, with = "serde_color")]
    pub foreground: Color,

    #[serde(default, with = "serde_color")]
    pub background: Color,

    #[serde(default)]
    pub decoration: Vec<Decoration>,
}

impl Style {
    pub fn to_ansi(&self) -> ANSIStyle {
        let mut style = ANSIStyle::new();

        for deco in self.decoration.iter() {
            style = match deco {
                Decoration::Bold => style.bold(),
                Decoration::Dimmed => style.dimmed(),
                Decoration::Italic => style.italic(),
                Decoration::Underline => style.underline(),
                Decoration::Blink => style.blink(),
                Decoration::Reverse => style.reverse(),
                Decoration::Hidden => style.hidden(),
                Decoration::Strikethrough => style.strikethrough(),
            }
        }
        if let Some(fg) = self.foreground.to_ansi() {
            style = style.fg(fg)
        }
        if let Some(bg) = self.background.to_ansi() {
            style = style.on(bg)
        }

        style
    }
}

mod serde_color {
    use serde::{Deserialize, Deserializer};

    use super::Color;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        Color::deserialize(deserializer)
            .map_err(|_| Error::custom("color must be a named color (default, black, red, ...), a fixed number (0-255), or a hex (#rrggbb)"))
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Named(NamedColor),
    Fixed(u8),
    Hex(HexColor),
}

impl Color {
    fn to_ansi(&self) -> Option<ANSIColor> {
        match self {
            Color::Named(name) => match name {
                NamedColor::Default => None,
                NamedColor::Black => Some(ANSIColor::Black),
                NamedColor::DarkGray => Some(ANSIColor::DarkGray),
                NamedColor::Red => Some(ANSIColor::Red),
                NamedColor::LightRed => Some(ANSIColor::LightRed),
                NamedColor::Green => Some(ANSIColor::Green),
                NamedColor::LightGreen => Some(ANSIColor::LightGreen),
                NamedColor::Yellow => Some(ANSIColor::Yellow),
                NamedColor::LightYellow => Some(ANSIColor::LightYellow),
                NamedColor::Blue => Some(ANSIColor::Blue),
                NamedColor::LightBlue => Some(ANSIColor::LightBlue),
                NamedColor::Purple => Some(ANSIColor::Purple),
                NamedColor::LightPurple => Some(ANSIColor::LightPurple),
                NamedColor::Magenta => Some(ANSIColor::Magenta),
                NamedColor::LightMagenta => Some(ANSIColor::LightMagenta),
                NamedColor::Cyan => Some(ANSIColor::Cyan),
                NamedColor::LightCyan => Some(ANSIColor::LightCyan),
                NamedColor::White => Some(ANSIColor::White),
                NamedColor::LightGray => Some(ANSIColor::LightGray),
            },
            Color::Fixed(fixed) => Some(ANSIColor::Fixed(*fixed)),
            Color::Hex(HexColor(r, g, b)) => Some(ANSIColor::Rgb(*r, *g, *b)),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Named(NamedColor::Default)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    Default,
    Black,
    DarkGray,
    Red,
    LightRed,
    Green,
    LightGreen,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Purple,
    LightPurple,
    Magenta,
    LightMagenta,
    Cyan,
    LightCyan,
    White,
    LightGray,
}

#[derive(Debug, PartialEq)]
pub struct HexColor(pub u8, pub u8, pub u8);

impl HexColor {
    fn try_from(value: &str) -> Option<Self> {
        if value.len() != 7 || !value.starts_with('#') {
            return None;
        }
        let hex = u32::from_str_radix(&value[1..], 16).ok()?;
        let r = ((hex >> 16) & 0xff) as u8;
        let g = ((hex >> 8) & 0xff) as u8;
        let b = (hex & 0xff) as u8;
        Some(Self(r, g, b))
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RGBVisitor;
        impl Visitor<'_> for RGBVisitor {
            type Value = HexColor;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`#rrggbb`")
            }

            fn visit_str<E>(self, value: &str) -> Result<HexColor, E>
            where
                E: de::Error,
            {
                HexColor::try_from(value).ok_or_else(|| {
                    E::custom(format!(
                        "invalid hex color format ({value}), expected `#rrggbb`"
                    ))
                })
            }
        }

        deserializer.deserialize_identifier(RGBVisitor)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decoration {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let text = "
        - foreground: default
          background: 1
          decoration: [bold, italic]

        - foreground: red
          background: '#12ABef'
          decoration: []

        - foreground: light_red
          background: magenta
          decoration: []
        ";

        let styles: Vec<Style> = serde_yaml::from_str(text).unwrap();

        assert_eq!(
            styles,
            vec![
                Style {
                    foreground: Color::Named(NamedColor::Default),
                    background: Color::Fixed(1),
                    decoration: vec![Decoration::Bold, Decoration::Italic]
                },
                Style {
                    foreground: Color::Named(NamedColor::Red),
                    background: Color::Hex(HexColor(0x12, 0xab, 0xef)),
                    decoration: vec![]
                },
                Style {
                    foreground: Color::Named(NamedColor::LightRed),
                    background: Color::Named(NamedColor::Magenta),
                    decoration: vec![]
                },
            ]
        );
    }
}
