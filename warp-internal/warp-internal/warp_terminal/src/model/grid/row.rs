use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Row {
    /// Stores cells indexed by column position
    cells: HashMap<usize, Cell>,
    /// Length of the row (rightmost cell position + 1)
    length: usize,
}

#[derive(Debug, Clone)]
pub struct Cell {
    /// The character in this cell
    character: char,
    /// Cell attributes (like colors, style, etc)
    attributes: CellAttributes,
}

#[derive(Debug, Clone, Default)]
pub struct CellAttributes {
    foreground: Option<Color>,
    background: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Rgb(u8, u8, u8),
    AnsiIndex(u8),
}

impl Row {
    pub fn new() -> Self {
        Row {
            cells: HashMap::new(),
            length: 0,
        }
    }

    pub fn set_cell(&mut self, column: usize, character: char, attributes: CellAttributes) {
        self.cells.insert(column, Cell { character, attributes });
        self.length = self.length.max(column + 1);
    }

    pub fn get_cell(&self, column: usize) -> Option<&Cell> {
        self.cells.get(&column)
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.length = 0;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
}
