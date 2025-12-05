// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VolumeToFileShare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VolumeToFileShare {

/// 
    #[serde(rename = "FileShare")]
    pub file_share: Option<MSFT_FileShare>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<MSFT_Volume>,
}

impl MSFT_VolumeToFileShare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            file_share: None,
            volume: None,
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

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: MSFT_Volume) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&MSFT_Volume> {
        self.volume.as_ref()
    }
}

