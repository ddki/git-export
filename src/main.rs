use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(name = "git-export")]
#[command(about = "Git导出工具", long_about = "Git提交记录导出文件。")]
struct Args {
    #[arg(
        short = 'f',
        long = "filter",
        help = "必填项，过滤，支持正则语法，支持tag,username,email,commit message..."
    )]
    filter: String,

    #[arg(
        short = 'o',
        long = "outdir",
        default_value = "git-export",
        help = "导出目录，默认当前文件夹/git-export"
    )]
    out_dir: Option<String>,

    #[arg(long = "commit", help = "commit哈希，filter限定在这些commit中")]
    commits: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let repo = match git2::Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("repo path = {:?}", repo.path());

    println!("{:?}", args);
}
