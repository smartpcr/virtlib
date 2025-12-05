// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToFileServer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToFileServer {

/// 
    #[serde(rename = "FileServer")]
    pub file_server: Option<MSFT_FileServer>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToFileServer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            file_server: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of FileServer
    pub fn set_file_server(&mut self, value: MSFT_FileServer) {
        self.file_server = Some(value);
    }

    /// Gets the value of FileServer
    pub fn get_file_server(&self) -> Option<&MSFT_FileServer> {
        self.file_server.as_ref()
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

