pub struct Worker {
    number: i32,
}

impl Worker {
    pub fn new() -> Worker {
        Worker { number: 2 }
    }

    pub fn action(&self, string: &str) -> String {
        string.to_owned().replace("World", "Łódź")
    }
}
