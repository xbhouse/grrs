/// functions and unit tests

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
  for line in content.lines() {
    if line.contains(pattern) {
        let result = writeln!(writer, "{}", line);
        match result {
          Ok(_result) => { println!("---") },
          Err(error) => { panic!("can't deal with {}, exiting here", error); }
        }
      }
  }
}  

#[test]
fn find_a_match() {
  let mut result = Vec::new();
  find_matches("hello world", "rld", &mut result);
  assert_eq!(result, b"hello world\n");
}