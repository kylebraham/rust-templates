use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// arg a
    #[arg(short, long, required = false, default_value_t = String::from("a"))]
    a: String,

    /// arg b
    #[arg(short, long, required = false, default_value_t = String::from("b"))]
    b: String,
}

#[derive(Debug, Deserialize)]
struct ResponeJson {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
}

fn main() {
    let args = Cli::parse();

    make_request(args.a, args.b);
}

fn make_request(param_one: String, param_two: String) {
    let client = Client::new();
    let url = format!("https://httpbin.org/anything?{}={}", param_one, param_two);

    println!("URL: {}", url);
    let respone = client.get(url).send();
    match respone {
        Ok(res) => {
            // Check the status code
            if res.status().is_success() {
                println!("Request successful with status code: {}", res.status());
                // Handle response body here
            } else {

                // Handle unsuccessful response
            }

            // Deserialize respones JSON
            match res.json::<ResponeJson>() {
                Ok(json) => {
                    print!("Args:{:#?}\nHEADERS:{:#?}", json.args, json.headers);
                }
                Err(err) => {
                    println!("Error occurred while Deserialize respones JSON: {}", err);
                }
            }
        }
        Err(err) => {
            // Handle errors from the send function
            println!("Error occurred while sending request: {}", err);
        }
    }
}
