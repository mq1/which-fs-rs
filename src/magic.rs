// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::cast_possible_wrap)]

#[cfg(target_os = "linux")]
pub const FAT32: rustix::fs::FsWord = 0x4d44;

#[cfg(target_os = "macos")]
pub const FAT32: [i8; 16] = [
    b'm' as i8, 's' as i8, b'd' as i8, b'o' as i8, b's' as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[cfg(windows)]
pub const FAT32: [u16; 8] = [
    b'F' as u16,
    b'A' as u16,
    b'T' as u16,
    b'3' as u16,
    b'2' as u16,
    0,
    0,
    0,
];
