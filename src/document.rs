use std::fs;
use std::io::{Error, Write};

use crate::Row;
use crate::Position;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    changed: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok( 
            Self { 
                rows, 
                file_name: Some(filename.to_string()),
                changed: false,
            } 
        )
    }

    pub fn row(&self, idx: usize) -> Option<&Row> {
        self.rows.get(idx)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_new_line(&mut self, pos: &Position) {
        if pos.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(pos.y).unwrap().split(pos.x);
        self.rows.insert(pos.y + 1, new_row);
    }

    pub fn insert(&mut self, pos: &Position, c: char) {
        if pos.y > self.len() {
            return;
        }
        self.changed = true;
        if c == '\n' {
            self.insert_new_line(pos);
            return;
        }
        if pos.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(pos.y).unwrap();
            row.insert(pos.x, c)
        }
    }

    pub fn delete(&mut self, pos: &Position) {
        if pos.y >= self.len() {
            return;
        }
        self.changed = true;
        // If backspace is pressed at the beginning of a line, append it to the preceding line
        if pos.x == self.rows.get_mut(pos.y).unwrap().len() && pos.y < self.len() - 1 {
            let next_row = self.rows.remove(pos.y + 1);
            let row = self.rows.get_mut(pos.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(pos.y).unwrap();
            row.delete(pos.x);
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.changed = false;
        }
        Ok(())
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn find(&self, query: &str, after: &Position) -> Option<Position> {
        let mut x = after.x;
        for (y, row) in self.rows.iter().enumerate().skip(after.y) {
            if let Some(x) = row.find(query, x) {
                return Some(Position {x, y});
            }
            x = 0;
        }
        None
    }
}