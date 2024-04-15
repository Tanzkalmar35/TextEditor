use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
use termion::color;

use crate::HighlightingOptions;
use crate::highlighting;
use crate::SearchDirection;

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<highlighting::Type>,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            highlighting: Vec::new(),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut res = String::new();
        let mut current_highilghting = &highlighting::Type::None;

        #[allow(clippy::integer_arithmetic)]
        for (idx, grapheme) in self.string[..].graphemes(true).enumerate().skip(start).take(end - start) {
            if let Some(c) = grapheme.chars().next() {
                let highlighting_type = self.highlighting.get(idx).unwrap_or(&highlighting::Type::None);
                if highlighting_type != current_highilghting {
                    current_highilghting = highlighting_type;
                    let start_hightlight = format!("{}", termion::color::Fg(highlighting_type.to_color()));
                    res.push_str(&start_hightlight[..]);
                }
                if c == '\t' {
                    res.push_str(" ");
                } else {
                    res.push(c)
                }
            }
        }
        
        let end_highlight = format!("{}", termion::color::Fg(color::Reset));
        res.push_str(&end_highlight[..]);
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
            highlighting: Vec::new(),
            len: splitted_length,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn find(&self, query: &str, pos: usize, dir: SearchDirection) -> Option<usize> {
        if pos > self.len || query.is_empty() {
            return None;
        }
        let start = if dir == SearchDirection::Forward {
            pos
        } else {
            0
        };
        let end = if dir == SearchDirection::Forward {
            self.len
        } else {
            pos
        };
        #[allow(clippy::integer_arithmetic)] 
        let substring: String = self.string[..].graphemes(true).skip(start).take(end - start).collect();
        let matching_byte_idx = if dir == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };
        if let Some(matching_byte_idx) = matching_byte_idx {
            for (grapheme_idx, (byte_idx, _)) in substring[..].grapheme_indices(true).enumerate() {
                if matching_byte_idx == byte_idx {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_idx);
                }
            }
        }
        None
    }

    fn highlight_search_res(&mut self, word: Option<&str>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }
            let mut idx = 0;
            while let Some(search_match) = self.find(word, idx, SearchDirection::Forward) {
                if let Some(next_idx) = search_match.checked_add(word[..].graphemes(true).count()) {
                    #[allow(clippy::indexing_slicing)]
                    for i in search_match..next_idx {
                        self.highlighting[i] = highlighting::Type::SearchResult;
                    }
                    idx = next_idx;
                } else {
                    break;
                }
            }
        }
    }

    fn highlight_char(&mut self, idx: &mut usize, opts: HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.characters() && c == '\'' {
            if let Some(next_char) = chars.get(idx.saturating_add(1)) {
                let closing_idx = if *next_char == '\\' {
                    idx.saturating_add(3)
                } else {
                    idx.saturating_add(2)
                };
                if let Some(closing_char) = chars.get(closing_idx) {
                    if *closing_char == '\'' {
                        for _ in 0..=closing_idx.saturating_sub(*idx) {
                            self.highlighting.push(highlighting::Type::Character);
                            *idx += 1;
                        }
                        return true;
                    }
                }
            };
        }
        false
    }

    fn highilght_comment(&mut self, idx: &mut usize, opts: HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.comments() && c == '/' && *idx < chars.len() {
            if let Some(next_char) = chars.get(idx.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *idx..chars.len() {
                        self.highlighting.push(highlighting::Type::Comment);
                    }
                    return true;
                }
            };
        }
        false
    }

    fn highlight_string(&mut self, idx: &mut usize, opts: HighlightingOptions, c: char, chars: &[char]) -> bool {
        if opts.strings() && c == '"' {
            loop {
                self.highlighting.push(highlighting::Type::String);
                *idx += 1;
                if let Some(next_char) = chars.get(*idx) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.highlighting.push(highlighting::Type::String);
            *idx += 1;
            return true;
        }
        false
    }

    fn highlight_number(&mut self, idx: &mut usize, otps: HighlightingOptions, c: char, chars: &[char]) -> bool {
        if otps.numbers() && c.is_ascii_digit() {
            if *idx > 0 {
                #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
                let prev_char = chars[*idx - 1];
                if !prev_char.is_ascii_punctuation() && !prev_char.is_ascii_whitespace() {
                    return false;
                }
            }
            loop {
                self.highlighting.push(highlighting::Type::Number);
                *idx += 1;
                if let Some(next_char) = chars.get(*idx) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        }
        false
    }

    pub fn highlight(&mut self, opts: HighlightingOptions, word: Option<&str>) {
        self.highlighting = Vec::new();
        let chars: Vec<char> = self.string.chars().collect();
        let mut idx = 0;
        while let Some(c) = chars.get(idx) {
            if self.highlight_char(&mut idx, opts, *c, &chars)
                || self.highilght_comment(&mut idx, opts, *c, &chars)
                || self.highlight_string(&mut idx, opts, *c, &chars)
                || self.highlight_number(&mut idx, opts, *c, &chars) {
                    continue;
            }
            self.highlighting.push(highlighting::Type::None);
            idx += 1;
        }
        self.highlight_search_res(word);
    }
}