use std::error::Error;

pub fn find_matches(
    buffer: impl std::io::BufRead,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn Error>> {
    for line in buffer.lines() {
        let l = line.unwrap();
        if l.contains(pattern) {
            writeln!(writer, "{}", l)?;
        }
    };
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let buffer = "lorem ipsum\ndolor sit amet".as_bytes();
    find_matches(buffer, "lorem", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}