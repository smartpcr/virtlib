// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PhysicalExtent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PhysicalExtent {

/// The column number associated with this extent.
    #[serde(rename = "ColumnNumber")]
    pub column_number: Option<u16>,

/// The copy number associated with this extent.
    #[serde(rename = "CopyNumber")]
    pub copy_number: Option<u16>,

/// The flags associated with this extent.
    #[serde(rename = "Flags")]
    pub flags: Option<u64>,

/// The operational details associated with this extent.
    #[serde(rename = "OperationalDetails")]
    pub operational_details: Vec<String>,

/// The operational statuses associated with this extent.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// The byte offset of this extent from the start of the physical disk.
    #[serde(rename = "PhysicalDiskOffset")]
    pub physical_disk_offset: Option<u64>,

/// The unique id of the physical disk associated with this extent.
    #[serde(rename = "PhysicalDiskUniqueId")]
    pub physical_disk_unique_id: Option<String>,

/// The copy number of the replacement for this extent.
    #[serde(rename = "ReplacementCopyNumber")]
    pub replacement_copy_number: Option<u16>,

/// The size of this extent in bytes.
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// The unique id of the storage tier associated with this extent.
    #[serde(rename = "StorageTierUniqueId")]
    pub storage_tier_unique_id: Option<String>,

/// The byte offset of this extent from the start of the virtual disk.
    #[serde(rename = "VirtualDiskOffset")]
    pub virtual_disk_offset: Option<u64>,

/// The unique id of the virtual disk associated with this extent.
    #[serde(rename = "VirtualDiskUniqueId")]
    pub virtual_disk_unique_id: Option<String>,
}

impl MSFT_PhysicalExtent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            column_number: None,
            copy_number: None,
            flags: None,
            operational_details: Vec::new(),
            operational_status: Vec::new(),
            physical_disk_offset: None,
            physical_disk_unique_id: None,
            replacement_copy_number: None,
            size: None,
            storage_tier_unique_id: None,
            virtual_disk_offset: None,
            virtual_disk_unique_id: None,
        }
    }


    /// Sets the value of ColumnNumber
    pub fn set_column_number(&mut self, value: u16) {
        self.column_number = Some(value);
    }

    /// Gets the value of ColumnNumber
    pub fn get_column_number(&self) -> Option<&u16> {
        self.column_number.as_ref()
    }

    /// Sets the value of CopyNumber
    pub fn set_copy_number(&mut self, value: u16) {
        self.copy_number = Some(value);
    }

    /// Gets the value of CopyNumber
    pub fn get_copy_number(&self) -> Option<&u16> {
        self.copy_number.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u64) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u64> {
        self.flags.as_ref()
    }

    /// Sets the value of OperationalDetails
    pub fn set_operational_details(&mut self, value: Vec<String>) {
        self.operational_details = value;
    }

    /// Gets the value of OperationalDetails
    pub fn get_operational_details(&self) -> &Vec<String> {
        &self.operational_details
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of PhysicalDiskOffset
    pub fn set_physical_disk_offset(&mut self, value: u64) {
        self.physical_disk_offset = Some(value);
    }

    /// Gets the value of PhysicalDiskOffset
    pub fn get_physical_disk_offset(&self) -> Option<&u64> {
        self.physical_disk_offset.as_ref()
    }

    /// Sets the value of PhysicalDiskUniqueId
    pub fn set_physical_disk_unique_id(&mut self, value: String) {
        self.physical_disk_unique_id = Some(value);
    }

    /// Gets the value of PhysicalDiskUniqueId
    pub fn get_physical_disk_unique_id(&self) -> Option<&String> {
        self.physical_disk_unique_id.as_ref()
    }

    /// Sets the value of ReplacementCopyNumber
    pub fn set_replacement_copy_number(&mut self, value: u16) {
        self.replacement_copy_number = Some(value);
    }

    /// Gets the value of ReplacementCopyNumber
    pub fn get_replacement_copy_number(&self) -> Option<&u16> {
        self.replacement_copy_number.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of StorageTierUniqueId
    pub fn set_storage_tier_unique_id(&mut self, value: String) {
        self.storage_tier_unique_id = Some(value);
    }

    /// Gets the value of StorageTierUniqueId
    pub fn get_storage_tier_unique_id(&self) -> Option<&String> {
        self.storage_tier_unique_id.as_ref()
    }

    /// Sets the value of VirtualDiskOffset
    pub fn set_virtual_disk_offset(&mut self, value: u64) {
        self.virtual_disk_offset = Some(value);
    }

    /// Gets the value of VirtualDiskOffset
    pub fn get_virtual_disk_offset(&self) -> Option<&u64> {
        self.virtual_disk_offset.as_ref()
    }

    /// Sets the value of VirtualDiskUniqueId
    pub fn set_virtual_disk_unique_id(&mut self, value: String) {
        self.virtual_disk_unique_id = Some(value);
    }

    /// Gets the value of VirtualDiskUniqueId
    pub fn get_virtual_disk_unique_id(&self) -> Option<&String> {
        self.virtual_disk_unique_id.as_ref()
    }
}

