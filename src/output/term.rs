use crate::msg::Msg;

pub trait Term {
    fn term_min(&self);
    fn term_full(&self);
}

impl Term for Msg<'_> {
    fn term_min(&self) {
        println!(" [{}]❯ {}", self.ico(), self.message());
    }

    fn term_full(&self) {
        let mut txt = format!(" [{}]❯ {}", self.ico(), self.message());
        if let Some(duration) = self.duration() {
            txt.push_str(&format!(" - {:?}", duration));
        }

        println!("{txt}")
    }
}
