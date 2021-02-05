mod args;
mod apiclient;

use apiclient::YodaAPIClient;
use args::ArgParser;

use clap::Clap;
const BASE_URL: &str = "https://api.funtranslations.com/translate/yoda.json";

fn main() {
    let args: ArgParser = ArgParser::parse();
    println!("Using Arg {}", args.text);

    let apiclient = YodaAPIClient::default(String::from(BASE_URL), String::from(args.text));
    let joke = apiclient.make_request().unwrap();
    println!("\n{}", joke.text);

    //Message::Write(String::from("hello"));
}

enum apiType {
	Yoda(String),
        Joke(String),
        Pokemon(String)
}

