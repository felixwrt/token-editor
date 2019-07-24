
// ideas


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

