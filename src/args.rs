use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0")]
pub struct ArgParser {
    #[clap(short, long, default_value = "txt")]
    pub text: String,
}
