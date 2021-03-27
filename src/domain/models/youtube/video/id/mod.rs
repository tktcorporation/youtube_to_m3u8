use std::fmt;

pub struct Id {
    value: String,
}
pub fn new(st: &str) -> Id {
    Id {
        value: st.to_string(),
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
