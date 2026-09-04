use crate::document::object::IdName;

struct IdCounter {
    next_id: u16,
}

impl Default for IdCounter {
    fn default() -> Self {
        Self { next_id: 1 }
    }
}

impl IdCounter {
    fn next(&mut self) -> IdName {
        let res = IdName {
            id: self.next_id,
            name: format!("#{}", self.next_id).to_string(),
        };
        self.next_id += 1;
        res
    }
}
