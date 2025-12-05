// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileServer_FileSharingProtocols
//////////////////////////////////////////////

/// FileServer_FileSharingProtocols enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileServer_FileSharingProtocols {
    /// NFS
    #[serde(rename = "NFS")]
    NFS = 2,
    /// SMB
    #[serde(rename = "SMB")]
    SMB = 3,
}

impl Default for FileServer_FileSharingProtocols {
    fn default() -> Self {
        Self::NFS
    }
}

