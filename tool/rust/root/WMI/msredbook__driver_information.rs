// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSRedbook_DriverInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSRedbook_DriverInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "CDDAAccurate")]
    pub cddaaccurate: Option<bool>,

/// 
    #[serde(rename = "CDDASupported")]
    pub cddasupported: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MaximumSectorsPerRead")]
    pub maximum_sectors_per_read: Option<u32>,

/// 
    #[serde(rename = "NumberOfBuffers")]
    pub number_of_buffers: Option<u32>,

/// 
    #[serde(rename = "PlayEnabled")]
    pub play_enabled: Option<bool>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<bool>,

/// 
    #[serde(rename = "SectorsPerRead")]
    pub sectors_per_read: Option<u32>,

/// 
    #[serde(rename = "SectorsPerReadMask")]
    pub sectors_per_read_mask: Option<u32>,
}

impl MSRedbook_DriverInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            cddaaccurate: None,
            cddasupported: None,
            instance_name: None,
            maximum_sectors_per_read: None,
            number_of_buffers: None,
            play_enabled: None,
            reserved1: None,
            sectors_per_read: None,
            sectors_per_read_mask: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of CDDAAccurate
    pub fn set_cddaaccurate(&mut self, value: bool) {
        self.cddaaccurate = Some(value);
    }

    /// Gets the value of CDDAAccurate
    pub fn get_cddaaccurate(&self) -> Option<&bool> {
        self.cddaaccurate.as_ref()
    }

    /// Sets the value of CDDASupported
    pub fn set_cddasupported(&mut self, value: bool) {
        self.cddasupported = Some(value);
    }

    /// Gets the value of CDDASupported
    pub fn get_cddasupported(&self) -> Option<&bool> {
        self.cddasupported.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MaximumSectorsPerRead
    pub fn set_maximum_sectors_per_read(&mut self, value: u32) {
        self.maximum_sectors_per_read = Some(value);
    }

    /// Gets the value of MaximumSectorsPerRead
    pub fn get_maximum_sectors_per_read(&self) -> Option<&u32> {
        self.maximum_sectors_per_read.as_ref()
    }

    /// Sets the value of NumberOfBuffers
    pub fn set_number_of_buffers(&mut self, value: u32) {
        self.number_of_buffers = Some(value);
    }

    /// Gets the value of NumberOfBuffers
    pub fn get_number_of_buffers(&self) -> Option<&u32> {
        self.number_of_buffers.as_ref()
    }

    /// Sets the value of PlayEnabled
    pub fn set_play_enabled(&mut self, value: bool) {
        self.play_enabled = Some(value);
    }

    /// Gets the value of PlayEnabled
    pub fn get_play_enabled(&self) -> Option<&bool> {
        self.play_enabled.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: bool) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&bool> {
        self.reserved1.as_ref()
    }

    /// Sets the value of SectorsPerRead
    pub fn set_sectors_per_read(&mut self, value: u32) {
        self.sectors_per_read = Some(value);
    }

    /// Gets the value of SectorsPerRead
    pub fn get_sectors_per_read(&self) -> Option<&u32> {
        self.sectors_per_read.as_ref()
    }

    /// Sets the value of SectorsPerReadMask
    pub fn set_sectors_per_read_mask(&mut self, value: u32) {
        self.sectors_per_read_mask = Some(value);
    }

    /// Gets the value of SectorsPerReadMask
    pub fn get_sectors_per_read_mask(&self) -> Option<&u32> {
        self.sectors_per_read_mask.as_ref()
    }
}

