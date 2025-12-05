// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileShare_FileSharingProtocol
//////////////////////////////////////////////

/// FileShare_FileSharingProtocol enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileShare_FileSharingProtocol {
    /// NFS
    #[serde(rename = "NFS")]
    NFS = 2,
    /// CIFS_SMB_
    #[serde(rename = "CIFS_SMB_")]
    CIFSSMB = 3,
}

impl Default for FileShare_FileSharingProtocol {
    fn default() -> Self {
        Self::NFS
    }
}

