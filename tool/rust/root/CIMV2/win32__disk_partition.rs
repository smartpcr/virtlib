// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DiskPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DiskPartition {
    #[serde(flatten)]
    pub base: CIM_DiskPartition,

/// 
    #[serde(rename = "BootPartition")]
    pub boot_partition: Option<bool>,

/// 
    #[serde(rename = "DiskIndex")]
    pub disk_index: Option<u32>,

/// 
    #[serde(rename = "HiddenSectors")]
    pub hidden_sectors: Option<u32>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "RewritePartition")]
    pub rewrite_partition: Option<bool>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "StartingOffset")]
    pub starting_offset: Option<u64>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl Win32_DiskPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DiskPartition::new(),
            boot_partition: None,
            disk_index: None,
            hidden_sectors: None,
            index: None,
            rewrite_partition: None,
            size: None,
            starting_offset: None,
            type: None,
        }
    }


    /// Sets the value of BootPartition
    pub fn set_boot_partition(&mut self, value: bool) {
        self.boot_partition = Some(value);
    }

    /// Gets the value of BootPartition
    pub fn get_boot_partition(&self) -> Option<&bool> {
        self.boot_partition.as_ref()
    }

    /// Sets the value of DiskIndex
    pub fn set_disk_index(&mut self, value: u32) {
        self.disk_index = Some(value);
    }

    /// Gets the value of DiskIndex
    pub fn get_disk_index(&self) -> Option<&u32> {
        self.disk_index.as_ref()
    }

    /// Sets the value of HiddenSectors
    pub fn set_hidden_sectors(&mut self, value: u32) {
        self.hidden_sectors = Some(value);
    }

    /// Gets the value of HiddenSectors
    pub fn get_hidden_sectors(&self) -> Option<&u32> {
        self.hidden_sectors.as_ref()
    }

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of RewritePartition
    pub fn set_rewrite_partition(&mut self, value: bool) {
        self.rewrite_partition = Some(value);
    }

    /// Gets the value of RewritePartition
    pub fn get_rewrite_partition(&self) -> Option<&bool> {
        self.rewrite_partition.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of StartingOffset
    pub fn set_starting_offset(&mut self, value: u64) {
        self.starting_offset = Some(value);
    }

    /// Gets the value of StartingOffset
    pub fn get_starting_offset(&self) -> Option<&u64> {
        self.starting_offset.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

