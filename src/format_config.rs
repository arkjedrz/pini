use crate::parser::{Entry, parse};
use std::fs;
use std::path::PathBuf;

pub enum AssignmentSpaces {
    /// 'x =y'.
    LeftOnly,
    /// 'x= y'.
    RightOnly,
    /// 'x = y'.
    Both,
    /// 'x=y'.
    None,
}

pub struct FormatConfig {
    pub assignment_spaces: AssignmentSpaces,
    pub empty_lines_before_section: u8,
    pub empty_line_at_the_end: bool,
}

impl FormatConfig {
    pub fn new() -> Self {
        FormatConfig {
            assignment_spaces: AssignmentSpaces::None,
            empty_lines_before_section: 1,
            empty_line_at_the_end: true,
        }
    }

    pub fn load(path: PathBuf) -> Self {
        // TODO: improve error handling.
        let content = fs::read_to_string(path).unwrap();
        let parsed = parse(content).unwrap();

        let mut assignment_spaces = AssignmentSpaces::Both;
        let mut empty_lines_before_section = 1;
        let mut empty_line_at_the_end = true;

        for entry in parsed {
            match entry {
                Entry::KeyValue(k, v) => match k.as_str() {
                    "AssignmentSpaces" => {
                        assignment_spaces = match v.as_str() {
                            "LeftOnly" => AssignmentSpaces::LeftOnly,
                            "RightOnly" => AssignmentSpaces::RightOnly,
                            "Both" => AssignmentSpaces::Both,
                            "None" => AssignmentSpaces::None,
                            _ => panic!("Unknown value"),
                        }
                    }
                    "EmptyLinesBeforeSection" => {
                        empty_lines_before_section = v.parse::<u8>().unwrap();
                    }
                    "EmptyLineAtTheEnd" => {
                        empty_line_at_the_end = v.parse::<bool>().unwrap();
                    }
                    _ => panic!("Unknown key"),
                },
                Entry::ValueOnly(_) => panic!("Values are disallowed"),
                Entry::Section(_) => panic!("Sections are disallowed"),
            };
        }

        FormatConfig {
            assignment_spaces,
            empty_lines_before_section,
            empty_line_at_the_end,
        }
    }
}
