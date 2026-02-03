# which-fs-rs

[![Latest version](https://img.shields.io/crates/v/which-fs.svg)](https://crates.io/crates/which-fs)
[![Documentation](https://docs.rs/which-rs/badge.svg)](https://docs.rs/which-fs)
![License](https://img.shields.io/crates/l/which-fs.svg)

Give me a path, I'll tell you the filesystem it's on

Works on Windows, Linux, and macOS

> [!WARNING]  
> This library only supports some popular filesystems (see [here](https://docs.rs/which-fs/latest/which_fs/enum.FsKind.html))

### Usage

```rust
let path = Path::new("/Volumes/My Volume/My Folder");
let fs_kind = which_fs::FsKind::try_from_path(path).unwrap();
println!("{} is on a {} filesystem", path.display(), fs_kind);
```

