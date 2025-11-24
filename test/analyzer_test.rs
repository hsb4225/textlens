use textlens::analyzer::*;

#[test]
fn test_count_lines() {
    let content = "hello\nworld\nrust\n";
    assert_eq!(count_lines(content), 3);
}

#[test]
fn test_tokenize() {
    let content = "Hello, Rust world!";
    let tokens = tokenize(content);
    assert_eq!(tokens, vec!["hello", "rust", "world"]);
}

#[test]
fn test_word_frequency() {
    let words = vec!["rust".into(), "rust".into(), "lang".into()];
    let freq = word_frequency(&words);
    assert_eq!(*freq.get("rust").unwrap(), 2);
    assert_eq!(*freq.get("lang").unwrap(), 1);
}
