use clap::{Parser, ValueEnum};

/// CLI 옵션 구조체
#[derive(Parser, Debug)]
#[clap(author = "Hyeongseongbin", version, about = "Rust Text Analyzer CLI")]
pub struct Config {
    /// 분석할 파일 경로
    pub filepath: String,

    /// 상위 N개의 단어 빈도 출력
    #[clap(long = "top")]
    pub top: Option<usize>,

    /// 문자 수 출력 여부
    #[clap(long = "chars")]
    pub chars: bool,

    /// 결과를 JSON 형태로 출력할지 여부
    #[clap(long = "json")]
    pub json: bool,
}

pub fn parse_args() -> Config {
    Config::parse()
}
