use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(short, long, help = "Show line count")]
    lines: bool,

    #[arg(short, long, help = "Show word count")]
    words: bool,

    #[arg(short = 'c', long, help = "Show byte count")]
    bytes: bool,

    #[arg(
        short = 'm',
        long,
        help = "Show character count",
        conflicts_with("bytes")
    )]
    chars: bool,
}

pub fn get_args() -> Result<Args> {
    let Args {
        files,
        mut lines,
        mut words,
        mut bytes,
        chars,
    } = Args::parse();
    if [lines, words, bytes, chars].iter().all(|v| !v) {
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Args {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(args: Args) -> Result<()> {
    dbg!(args);
    Ok(())
}
