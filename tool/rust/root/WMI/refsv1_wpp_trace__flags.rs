// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Refsv1WppTrace_Flags
//////////////////////////////////////////////

/// Refsv1WppTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Refsv1WppTrace_Flags {
    /// FATAL
    #[serde(rename = "FATAL")]
    FATAL = 1,
    /// ERROR
    #[serde(rename = "ERROR")]
    ERROR = 2,
    /// WARNING
    #[serde(rename = "WARNING")]
    WARNING = 3,
    /// READ
    #[serde(rename = "READ")]
    READ = 4,
    /// WRITE
    #[serde(rename = "WRITE")]
    WRITE = 5,
    /// COMPRESSED
    #[serde(rename = "COMPRESSED")]
    COMPRESSED = 6,
    /// EFS
    #[serde(rename = "EFS")]
    EFS = 7,
    /// MFT
    #[serde(rename = "MFT")]
    MFT = 8,
    /// VOLBITMAP
    #[serde(rename = "VOLBITMAP")]
    VOLBITMAP = 9,
    /// CREATE
    #[serde(rename = "CREATE")]
    CREATE = 10,
    /// ALTSTREAMS
    #[serde(rename = "ALTSTREAMS")]
    ALTSTREAMS = 11,
    /// OBJID
    #[serde(rename = "OBJID")]
    OBJID = 12,
    /// INDEXES
    #[serde(rename = "INDEXES")]
    INDEXES = 13,
    /// TXFKTM
    #[serde(rename = "TXFKTM")]
    TXFKTM = 14,
    /// TXFRECOVERY
    #[serde(rename = "TXFRECOVERY")]
    TXFRECOVERY = 15,
    /// TXFRM
    #[serde(rename = "TXFRM")]
    TXFRM = 16,
    /// TXFFCB
    #[serde(rename = "TXFFCB")]
    TXFFCB = 17,
    /// SELFHEAL
    #[serde(rename = "SELFHEAL")]
    SELFHEAL = 18,
    /// HEALBITMAP
    #[serde(rename = "HEALBITMAP")]
    HEALBITMAP = 19,
    /// USNJRNL
    #[serde(rename = "USNJRNL")]
    USNJRNL = 20,
    /// DELNOTIFY
    #[serde(rename = "DELNOTIFY")]
    DELNOTIFY = 21,
    /// MINSTORE
    #[serde(rename = "MINSTORE")]
    MINSTORE = 22,
    /// SCRUB
    #[serde(rename = "SCRUB")]
    SCRUB = 23,
    /// TREEUPDATE
    #[serde(rename = "TREEUPDATE")]
    TREEUPDATE = 24,
}

impl Default for Refsv1WppTrace_Flags {
    fn default() -> Self {
        Self::FATAL
    }
}

