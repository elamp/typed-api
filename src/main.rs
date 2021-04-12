mod exporter;
use std::fs;
use serde_json;
use openapiv3::OpenAPI;

fn main() {

    let filename = "/home/yann/Documents/git/elamp/back-libs/protos/src/openapi/availability-api.json";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result: OpenAPI = serde_json::from_str(&contents).expect("Could not deserialize input");
    println!("{:?}", result);

}
