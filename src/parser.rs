#[derive(Debug)]
pub enum Entry {
    /// Key-value entry - 'key = value', 'x=y'.
    KeyValue(String, String),
    /// Only-value entry - 'example'.
    ValueOnly(String),
    /// Section - '[section_name]', '[section.subsection]'.
    Section(String),
}

/// Parse content into entries.
pub fn parse(in_content: String) -> Option<Vec<Entry>> {
    let mut result = vec![];
    // Parse line by line.
    let in_content_lines: Vec<String> = in_content.split("\n").map(str::to_string).collect();
    for line in in_content_lines {
        // Trim surrounding whitespace.
        let line = line.trim();

        // Ignore empty lines.
        if line.is_empty() {
            continue;
        }

        // Detect section.
        {
            let bracket_start = line.starts_with('[');
            let bracket_end = line.ends_with(']');
            if bracket_start && bracket_end {
                let name_len = line.len() - 2;
                let section_name: String = line.chars().skip(1).take(name_len).collect();
                result.push(Entry::Section(section_name));
                continue;
            } else if bracket_start && !bracket_end {
                // TODO: improve error handling.
                return None;
            }
        }

        // Check if key-value or only-value entry.
        // Only key-value entries contain '='.
        {
            let assing_pos_res = line.find('=');
            if let Some(assign_pos) = assing_pos_res {
                let (left, right) = line.split_at(assign_pos);
                // Only trim whitespace for left side.
                let left = left.trim().to_string();

                // Remove starting '=' for right side.
                let right: String = right.chars().skip(1).collect();
                let right = right.trim().to_string();
                result.push(Entry::KeyValue(left, right))
            } else {
                result.push(Entry::ValueOnly(line.to_string()));
            }
        }
    }

    Some(result)
}
