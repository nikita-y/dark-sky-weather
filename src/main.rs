extern crate json;
extern crate reqwest;
#[macro_use]
extern crate clap;

use clap::App;
use std::str;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let api_key = matches.value_of("key").unwrap_or("");
    let lat = matches.value_of("latitude").unwrap_or("");
    let lng = matches.value_of("longitude").unwrap_or("");

    let query = build_qery(api_key, lat, lng);
    if let Ok(server_response) = request(&query) {
        let print_message = pharse_output(&server_response);
        println!("{}", print_message);
    }
}

fn pharse_output(arg: &str) -> String {
    match json::parse(arg) {
        Ok(parsed) => format!(
            "{sum:}: {temp:.0}℃\n{forecast}",
            sum = &parsed["currently"]["summary"],
            temp = &parsed["currently"]["temperature"],
            forecast = &parsed["hourly"]["summary"]
        ),
        Err(_) => "Unable to parse the data".to_owned(),
    }
}

fn request(arg: &str) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(arg)?.text()
}

fn build_qery(key: &str, n: &str, e: &str) -> String {
    format!(
        "https://api.darksky.net/forecast/{}/{},{}?units=si",
        key, n, e
    )
}
