use reqwest::header::HeaderMap;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Url;
use std::process::Command;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::{BytesStart, BytesEnd};
use std::net::{ToSocketAddrs, SocketAddr, TcpStream};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::collections::HashSet;


struct Book{ 
    title: String,
    author: String,
}

struct Connection {
    reader: TcpStream,
    writer: TcpStream,
    nick: String,
}

fn parse_xml(xml: &String) -> Result<Vec<Book>, quick_xml::errors::Error> {
    let mut books = Vec::new();
    let mut reader = Reader::from_str(xml);
    let mut author = String::new();
    let mut title = String::new();
    loop {
        match reader.read_event() {
            Err(e) => panic!("Error"),
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) if e == BytesStart::from_content("title", 5) => {
                title = String::from_utf8(reader.read_event()?.to_vec()).unwrap();
            },

            Ok(Event::Start(e)) if e == BytesStart::from_content("author_name", 11) => {
                author = String::from_utf8(reader.read_event()?.to_vec()).unwrap();
            },

            Ok(Event::End(e)) if e == BytesEnd::new("item")  => {
                let book = Book{
                    title: title,
                    author: author,
                };
                books.push(book);
                title = "".to_string();
                author= "".to_string();
            },
            _ => (),
        }
    }
    Ok(books)
}


fn main() {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "rust-rss".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    //jonah
    let url = format!("https://www.goodreads.com/review/list_rss/186388648?shelf=to-read");

    //brit
    //let url = format!("https://www.goodreads.com/review/list_rss/121576883");
    let url = Url::parse(&url).unwrap();

    let response = client
        .get(url)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let parsed_xml = parse_xml(&response).unwrap(); 
    println!("books length is: {}", parsed_xml.len());
    irc_connect();
}

fn irc_setup_connection(url: String, port: u32, nick: String) -> Result<Connection> {
    let mut addresses = format!("{address}:{port}").to_socket_addrs()?;

    let mut stream = None;
    for address in addresses {
        match TcpStream.connect(address) {
            Ok(c) => {
                stream = Some(c);
                break;
            },
            _ => (),
        }
    }

    let stream = match stream {
        Some(c) => s,
        None => (),
    }

    let (mut reader, mut writer) = (BufReader::new(stream.try_clone()?), stream);

    let irc = Connection {
        reader = reader,
        writer = writer,
        nick = nick,
    }

    
}

fn irc_bots_online(conn: Connection) -> Result<bots : HashSet> {

}


fn irc_connect(books: Vec<Book>, conn: Connection){
    writer.write_all(format!("USER {nick} * irc.irchighway.net :{nick}\r\n").as_bytes());
    writer.write_all(format!("NICK {nick}\r\n").as_bytes());
    writer.write_all(format!("JOIN #ebooks\r\n").as_bytes());

    let mut bots_online = HashSet::new();
    let mut buf = String::new();

    loop {
        buf.clear();
        reader.read_line(&mut buf);
        let line = buf.clone();
        let split_line : Vec<&str> = line.split_whitespace().collect();
        println!("line: {buf}");
        match split_line[1] {
            "443" => {
                //TODO doesn't handle 9
                let mut last_char = nick.chars().last().unwrap();
                if last_char.is_digit(10) {
                    let last_num = last_char.to_digit(10).unwrap() + 1;
                    last_char = char::from_digit(last_num, 10).unwrap();
                    nick.pop();
                } else {
                    last_char = '1';
                }
                nick.push(last_char);
                writer.write(format!("NICK {nick}\r\n").as_bytes());
            },
            // whois responses
            "352" if split_line[8].contains("@") || split_line[8].contains("+") => {
                bots_online.insert(split_line[4].to_string());
                println!("Bot online: {}", split_line[4]);
            },
            // possible DCC
            //"PRIVMSG" => private_message_handler(line),
            
            //"PING" => ping_handler(line),
            //"QUIT" => quit_handler(line),
            _ => (),
        }
    }
}


fn generate_new_nickname(mut nick: String) -> String {
    let chars = nick.chars().rev().collect::<Vec<_>>();
    let mut num_index = None;

    for i in 0..chars.len() {
        if !(chars[i].is_digit(10)) {
            num_index = Some(i);
            break;
        }
    }

    match num_index {
        Some(i) => {
            let index = chars.len()-i;
            let num: usize = nick[index..].parse().unwrap();
            let num += 1;
            nick.truncate(index);
            nick.push_str(num.to_string().as_str());

        },
        None => {
            nick.push_str(nick);
            nick.push('1');
        },
    }
    nick
}


//gets both whois response messages and quit messages, edits the user online vector
//fn user_online_handler(name:





//fn parse_users(line: String) -> bots: vec(users)


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

