use reqwest::header::HeaderMap;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Url;
use std::process::Command;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::{BytesStart, BytesEnd};
use std::net::{ToSocketAddrs, SocketAddr, TcpStream};
use std::io::{self, Read, Write};



enum Messages {
    ERR_NICKNAMEINUSE(Vec<&str>),
    RPL_WHOREPLY(Vec<&str>),
    PRIVMSG(Vec<&str>),
    PING,
    QUIT(Vec<&str>),
};



/*

   struct Book{ title: String,
   author: String,
   }


   fn parse_xml(xml: &String) -> Vec<Book> {
   let mut books = Vec::new();
   let mut reader = Reader::from_str(xml);
   let mut author = String::new();
   let mut title = String::new();
   loop {
   match reader.read_event() {
   Err(e) => panic!("Error"),
   Ok(Event::Eof) => break,

   Ok(Event::Start(e)) if e == BytesStart::from_content("title", 5) => {
   title = String::from_utf8(reader.read_event().unwrap().to_vec()).unwrap();
   },

   Ok(Event::Start(e)) if e == BytesStart::from_content("author_name", 11) => {
   author = String::from_utf8(reader.read_event().unwrap().to_vec()).unwrap();
   },

   Ok(Event::End(e)) if e == BytesEnd::new("item")  => {
   let book = Book{
   title: title,
   author: author,
   };
   println!("adding book: {} by {}", book.title, book.author);
   books.push(book);
   title = "".to_string();
   author= "".to_string();
   },
   _ => (),
   }
   }
   books

   }
   */

fn main() {
    /*
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "rust-rss".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    //jonah
    //let url = format!("https://www.goodreads.com/review/list_rss/186388648");
    //
    //brit
    let url = format!("https://www.goodreads.com/review/list_rss/121576883");
    let url = Url::parse(&url).unwrap();

    let response = client
        .get(url)
        .send()
        .unwrap()
        .text()
        .unwrap();


   let parsed_xml = parse_xml(&response); 
   println!("books length is: {}", parsed_xml.len());
*/

    irc_connect();
}


fn irc_connect(){
    let mut addr = "irc.irchighway.net:6660".to_socket_addrs().unwrap();
    let irc = addr.next().unwrap();

    let mut stream = TcpStream::connect(irc).unwrap();
    let mut reader = BufReader::new(stream);
    //TODO pull nickname from config
    let nick = String::from("fish");

    while true {
        let line : Vec<&str> = reader
            .read_line(&mut line)
            .unwrap()
            .split_whitespace()
            .collect();
        match line[1] {
            "443" => {
                generate_new_nickname(&mut nick);
                stream.write("NICK {nick}\r\n");
            },
            "352" => user_online_handler(line),
            "PRIVMSG" => private_message_handler(line),
            "PING" => ping_handler(line),
            "QUIT" => quit_handler(line),
        }
    }
}

//check if last char is \d, if so increment, if not, append
fn generate_new_nickname(nick : &mut String){
    let last_character = nick.chars().last().unwrap();
    match last_character {
        digit if digit.is_digit(10) => String::from(nick)
            .split_at(nick.len() - 1)
            .
    }
}

//gets both whois response messages and quit messages, edits the user online vector
fn user_online_handler(line, 





fn parse_users(line: String) -> bots: vec(users)


/*
fn check_database(book: Book) -> bool {
    let res = Command::new("calibredb")
        .arg("search")
        .arg(book.title)
        .output()
        .unwrap();

    let res_str = String::from_utf8(res.stdout).unwrap();
    println!("{}", res_str);
    true
}
*/
