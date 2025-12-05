// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ClusterDiskPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ClusterDiskPartition {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "FileSystem")]
    pub file_system: Option<String>,

/// 
    #[serde(rename = "FileSystemFlags")]
    pub file_system_flags: Option<u32>,

/// 
    #[serde(rename = "FreeSpace")]
    pub free_space: Option<u32>,

/// 
    #[serde(rename = "MaximumComponentLength")]
    pub maximum_component_length: Option<u32>,

/// 
    #[serde(rename = "MountPoints")]
    pub mount_points: Vec<String>,

/// 
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<u32>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<u32>,

/// 
    #[serde(rename = "TotalSize")]
    pub total_size: Option<u32>,

/// 
    #[serde(rename = "VolumeGuid")]
    pub volume_guid: Option<String>,

/// 
    #[serde(rename = "VolumeLabel")]
    pub volume_label: Option<String>,
}

impl MSCluster_ClusterDiskPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            file_system: None,
            file_system_flags: None,
            free_space: None,
            maximum_component_length: None,
            mount_points: Vec::new(),
            partition_number: None,
            path: None,
            serial_number: None,
            total_size: None,
            volume_guid: None,
            volume_label: None,
        }
    }


    /// Sets the value of FileSystem
    pub fn set_file_system(&mut self, value: String) {
        self.file_system = Some(value);
    }

    /// Gets the value of FileSystem
    pub fn get_file_system(&self) -> Option<&String> {
        self.file_system.as_ref()
    }

    /// Sets the value of FileSystemFlags
    pub fn set_file_system_flags(&mut self, value: u32) {
        self.file_system_flags = Some(value);
    }

    /// Gets the value of FileSystemFlags
    pub fn get_file_system_flags(&self) -> Option<&u32> {
        self.file_system_flags.as_ref()
    }

    /// Sets the value of FreeSpace
    pub fn set_free_space(&mut self, value: u32) {
        self.free_space = Some(value);
    }

    /// Gets the value of FreeSpace
    pub fn get_free_space(&self) -> Option<&u32> {
        self.free_space.as_ref()
    }

    /// Sets the value of MaximumComponentLength
    pub fn set_maximum_component_length(&mut self, value: u32) {
        self.maximum_component_length = Some(value);
    }

    /// Gets the value of MaximumComponentLength
    pub fn get_maximum_component_length(&self) -> Option<&u32> {
        self.maximum_component_length.as_ref()
    }

    /// Sets the value of MountPoints
    pub fn set_mount_points(&mut self, value: Vec<String>) {
        self.mount_points = value;
    }

    /// Gets the value of MountPoints
    pub fn get_mount_points(&self) -> &Vec<String> {
        &self.mount_points
    }

    /// Sets the value of PartitionNumber
    pub fn set_partition_number(&mut self, value: u32) {
        self.partition_number = Some(value);
    }

    /// Gets the value of PartitionNumber
    pub fn get_partition_number(&self) -> Option<&u32> {
        self.partition_number.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: u32) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&u32> {
        self.serial_number.as_ref()
    }

    /// Sets the value of TotalSize
    pub fn set_total_size(&mut self, value: u32) {
        self.total_size = Some(value);
    }

    /// Gets the value of TotalSize
    pub fn get_total_size(&self) -> Option<&u32> {
        self.total_size.as_ref()
    }

    /// Sets the value of VolumeGuid
    pub fn set_volume_guid(&mut self, value: String) {
        self.volume_guid = Some(value);
    }

    /// Gets the value of VolumeGuid
    pub fn get_volume_guid(&self) -> Option<&String> {
        self.volume_guid.as_ref()
    }

    /// Sets the value of VolumeLabel
    pub fn set_volume_label(&mut self, value: String) {
        self.volume_label = Some(value);
    }

    /// Gets the value of VolumeLabel
    pub fn get_volume_label(&self) -> Option<&String> {
        self.volume_label.as_ref()
    }
}

