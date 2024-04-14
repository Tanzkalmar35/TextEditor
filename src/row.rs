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

    pub fn highlight(&mut self, opts: HighlightingOptions, word: Option<&str>) {
        let mut highlighting = Vec::new();
        let chars: Vec<char> = self.string.chars().collect();
        let mut matches = Vec::new();
        let mut search_idx = 0;

        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_idx, SearchDirection::Forward) {
                matches.push(search_match);
                if let Some(next_idx) = search_match.checked_add(word[..].graphemes(true).count()) {
                    search_idx = next_idx;
                } else {
                    break;
                }
            }
        }

        let mut prev_is_seperator = true;
        let mut in_string = false;
        let mut idx = 0;
        while let Some(c) = chars.get(idx) {
            if let Some(word) = word {
                if matches.contains(&idx) {
                    for _ in word[..].graphemes(true) {
                        idx += 1;
                        highlighting.push(highlighting::Type::SearchResult);
                    }
                    continue;
                }
            }
            
            let prev_highlight = if idx > 0 {
                highlighting.get(idx - 1).unwrap_or(&highlighting::Type::None)
            } else {
                &highlighting::Type::None
            };
            if opts.strings() {
                if in_string {
                    highlighting.push(highlighting::Type::String);

                    if *c == '\\' && idx < self.len().saturating_sub(1) {
                        highlighting.push(highlighting::Type::String);
                        idx += 2;
                        continue;
                    }

                    if *c == '"' {
                        in_string = false;
                        prev_is_seperator = true;
                    } else {
                        prev_is_seperator = false;
                    }
                    idx += 1;
                    continue;
                } else if prev_is_seperator && *c == '"' {
                    highlighting.push(highlighting::Type::String);
                    in_string = true;
                    prev_is_seperator = true;
                    idx += 1;
                    continue;
                }
            }
            if opts.numbers() {
                if (c.is_ascii_digit() && (prev_is_seperator || *prev_highlight == highlighting::Type::Number))
                    || (*c == '.' && *prev_highlight == highlighting::Type::Number) {
                    highlighting.push(highlighting::Type::Number);
                } else {
                    highlighting.push(highlighting::Type::None);
                };
                prev_is_seperator = c.is_ascii_punctuation() || c.is_ascii_whitespace();
                idx += 1;
            } else {
                highlighting.push(highlighting::Type::None);
            }
        }

        self.highlighting = highlighting;
    }
}