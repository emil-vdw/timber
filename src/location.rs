#[derive(Debug, Clone)]
pub struct Location {
    pub char: usize,
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(char: usize, line: usize, column: usize) -> Self {
        Location {
            char,
            line,
            column,
        }
    }
}
