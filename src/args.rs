use clap::{Parser};

/// TextLens - 텍스트 분석 CLI 유틸리티
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// 분석할 파일 경로
    pub file_path: String,

    /// JSON 형태로 출력
    #[arg(long)]
    pub json: bool,

    /// 상위 n개의 단어 빈도 출력
    #[arg(long, default_value = "10")]
    pub top: usize,
}
