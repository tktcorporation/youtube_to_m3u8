pub mod id {
    pub struct Id {
        value: String,
    }
    pub fn new(st: &str) -> Id {
        Id {
            value: st.to_string(),
        }
    }
    impl Id {
        pub fn to_string(&self) -> String {
            self.value.clone()
        }
    }
}
