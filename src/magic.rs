// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::cast_possible_wrap)]

use crate::FsKind;

#[cfg(target_os = "macos")]
type Magic = [i8; 16];

#[cfg(target_os = "linux")]
type Magic = rustix::fs::FsWord;

#[cfg(windows)]
type Magic = [u16; 8];

#[cfg(target_os = "linux")]
const FAT32: Magic = 0x4d44;

#[cfg(target_os = "macos")]
const FAT32: Magic = [
    b'm' as i8, 's' as i8, b'd' as i8, b'o' as i8, b's' as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[cfg(windows)]
const FAT32: Magic = [
    b'F' as u16,
    b'A' as u16,
    b'T' as u16,
    b'3' as u16,
    b'2' as u16,
    0,
    0,
    0,
];

pub fn which_kind(magic: Magic) -> FsKind {
    match magic {
        FAT32 => FsKind::Fat32,
        _ => FsKind::Unknown,
    }
}
