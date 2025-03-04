///test if the given input is a visual square (all lines delimited by LF, CR or EOI are of same length)
///return a size (width, height) if the input is square or none if the input is not square
///note: CRLF and LFCR are interpreted as a single line delimiter, but LFLF or CRCR would be considered like 2 lines (with the second of size 0)
///note: the height is always rounded up (the last line is not required to be terminated by a LF/CR/CRLF/LFCR)
pub fn is_text_square(text: &str) -> Option<(usize, usize)> {
    let mut width = 0;
    let mut line = 0;
    let mut height = 0;
    let mut lf = false;
    let mut cr = false;
    for c in text.chars() {
        if match c {
            '\n' => {
                if lf {
                    true
                } else if cr {
                    cr = false;
                    true
                } else {
                    lf = true;
                    false
                }
            }
            '\r' => {
                if cr {
                    true
                } else if lf {
                    lf = false;
                    true
                } else {
                    cr = true;
                    false
                }
            }
            _ => {
                if cr || lf {
                    cr = false;
                    lf = false;
                    true
                } else {
                    false
                }
            }
        } {
            if height == 0 {
                width = line;
            }
            if line != width {
                return None;
            }
            height += 1;
            line = 0;
        }
        if c != '\n' && c != '\r' {
            line += 1;
        }
    }
    if height == 0 {
        return Some((line, 1));
    }
    if width != line {
        return None;
    }
    Some((width, height + if !cr && !lf { 1 } else { 0 }))
}

struct LineIterator<'s> {
    include_empty: bool,
    text: &'s str,
    raw: Vec<char>,
    line: usize,
    index: usize,
}

impl <'s> Iterator for LineIterator<'s> {
    type Item = (usize, &'s str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.include_empty && self.index == 0 && self.raw.len() == 0 {
            self.index = 1;
            return Some((0, &self.text[..]));
        }
        if self.index >= self.raw.len() { return None; }
        let sub = &self.text[self.index..];
        let prev = self.index;
        while self.index < self.raw.len() {
            let r = match (sub.find('\n'), sub.find('\r')) {
                (Some(lf), Some(cr)) => {
                    self.line += 1;
                    if lf == cr + 1 || lf == cr - 1 {
                        self.index += lf.max(cr) + 1;
                        Some((self.line - 1, &self.text[prev..self.index - 2]))
                    } else {
                        self.index += lf.min(cr) + 1;
                        Some((self.line - 1, &self.text[prev..self.index - 1]))
                    }
                }
                (Some(lf), None) => {
                    self.line += 1;
                    self.index += lf + 1;
                    Some((self.line - 1, &self.text[prev..self.index - 1]))
                }
                (None, Some(cr)) => {
                    self.line += 1;
                    self.index += cr + 1;
                    Some((self.line - 1, &self.text[prev..self.index - 1]))
                }
                (None, None) => {
                    self.index = self.raw.len();
                    Some((self.line, &self.text[prev..self.index]))
                }
            };
            if !self.include_empty && self.index - prev == 0 {
                continue;
            }
            return r;
        }
        None
    }
}

pub fn iter_lines(text: &str, include_empty: bool) -> impl Iterator<Item = (usize, &str)> {
    LineIterator {
        include_empty,
        index: 0,
        line: 0,
        raw: text.chars().collect(),
        text,
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_text_square() {
        assert_eq!(is_text_square(""), Some((0, 1)));
        assert_eq!(is_text_square("test!\n\r3 . .\nlines"), Some((5, 3)));
        assert_eq!(is_text_square("1\n21"), None);
    }

    #[test]
    fn test_iter_lines() {
        assert_eq!(iter_lines("", true).collect::<Vec<(usize, &str)>>(), vec![(0, "")], "test empty string with include empty");
        assert_eq!(iter_lines("", false).collect::<Vec<(usize, &str)>>(), vec![], "test empty string without include empty");
    }
}