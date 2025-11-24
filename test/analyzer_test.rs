use textlens::analyzer::*;

#[test]
fn test_basic_analysis() {
    let content = "hello world\nhello rust";
    std::fs::write("test_sample.txt", content).unwrap();

    let result = analyze("test_sample.txt", 10).unwrap();

    // 라인 수 확인
    assert_eq!(result.line_count, 2);

    // 단어 수 확인
    assert_eq!(result.word_count, 4);

    // 특정 단어 빈도 확인
    assert_eq!(*result.frequencies.get("hello").unwrap(), 2);
    assert_eq!(*result.frequencies.get("world").unwrap(), 1);
    assert_eq!(*result.frequencies.get("rust").unwrap(), 1);

    // 파일 삭제
    std::fs::remove_file("test_sample.txt").unwrap();
}

#[test]
fn test_empty_file() {
    std::fs::write("empty.txt", "").unwrap();
    let result = analyze("empty.txt", 10).unwrap();

    assert_eq!(result.line_count, 0);
    assert_eq!(result.word_count, 0);
    assert!(result.frequencies.is_empty());

    std::fs::remove_file("empty.txt").unwrap();
}

