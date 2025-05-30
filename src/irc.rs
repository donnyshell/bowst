use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use crate::goodreads::Book;

pub struct Connection {
    pub reader: io::BufReader<TcpStream>,
    pub writer: TcpStream,
    pub nick: String,
    pub url: String,
    pub channel: String,
}

impl Connection {
    pub fn build(url: String, port: u32, nick: String, chan: String) -> Result<Connection, Box<dyn Error>> {
    let addrs = format!("{url}:{port}").to_socket_addrs()?;

    let mut stream = connect_irc_first_available(addrs)?;

    let (mut ircReader, mut ircWriter) = (io::BufReader::new(stream.try_clone()?), stream);

    Ok(Connection {
        reader: ircReader,
        writer: ircWriter,
        nick: nick,
        url: url,
        channel: chan,
    })
    }
}


fn connect_irc_first_available(addrs: impl Iterator<Item = SocketAddr>) -> io::Result<TcpStream> {
    let mut last_error = None;

    for address in addrs {
        match TcpStream::connect(address) {
            Ok(conn) => return Ok(conn),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.unwrap_or_else(||
            io::Error::new(io::ErrorKind::NotFound, "No addresses to connect to")))

}


fn irc_manager(mut irc: Connection, mut books: Vec<Book>){
    let mut requestPending = false;
    let mut botsOnline = HashSet::new();
//    conn.writer.write_all(format!("USER {} * {} :{}\r\n", conn.nick, conn.url, conn.nick).as_bytes());
//    conn.writer.write_all(format!("NICK {conn.nick}\r\n").as_bytes());
//    conn.writer.write_all(format!("JOIN #{conn.channel}\r\n").as_bytes());



    while books.is_empty() == false {
        if requestPending == false {
            //Search for book
        }

        let mut line = String::new();
        irc.reader.read_line(&mut line);

        let split_line: Vec<&str>= line.split_whitespace().collect();

        // match the numeric we care about
        match split_line[1] {
            "443" => {
                //Nick in use, rename
            },
            "352" if split_line[8].contains("@") || split_line[8].contains("+") => {
                //whois response, check if bot and add to map
                botsOnline.insert(split_line[4].to_string());

            },
//            "NOTICE" if split_line[2] == irc.nick => {
                //notice sent to self check if search bot results
//               }
//            },
            "PRIVMSG" if split_line[2] == irc.nick => {
                 if (split_line[3], split_line[4]) == (":.DCC", "SEND") {
                     //TODO something here dcc_manager(&split_line);
            }},
            _ if split_line[0] == "PING" => {
                irc.writer.write_all("PONG\r\n".as_bytes());
            },
            _ => {
                irc.writer.write_all("hello\r\n".as_bytes());
                books.pop(); 
            }
        }


    }
    shutdown(irc);

}


//TODO fix the unwrap
fn dcc_manager(split_line: &Vec<&str>, bots_online: &HashSet<&str>){
    //filesize. is last arg, then port, then IP
    let line_length = split_line.len();
    let ip = split_line[line_length-3];
    let port = split_line[line_length-2];
    let filesize: usize = usize::from_str_radix(split_line[line_length-1].trim_matches('.'), 10).unwrap();

    let mut file: Vec<u8> = Vec::with_capacity(filesize);

    //TODO connect to ip over port, write output to file vec
    //TODO get zip library to unzip and then process text

}

fn parse_searchbot() {

}

fn add_book(){
}



fn shutdown(mut irc: Connection){
    let part_string = format!("PART {}\r\n", irc.channel);
    let part: &[u8] = part_string.as_bytes();
    let quit: &[u8] = b"QUIT :goodbye\r\n";

    irc.writer.write_all(part);
    irc.writer.write_all(quit);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_connection() {
        let mut testConn = Connection::build("irc.irchighway.net".to_string(), 6660, "nick".to_string(), "#ebooks".to_string()).unwrap();
        assert_eq!(testConn.channel, "#ebooks");
    }

    /*
    #[test]
    fn test_irc_manager(){
        let mut con = Connection::build("127.0.0.1".to_string(), 303, "fish1".to_string(), "#test".to_string()).unwrap();
        let mut books = vec!(Book{ title: "this".to_string(), author: "that".to_string()}, Book{title:"this".to_string(), author: "that".to_string()});
        irc_manager(con, books);
        assert_eq!(1, 1);
    }*/
}
