pub fn get_escape_code(code: char) -> Option<char> {
    match code {
        'n' => Some('\n'),
        '\\' => Some('\\'),
        _ => None,
    }
}
