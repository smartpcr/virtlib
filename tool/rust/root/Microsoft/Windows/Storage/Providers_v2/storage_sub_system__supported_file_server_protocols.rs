// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_SupportedFileServerProtocols
//////////////////////////////////////////////

/// StorageSubSystem_SupportedFileServerProtocols enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_SupportedFileServerProtocols {
    /// NFS
    #[serde(rename = "NFS")]
    NFS = 2,
    /// CIFS_SMB_
    #[serde(rename = "CIFS_SMB_")]
    CIFSSMB = 3,
}

impl Default for StorageSubSystem_SupportedFileServerProtocols {
    fn default() -> Self {
        Self::NFS
    }
}

