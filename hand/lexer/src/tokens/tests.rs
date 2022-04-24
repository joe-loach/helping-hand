use super::*;
use SyntaxKind::*;

fn kind(text: &str, kind: SyntaxKind) {
    let next = Cursor::new(text).kind();
    assert_eq!(next, kind);
}

#[test]
fn comments() {
    kind("; Comment", COMMENT);
    kind("/* Comment */", COMMENT);
    kind("// Comment", COMMENT);
}

#[test]
fn numbers() {
    kind("0", NUMBER);
    kind("42", NUMBER);
    kind("0xAFaf09", NUMBER);
}

#[test]
fn strings() {
    kind(r#""""#, STRING);
    kind(r#""Hello, world""#, STRING);
    kind(r#""\"Hello, world\"""#, STRING);
}

#[test]
fn chars() {
    kind("'a'", CHAR);
    kind("'Z'", CHAR);
}