use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    SearchResult,
    String,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::SearchResult => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            _ => color::Rgb(255, 255, 255),
        }
    }
}