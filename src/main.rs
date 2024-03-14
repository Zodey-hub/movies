mod constants;

use std::error::Error;

use tabled::{Table, Tabled};

use crate::constants::{
    errors::{ERROR_LANGUAGE, ERROR_LANGUAGE_END, ERROR_QUALITY_END, ERROR_TITLE},
    LANGUAGE, LANGUAGE_END, NETMOZI_URL, QUALITY_END, TITLE, TITLE_END,
};

#[allow(non_snake_case)]
#[derive(Debug, Tabled)]
struct Movie<'a> {
    Cím: &'a str,
    Nyelv: &'a str,
    Minőség: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::new();
    http_req::request::get(NETMOZI_URL, &mut buffer)?;
    let website = String::from_utf8(buffer)?;

    let mut movies = Vec::new();

    let titles: Vec<(usize, &str)> = website.match_indices(TITLE).collect();
    for title in titles {
        let title_end = website[title.0..].find(TITLE_END).ok_or(ERROR_TITLE)? + title.0;

        let title = website[TITLE.len() + title.0..title_end].trim();

        let language_start =
            website[title_end..].find(LANGUAGE).ok_or(ERROR_LANGUAGE)? + title_end + LANGUAGE.len();

        let language_end = website[language_start..]
            .find(LANGUAGE_END)
            .ok_or(ERROR_LANGUAGE_END)?
            + language_start;

        let language = &website[language_start..language_end];

        let quality_end = website[language_end..]
            .find(QUALITY_END)
            .ok_or(ERROR_QUALITY_END)?
            + language_end;

        let quality = website[language_end + LANGUAGE_END.len()..quality_end].trim();

        movies.push(Movie {
            Cím: title,
            Nyelv: language,
            Minőség: quality,
        });
    }
    println!("{}", Table::new(movies));
    Ok(())
}
