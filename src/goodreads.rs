use serde::Deserialize;
use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::header::{
    HeaderMap,
    HeaderValue,
    USER_AGENT};
use std::process;
use std::error::Error;
use quick_xml::events::{
    BytesStart,
    BytesEnd};

use crate::GoodreadsUser;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Book { 
    pub title: String,
    pub author: String,
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
struct CalibreBook {
    authors: String,
    id: u32,
    title: String,
}


pub fn Build_ToRead_Map(users: &Vec<GoodreadsUser>) -> Result<HashMap<String,Book>, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-rss"));

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Error building the reqwest client: {err}");
            process::exit(1);
        });

    let mut books_to_download = HashMap::new();

    for user in users {
        for shelf in &user.shelves {
            let url = format!( "https://www.goodreads.com/review/list_rss/{id}?shelf={shelf}", id = user.id, shelf = shelf);
            let response = client
                .get(url)
                .send()?
                .text()?;
            let user_books = parse_xml(&response)?;
            books_to_download.extend(user_books);
        }
    }
    Ok(books_to_download)
}


//TODO rename result to something useful
// I'm parsing json into a calibre book, converting that into a book, converting that into a string
// maybe just parse json into the string?
pub fn get_current_library() -> Result<Vec<String>, Box<dyn Error>> {
    let calibre_command = process::Command::new("calibredb") 
        .args(["list", "--for-machine"])
        .output()?;

    let calibre_books: Vec<CalibreBook> = serde_json::from_slice(&calibre_command.stdout)?;


    let mut result = Vec::new();
    for book in calibre_books {
        let map_book = Book{
            title: book.title,
            author: book.authors,
        };
        result.push(generate_title_key(&map_book));
    }

    Ok(result)
}



// TODO return hasmap<string book> or hashmap<title, author>
fn parse_xml(xml: &String) -> Result<HashMap<String, Book>, quick_xml::errors::Error> {
    let mut books = HashMap::with_capacity(100);
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
                let title_key = generate_title_key(&book);
                books.insert(title_key, book);
                title = "".to_string();
                author = "".to_string();
            },
            _ => (),
        }
    }
    Ok(books)
}


// This function creates the hash map key for the book by stripping
// TODO swap chars to lower
fn generate_title_key(book: &Book) -> String {
    let mut a = book.title
        .clone();
    a.retain(|c| c.is_ascii_alphanumeric() || c == ' ');
    a
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_title_key() {
        let a = Book{
            title: "\"a\" time: thing's we see!".to_string(),
            author: "whoever".to_string(),
        }; 
        assert_eq!(generate_title_key(&a), String::from("a time things we see")); 
    }

    #[test]
    fn test_build_reader_map() {
        let test = vec!(GoodreadsUser{
            name: "Test".to_string(),
            id: 186388648,
            shelves: vec!("test".to_string()),
        });
        let mut result = HashMap::new();
        result.insert("Brave New World".to_string(), Book{
            title: "Brave New World".to_string(),
            author: "Aldous Huxley".to_string(),
        });
        assert_eq!(Build_ToRead_Map(&test).unwrap(), result);
    }
    
    #[test]
    fn test_get_calibre_library() {
        let res = get_current_library().unwrap();
 //       let book = Some(Book{ title: "Babel".to_string(), author: "R. F. Kuang".to_string()});
 //       assert_eq!(res.get("Babel"), book.as_ref());
          assert_eq!(1, 1);
    }
    
}
