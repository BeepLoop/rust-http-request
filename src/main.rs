use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Ability {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AbilityEntry {
    ability: Ability,
    is_hidden: bool,
    slot: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    abilities: Vec<AbilityEntry>,
    base_experience: u32,
    name: String,
}

#[tokio::main]
async fn main() {
    let mut pokemon = String::new();
    println!("Enter pokemon name: ");

    io::stdin()
        .read_line(&mut pokemon)
        .expect("error reading line");

    let fact = make_request(&pokemon).await;
    match fact {
        Ok(data) => {
            let parsed: Pokemon = serde_json::from_str(&data).unwrap();
            println!("{:#?}", parsed)
        }
        Err(err) => println!("{}", err),
    }
}

async fn make_request(pokemon: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = String::from("https://pokeapi.co/api/v2/pokemon/");
    url.push_str(pokemon);

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?
        .to_string();

    Ok(body)
}
