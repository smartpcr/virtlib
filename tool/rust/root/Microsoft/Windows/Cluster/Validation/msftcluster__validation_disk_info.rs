// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Validation
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFTCluster_ValidationDiskInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFTCluster_ValidationDiskInfo {

/// 2
    #[serde(rename = "AdapterDescription")]
    pub adapter_description: Option<String>,

/// 2
    #[serde(rename = "BusType")]
    pub bus_type: Option<ValidationDiskInfo_BusType>,

/// 2
    #[serde(rename = "DevicePath")]
    pub device_path: Option<String>,

/// 2
    #[serde(rename = "DiskId")]
    pub disk_id: Option<String>,

/// 2
    #[serde(rename = "DiskIdType")]
    pub disk_id_type: Option<ValidationDiskInfo_DiskIdType>,

/// 1
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// 2
    #[serde(rename = "ExcludeFromTests")]
    pub exclude_from_tests: Option<bool>,

/// 2
    #[serde(rename = "ExtendedFlags")]
    pub extended_flags: Option<u32>,

/// 2
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 2
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 2
    #[serde(rename = "GptPartitionType")]
    pub gpt_partition_type: Vec<ValidationDiskInfo_GptPartitionType>,

/// 2
    #[serde(rename = "IsClusterable")]
    pub is_clusterable: Option<bool>,

/// 2
    #[serde(rename = "IsClustered")]
    pub is_clustered: Option<bool>,

/// 2
    #[serde(rename = "IsHiddenDisk")]
    pub is_hidden_disk: Option<bool>,

/// 2
    #[serde(rename = "IsPage89Supported")]
    pub is_page89_supported: Option<bool>,

/// 2
    #[serde(rename = "IsPoolDisk")]
    pub is_pool_disk: Option<bool>,

/// 2
    #[serde(rename = "MbrPartitionType")]
    pub mbr_partition_type: Vec<ValidationDiskInfo_MbrPartitionType>,

/// 91
    #[serde(rename = "MediaType")]
    pub media_type: Option<ValidationDiskInfo_MediaType>,

/// 2
    #[serde(rename = "MiniportDriver")]
    pub miniport_driver: Option<String>,

/// 2
    #[serde(rename = "Page83Id")]
    pub page83_id: Option<String>,

/// 2
    #[serde(rename = "ScsiAddress")]
    pub scsi_address: Option<String>,

/// 2
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 2
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

/// 2
    #[serde(rename = "StackType")]
    pub stack_type: Option<ValidationDiskInfo_StackType>,
}

