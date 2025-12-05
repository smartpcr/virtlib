// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualHardDiskSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualHardDiskSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BlockSize")]
    pub block_size: Option<u32>,

/// 
    #[serde(rename = "DataAlignment")]
    pub data_alignment: Option<u64>,

/// 
    #[serde(rename = "Format")]
    pub format: Option<u16>,

/// 
    #[serde(rename = "IsPmemCompatible")]
    pub is_pmem_compatible: Option<bool>,

/// 
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u32>,

/// 
    #[serde(rename = "MaxInternalSize")]
    pub max_internal_size: Option<u64>,

/// 
    #[serde(rename = "ParentIdentifier")]
    pub parent_identifier: Option<String>,

/// 
    #[serde(rename = "ParentPath")]
    pub parent_path: Option<String>,

/// 
    #[serde(rename = "ParentTimestamp")]
    pub parent_timestamp: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u32>,

/// 
    #[serde(rename = "PmemAddressAbstractionType")]
    pub pmem_address_abstraction_type: Option<u16>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,

/// 
    #[serde(rename = "VirtualDiskId")]
    pub virtual_disk_id: Option<String>,
}

impl Msvm_VirtualHardDiskSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            block_size: None,
            data_alignment: None,
            format: None,
            is_pmem_compatible: None,
            logical_sector_size: None,
            max_internal_size: None,
            parent_identifier: None,
            parent_path: None,
            parent_timestamp: None,
            path: None,
            physical_sector_size: None,
            pmem_address_abstraction_type: None,
            type: None,
            virtual_disk_id: None,
        }
    }


    /// Sets the value of BlockSize
    pub fn set_block_size(&mut self, value: u32) {
        self.block_size = Some(value);
    }

    /// Gets the value of BlockSize
    pub fn get_block_size(&self) -> Option<&u32> {
        self.block_size.as_ref()
    }

    /// Sets the value of DataAlignment
    pub fn set_data_alignment(&mut self, value: u64) {
        self.data_alignment = Some(value);
    }

    /// Gets the value of DataAlignment
    pub fn get_data_alignment(&self) -> Option<&u64> {
        self.data_alignment.as_ref()
    }

    /// Sets the value of Format
    pub fn set_format(&mut self, value: u16) {
        self.format = Some(value);
    }

    /// Gets the value of Format
    pub fn get_format(&self) -> Option<&u16> {
        self.format.as_ref()
    }

    /// Sets the value of IsPmemCompatible
    pub fn set_is_pmem_compatible(&mut self, value: bool) {
        self.is_pmem_compatible = Some(value);
    }

    /// Gets the value of IsPmemCompatible
    pub fn get_is_pmem_compatible(&self) -> Option<&bool> {
        self.is_pmem_compatible.as_ref()
    }

    /// Sets the value of LogicalSectorSize
    pub fn set_logical_sector_size(&mut self, value: u32) {
        self.logical_sector_size = Some(value);
    }

    /// Gets the value of LogicalSectorSize
    pub fn get_logical_sector_size(&self) -> Option<&u32> {
        self.logical_sector_size.as_ref()
    }

    /// Sets the value of MaxInternalSize
    pub fn set_max_internal_size(&mut self, value: u64) {
        self.max_internal_size = Some(value);
    }

    /// Gets the value of MaxInternalSize
    pub fn get_max_internal_size(&self) -> Option<&u64> {
        self.max_internal_size.as_ref()
    }

    /// Sets the value of ParentIdentifier
    pub fn set_parent_identifier(&mut self, value: String) {
        self.parent_identifier = Some(value);
    }

    /// Gets the value of ParentIdentifier
    pub fn get_parent_identifier(&self) -> Option<&String> {
        self.parent_identifier.as_ref()
    }

    /// Sets the value of ParentPath
    pub fn set_parent_path(&mut self, value: String) {
        self.parent_path = Some(value);
    }

    /// Gets the value of ParentPath
    pub fn get_parent_path(&self) -> Option<&String> {
        self.parent_path.as_ref()
    }

    /// Sets the value of ParentTimestamp
    pub fn set_parent_timestamp(&mut self, value: String) {
        self.parent_timestamp = Some(value);
    }

    /// Gets the value of ParentTimestamp
    pub fn get_parent_timestamp(&self) -> Option<&String> {
        self.parent_timestamp.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of PhysicalSectorSize
    pub fn set_physical_sector_size(&mut self, value: u32) {
        self.physical_sector_size = Some(value);
    }

    /// Gets the value of PhysicalSectorSize
    pub fn get_physical_sector_size(&self) -> Option<&u32> {
        self.physical_sector_size.as_ref()
    }

    /// Sets the value of PmemAddressAbstractionType
    pub fn set_pmem_address_abstraction_type(&mut self, value: u16) {
        self.pmem_address_abstraction_type = Some(value);
    }

    /// Gets the value of PmemAddressAbstractionType
    pub fn get_pmem_address_abstraction_type(&self) -> Option<&u16> {
        self.pmem_address_abstraction_type.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
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

