// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTDiskVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTDiskVolume {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "FormattedSize")]
    pub formatted_size: Option<u64>,

/// 
    #[serde(rename = "PageFile")]
    pub page_file: Option<bool>,

/// 
    #[serde(rename = "SystemDisk")]
    pub system_disk: Option<bool>,

/// 
    #[serde(rename = "VolumePath")]
    pub volume_path: Option<String>,
}

impl MSFT_MTDiskVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            formatted_size: None,
            page_file: None,
            system_disk: None,
            volume_path: None,
        }
    }


    /// Sets the value of FormattedSize
    pub fn set_formatted_size(&mut self, value: u64) {
        self.formatted_size = Some(value);
    }

    /// Gets the value of FormattedSize
    pub fn get_formatted_size(&self) -> Option<&u64> {
        self.formatted_size.as_ref()
    }

    /// Sets the value of PageFile
    pub fn set_page_file(&mut self, value: bool) {
        self.page_file = Some(value);
    }

    /// Gets the value of PageFile
    pub fn get_page_file(&self) -> Option<&bool> {
        self.page_file.as_ref()
    }

    /// Sets the value of SystemDisk
    pub fn set_system_disk(&mut self, value: bool) {
        self.system_disk = Some(value);
    }

    /// Gets the value of SystemDisk
    pub fn get_system_disk(&self) -> Option<&bool> {
        self.system_disk.as_ref()
    }

    /// Sets the value of VolumePath
    pub fn set_volume_path(&mut self, value: String) {
        self.volume_path = Some(value);
    }

    /// Gets the value of VolumePath
    pub fn get_volume_path(&self) -> Option<&String> {
        self.volume_path.as_ref()
    }
}

