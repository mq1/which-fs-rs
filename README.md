# which-fs-rs

Give me a path, I'll tell you the filesystem it's on

Works on Windows, Linux, and macOS

> [!WARNING]  
> This library only supports some popular filesystems (see [here](https://docs.rs/which-fs/latest/which_fs/enum.FsKind.html))

### Usage

```rust
let path = Path::new("/Volumes/My Volume/My Folder");
let fs_kind = which_fs::FsKind::from_path(path).unwrap();
println!("{} is on {}", path.display(), fs_kind);
```

