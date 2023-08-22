/// Specify how a variable should be updated (e.g. appended or replaced).
pub trait UpdateField {
    fn update(&mut self, value: Self) -> &mut Self;
}

impl<T> UpdateField for Vec<T> {
    fn update(&mut self, mut value: Self) -> &mut Self {
        self.append(&mut value);
        self
    }
}

impl UpdateField for i64 {
    fn update(&mut self, value: Self) -> &mut Self {
        *self = value;
        self
    }
}
