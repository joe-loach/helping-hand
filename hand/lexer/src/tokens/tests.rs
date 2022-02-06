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
fn literals() {
    kind("0", LITERAL);
    kind("42", LITERAL);
    kind("0xAFaf09", LITERAL);
}
