
struct Book { 
    title: String,
    author: String,
}

#[derive(Deserialize)]
struct CalibreBook {
    authors: String,
    id: usize,
    title: String,
}


// TODO implement download shelf and figure out how i want to provide the url
fn Goodreads_Download_Manager(users: &Vec<GoodreadsUser) -> Result<HashMap<String,String>, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", String::from("rust-rss"));

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Error building the reqwest client: {err}");
            process::exit(1);
        });
    let url: &str =;

    let mut books_to_download = HashMap::new();

    for user in users {
        for shelf in user.shelves {
            let url = format!( "https://www.goodreads.com/review/list_rss/{id}?shelf={shelf}", id = user.id, shelf = shelf);
            let response = client
                .get(url)
                .send()?
                .text()?
                }
                }
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
                let titleKey = generate_title_key(&book);
                books.insert(titleKey, book);
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
    book.title
        .copy()
        .retain(|c| c.is_ascii_alphanumeric())
}


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



