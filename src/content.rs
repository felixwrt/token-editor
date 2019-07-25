
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct Content {
    elmts: Vec<Elmt>,
    cursor: (usize, usize),  // first element is the index of the selected whitespace element.
                             // the sectond element is the selection index within that whitespace element
    final_whitespace: Whitespace
}

#[derive(Clone, Debug)]
pub struct Elmt {
    character: char,
    whitespace: Whitespace,  // whitespace that's preceeding the character
}

#[derive(Clone, Debug)]
pub struct Whitespace {
    typed: Vec<WhitespaceChar>,
    virtual_newlines: usize,
    virtual_spaces: usize,  // on last line
}

#[derive(Clone, Debug)]
pub enum WhitespaceChar {
    Space,
    Newline
}

#[derive(Clone, Debug)]
pub struct CursorPos {
    pub line: usize,
    pub col: usize,
    pub between: bool,
}


impl WhitespaceChar {
    fn is_newline(&self) -> bool {
        match self {
            WhitespaceChar::Newline => true,
            _ => false,
        }
    }
}


pub trait GetString {
    fn get_string(&self) -> String;   // TODO: add option for visible whitespace
}

impl GetString for WhitespaceChar {
    fn get_string(&self) -> String {
        match self {
            WhitespaceChar::Space => " ".to_string(),
            WhitespaceChar::Newline => "\n".to_string(),
        }
    }
}

impl GetString for Whitespace {
    fn get_string(&self) -> String {
        let num_typed_newlines = self.typed.iter().filter(|x| x.is_newline()).count();
        let num_spaces_last_line = self.typed.iter().rev().take_while(|x| !x.is_newline()).count();
        let mut s: String = self.typed.iter().map(|x| x.get_string()).collect();
        
        if num_typed_newlines < self.virtual_newlines {
            s.push_str(&"\n".repeat(self.virtual_newlines - num_typed_newlines));
            s.push_str(&" ".repeat(self.virtual_spaces));
        } else if num_typed_newlines == self.virtual_newlines && num_spaces_last_line < self.virtual_spaces {
            s.push_str(&" ".repeat(self.virtual_spaces - num_spaces_last_line));
        }
        
        s
    }
}

impl GetString for Elmt {
    fn get_string(&self) -> String {
        let mut s = self.whitespace.get_string();
        s.push(self.character);
        s
    }
}

impl GetString for Content {
    fn get_string(&self) -> String {
        let mut s: String = self.elmts.iter().map(|x| x.get_string()).collect();
        s.push_str(&self.final_whitespace.get_string());
        s
    }
}



impl Content {
    pub fn from_strings(typed: &str, visible: &str) -> Content {
        let mut chars = typed.chars();
        let mut visible_chars = visible.chars();
        let mut elmts = vec!();
        let mut current_whitespace = vec!();
        
        while let Some(c) = chars.next() {
            if c == ' ' {
                current_whitespace.push(WhitespaceChar::Space);
            } else if c == '\n' {
                current_whitespace.push(WhitespaceChar::Newline);
            } else {
                let mut virtual_newlines = 0;
                let mut virtual_spaces = 0;
                while let Some(vc) = visible_chars.next() {
                    match vc {
                        '\n' => {virtual_newlines += 1; virtual_spaces = 0;},
                        ' ' => {virtual_spaces += 1;},
                        x if x == c => break,
                        x => println!("Ignoring character {:?}.", x)
                    }
                }
                
                elmts.push(Elmt{
                    character: c,
                    whitespace: Whitespace {
                        typed: current_whitespace.clone(),
                        virtual_newlines,
                        virtual_spaces
                    }
                });
                current_whitespace = vec!();
            }
        }

        let mut virtual_newlines = 0;
        let mut virtual_spaces = 0;
        while let Some(vc) = visible_chars.next() {
            match vc {
                '\n' => {virtual_newlines += 1; virtual_spaces = 0;},
                ' ' => {virtual_spaces += 1;},
                _ => panic!("this shouldn't happen!")
            }
        }

        Content {
            elmts,
            cursor: (0, 0),
            final_whitespace: Whitespace {
                typed: current_whitespace,
                virtual_newlines,
                virtual_spaces
            }
        }
    }

