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

    let books_for_irc = goodreads::Build_ToRead_Map(&config.goodreads_users);

//    let conn = Connection::build(URL, PORT, config.nick, CHAN)
//        .unwrap_or_else();
//    conn.writer.write_all(format!("USER {} * {} :{}\r\n", conn.nick, conn.url, conn.nick).as_bytes());
//    conn.writer.write_all(format!("NICK {conn.nick}\r\n").as_bytes());
//    conn.writer.write_all(format!("JOIN #{conn.channel}\r\n").as_bytes());


//    let parsed_xml = goodreads::parse_xml(&response).unwrap(); 
//    println!("books length is: {}", parsed_xml.len());
//    irc_connect();

}

