pub struct Header {
    name: String,
    value: String,
}

impl Header {
    fn new(maybe_header: String) -> Option<Header> {
        if maybe_header.is_empty() {
            return None;
        }

        let mid = maybe_header.find(": ").unwrap();

        let (name, value) = maybe_header.split_at(mid);

        return Some(Header {
            name: name.to_string(),
            value: value.to_string(),
        });
    }

    fn to_str(&self) -> String {
        return format!("{}: {}", &self.name, &self.value);
    }
}
