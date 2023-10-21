
#[derive(Default, Debug)]
pub struct Application {}

impl Application {
    pub fn run(self) {
        println!("Hello, keep account is here to help!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

