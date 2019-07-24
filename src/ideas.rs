
// ideas

struct Content {
    elmts: Vec<Elmt>,
    cursor: (usize, usize),
}

struct Elmt {
    character: char,
    whitespace: Whitespace,
}

struct Whitespce {
    typed: Vec<WhitespaceChar>,
    virtual_newlines: usize,
    virtual_spaces: usize, // on last line
}

enum WhitespaceChar {
    Space,
    Newline
}

// For the view

struct ViewModel {
    cursor: CursorPos,
    text: String,
}

struct CursorPos {
    line: usize,
    col: usize,
    between: bool,
}

