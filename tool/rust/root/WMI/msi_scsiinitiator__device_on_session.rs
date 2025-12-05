// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_DeviceOnSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_DeviceOnSession {

/// PNP Device interface guid
    #[serde(rename = "DeviceInterfaceGuid")]
    pub device_interface_guid: Option<String>,

/// PNP Device interface name
    #[serde(rename = "DeviceInterfaceName")]
    pub device_interface_name: Option<String>,

/// The number of this device
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<u32>,

/// The FILE_DEVICE_XXX type for this device.
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// Name of initiator
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,

/// Legacy Device interface name
    #[serde(rename = "LegacyName")]
    pub legacy_name: Option<String>,

/// If the device is partitionable, the partition number of the device. Otherwise -1
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<u32>,

/// OS SCSI Logical Unit Number
    #[serde(rename = "ScsiLun")]
    pub scsi_lun: Option<u8>,

/// OS SCSI path identifier or bus number
    #[serde(rename = "ScsiPathId")]
    pub scsi_path_id: Option<u8>,

/// OS SCSI port number
    #[serde(rename = "ScsiPortNumber")]
    pub scsi_port_number: Option<u8>,

/// OS SCSI Target Id
    #[serde(rename = "ScsiTargetId")]
    pub scsi_target_id: Option<u8>,

/// Name of target
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,
}

impl MSiSCSIInitiator_DeviceOnSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_interface_guid: None,
            device_interface_name: None,
            device_number: None,
            device_type: None,
            initiator_name: None,
            legacy_name: None,
            partition_number: None,
            scsi_lun: None,
            scsi_path_id: None,
            scsi_port_number: None,
            scsi_target_id: None,
            target_name: None,
        }
    }


    /// Sets the value of DeviceInterfaceGuid
    pub fn set_device_interface_guid(&mut self, value: String) {
        self.device_interface_guid = Some(value);
    }

    /// Gets the value of DeviceInterfaceGuid
    pub fn get_device_interface_guid(&self) -> Option<&String> {
        self.device_interface_guid.as_ref()
    }

    /// Sets the value of DeviceInterfaceName
    pub fn set_device_interface_name(&mut self, value: String) {
        self.device_interface_name = Some(value);
    }

    /// Gets the value of DeviceInterfaceName
    pub fn get_device_interface_name(&self) -> Option<&String> {
        self.device_interface_name.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: u32) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&u32> {
        self.device_number.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }

    /// Sets the value of LegacyName
    pub fn set_legacy_name(&mut self, value: String) {
        self.legacy_name = Some(value);
    }

    /// Gets the value of LegacyName
    pub fn get_legacy_name(&self) -> Option<&String> {
        self.legacy_name.as_ref()
    }

    /// Sets the value of PartitionNumber
    pub fn set_partition_number(&mut self, value: u32) {
        self.partition_number = Some(value);
    }

    /// Gets the value of PartitionNumber
    pub fn get_partition_number(&self) -> Option<&u32> {
        self.partition_number.as_ref()
    }

    /// Sets the value of ScsiLun
    pub fn set_scsi_lun(&mut self, value: u8) {
        self.scsi_lun = Some(value);
    }

    /// Gets the value of ScsiLun
    pub fn get_scsi_lun(&self) -> Option<&u8> {
        self.scsi_lun.as_ref()
    }

    /// Sets the value of ScsiPathId
    pub fn set_scsi_path_id(&mut self, value: u8) {
        self.scsi_path_id = Some(value);
    }

    /// Gets the value of ScsiPathId
    pub fn get_scsi_path_id(&self) -> Option<&u8> {
        self.scsi_path_id.as_ref()
    }

    /// Sets the value of ScsiPortNumber
    pub fn set_scsi_port_number(&mut self, value: u8) {
        self.scsi_port_number = Some(value);
    }

    /// Gets the value of ScsiPortNumber
    pub fn get_scsi_port_number(&self) -> Option<&u8> {
        self.scsi_port_number.as_ref()
    }

    /// Sets the value of ScsiTargetId
    pub fn set_scsi_target_id(&mut self, value: u8) {
        self.scsi_target_id = Some(value);
    }

    /// Gets the value of ScsiTargetId
    pub fn get_scsi_target_id(&self) -> Option<&u8> {
        self.scsi_target_id.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }
}

