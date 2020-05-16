# 总结
- 利用io读取elf文件到buffer
- 创建自己的File格式对象，将buffer的内容填充到File并返回
- 然后我们就可以使用格式好的File对象操作elf文件

[![Build Status](https://travis-ci.org/cole14/rust-elf.svg?branch=master)](https://travis-ci.org/cole14/rust-elf)

# rust-elf
Pure-Rust library for parsing ELF files

[Documentation](http://cole14.github.io/rust-elf/)

## Example:
```rust
extern crate elf;

use std::path::PathBuf;

let path = PathBuf::from("/some/file/path");
let file = match elf::File::open_path(&path) {
    Ok(f) => f,
    Err(e) => panic!("Error: {:?}", e),
};

let text_scn = match file.get_section(".text") {
    Some(s) => s,
    None => panic!("Failed to look up .text section"),
};

println!("{:?}", text_scn.data);

```
