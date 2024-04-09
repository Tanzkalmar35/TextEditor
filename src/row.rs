use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut res = String::new();
        // FYI: @link https://en.wikipedia.org/wiki/Grapheme
        for grapheme in self.string[..].graphemes(true).skip(start).take(end - start) {
            if grapheme == "\t" {
                res.push_str("    ")
            } else {
                res.push_str(grapheme)
            }
        }
        res
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, pos_in_line: usize, c: char) {
        if pos_in_line >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut res: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == pos_in_line {
                length += 1;
                res.push(c);
            }
            res.push_str(grapheme);
        }
        self.len = length;
        self.string = res;
    }

    pub fn delete(&mut self, pos_in_line: usize) {
        if pos_in_line >= self.len() {
            return;
        }
        let mut res: String = String::new();
        let mut length = 0;
        for (idx, grapheme) in self.string[..].graphemes(true).enumerate() {
            if idx != pos_in_line {
                length += 1;
                res.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = res;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, pos_in_line: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;

        for (idx, grapheme) in self.string[..].graphemes(true).enumerate() {
            if idx < pos_in_line {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length = length + 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;
        Self {
            string: splitted_row,
            len: splitted_length,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}