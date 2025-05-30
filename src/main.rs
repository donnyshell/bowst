use serde::Deserialize;
use serde_json;
use std::fs;
use std::error::Error;
use std::process;

mod irc;
mod goodreads;

#[derive(Deserialize, Debug)]
struct BowstConfig{
    goodreads_users: Vec<GoodreadsUser>,
    irc_nick: String,
}

#[derive(Deserialize, Debug)]
struct GoodreadsUser {
    name: String,
    id: u32,
    shelves: Vec<String>,
}

impl BowstConfig {
    fn build() -> Result<BowstConfig, Box<dyn Error>> {
        let config_file = fs::read_to_string("config.json")?;
        let config = serde_json::from_str(&config_file)?;
        Ok(config)
    }
}



fn main() {

    //read config file
    let config = BowstConfig::build()
        .unwrap_or_else(|err| {
            eprintln!("Error with config: {err}");
            process::exit(1);
        });

    let mut books_for_irc = goodreads::Build_ToRead_Map(&config.goodreads_users).unwrap();

    let calibre_lib = goodreads::get_current_library().unwrap();

    for book in calibre_lib {
        match books_for_irc.contains_key(&book) {
            true => { books_for_irc.remove(&book); },
            false => (),
        };
    }

    println!("{:?}", books_for_irc);
}

