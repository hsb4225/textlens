use anyhow::Result;
use std::collections::HashMap;
use std::fs;

/// 분석 결과 구조체
pub struct AnalysisResult {
    pub line_count: usize,
    pub word_count: usize,
    pub char_count: usize,
    pub frequencies: HashMap<String, usize>,
}

/// 실제 분석 함수
pub fn analyze(path: &str, top_n: usize) -> Result<AnalysisResult> {
    let content = fs::read_to_string(path)?;

    let line_count = content.lines().count();
    let char_count = content.chars().count();

    let mut frequencies = HashMap::new();
    let mut word_count = 0;

    for word in content.split(|c: char| !c.is_alphanumeric()) {
        let w = word.to_lowercase();
        if !w.is_empty() {
            *frequencies.entry(w).or_insert(0) += 1;
            word_count += 1;
        }
    }

    // 상위 n개 단어만 정렬
    let mut freq_vec: Vec<(String, usize)> = frequencies.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let freq_top_n: HashMap<String, usize> = freq_vec.into_iter().take(top_n).collect();

    Ok(AnalysisResult {
        line_count,
        word_count,
        char_count,
        frequencies: freq_top_n,
    })
}

/// 출력용 헬퍼
pub fn print_result(result: AnalysisResult, top_n: usize, show_chars: bool, json: bool) -> Result<()> {
    if json {
        let data = serde_json::json!({
            "lines": result.line_count,
            "words": result.word_count,
            "chars": result.char_count,
            "frequency": result.frequencies,
        });
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }

    println!("라인 수: {}", result.line_count);
    println!("단어 수: {}", result.word_count);

    if show_chars {
        println!("문자 수: {}", result.char_count);
    }

    println!("\n[가장 많이 등장한 단어 TOP {}]", top_n);
    for (i, (word, count)) in result.frequencies.into_iter().enumerate() {
        println!("{}. {} ({}회)", i + 1, word, count);
    }

    Ok(())
}

