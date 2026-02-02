# which-fs-rs

Give me a path, I'll tell you the filesystem it's on

Works on Windows, Linux, and macOS

WIP: only supports FAT32

### Usage

```rust
let path = Path::new("/Volumes/My\ Volume/My\ Folder");
let fs_kind = which_fs::detect(&path).unwrap();
println!("{} is on {}", path.display(), fs_kind);
```

