// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DiskImageToVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DiskImageToVolume {

/// 
    #[serde(rename = "DiskImage")]
    pub disk_image: Option<MSFT_DiskImage>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<MSFT_Volume>,
}

impl MSFT_DiskImageToVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk_image: None,
            volume: None,
        }
    }


    /// Sets the value of DiskImage
    pub fn set_disk_image(&mut self, value: MSFT_DiskImage) {
        self.disk_image = Some(value);
    }

    /// Gets the value of DiskImage
    pub fn get_disk_image(&self) -> Option<&MSFT_DiskImage> {
        self.disk_image.as_ref()
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

