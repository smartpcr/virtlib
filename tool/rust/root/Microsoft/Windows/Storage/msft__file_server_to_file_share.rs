// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileServerToFileShare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileServerToFileShare {

/// 
    #[serde(rename = "FileServer")]
    pub file_server: Option<MSFT_FileServer>,

/// 
    #[serde(rename = "FileShare")]
    pub file_share: Option<MSFT_FileShare>,
}

impl MSFT_FileServerToFileShare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            file_server: None,
            file_share: None,
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

    /// Sets the value of FileShare
    pub fn set_file_share(&mut self, value: MSFT_FileShare) {
        self.file_share = Some(value);
    }

    /// Gets the value of FileShare
    pub fn get_file_share(&self) -> Option<&MSFT_FileShare> {
        self.file_share.as_ref()
    }
}

