mod zip_utils;

use std::{
    env,
    fs::{self, File},
    io::Write,
    path,
};

use clap::{arg, command, Parser};
use console::style;
use git2::{DiffFormat, Oid};

fn match_str(s1: &str, s2: &str) -> bool {
    s1.eq(s2) || s1.contains(s2) || s1.to_lowercase().eq(&s2.to_lowercase())
}

#[derive(Parser, Debug)]
#[command(name = "git-export")]
#[command(
    about = "Git提交记录文件导出工具",
    long_about = "Git export command for files of commits. Git 提交记录文件导出命令行工具。"
)]
struct Args {
    #[arg(
        short = 'f',
        long = "filter",
        help = "必填项，过滤，支持username,email,commit message..."
    )]
    filter: String,

    #[arg(
        short = 'o',
        long = "outdir",
        default_value = "git-export",
        help = "导出目录"
    )]
    out_dir: Option<String>,

    #[arg(
        long = "in-commit",
        help = "commit哈希，filter限定在这些commit中，多个可以使用逗号分隔"
    )]
    in_commits: Vec<String>,

    #[arg(long = "zip", default_value = "source.zip", help = "zip文件名称")]
    zip: String,

    #[arg(short = 'V', long, default_value = "false", help = "是否打印日志")]
    print_log: bool,
}

fn main() {
    let args = Args::parse();

    let filter = args.filter;
    let out_dir = match args.out_dir {
        Some(s) => s,
        None => return,
    };
    let in_commits = args.in_commits;
    let zip = args.zip;
    let print_log = args.print_log;

    if !zip.ends_with(".zip") {
        println!(
            "error: {}",
            style("zip 文件名称有误，请使用.zip结尾").red().bold()
        );
    }

    let current_dir = env::current_dir().unwrap();
    let out_path = current_dir.join(&out_dir);

    if print_log {
        println!(
            "filter = {}, out_dir = {:?}, in_commits = {:?}",
            filter, out_dir, in_commits
        );
    }

    let repo = match git2::Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    if print_log {
        println!("repo path = {:?}", repo.path());
    }

    let mut revwalk = repo.revwalk().unwrap();

    // 设置遍历开始的提交
    revwalk.push_head().unwrap();

    // 使用 `filter_map` 方法来筛选并映射提交
    let result: Vec<Oid> = revwalk
        .filter_map(|oid| {
            let commit = repo.find_commit(oid.unwrap()).unwrap();
            let message = commit.message().unwrap();
            let author_binding = commit.author();
            let author_name = author_binding.name().unwrap();
            let author_email = author_binding.email().unwrap();

            let match_filter = match_str(message, &filter)
                || match_str(author_name, &filter)
                || match_str(author_email, &filter);

            if in_commits.is_empty() {
                if match_filter {
                    Some(commit.id())
                } else {
                    None
                }
            } else {
                if in_commits.contains(&commit.id().to_string()) {
                    Some(commit.id())
                } else {
                    None
                }
            }
        })
        .collect();

    // 打印符合条件的提交
    for oid in result {
        let commit = repo.find_commit(oid).unwrap();
        let commit_tree = commit.tree().unwrap();

        if print_log {
            println!("author: {}", commit.author().name().unwrap());
            println!("email: {}", commit.author().email().unwrap());
            println!("message: {}", commit.message().unwrap().trim());
            println!("tree: {:?}", commit_tree);
            println!("id: {}", commit.id());
        }

        for parent in commit.parents() {
            let parent_tree = parent.tree().unwrap();

            let diff = repo
                .diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)
                .unwrap();

            diff.print(DiffFormat::Patch, |_delta, _hunk, _line| {
                let new_file = _delta.new_file();
                let new_file_path = match new_file.path() {
                    Some(path) => path
                        .display()
                        .to_string()
                        .replace("/", path::MAIN_SEPARATOR_STR)
                        .replace("\\", path::MAIN_SEPARATOR_STR),
                    None => {
                        if print_log {
                            println!("error: {}", style("Not found file path").red().bold());
                        }
                        String::default()
                    },
                };
                
                let new_file_blob = repo.find_blob(new_file.id());

                if let Ok(blob_content) = new_file_blob {
                    let new_file_content = std::str::from_utf8(blob_content.content()).unwrap();

                    let binding = out_path.join(new_file_path);
                    let file_path = binding.as_path();

                    let file_dir = file_path.parent().ok_or("No Parent Directory").unwrap();
                    if !file_dir.exists() {
                        fs::create_dir_all(file_dir).expect("create dir failed");
                    }

                    let mut file = File::create(file_path).expect("create file failed");
                    file.write_all(new_file_content.as_bytes())
                        .expect("diff file write failed");
                } else {
                    if print_log {
                        println!("error: {}", style("Not found file content").red().bold());
                    }
                }

                true
            })
            .unwrap();
        }
    }

    if !zip.is_empty() {
        zip_utils::ZipUtils::zip_dir_to_file(out_path.to_str().unwrap(), &zip);
    }
}
