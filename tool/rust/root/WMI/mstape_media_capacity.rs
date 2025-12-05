// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSTapeMediaCapacity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSTapeMediaCapacity {
    #[serde(flatten)]
    pub base: MSTapeDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "AvailableCapacity")]
    pub available_capacity: Option<u64>,

/// 
    #[serde(rename = "BlockSize")]
    pub block_size: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MaximumCapacity")]
    pub maximum_capacity: Option<u64>,

/// 
    #[serde(rename = "MediaWriteProtected")]
    pub media_write_protected: Option<bool>,

/// 
    #[serde(rename = "PartitionCount")]
    pub partition_count: Option<u32>,
}

impl MSTapeMediaCapacity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSTapeDriver::new(),
            active: None,
            available_capacity: None,
            block_size: None,
            instance_name: None,
            maximum_capacity: None,
            media_write_protected: None,
            partition_count: None,
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

    /// Sets the value of AvailableCapacity
    pub fn set_available_capacity(&mut self, value: u64) {
        self.available_capacity = Some(value);
    }

    /// Gets the value of AvailableCapacity
    pub fn get_available_capacity(&self) -> Option<&u64> {
        self.available_capacity.as_ref()
    }

    /// Sets the value of BlockSize
    pub fn set_block_size(&mut self, value: u32) {
        self.block_size = Some(value);
    }

    /// Gets the value of BlockSize
    pub fn get_block_size(&self) -> Option<&u32> {
        self.block_size.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MaximumCapacity
    pub fn set_maximum_capacity(&mut self, value: u64) {
        self.maximum_capacity = Some(value);
    }

    /// Gets the value of MaximumCapacity
    pub fn get_maximum_capacity(&self) -> Option<&u64> {
        self.maximum_capacity.as_ref()
    }

    /// Sets the value of MediaWriteProtected
    pub fn set_media_write_protected(&mut self, value: bool) {
        self.media_write_protected = Some(value);
    }

    /// Gets the value of MediaWriteProtected
    pub fn get_media_write_protected(&self) -> Option<&bool> {
        self.media_write_protected.as_ref()
    }

    /// Sets the value of PartitionCount
    pub fn set_partition_count(&mut self, value: u32) {
        self.partition_count = Some(value);
    }

    /// Gets the value of PartitionCount
    pub fn get_partition_count(&self) -> Option<&u32> {
        self.partition_count.as_ref()
    }
}

