use std::io::{self, stdout, Write, Stdout, Error};
use termion::color::{Rgb, Bg, Fg, Reset};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::clear::{CurrentLine, All};
use termion::cursor::{Show, Goto, Hide};

use crate::Position;

pub struct Size {            
    pub width: u16,  
    pub height: u16,          
}            
pub struct Terminal {   
    size: Size,     
    _stdout: RawTerminal<Stdout>    
}            
                               
impl Terminal {     
    /// # Errors
    /// 
    /// Will return an `std::io::Error` 
    pub fn default() -> Result<Self, Error> {            
        let size = termion::terminal_size()?;            
        Ok(Self {            
            size: Size {            
                width: size.0,            
                height: size.1.saturating_sub(2),            
            },    
            _stdout: stdout().into_raw_mode()?,        
        })            
    }          

    #[must_use]
    pub fn size(&self) -> &Size {            
        &self.size            
    }  

    pub fn clear_screen() {
        print!("{All}");
    }

    /// # Panics
    ///
    /// Will panic if moving the cursor to the position fails.    
    #[allow(clippy::cast_possible_truncation)]      
    pub fn cursor_position(position: &Position) {
        let Position{x, y} = position;
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", Goto(
            x.try_into().expect(format!("Wasn't able to move the cursor to x: {x}").as_str()), 
            y.try_into().expect(format!("Wasn't able to move the cursor to y: {y}").as_str())
        ));
    }

    /// # Errors
    /// 
    /// Will return an `std::io::Error` if not all bytes could be written.
    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }

    /// # Errors
    /// 
    /// Will return an `std::io::Error` if there was a problem reading a Key.
    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }   

    pub fn cursor_hide() {
        print!("{Hide}");
    }

    pub fn cursor_show() {
        print!("{Show}");
    }   

    pub fn clear_current_line() {
        print!("{CurrentLine}");
    }   

    pub fn set_bg_color(color: Rgb) {
        print!("{}", Bg(color));
    } 

    pub fn reset_bg_color() {
        print!("{}", Bg(Reset));
    }

    pub fn set_fg_color(color: Rgb) {
        print!("{}", Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", Fg(Reset));
    }
}