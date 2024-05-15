use clap::Parser;
use reqwest::Error;
use serde::Deserialize;
use std::{collections::HashMap, future::Future};

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
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    // let req1 = make_request(&args.a, &args.b);

    // let req_vec = vec![req1, req2];

    let mut futures = vec![];
    for _ in 0..2 {
        let mr = make_request(&args.a, &args.b);
        futures.push(mr);
    }

    let results = tokio::join!(futures);
    // req_vec.push(req1);
    // req_vec.push(req2);

    // let (req1, req2) = tokio::join!(req1, req2);
    // let req1 = req1?;
    // let req2 = req2?;

    // print!(
    //     "Request 1 Args:{:#?}\nRequest 1 HEADERS:{:#?}",
    //     req1.args, req1.headers
    // );

    // print!(
    //     "Request 2 Args:{:#?}\nRequest 2 HEADERS:{:#?}",
    //     req2.args, req2.headers
    // );

    Ok(())
}

// async fn make_request(param_one: String, param_two: String) {
//     let client = Client::new();
//     let url = format!("https://httpbin.org/anything?{}={}", param_one, param_two);

//     println!("URL: {}", url);
//     let respone = client.get(url).send();
//     match respone {
//         Ok(res) => {
//             // Check the status code
//             if res.status().is_success() {
//                 println!("Request successful with status code: {}", res.status());
//                 // Handle response body here
//             } else {

//                 // Handle unsuccessful response
//             }

//             // Deserialize respones JSON
//             match res.json::<ResponeJson>() {
//                 Ok(json) => {
//                     print!("Args:{:#?}\nHEADERS:{:#?}", json.args, json.headers);
//                 }
//                 Err(err) => {
//                     println!("Error occurred while Deserialize respones JSON: {}", err);
//                 }
//             }
//         }
//         Err(err) => {
//             // Handle errors from the send function
//             println!("Error occurred while sending request: {}", err);
//         }
//     }
// }

async fn make_request(param_one: &str, param_two: &str) -> Result<ResponeJson, reqwest::Error> {
    let url = format!("https://httpbin.org/anything?{}={}", param_one, param_two);
    let res = reqwest::get(url).await?;
    let json = res.json::<ResponeJson>().await?;

    Ok(json)
}
