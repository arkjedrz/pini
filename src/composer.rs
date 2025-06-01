use crate::parser::Entry;
use std::fmt::Write;

/// Compose string based on entries.
pub fn compose(parsed_content: Vec<Entry>) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    for entry in parsed_content {
        match entry {
            Entry::KeyValue(k, v) => match writeln!(&mut result, "{}={}", k, v) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            Entry::ValueOnly(v) => match writeln!(&mut result, "{}", v) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            Entry::Section(n) => {
                match writeln!(&mut result) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
                match writeln!(&mut result, "[{}]", n) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        }
    }

    Ok(result)
}
