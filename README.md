# rtree
**tree** command implement in rust
## Installation
```shell
cargo install rtree --git https://github.com/misssonder/rtree
```
## Usage
```shell
Usage: rtree [OPTIONS] [DIR]

Arguments:
  [DIR]  Directory to list; defaults to current working directory

Options:
  -c, --charset <CHARSET>  Character set to use in output [default: utf8] [possible values: utf8, ascii]
  -s, --sort <SORT>        Sorting options [default: filename] [possible values: filename, size, created-time, modified-time]
  -a, --all                All files are listed
  -d, --dir                List directories only
  -f, --full               Print the full path prefix for each file
  -h, --help               Print help
```

```shell
$ rtree
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── args.rs
    ├── dir.rs
    ├── format.rs
    └── main.rs
```