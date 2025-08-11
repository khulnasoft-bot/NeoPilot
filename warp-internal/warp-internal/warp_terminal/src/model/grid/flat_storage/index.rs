use std::collections::HashMap;

pub struct FlatStorageIndex {
    // Maps row numbers to their starting indices in the flat storage
    row_indices: HashMap<usize, usize>,
    // Total number of columns (width) of the grid
    columns: usize,
    // Total number of rows currently indexed
    row_count: usize,
}

impl FlatStorageIndex {
    pub fn new() -> Self {
        FlatStorageIndex {
            row_indices: HashMap::new(),
            columns: 0,
            row_count: 0,
        }
    }

    pub fn set_dimensions(&mut self, columns: usize) {
        self.columns = columns;
    }

    pub fn add_row(&mut self, row: usize, start_index: usize) {
        self.row_indices.insert(row, start_index);
        self.row_count = self.row_count.max(row + 1);
    }

    pub fn get_index(&self, row: usize, column: usize) -> Option<usize> {
        if column >= self.columns {
            return None;
        }

        self.row_indices.get(&row).map(|&start_index| start_index + column)
    }

    pub fn clear(&mut self) {
        self.row_indices.clear();
        self.row_count = 0;
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}
