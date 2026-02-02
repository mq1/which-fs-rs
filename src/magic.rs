// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

// https://github.com/torvalds/linux/blob/master/include/uapi/linux/magic.h

#![allow(clippy::cast_possible_wrap)]

use crate::FsKind;

#[cfg(target_os = "macos")]
pub type Magic = [i8; 16];

#[cfg(target_os = "macos")]
type CharType = i8;

#[cfg(target_os = "linux")]
pub type Magic = rustix::fs::FsWord;

#[cfg(windows)]
pub type Magic = [u16; 8];

#[cfg(windows)]
type CharType = u16;

#[cfg(any(target_os = "macos", target_os = "windows"))]
const fn to_magic<const L: usize>(s: &'static [u8; L]) -> Magic {
    let mut buf = [0; 16];

    let mut i = 0;
    while i < L {
        buf[i] = s[i] as CharType;
        i += 1;
    }

    buf
}

// FAT32
// =====

#[cfg(target_os = "linux")]
const FAT32: Magic = 0x4d44;

#[cfg(target_os = "macos")]
const FAT32: Magic = to_magic(b"msdos");

#[cfg(windows)]
const FAT32: Magic = to_magic(b"FAT32");

// EXFAT
// =====

#[cfg(target_os = "linux")]
const EXFAT: Magic = 0x2011BAB0;

#[cfg(target_os = "macos")]
const EXFAT: Magic = to_magic(b"exfat");

#[cfg(windows)]
const EXFAT: Magic = to_magic(b"exFAT");

// APFS
// ====

#[cfg(target_os = "macos")]
const APFS: Magic = to_magic(b"apfs");

// HFS+
// ====

#[cfg(target_os = "macos")]
const HFS: Magic = to_magic(b"hfs");

// EXT2/3/4
// ========

#[cfg(target_os = "linux")]
const EXT2: Magic = 0xef53;

// BTRFS
// =====

#[cfg(target_os = "linux")]
const BTRFS: Magic = 0x9123683E;

// F2FS
// ====

#[cfg(target_os = "linux")]
const F2FS: Magic = 0xF2F52010;

// BCACHEFS
// ========

#[cfg(target_os = "linux")]
const BCACHEFS: Magic = 0xca451a4e;

// XFS
// ===

#[cfg(target_os = "linux")]
const XFS: Magic = 0x58465342;

// FUSE
// ====

#[cfg(target_os = "linux")]
const FUSE: Magic = 0x65735546;

#[cfg(target_os = "macos")]
const FUSE: Magic = to_magic(b"fusefs");

// NTFS
// ====

#[cfg(target_os = "linux")]
const NTFS: Magic = 0x5346544e;

#[cfg(windows)]
const NTFS: Magic = to_magic(b"NTFS");

#[cfg(target_os = "macos")]
const NTFS: Magic = to_magic(b"ntfs");

#[must_use]
pub fn which_kind(magic: Magic) -> FsKind {
    match magic {
        FAT32 => FsKind::Fat32,
        EXFAT => FsKind::ExFat,
        NTFS => FsKind::Ntfs,

        #[cfg(unix)]
        FUSE => FsKind::Fuse,

        #[cfg(target_os = "macos")]
        APFS => FsKind::Apfs,
        #[cfg(target_os = "macos")]
        HFS => FsKind::Hfs,

        #[cfg(target_os = "linux")]
        EXT2 => FsKind::Ext2,
        #[cfg(target_os = "linux")]
        BTRFS => FsKind::Btrfs,
        #[cfg(target_os = "linux")]
        F2FS => FsKind::F2fs,
        #[cfg(target_os = "linux")]
        BCACHEFS => FsKind::Bcachefs,
        #[cfg(target_os = "linux")]
        XFS => FsKind::Xfs,

        idk => FsKind::Unknown(idk),
    }
}
