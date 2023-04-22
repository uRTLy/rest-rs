struct Header {
    name: &str,
    value: &str,
}

fn to_header(name: &str, value: &str) -> Header {
    return Header { name, value };
}

fn parse_header(header: Header) -> &str {
    return format!("{header.name}: {header.value");
}
