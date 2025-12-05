// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTDisk {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "ActiveTime")]
    pub active_time: Vec<f32>,

/// 
    #[serde(rename = "AverageResponseTime")]
    pub average_response_time: Option<f32>,

/// 
    #[serde(rename = "Capacity")]
    pub capacity: Option<u64>,

/// 
    #[serde(rename = "CurrentIndex")]
    pub current_index: Option<u16>,

/// 
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// 
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ReadTransferRate")]
    pub read_transfer_rate: Vec<f32>,

/// 
    #[serde(rename = "Volumes")]
    pub volumes: Vec<MSFT_MTDiskVolume>,

/// 
    #[serde(rename = "WriteTransferRate")]
    pub write_transfer_rate: Vec<f32>,
}

impl MSFT_MTDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            active_time: Vec::new(),
            average_response_time: None,
            capacity: None,
            current_index: None,
            disk_number: None,
            interval_seconds: None,
            name: None,
            read_transfer_rate: Vec::new(),
            volumes: Vec::new(),
            write_transfer_rate: Vec::new(),
        }
    }


    /// Sets the value of ActiveTime
    pub fn set_active_time(&mut self, value: Vec<f32>) {
        self.active_time = value;
    }

    /// Gets the value of ActiveTime
    pub fn get_active_time(&self) -> &Vec<f32> {
        &self.active_time
    }

    /// Sets the value of AverageResponseTime
    pub fn set_average_response_time(&mut self, value: f32) {
        self.average_response_time = Some(value);
    }

    /// Gets the value of AverageResponseTime
    pub fn get_average_response_time(&self) -> Option<&f32> {
        self.average_response_time.as_ref()
    }

    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: u64) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&u64> {
        self.capacity.as_ref()
    }

    /// Sets the value of CurrentIndex
    pub fn set_current_index(&mut self, value: u16) {
        self.current_index = Some(value);
    }

    /// Gets the value of CurrentIndex
    pub fn get_current_index(&self) -> Option<&u16> {
        self.current_index.as_ref()
    }

    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of IntervalSeconds
    pub fn set_interval_seconds(&mut self, value: u16) {
        self.interval_seconds = Some(value);
    }

    /// Gets the value of IntervalSeconds
    pub fn get_interval_seconds(&self) -> Option<&u16> {
        self.interval_seconds.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ReadTransferRate
    pub fn set_read_transfer_rate(&mut self, value: Vec<f32>) {
        self.read_transfer_rate = value;
    }

    /// Gets the value of ReadTransferRate
    pub fn get_read_transfer_rate(&self) -> &Vec<f32> {
        &self.read_transfer_rate
    }

    /// Sets the value of Volumes
    pub fn set_volumes(&mut self, value: Vec<MSFT_MTDiskVolume>) {
        self.volumes = value;
    }

    /// Gets the value of Volumes
    pub fn get_volumes(&self) -> &Vec<MSFT_MTDiskVolume> {
        &self.volumes
    }

    /// Sets the value of WriteTransferRate
    pub fn set_write_transfer_rate(&mut self, value: Vec<f32>) {
        self.write_transfer_rate = value;
    }

    /// Gets the value of WriteTransferRate
    pub fn get_write_transfer_rate(&self) -> &Vec<f32> {
        &self.write_transfer_rate
    }
}

