#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(            
    clippy::missing_docs_in_private_items,            
    clippy::implicit_return,            
    clippy::shadow_reuse,            
    clippy::print_stdout,            
    clippy::wildcard_enum_match_arm,            
    clippy::else_if_without_else            
)]
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
/// 07: Colorful Characters
fn main() {
    Editor::default().run();
}
