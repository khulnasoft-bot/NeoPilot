use std::collections::HashMap;

/// Represents indexed terminal data
#[derive(Default)]
pub struct TerminalIndex {
    // Map line numbers to content
    lines: HashMap<usize, String>,
    // Store searchable keywords
    keywords: HashMap<String, Vec<usize>>,
}

pub fn index_data(content: &str) -> TerminalIndex {
    let mut index = TerminalIndex::default();
    
    // Index each line
    for (line_num, line) in content.lines().enumerate() {
        // Store the full line
        index.lines.insert(line_num, line.to_string());
        
        // Index individual words
        for word in line.split_whitespace() {
            index.keywords
                .entry(word.to_lowercase())
                .or_default()
                .push(line_num);
        }
    }
    
    index
}

impl TerminalIndex {
    /// Search for lines containing the given keyword
    pub fn search(&self, keyword: &str) -> Vec<&String> {
        self.keywords
            .get(&keyword.to_lowercase())
            .map(|line_nums| {
                line_nums
                    .iter()
                    .filter_map(|&num| self.lines.get(&num))
                    .collect()
            })
            .unwrap_or_default()
    }
}
