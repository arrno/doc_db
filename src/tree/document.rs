use std::error::Error;

pub trait Document {
    type U;
    fn patch(&mut self, update: Self::U) -> Result<&Self, Box<dyn Error>>;
    fn put(&mut self, update: Self) -> Result<&Self, Box<dyn Error>>;
}

impl Document for String {
    type U = Box<str>;
    fn patch(&mut self, update: Box<str>) -> Result<&Self, Box<dyn Error>> {
        self.push_str(&update);
        Ok(self)
    }
    fn put(&mut self, update: String) -> Result<&Self, Box<dyn Error>> {
        *self = update;
        Ok(self)
    }
}
