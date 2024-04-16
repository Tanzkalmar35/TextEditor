mod document;
mod row;
mod editor;
mod filetype;
mod terminal;
mod highlighting;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use row::Row;
pub use document::Document;
pub use editor::SearchDirection;

/// This text editor is built using the foundation from this blog:
/// https://archive.flenker.blog/hecto/
/// 07: Color keywords
fn main() {
    Editor::default().run();
}
