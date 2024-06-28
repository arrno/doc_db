use std::error::Error;

pub trait Document {
    type U;
    fn update(&mut self, update: Self::U) -> Result<&Self, Box<dyn Error>>;
}

impl Document for String {
    type U = Box<str>;
    fn update(&mut self, update: Box<str>) -> Result<&Self, Box<dyn Error>> {
        self.push_str(&update);
        Ok(self)
    }
}
