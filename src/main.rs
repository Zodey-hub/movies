use std::error::Error;

use tabled::{Table, Tabled};

#[allow(non_snake_case)]
#[derive(Debug, Tabled)]
struct Movie {
    Cím: String,
    Nyelv: String,
    Minőség: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    const TITLE: &str = "<div class=\"recommended_block_name\">";
    const TITLE_END: &str = "<br />";

    const LANGUAGE: &str = "title=\"";
    const LANGUAGE_END: &str = "\">";

    const QUALITY_END: &str = "</div>";

    let mut buffer = Vec::new();
    http_req::request::get("https://netmozi.com/", &mut buffer)?;
    let mut website = String::from_utf8(buffer)?;

    let mut number_of_movies = get_occurrences(&website, TITLE);
    let mut movies: Vec<Movie> = Vec::new();

    while number_of_movies != 0 {
        website = website[website.find(TITLE).ok_or("Failed to find title!")?..].to_string();
        let temp_title = give_text_between(&website, TITLE, TITLE_END)?;

        website = website[website.find(LANGUAGE).ok_or("Failed to find language!")?..].to_string();
        let temp_language = give_text_between(&website, LANGUAGE, LANGUAGE_END)?;

        let temp_quality = give_text_between(&website, "\">", QUALITY_END)?;

        movies.push(Movie {
            Cím: temp_title.to_string(),
            Nyelv: temp_language.to_string(),
            Minőség: temp_quality.to_string(),
        });

        number_of_movies -= 1;
    }

    println!("{}", Table::new(movies));
    Ok(())
}

fn get_occurrences(source: &str, string_to_find: &str) -> usize {
    source.matches(string_to_find).count()
}

fn give_text_between(source: &str, before: &str, after: &str) -> Result<String, &'static str> {
    let start_bytes = source.find(before).ok_or("Before string not found")? + before.len();
    let end_bytes = source[start_bytes..]
        .find(after)
        .ok_or("After string not found")?;

    Ok(source[start_bytes..start_bytes + end_bytes]
        .trim()
        .to_string())
}
