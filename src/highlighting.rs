use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    SearchResult,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::SearchResult => color::Rgb(38, 139, 210),
            _ => color::Rgb(255, 255, 255),
        }
    }
}