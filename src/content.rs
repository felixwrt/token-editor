
use std::string::ToString;

#[derive(Clone, Debug)]
struct Content {
    elmts: Vec<Elmt>,
    cursor: (usize, usize),  // first element is the index of the selected whitespace element.
                             // the sectond element is the selection index within that whitespace element
    final_whitespace: Whitespace
}

#[derive(Clone, Debug)]
struct Elmt {
    character: char,
    whitespace: Whitespace,  // whitespace that's preceeding the character
}

#[derive(Clone, Debug)]
struct Whitespace {
    typed: Vec<WhitespaceChar>,
    virtual_newlines: usize,
    virtual_spaces: usize,  // on last line
}

#[derive(Clone, Debug)]
enum WhitespaceChar {
    Space,
    Newline
}

#[derive(Clone, Debug)]
struct CursorPos {
    line: usize,
    col: usize,
    between: bool,
}


impl WhitespaceChar {
    fn is_newline(&self) -> bool {
        match self {
            WhitespaceChar::Newline => true,
            _ => false,
        }
    }
}


trait GetString {
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
    fn from_strings(typed: &str, visible: &str) -> Content {
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
                        _ => panic!("this shouldn't happen!")
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

    fn cursor_pos(&self) -> CursorPos {
        unimplemented!()
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