impl MSFTCluster_ValidationDiskInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            adapter_description: None,
            bus_type: None,
            device_path: None,
            disk_id: None,
            disk_id_type: None,
            disk_number: None,
            exclude_from_tests: None,
            extended_flags: None,
            flags: None,
            friendly_name: None,
            gpt_partition_type: Vec::new(),
            is_clusterable: None,
            is_clustered: None,
            is_hidden_disk: None,
            is_page89_supported: None,
            is_pool_disk: None,
            mbr_partition_type: Vec::new(),
            media_type: None,
            miniport_driver: None,
            page83_id: None,
            scsi_address: None,
            serial_number: None,
            service_name: None,
            stack_type: None,
        }
    }


    /// Sets the value of AdapterDescription
    pub fn set_adapter_description(&mut self, value: String) {
        self.adapter_description = Some(value);
    }

    /// Gets the value of AdapterDescription
    pub fn get_adapter_description(&self) -> Option<&String> {
        self.adapter_description.as_ref()
    }

    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: ValidationDiskInfo_BusType) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&ValidationDiskInfo_BusType> {
        self.bus_type.as_ref()
    }

    /// Sets the value of DevicePath
    pub fn set_device_path(&mut self, value: String) {
        self.device_path = Some(value);
    }

    /// Gets the value of DevicePath
    pub fn get_device_path(&self) -> Option<&String> {
        self.device_path.as_ref()
    }

    /// Sets the value of DiskId
    pub fn set_disk_id(&mut self, value: String) {
        self.disk_id = Some(value);
    }

    /// Gets the value of DiskId
    pub fn get_disk_id(&self) -> Option<&String> {
        self.disk_id.as_ref()
    }

    /// Sets the value of DiskIdType
    pub fn set_disk_id_type(&mut self, value: ValidationDiskInfo_DiskIdType) {
        self.disk_id_type = Some(value);
    }

    /// Gets the value of DiskIdType
    pub fn get_disk_id_type(&self) -> Option<&ValidationDiskInfo_DiskIdType> {
        self.disk_id_type.as_ref()
    }

    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of ExcludeFromTests
    pub fn set_exclude_from_tests(&mut self, value: bool) {
        self.exclude_from_tests = Some(value);
    }

    /// Gets the value of ExcludeFromTests
    pub fn get_exclude_from_tests(&self) -> Option<&bool> {
        self.exclude_from_tests.as_ref()
    }

    /// Sets the value of ExtendedFlags
    pub fn set_extended_flags(&mut self, value: u32) {
        self.extended_flags = Some(value);
    }

    /// Gets the value of ExtendedFlags
    pub fn get_extended_flags(&self) -> Option<&u32> {
        self.extended_flags.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of GptPartitionType
    pub fn set_gpt_partition_type(&mut self, value: Vec<ValidationDiskInfo_GptPartitionType>) {
        self.gpt_partition_type = value;
    }

    /// Gets the value of GptPartitionType
    pub fn get_gpt_partition_type(&self) -> &Vec<ValidationDiskInfo_GptPartitionType> {
        &self.gpt_partition_type
    }

    /// Sets the value of IsClusterable
    pub fn set_is_clusterable(&mut self, value: bool) {
        self.is_clusterable = Some(value);
    }

    /// Gets the value of IsClusterable
    pub fn get_is_clusterable(&self) -> Option<&bool> {
        self.is_clusterable.as_ref()
    }

    /// Sets the value of IsClustered
    pub fn set_is_clustered(&mut self, value: bool) {
        self.is_clustered = Some(value);
    }

    /// Gets the value of IsClustered
    pub fn get_is_clustered(&self) -> Option<&bool> {
        self.is_clustered.as_ref()
    }

    /// Sets the value of IsHiddenDisk
    pub fn set_is_hidden_disk(&mut self, value: bool) {
        self.is_hidden_disk = Some(value);
    }

    /// Gets the value of IsHiddenDisk
    pub fn get_is_hidden_disk(&self) -> Option<&bool> {
        self.is_hidden_disk.as_ref()
    }

    /// Sets the value of IsPage89Supported
    pub fn set_is_page89_supported(&mut self, value: bool) {
        self.is_page89_supported = Some(value);
    }

    /// Gets the value of IsPage89Supported
    pub fn get_is_page89_supported(&self) -> Option<&bool> {
        self.is_page89_supported.as_ref()
    }

    /// Sets the value of IsPoolDisk
    pub fn set_is_pool_disk(&mut self, value: bool) {
        self.is_pool_disk = Some(value);
    }

    /// Gets the value of IsPoolDisk
    pub fn get_is_pool_disk(&self) -> Option<&bool> {
        self.is_pool_disk.as_ref()
    }

    /// Sets the value of MbrPartitionType
    pub fn set_mbr_partition_type(&mut self, value: Vec<ValidationDiskInfo_MbrPartitionType>) {
        self.mbr_partition_type = value;
    }

    /// Gets the value of MbrPartitionType
    pub fn get_mbr_partition_type(&self) -> &Vec<ValidationDiskInfo_MbrPartitionType> {
        &self.mbr_partition_type
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: ValidationDiskInfo_MediaType) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&ValidationDiskInfo_MediaType> {
        self.media_type.as_ref()
    }

    /// Sets the value of MiniportDriver
    pub fn set_miniport_driver(&mut self, value: String) {
        self.miniport_driver = Some(value);
    }

    /// Gets the value of MiniportDriver
    pub fn get_miniport_driver(&self) -> Option<&String> {
        self.miniport_driver.as_ref()
    }

    /// Sets the value of Page83Id
    pub fn set_page83_id(&mut self, value: String) {
        self.page83_id = Some(value);
    }

    /// Gets the value of Page83Id
    pub fn get_page83_id(&self) -> Option<&String> {
        self.page83_id.as_ref()
    }

    /// Sets the value of ScsiAddress
    pub fn set_scsi_address(&mut self, value: String) {
        self.scsi_address = Some(value);
    }

    /// Gets the value of ScsiAddress
    pub fn get_scsi_address(&self) -> Option<&String> {
        self.scsi_address.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: String) {
        self.service_name = Some(value);
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    /// Sets the value of StackType
    pub fn set_stack_type(&mut self, value: ValidationDiskInfo_StackType) {
        self.stack_type = Some(value);
    }

    /// Gets the value of StackType
    pub fn get_stack_type(&self) -> Option<&ValidationDiskInfo_StackType> {
        self.stack_type.as_ref()
    }
}

