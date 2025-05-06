use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::error::Error;
use std::io;


pub struct Connection {
    pub reader: io::BufReader<TcpStream>,
    pub writer: TcpStream,
    pub irc_nick: String,
    pub url: String,
    pub channel: String,
}

impl Connection {
    pub fn build(url: String, port: u32, nick: String, chan: String) -> Result<Connection, Box<dyn Error>> {
    let mut addrs = format!("{url}:{port}").to_socket_addrs()?;

    let stream = connect_irc_first_available(addrs)?;

    let (mut ircReader, mut ircWriter) = (io::BufReader::new(stream.try_clone()?), stream);

    Ok(Connection {
        reader: ircReader,
        writer: ircWriter,
        irc_nick: nick,
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

/*
fn irc_manager(irc: Connection, books: Vec<Book>){
    let mut requestPending = false;
    let mut botsOnline = HashSet::new();

    while books.is_empty() == false {
        if requestPending == false {
            //Search for book
        }

        let mut line = String::new();
        irc.reader.read_line(&mut line);

        let split_line = line.split_whitespace().collect();

        // match the numeric we care about
        match split_line[1] {
            "443" => {
                //Nick in use, rename
            },
            "352" if split_line[8].contains("@") || split_line[8].contains("+") => {
                //whois response, check if bot and add to map
                bots_online.insert(split_line[4].to_string());

            },
            "NOTICE" if split_line[2] == irc.nick => {
                //notice sent to self check if search bot results
            },
            "PRIVMSG" if split_line[2] == irc.nick => {
                //private message sent to self, check if dcc
            },
            _ if split_line[0] == "PING" => {
                irc.writer.write_all("PONG\r\n".as_bytes());
            },
            _ => ()
        }


    }

}
*/
fn shutdown(&mut irc: Connection){
    let part = format!("PART {}\r\n", irc.channel);
    let quit = String::from("QUIT :goodbye\r\n");

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
}
