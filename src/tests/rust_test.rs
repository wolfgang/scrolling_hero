#[test]
fn split_string_across_newlines() {
    let str = String::from("line1\nline2\n");
    assert_eq!(str.split("\n").collect::<Vec<&str>>(), vec!["line1", "line2", ""]);
}

#[test]
fn split_string_across_newlines_and_trim_last_line() {
    let str = String::from("line1\nline2\n");
    let lines: Vec<&str> = str.split("\n").collect();
    let lines_trimmed = lines[0..lines.len() - 1].to_vec();
    assert_eq!(lines_trimmed, vec!["line1", "line2"]);
}