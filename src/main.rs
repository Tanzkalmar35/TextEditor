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
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;
pub use document::Document;

/// This text editor is built using the foundation from this blog:
/// https://www.flenker.blog/hecto/
/// TODO: Can't submit search results and stay at pos
fn main() {
    Editor::default().run();
}
