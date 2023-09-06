use std::{borrow::Cow, env};

use clap::Parser;

/// Configure and print app.config
#[derive(Parser)]
struct Cli {
    /// path to config
    #[arg(long)]
    conf: Option<String>,
}

fn main() {
    let Cli { conf: cli_path } = Cli::parse();

    cli_path
        .iter()
        .filter(|s| s.is_empty())
        .for_each(|_| panic!("Can't supply empty string in --conf argument"));

    let resulting_path: Cow<str> = {
        let default_path = "etc/app/app.conf";

        if let Some(cli_path) = cli_path {
            Cow::Owned(cli_path)
        } else if let Ok(env_path) = env::var("APP_CONF") {
            Cow::Owned(env_path)
        } else {
            Cow::Borrowed(default_path)
        }
    };

    println!("{}", resulting_path);
}
