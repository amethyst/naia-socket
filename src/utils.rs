pub trait StringUtils {
    fn trim_front(&self, count: usize) -> Self;
    fn peek_front(&self) -> u8;
    fn push_front(&self, symbol: u8) -> Self;
}

impl StringUtils for String {
    fn trim_front(&self, count: usize) -> Self {
        self.chars().skip(count).take(self.len()).collect()
    }
    fn peek_front(&self) -> u8 {
        self.chars().next().unwrap() as u8
    }
    fn push_front(&self, symbol: u8) -> Self {
        let prefix_string = String::from(std::str::from_utf8(&[symbol]).unwrap());
        prefix_string + self
    }
}