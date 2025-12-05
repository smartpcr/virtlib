// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToFileShare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToFileShare {

/// 
    #[serde(rename = "FileShare")]
    pub file_share: Option<MSFT_FileShare>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToFileShare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            file_share: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of FileShare
    pub fn set_file_share(&mut self, value: MSFT_FileShare) {
        self.file_share = Some(value);
    }

    /// Gets the value of FileShare
    pub fn get_file_share(&self) -> Option<&MSFT_FileShare> {
        self.file_share.as_ref()
    }

    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
    }
}

