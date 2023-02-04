fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Server is running on [{}]...", self.addr);
    }
}

pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expect = 4;
        assert_eq!(expect, add_numbers(2, 2));
    }
}
