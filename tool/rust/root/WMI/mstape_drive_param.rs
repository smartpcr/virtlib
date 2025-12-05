// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSTapeDriveParam struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSTapeDriveParam {
    #[serde(flatten)]
    pub base: MSTapeDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "CompressionCapable")]
    pub compression_capable: Option<bool>,

/// 
    #[serde(rename = "CompressionEnabled")]
    pub compression_enabled: Option<bool>,

/// 
    #[serde(rename = "DefaultBlockSize")]
    pub default_block_size: Option<u32>,

/// 
    #[serde(rename = "HardwareErrorCorrection")]
    pub hardware_error_correction: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MaximumBlockSize")]
    pub maximum_block_size: Option<u32>,

/// 
    #[serde(rename = "MaximumPartitionCount")]
    pub maximum_partition_count: Option<u32>,

/// 
    #[serde(rename = "MinimumBlockSize")]
    pub minimum_block_size: Option<u32>,

/// 
    #[serde(rename = "ReportSetmarks")]
    pub report_setmarks: Option<bool>,
}

impl MSTapeDriveParam {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSTapeDriver::new(),
            active: None,
            compression_capable: None,
            compression_enabled: None,
            default_block_size: None,
            hardware_error_correction: None,
            instance_name: None,
            maximum_block_size: None,
            maximum_partition_count: None,
            minimum_block_size: None,
            report_setmarks: None,
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

    /// Sets the value of CompressionCapable
    pub fn set_compression_capable(&mut self, value: bool) {
        self.compression_capable = Some(value);
    }

    /// Gets the value of CompressionCapable
    pub fn get_compression_capable(&self) -> Option<&bool> {
        self.compression_capable.as_ref()
    }

    /// Sets the value of CompressionEnabled
    pub fn set_compression_enabled(&mut self, value: bool) {
        self.compression_enabled = Some(value);
    }

    /// Gets the value of CompressionEnabled
    pub fn get_compression_enabled(&self) -> Option<&bool> {
        self.compression_enabled.as_ref()
    }

    /// Sets the value of DefaultBlockSize
    pub fn set_default_block_size(&mut self, value: u32) {
        self.default_block_size = Some(value);
    }

    /// Gets the value of DefaultBlockSize
    pub fn get_default_block_size(&self) -> Option<&u32> {
        self.default_block_size.as_ref()
    }

    /// Sets the value of HardwareErrorCorrection
    pub fn set_hardware_error_correction(&mut self, value: bool) {
        self.hardware_error_correction = Some(value);
    }

    /// Gets the value of HardwareErrorCorrection
    pub fn get_hardware_error_correction(&self) -> Option<&bool> {
        self.hardware_error_correction.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MaximumBlockSize
    pub fn set_maximum_block_size(&mut self, value: u32) {
        self.maximum_block_size = Some(value);
    }

    /// Gets the value of MaximumBlockSize
    pub fn get_maximum_block_size(&self) -> Option<&u32> {
        self.maximum_block_size.as_ref()
    }

    /// Sets the value of MaximumPartitionCount
    pub fn set_maximum_partition_count(&mut self, value: u32) {
        self.maximum_partition_count = Some(value);
    }

    /// Gets the value of MaximumPartitionCount
    pub fn get_maximum_partition_count(&self) -> Option<&u32> {
        self.maximum_partition_count.as_ref()
    }

    /// Sets the value of MinimumBlockSize
    pub fn set_minimum_block_size(&mut self, value: u32) {
        self.minimum_block_size = Some(value);
    }

    /// Gets the value of MinimumBlockSize
    pub fn get_minimum_block_size(&self) -> Option<&u32> {
        self.minimum_block_size.as_ref()
    }

    /// Sets the value of ReportSetmarks
    pub fn set_report_setmarks(&mut self, value: bool) {
        self.report_setmarks = Some(value);
    }

    /// Gets the value of ReportSetmarks
    pub fn get_report_setmarks(&self) -> Option<&bool> {
        self.report_setmarks.as_ref()
    }
}

