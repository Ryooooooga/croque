use super::ConfigArgs;
use crate::{command::ConfigTheme, config::Config};

const CONFIG_THEME_AGNOSTER: &str = include_str!("agnoster.yaml");

pub fn run(args: &ConfigArgs) {
    if args.print_config_path {
        let config_path = Config::config_path();
        println!("{}", config_path.to_string_lossy());
        return;
    }

    let theme = match &args.theme {
        ConfigTheme::Agnoster => CONFIG_THEME_AGNOSTER,
    };

    print!("{theme}");
}

#[test]
fn test_theme() {
    Config::load_from_str(CONFIG_THEME_AGNOSTER).unwrap();
}