    pub fn cursor_pos(&self) -> CursorPos {
        let mut s: String = self.elmts.iter().take(self.cursor.0).map(|x| x.get_string()).collect();
        let mut line = s.chars().filter(|x| x == &'\n').count();
        let mut col = 0;
        let mut init_col = s.chars().rev().take_while(|x| x != &'\n').count();
        let mut between = false;
        
        let typed = &self.elmts[self.cursor.0].whitespace.typed;
        for wc in typed.iter().take(self.cursor.1) {
            match wc {
                WhitespaceChar::Space => col += 1,
                WhitespaceChar::Newline => {col = 0; line += 1; init_col = 0},
            }
        }

        if self.cursor.1 > typed.len() {
            init_col = 0;
            col = 0;
            line += self.cursor.1 - typed.len();
        }

        // last element selected
        let virtual_spaces = self.elmts[self.cursor.0].whitespace.virtual_spaces;
        let virtual_newlines = self.elmts[self.cursor.0].whitespace.virtual_newlines;
        let num_typed_newlines = typed.iter().filter(|x| x.is_newline()).count();
        if self.cursor.1 == self.elmts[self.cursor.0].whitespace.get_num_cursor_positions() - 1 && virtual_spaces > col && num_typed_newlines <= virtual_newlines {
            between = ((virtual_spaces - col) % 2) > 0;
            col += (virtual_spaces - col) / 2;
        }

        CursorPos {
            line, col: init_col + col, between
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor = (self.cursor.0 - 1, self.elmts[self.cursor.0 - 1].whitespace.get_num_cursor_positions() - 1);
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.1 < self.elmts[self.cursor.0].whitespace.get_num_cursor_positions() - 1 {
            self.cursor.1 += 1;
        } else if self.cursor.0 < self.elmts.len() - 1 {
            self.cursor = (self.cursor.0 + 1, 0);
        }
        // fixme: selection of final whitespace!
    }

    pub fn insert(&mut self, c: char) {
        // check for whitespace
        if c == '\n' || c == ' ' {
            let typed_len = self.elmts[self.cursor.0].whitespace.typed.len();
            let ws_char = if c == '\n' { WhitespaceChar::Newline } else { WhitespaceChar::Space };
            self.elmts[self.cursor.0].whitespace.typed.insert(std::cmp::min(self.cursor.1, typed_len), ws_char);
            self.cursor.1 += 1;
            return;
        }

        let mut ws_left = self.elmts[self.cursor.0].whitespace.typed.clone();
        let ws_left_len = ws_left.len();
        let ws_right = ws_left.split_off(std::cmp::min(self.cursor.1, ws_left_len));
        let new_elmt = Elmt {
            character: c,
            whitespace: Whitespace {
                typed: ws_left,
                virtual_newlines: 0,
                virtual_spaces: 0,
            }
        };
        self.elmts[self.cursor.0].whitespace.typed = ws_right;
        self.elmts.insert(self.cursor.0, new_elmt);
        self.cursor = (self.cursor.0 + 1, 0);
    }
}

impl Whitespace {
    fn get_num_cursor_positions(&self) -> usize {
        let num_typed_newlines = self.typed.iter().filter(|x| x.is_newline()).count();
        let add_newlines = if num_typed_newlines < self.virtual_newlines {
            self.virtual_newlines - num_typed_newlines
        } else { 0 };
        
        self.typed.len() + 1 + add_newlines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let typed = "fn test(&self,other:&mut usize){let x=(self+1)*other;return1<y}";
        let visible = "fn test(&self, other: &mut usize) {\n    let x = (self + 1) * other;\n    return 1 < y\n}";
        let c = Content::from_strings(&typed, &visible);
        let s = c.get_string();
        assert_eq!(&s, visible);
    }

    #[test]
    fn test_extra_whitespace() {
        let typed = "fn test(&self,  other:\n  \n&mut usize){let x=(self+1)*other;\n return1<y}";
        let visible = "fn test(&self, other: &mut usize) {\n    let x = (self + 1) * other;\n    return 1 < y\n}";
        let out = "fn test(&self,  other:\n  \n&mut usize) {\n    let x = (self + 1) * other;\n    return 1 < y\n}";
        let c = Content::from_strings(&typed, &visible);
        let s = c.get_string();
        assert_eq!(&s, out);
    }

    #[test]
    fn test_num_cursor_positions() {
        use WhitespaceChar::*;
        let ws = Whitespace {
            typed: vec!(),
            virtual_newlines: 0,
            virtual_spaces: 0,
        };
        assert_eq!(ws.get_num_cursor_positions(), 1);
    }
    #[test]
    fn test_num_cursor_positions_typed_only() {
        use WhitespaceChar::*;
        let ws = Whitespace {
            typed: vec!(Space, Space),
            virtual_newlines: 0,
            virtual_spaces: 0,
        };
        assert_eq!(ws.get_num_cursor_positions(), 3);
        
        let ws = Whitespace {
            typed: vec!(Newline),
            virtual_newlines: 0,
            virtual_spaces: 0,
        };
        assert_eq!(ws.get_num_cursor_positions(), 2);
    }

    #[test]
    fn test_num_cursor_positions_virtual_only() {
        use WhitespaceChar::*;
        let ws = Whitespace {
            typed: vec!(),
            virtual_newlines: 0,
            virtual_spaces: 3,
        };
        assert_eq!(ws.get_num_cursor_positions(), 1);
        
        let ws = Whitespace {
            typed: vec!(),
            virtual_newlines: 2,
            virtual_spaces: 10,
        };
        assert_eq!(ws.get_num_cursor_positions(), 3);
    }

    #[test]
    fn test_num_cursor_positions_mixed() {
        use WhitespaceChar::*;
        let ws = Whitespace {
            typed: vec!(Space, Space),
            virtual_newlines: 0,
            virtual_spaces: 5,
        };
        assert_eq!(ws.get_num_cursor_positions(), 3);
        
        let ws = Whitespace {
            typed: vec!(Space, Newline, Space),
            virtual_newlines: 2,
            virtual_spaces: 0,
        };
        assert_eq!(ws.get_num_cursor_positions(), 5);
    }
}
