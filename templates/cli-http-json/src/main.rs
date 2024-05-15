use clap::Parser;
use reqwest::Error;
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let (res_one, res_two) = tokio::join!(
        make_request(&args.a, &args.b),
        make_request(&args.a, &args.b)
    );

    let res_one = res_one?;
    println!(
        "Res1 Args:\n{:?}\n\n\nRes1 Headers:\n{:#?}",
        res_one.args, res_one.headers
    );

    let res_two = res_two?;
    println!(
        "Res2 Args:\n{:?}\n\n\nRes2 Headers:\n{:#?}",
        res_two.args, res_two.headers
    );

    Ok(())
}

async fn make_request(param_one: &str, param_two: &str) -> Result<ResponeJson, reqwest::Error> {
    let url = format!("https://httpbin.org/anything?{}={}", param_one, param_two);
    let res = reqwest::get(url).await?;
    let json = res.json::<ResponeJson>().await?;

    Ok(json)
}
