use crate::format_config::{FormatConfig, AssignmentSpaces};
use crate::parser::Entry;
use std::fmt::Write;

/// Compose string based on entries.
pub fn compose(
    parsed_content: Vec<Entry>,
    config: &FormatConfig,
) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    for entry in parsed_content {
        match entry {
            Entry::KeyValue(k, v) => {
                // Macros require string literal, they can't be dynamically selected right now.
                let writeln_result = match config.assignment_spaces {
                    AssignmentSpaces::LeftOnly => writeln!(&mut result, "{} ={}", k, v),
                    AssignmentSpaces::RightOnly => writeln!(&mut result, "{}= {}", k, v),
                    AssignmentSpaces::Both => writeln!(&mut result, "{} = {}", k, v),
                    AssignmentSpaces::None => writeln!(&mut result, "{}={}", k, v),
                };
                match writeln_result {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            Entry::ValueOnly(v) => match writeln!(&mut result, "{}", v) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            Entry::Section(n) => {
                for _ in 0..config.empty_lines_before_section {
                    match writeln!(&mut result) {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
                }

                match writeln!(&mut result, "[{}]", n) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        }
    }

    if config.empty_line_at_the_end {
        match writeln!(&mut result) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}
