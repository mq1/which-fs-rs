// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

mod magic;

use crate::magic::which_kind;
use std::{fmt, path::Path};

pub enum FsKind {
    Fat32,
    ExFat,
    Unknown,
}

impl fmt::Display for FsKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsKind::Fat32 => write!(f, "FAT32"),
            FsKind::ExFat => write!(f, "exFAT"),
            FsKind::Unknown => write!(f, "Unknown"),
        }
    }
}

#[cfg(unix)]
pub fn detect(path: &Path) -> rustix::io::Result<FsKind> {
    let stat = rustix::fs::statfs(path)?;

    #[cfg(target_os = "linux")]
    let data = stat.f_type;

    #[cfg(target_os = "macos")]
    let data = stat.f_fstypename;

    let kind = which_kind(data);

    Ok(kind)
}

#[cfg(windows)]
pub fn detect(path: &Path) -> windows::core::Result<FsKind> {
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::Foundation::MAX_PATH;
    use windows::Win32::Storage::FileSystem::{GetVolumeInformationW, GetVolumePathNameW};
    use windows::core::PCWSTR;

    // Convert input path to null-terminated UTF-16 wide string
    let path_wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // Get the Volume Path Name (Mount Point)
    let mut volume_path = [0u16; MAX_PATH as usize];
    unsafe {
        GetVolumePathNameW(PCWSTR(path_wide.as_ptr()), &mut volume_path)?;
    }

    let mut fs_name_buffer = [0u16; 16];
    unsafe {
        GetVolumeInformationW(
            PCWSTR(volume_path.as_ptr()),
            None,
            None,
            None,
            None,
            Some(&mut fs_name_buffer),
        )?;
    }

    let kind = which_kind(fs_name_buffer);

    Ok(kind)
}
