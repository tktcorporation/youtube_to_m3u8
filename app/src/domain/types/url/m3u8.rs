pub struct M3U8 {
    value: String,
}

pub fn new(st: &str) -> M3U8 {
    M3U8 {
        value: st.to_string(),
    }
}

impl M3U8 {
    pub fn to_string(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {}
