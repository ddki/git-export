## 更新日志

### [v0.0.1](https://github.com/ddki/git-export/releases/tag/v0.0.1)

Git export command for files of commits. Git 提交记录文件导出命令行工具。 

Command:
```sh
❯ git-export --help
Git export command for files of commits. Git 提交记录文件导出命令行工具。

Usage: git-export.exe [OPTIONS] --filter <FILTER>

Options:
  -f, --filter <FILTER>
          必填项，过滤，支持username,email,commit message...

  -o, --outdir <OUT_DIR>
          导出目录

          [default: git-export]

      --in-commit <IN_COMMITS>
          commit哈希，filter限定在这些commit中，多个可以使用逗号分隔

      --zip <ZIP>
          是否打包成zip文件

          [default: source.zip]

  -h, --help
          Print help (see a summary with '-h')
```