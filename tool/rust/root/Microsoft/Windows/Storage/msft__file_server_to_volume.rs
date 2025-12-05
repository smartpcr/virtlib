// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileServerToVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileServerToVolume {

/// 
    #[serde(rename = "FileServer")]
    pub file_server: Option<MSFT_FileServer>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<MSFT_Volume>,
}

impl MSFT_FileServerToVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            file_server: None,
            volume: None,
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

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: MSFT_Volume) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&MSFT_Volume> {
        self.volume.as_ref()
    }
}

