const SOLIDS: &str = "Oo█#@%&|";

#[derive(Clone)]
pub struct Cell {
    pub state: bool,
    pub num_on: u8,
    pub num_on_next: u8
}

impl Cell {
    pub fn new() -> Cell {
        Cell { state: false, num_on: 0, num_on_next: 0 }
    }

    pub fn adjust_all(&mut self, do_inc: bool) {
        self.num_on = if do_inc { self.num_on + 1 } else { self.num_on - 1 };
        self.num_on_next = if do_inc { self.num_on_next + 1 } else { self.num_on_next - 1 };
    }

    pub fn adjust_next(&mut self, do_inc: bool) {
        self.num_on_next = if do_inc { self.num_on_next + 1 } else { self.num_on_next - 1 };
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new()
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.state { "██" } else { "  " })
    }
}

impl From<bool> for Cell {
    fn from(state: bool) -> Self {
        Cell { state, num_on: 0, num_on_next: 0 }
    }
}

impl From<char> for Cell {
    fn from(char: char) -> Self {
        Cell::from(SOLIDS.contains(char))
    }
}