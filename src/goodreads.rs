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

pub struct Book { 
    pub title: String,
    pub author: String,
}

#[derive(Deserialize)]
struct CalibreBook {
    authors: String,
    id: usize,
    title: String,
}


pub fn Build_ToRead_Map(users: &Vec<GoodreadsUser>) -> Result<HashMap<String,String>, Box<dyn Error>> {
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
                }
                }
    Ok(books_to_download)
}

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
fn generate_title_key(book: &Book) -> String {
    let mut a = book.title
        .clone();
    a.retain(|c| c.is_ascii_alphanumeric() || c == ' ');
    a
}


/*
// This function takes a vec of Book, creates a set() of book names from calibredb and removes any
// duplicates
// The resulting list of books should be downloaded
fn remove_duplicate_books(books: Vec<Book>) {
    // get current library via calibredb list as json with --for-machine
    let mut output = Command::new("calibredb")
        .args(["list", "--for-machine"])
        .output()
        .expect("failed to get calibre books");

    //TODO output is a structure that contains stdout which is a vec<u8> of json containing all the
    //library
    calibreOutputIter = output.stdout.split_inclusive("},\n");

    let calibreBooks = HashMap::new();

    for existingBook in calibreOutputIter {
        //parse into calibre book, strip non-alphanumerics from title, add to hashset
        let mut book = serde_json::from_str(existingBook);
        book.title.retain(|c| c.is_ascii_alphanumeric());
        calibreBooks.insert(book.title);
    }

    for book in books {
        let mut titleclone = book.title.clone();
        titleclone.retain(|c| c.is_ascii_alphanumeric());
        if calibreBooks.contains(titleclone) {
            //remove the book from the new books
        }

}
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_title_key() {
        let a = Book{
            title: "\"a\" time: thing's we see!".to_string(),
            author: "whoever".to_string(),
        }; assert_eq!(generate_title_key(&a), String::from("a time things we see")); }
}
