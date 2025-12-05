// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ClusterDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ClusterDisk {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "GptGuid")]
    pub gpt_guid: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Number")]
    pub number: Option<u32>,

/// 
    #[serde(rename = "ScsiBus")]
    pub scsi_bus: Option<u32>,

/// 
    #[serde(rename = "ScsiLun")]
    pub scsi_lun: Option<u32>,

/// 
    #[serde(rename = "ScsiPort")]
    pub scsi_port: Option<u32>,

/// 
    #[serde(rename = "ScsiTargetId")]
    pub scsi_target_id: Option<u32>,

/// 
    #[serde(rename = "Signature")]
    pub signature: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "StoragePoolId")]
    pub storage_pool_id: Option<String>,

/// 
    #[serde(rename = "VirtualDiskId")]
    pub virtual_disk_id: Option<String>,
}

impl MSCluster_ClusterDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            gpt_guid: None,
            id: None,
            number: None,
            scsi_bus: None,
            scsi_lun: None,
            scsi_port: None,
            scsi_target_id: None,
            signature: None,
            size: None,
            storage_pool_id: None,
            virtual_disk_id: None,
        }
    }


    /// Sets the value of GptGuid
    pub fn set_gpt_guid(&mut self, value: String) {
        self.gpt_guid = Some(value);
    }

    /// Gets the value of GptGuid
    pub fn get_gpt_guid(&self) -> Option<&String> {
        self.gpt_guid.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Number
    pub fn set_number(&mut self, value: u32) {
        self.number = Some(value);
    }

    /// Gets the value of Number
    pub fn get_number(&self) -> Option<&u32> {
        self.number.as_ref()
    }

    /// Sets the value of ScsiBus
    pub fn set_scsi_bus(&mut self, value: u32) {
        self.scsi_bus = Some(value);
    }

    /// Gets the value of ScsiBus
    pub fn get_scsi_bus(&self) -> Option<&u32> {
        self.scsi_bus.as_ref()
    }

    /// Sets the value of ScsiLun
    pub fn set_scsi_lun(&mut self, value: u32) {
        self.scsi_lun = Some(value);
    }

    /// Gets the value of ScsiLun
    pub fn get_scsi_lun(&self) -> Option<&u32> {
        self.scsi_lun.as_ref()
    }

    /// Sets the value of ScsiPort
    pub fn set_scsi_port(&mut self, value: u32) {
        self.scsi_port = Some(value);
    }

    /// Gets the value of ScsiPort
    pub fn get_scsi_port(&self) -> Option<&u32> {
        self.scsi_port.as_ref()
    }

    /// Sets the value of ScsiTargetId
    pub fn set_scsi_target_id(&mut self, value: u32) {
        self.scsi_target_id = Some(value);
    }

    /// Gets the value of ScsiTargetId
    pub fn get_scsi_target_id(&self) -> Option<&u32> {
        self.scsi_target_id.as_ref()
    }

    /// Sets the value of Signature
    pub fn set_signature(&mut self, value: u32) {
        self.signature = Some(value);
    }

    /// Gets the value of Signature
    pub fn get_signature(&self) -> Option<&u32> {
        self.signature.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of StoragePoolId
    pub fn set_storage_pool_id(&mut self, value: String) {
        self.storage_pool_id = Some(value);
    }

    /// Gets the value of StoragePoolId
    pub fn get_storage_pool_id(&self) -> Option<&String> {
        self.storage_pool_id.as_ref()
    }

    /// Sets the value of VirtualDiskId
    pub fn set_virtual_disk_id(&mut self, value: String) {
        self.virtual_disk_id = Some(value);
    }

    /// Gets the value of VirtualDiskId
    pub fn get_virtual_disk_id(&self) -> Option<&String> {
        self.virtual_disk_id.as_ref()
    }
}

