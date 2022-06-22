use tabled::{Table, Tabled};

mod httputils;
mod stringutils;

#[derive(Tabled)]
struct Movie {
    Cím: String,
    Nyelv: String,
    Minőség: String,
}

fn main() {
    let title = "<div class=\"recommended_block_name\">";
    let title_end = "<br />";
    let mut temp_title: String;

    let language = "title=\"";
    let language_end = "\">";
    let mut temp_language: String;

    let mut quality: String;
    let quality_end = "</div>";
    let mut temp_quality: String;

    let mut chunk = httputils::get_content("https://netmozi.com/");
    let mut number_of_movies = stringutils::get_occurrences(&chunk, title);
    let mut movies: Vec<Movie> = Vec::new();

    while number_of_movies != 0 {
        chunk = chunk[chunk.find(title).unwrap()..].to_string();
        temp_title = stringutils::give_text_between(&chunk, title, title_end);

        chunk = chunk[chunk.find(language).unwrap()..].to_string();
        temp_language = stringutils::give_text_between(&chunk, language, language_end);

        quality = chunk[chunk.find(language).unwrap()..].to_string();
        temp_quality = stringutils::give_text_between(&quality, "\">", quality_end);

        movies.push(Movie {
            Cím: temp_title,
            Nyelv: temp_language,
            Minőség: temp_quality,
        });

        number_of_movies -= 1;
    }

    println!("{}", Table::new(movies).to_string());
}