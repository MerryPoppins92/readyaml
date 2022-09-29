use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use std::net::SocketAddr;

fn main() {
    println!("Hello, Yaml");
    let file = "./src/test.yaml";
    load_file(file);
}
fn load_file(file: &str) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
        let mut bootstrap_list: Vec<(SocketAddr, String)> = vec![];
    for s in docs[0]["whitelist"].as_vec().unwrap() {
        let address: SocketAddr = s[0].as_str().unwrap().parse().expect("Unable to parse socket address");
        bootstrap_list.push((address, s[1].as_str().unwrap().to_owned()))

    }
    println!("{:?}", bootstrap_list);
}