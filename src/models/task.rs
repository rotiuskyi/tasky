use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Task {
    pub title: String,
    pub priority: Priority,
    pub description: String,
    pub is_done: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        })
    }
}
