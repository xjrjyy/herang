#[derive(Debug, Clone)]
pub struct CppCode {
    pub code: String,
    pub tab_count: usize,
}

impl CppCode {
    pub fn enter(&mut self) {
        self.tab_count += 1;
    }

    pub fn leave(&mut self) {
        self.tab_count -= 1;
    }

    // pub fn is_in_end_line(&self) -> bool {
    //     self.code.is_empty() || self.code.as_bytes().last().unwrap() == &('\n' as u8)
    // }

    pub fn push(&mut self, new_code: &str) {
        self.code.push_str(new_code);
    }

    pub fn push_tab(&mut self) {
        for _ in 0..self.tab_count {
            self.push("    ");
        }
    }

    pub fn push_line(&mut self, new_code: &str) {
        self.push_tab();
        self.push(new_code);
        self.push("\n");
    }

    pub fn push_var_def(&mut self, name: &str) {
        self.push_line(format!("u8 {};", name).as_str());
    }
}

impl CppCode {
    pub fn new(code: String, tab_count: usize) -> Self {
        Self { code, tab_count }
    }
}

impl Default for CppCode {
    fn default() -> Self {
        Self::new(String::new(), 0usize)
    }
}
