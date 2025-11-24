use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

use crate::cli::Config;

pub fn run(config: Config) -> Result<()> {
    let content = fs::read_to_string(&config.filepath)
        .with_context(|| format!("파일을 읽을 수 없습니다: {}", config.filepath))?;

    let lines = count_lines(&content);
    let words = tokenize(&content);
    let word_count = words.len();
    let char_count = content.chars().count();
    let freq = word_frequency(&words);

    if config.json {
        print_json(lines, word_count, char_count, &freq)?;
        return Ok(());
    }

    println!("라인 수: {}", lines);
    println!("단어 수: {}", word_count);

    if config.chars {
        println!("문자 수: {}", char_count);
    }

    if let Some(top) = config.top {
        print_top_words(&freq, top);
    }

    Ok(())
}

pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}

pub fn tokenize(content: &str) -> Vec<String> {
    content
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

pub fn word_frequency(words: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for w in words {
        *map.entry(w.clone()).or_insert(0) += 1;
    }
    map
}

fn print_top_words(freq: &HashMap<String, usize>, n: usize) {
    let mut pairs: Vec<_> = freq.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n[가장 많이 등장한 단어 TOP {}]", n);
    for (i, (word, count)) in pairs.into_iter().take(n).enumerate() {
        println!("{}. {} ({}회)", i + 1, word, count);
    }
}

fn print_json(lines: usize, words: usize, chars: usize, freq: &HashMap<String, usize>) -> Result<()> {
    let data = serde_json::json!({
        "lines": lines,
        "words": words,
        "chars": chars,
        "frequency": freq,
    });

    println!("{}", serde_json::to_string_pretty(&data)?);
    Ok(())
}
