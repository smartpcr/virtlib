// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualHardDiskState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualHardDiskState {

/// 
    #[serde(rename = "Alignment")]
    pub alignment: Option<u32>,

/// 
    #[serde(rename = "FileSize")]
    pub file_size: Option<u64>,

/// 
    #[serde(rename = "FragmentationPercentage")]
    pub fragmentation_percentage: Option<u32>,

/// 
    #[serde(rename = "InUse")]
    pub in_use: Option<bool>,

/// 
    #[serde(rename = "MinInternalSize")]
    pub min_internal_size: Option<u64>,

/// 
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u32>,

/// 
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,
}

impl Msvm_VirtualHardDiskState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alignment: None,
            file_size: None,
            fragmentation_percentage: None,
            in_use: None,
            min_internal_size: None,
            physical_sector_size: None,
            timestamp: None,
        }
    }


    /// Sets the value of Alignment
    pub fn set_alignment(&mut self, value: u32) {
        self.alignment = Some(value);
    }

    /// Gets the value of Alignment
    pub fn get_alignment(&self) -> Option<&u32> {
        self.alignment.as_ref()
    }

    /// Sets the value of FileSize
    pub fn set_file_size(&mut self, value: u64) {
        self.file_size = Some(value);
    }

    /// Gets the value of FileSize
    pub fn get_file_size(&self) -> Option<&u64> {
        self.file_size.as_ref()
    }

    /// Sets the value of FragmentationPercentage
    pub fn set_fragmentation_percentage(&mut self, value: u32) {
        self.fragmentation_percentage = Some(value);
    }

    /// Gets the value of FragmentationPercentage
    pub fn get_fragmentation_percentage(&self) -> Option<&u32> {
        self.fragmentation_percentage.as_ref()
    }

    /// Sets the value of InUse
    pub fn set_in_use(&mut self, value: bool) {
        self.in_use = Some(value);
    }

    /// Gets the value of InUse
    pub fn get_in_use(&self) -> Option<&bool> {
        self.in_use.as_ref()
    }

    /// Sets the value of MinInternalSize
    pub fn set_min_internal_size(&mut self, value: u64) {
        self.min_internal_size = Some(value);
    }

    /// Gets the value of MinInternalSize
    pub fn get_min_internal_size(&self) -> Option<&u64> {
        self.min_internal_size.as_ref()
    }

    /// Sets the value of PhysicalSectorSize
    pub fn set_physical_sector_size(&mut self, value: u32) {
        self.physical_sector_size = Some(value);
    }

    /// Gets the value of PhysicalSectorSize
    pub fn get_physical_sector_size(&self) -> Option<&u32> {
        self.physical_sector_size.as_ref()
    }

    /// Sets the value of Timestamp
    pub fn set_timestamp(&mut self, value: String) {
        self.timestamp = Some(value);
    }

    /// Gets the value of Timestamp
    pub fn get_timestamp(&self) -> Option<&String> {
        self.timestamp.as_ref()
    }
}

