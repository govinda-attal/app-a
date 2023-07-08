use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("./config/app.yaml"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

pub fn parse() -> Args {
    Args::parse()
}
