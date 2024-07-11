# make-epub

make-epub 是一个基于txt文件制作epub电子书的命令行工具. 

#### Usage

```
Usage: make-epub [OPTIONS] --input <INPUT>

Options:
  -a, --author <AUTHOR>                [作者]
  -i, --input <INPUT>                  [输入文件(txt文件)]
  -o, --out <OUT>                      [输出目录名]
  -n, --name <NAME>                    [书名]
  -c, --cover <COVER>                  [封面图路径]
  -s, --slim <SLIM>                    [侧封面路径]
      --extra-volume <EXTRA_VOLUME>    [额外的卷匹配模式]
      --extra-chapter <EXTRA_CHAPTER>  [额外的章节匹配模式]
  -I                                   [互动模式]
  -h, --help                           Print help
```

必须输入的参数为 `-i`, 其它参数可以为空白, 书名可以从输入文件的文件名中提取.

​	默认的卷和章节匹配的模式为

```
^\s*第[零一二两三四五六七八九十百千万0-9]{1,4}卷\s*(?<vol_name>\S*)\s*$
^\s*第[零一二两三四五六七八九十百千万0-9]{1,7}章\s*(?<chap_name>\S*)\s*$
```

你可以通过 `--extra-volume-match` 和 `--extra-chapter-match` 两个参数来指定额外的模式匹配. 在指定额外的模式匹配, 请务必阅读 [Regex crate](https://docs.rs/regex/latest/regex/#syntax) 的文档, 并将捕捉的卷名和章节名分别命名为 `vol_name` 和 `chap_name`. 在没有禁止默认匹配模式的情况下, 默认匹配模式具有最低的优先级.

##### 互动模式

​	通过 `-I` 参数可以直接进入互动模式, 互动模式下程序会询问各个参数. 
