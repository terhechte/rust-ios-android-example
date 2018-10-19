pub struct Worker {
    number: i32,
}

impl Worker {
    pub fn new() -> Worker {
        Worker { number: 2 }
    }

    pub fn action(&self, string: &str) -> String {
        return "WOOHOO".to_string();
        //format!("number: {}, {}", self.number, string.chars().rev().collect::<String>())
    }
}
