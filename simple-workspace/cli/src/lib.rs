use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 60f64)]
    pub frame: f64,
    #[arg(short, long, default_value_t = 10f64)]
    pub tick: f64,
}
