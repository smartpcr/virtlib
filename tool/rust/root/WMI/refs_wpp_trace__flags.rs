// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RefsWppTrace_Flags
//////////////////////////////////////////////

/// RefsWppTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RefsWppTrace_Flags {
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
    /// NONCACHEDIO
    #[serde(rename = "NONCACHEDIO")]
    NONCACHEDIO = 6,
    /// PAGETABLEWRITE
    #[serde(rename = "PAGETABLEWRITE")]
    PAGETABLEWRITE = 7,
    /// FILEID
    #[serde(rename = "FILEID")]
    FILEID = 8,
    /// STATUS
    #[serde(rename = "STATUS")]
    STATUS = 9,
    /// MINSTORE
    #[serde(rename = "MINSTORE")]
    MINSTORE = 10,
    /// SCRUB
    #[serde(rename = "SCRUB")]
    SCRUB = 11,
    /// TREEUPDATE
    #[serde(rename = "TREEUPDATE")]
    TREEUPDATE = 12,
    /// LOG
    #[serde(rename = "LOG")]
    LOG = 13,
    /// READCACHE
    #[serde(rename = "READCACHE")]
    READCACHE = 14,
    /// ACTIVITYID
    #[serde(rename = "ACTIVITYID")]
    ACTIVITYID = 15,
    /// COMPACTION
    #[serde(rename = "COMPACTION")]
    COMPACTION = 16,
    /// ENCRYPTION
    #[serde(rename = "ENCRYPTION")]
    ENCRYPTION = 17,
    /// SHARING_VIOLATION
    #[serde(rename = "SHARING_VIOLATION")]
    SHARINGVIOLATION = 18,
}

impl Default for RefsWppTrace_Flags {
    fn default() -> Self {
        Self::FATAL
    }
}

