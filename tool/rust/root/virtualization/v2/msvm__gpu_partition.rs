// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_GpuPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_GpuPartition {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "CurrentCompute")]
    pub current_compute: Option<u64>,

/// 
    #[serde(rename = "CurrentDecode")]
    pub current_decode: Option<u64>,

/// 
    #[serde(rename = "CurrentEncode")]
    pub current_encode: Option<u64>,

/// 
    #[serde(rename = "CurrentVRAM")]
    pub current_vram: Option<u64>,

/// 
    #[serde(rename = "DeviceInstancePath")]
    pub device_instance_path: Option<String>,

/// 
    #[serde(rename = "PartitionId")]
    pub partition_id: Option<u16>,

/// 
    #[serde(rename = "PartitionVfLuid")]
    pub partition_vf_luid: Option<String>,

/// 
    #[serde(rename = "SupportsOutgoingLiveMigration")]
    pub supports_outgoing_live_migration: Option<bool>,
}

impl Msvm_GpuPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            current_compute: None,
            current_decode: None,
            current_encode: None,
            current_vram: None,
            device_instance_path: None,
            partition_id: None,
            partition_vf_luid: None,
            supports_outgoing_live_migration: None,
        }
    }


    /// Sets the value of CurrentCompute
    pub fn set_current_compute(&mut self, value: u64) {
        self.current_compute = Some(value);
    }

    /// Gets the value of CurrentCompute
    pub fn get_current_compute(&self) -> Option<&u64> {
        self.current_compute.as_ref()
    }

    /// Sets the value of CurrentDecode
    pub fn set_current_decode(&mut self, value: u64) {
        self.current_decode = Some(value);
    }

    /// Gets the value of CurrentDecode
    pub fn get_current_decode(&self) -> Option<&u64> {
        self.current_decode.as_ref()
    }

    /// Sets the value of CurrentEncode
    pub fn set_current_encode(&mut self, value: u64) {
        self.current_encode = Some(value);
    }

    /// Gets the value of CurrentEncode
    pub fn get_current_encode(&self) -> Option<&u64> {
        self.current_encode.as_ref()
    }

    /// Sets the value of CurrentVRAM
    pub fn set_current_vram(&mut self, value: u64) {
        self.current_vram = Some(value);
    }

    /// Gets the value of CurrentVRAM
    pub fn get_current_vram(&self) -> Option<&u64> {
        self.current_vram.as_ref()
    }

    /// Sets the value of DeviceInstancePath
    pub fn set_device_instance_path(&mut self, value: String) {
        self.device_instance_path = Some(value);
    }

    /// Gets the value of DeviceInstancePath
    pub fn get_device_instance_path(&self) -> Option<&String> {
        self.device_instance_path.as_ref()
    }

    /// Sets the value of PartitionId
    pub fn set_partition_id(&mut self, value: u16) {
        self.partition_id = Some(value);
    }

    /// Gets the value of PartitionId
    pub fn get_partition_id(&self) -> Option<&u16> {
        self.partition_id.as_ref()
    }

    /// Sets the value of PartitionVfLuid
    pub fn set_partition_vf_luid(&mut self, value: String) {
        self.partition_vf_luid = Some(value);
    }

    /// Gets the value of PartitionVfLuid
    pub fn get_partition_vf_luid(&self) -> Option<&String> {
        self.partition_vf_luid.as_ref()
    }

    /// Sets the value of SupportsOutgoingLiveMigration
    pub fn set_supports_outgoing_live_migration(&mut self, value: bool) {
        self.supports_outgoing_live_migration = Some(value);
    }

    /// Gets the value of SupportsOutgoingLiveMigration
    pub fn get_supports_outgoing_live_migration(&self) -> Option<&bool> {
        self.supports_outgoing_live_migration.as_ref()
    }
}

