// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::cast_possible_wrap)]

use crate::FsKind;

#[cfg(target_os = "macos")]
type Magic = [i8; 16];

#[cfg(target_os = "macos")]
type Char = i8;

#[cfg(target_os = "linux")]
type Magic = rustix::fs::FsWord;

#[cfg(windows)]
type Magic = [u16; 8];

#[cfg(windows)]
type Char = u16;

#[cfg(any(target_os = "macos", target_os = "windows"))]
const fn to_magic<const L: usize>(s: &'static [u8; L]) -> Magic {
    let mut buf = [0; 16];

    let mut i = 0;
    while i < L {
        buf[i] = s[i] as Char;
        i += 1;
    }

    buf
}

#[cfg(target_os = "linux")]
const FAT32: Magic = 0x4d44;

#[cfg(target_os = "macos")]
const FAT32: Magic = to_magic(b"msdos");

#[cfg(windows)]
const FAT32: Magic = to_magic(b"FAT32");

pub fn which_kind(magic: Magic) -> FsKind {
    match magic {
        FAT32 => FsKind::Fat32,
        _ => FsKind::Unknown,
    }
}
