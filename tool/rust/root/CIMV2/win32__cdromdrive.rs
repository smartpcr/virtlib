// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CDROMDrive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CDROMDrive {
    #[serde(flatten)]
    pub base: CIM_CDROMDrive,

/// 
    #[serde(rename = "Drive")]
    pub drive: Option<String>,

/// 
    #[serde(rename = "DriveIntegrity")]
    pub drive_integrity: Option<bool>,

/// 
    #[serde(rename = "FileSystemFlags")]
    pub file_system_flags: Option<u16>,

/// 
    #[serde(rename = "FileSystemFlagsEx")]
    pub file_system_flags_ex: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MaximumComponentLength")]
    pub maximum_component_length: Option<u32>,

/// 
    #[serde(rename = "MediaLoaded")]
    pub media_loaded: Option<bool>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,

/// 
    #[serde(rename = "MfrAssignedRevisionLevel")]
    pub mfr_assigned_revision_level: Option<String>,

/// 
    #[serde(rename = "RevisionLevel")]
    pub revision_level: Option<String>,

/// 
    #[serde(rename = "SCSIBus")]
    pub scsibus: Option<u32>,

/// 
    #[serde(rename = "SCSILogicalUnit")]
    pub scsilogical_unit: Option<u16>,

/// 
    #[serde(rename = "SCSIPort")]
    pub scsiport: Option<u16>,

/// 
    #[serde(rename = "SCSITargetId")]
    pub scsitarget_id: Option<u16>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "TransferRate")]
    pub transfer_rate: Option<f64>,

/// 
    #[serde(rename = "VolumeName")]
    pub volume_name: Option<String>,

/// 
    #[serde(rename = "VolumeSerialNumber")]
    pub volume_serial_number: Option<String>,
}

impl Win32_CDROMDrive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_CDROMDrive::new(),
            drive: None,
            drive_integrity: None,
            file_system_flags: None,
            file_system_flags_ex: None,
            id: None,
            manufacturer: None,
            maximum_component_length: None,
            media_loaded: None,
            media_type: None,
            mfr_assigned_revision_level: None,
            revision_level: None,
            scsibus: None,
            scsilogical_unit: None,
            scsiport: None,
            scsitarget_id: None,
            serial_number: None,
            size: None,
            transfer_rate: None,
            volume_name: None,
            volume_serial_number: None,
        }
    }


    /// Sets the value of Drive
    pub fn set_drive(&mut self, value: String) {
        self.drive = Some(value);
    }

    /// Gets the value of Drive
    pub fn get_drive(&self) -> Option<&String> {
        self.drive.as_ref()
    }

    /// Sets the value of DriveIntegrity
    pub fn set_drive_integrity(&mut self, value: bool) {
        self.drive_integrity = Some(value);
    }

    /// Gets the value of DriveIntegrity
    pub fn get_drive_integrity(&self) -> Option<&bool> {
        self.drive_integrity.as_ref()
    }

    /// Sets the value of FileSystemFlags
    pub fn set_file_system_flags(&mut self, value: u16) {
        self.file_system_flags = Some(value);
    }

    /// Gets the value of FileSystemFlags
    pub fn get_file_system_flags(&self) -> Option<&u16> {
        self.file_system_flags.as_ref()
    }

    /// Sets the value of FileSystemFlagsEx
    pub fn set_file_system_flags_ex(&mut self, value: u32) {
        self.file_system_flags_ex = Some(value);
    }

    /// Gets the value of FileSystemFlagsEx
    pub fn get_file_system_flags_ex(&self) -> Option<&u32> {
        self.file_system_flags_ex.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MaximumComponentLength
    pub fn set_maximum_component_length(&mut self, value: u32) {
        self.maximum_component_length = Some(value);
    }

    /// Gets the value of MaximumComponentLength
    pub fn get_maximum_component_length(&self) -> Option<&u32> {
        self.maximum_component_length.as_ref()
    }

    /// Sets the value of MediaLoaded
    pub fn set_media_loaded(&mut self, value: bool) {
        self.media_loaded = Some(value);
    }

    /// Gets the value of MediaLoaded
    pub fn get_media_loaded(&self) -> Option<&bool> {
        self.media_loaded.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: String) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&String> {
        self.media_type.as_ref()
    }

    /// Sets the value of MfrAssignedRevisionLevel
    pub fn set_mfr_assigned_revision_level(&mut self, value: String) {
        self.mfr_assigned_revision_level = Some(value);
    }

    /// Gets the value of MfrAssignedRevisionLevel
    pub fn get_mfr_assigned_revision_level(&self) -> Option<&String> {
        self.mfr_assigned_revision_level.as_ref()
    }

    /// Sets the value of RevisionLevel
    pub fn set_revision_level(&mut self, value: String) {
        self.revision_level = Some(value);
    }

    /// Gets the value of RevisionLevel
    pub fn get_revision_level(&self) -> Option<&String> {
        self.revision_level.as_ref()
    }

    /// Sets the value of SCSIBus
    pub fn set_scsibus(&mut self, value: u32) {
        self.scsibus = Some(value);
    }

    /// Gets the value of SCSIBus
    pub fn get_scsibus(&self) -> Option<&u32> {
        self.scsibus.as_ref()
    }

    /// Sets the value of SCSILogicalUnit
    pub fn set_scsilogical_unit(&mut self, value: u16) {
        self.scsilogical_unit = Some(value);
    }

    /// Gets the value of SCSILogicalUnit
    pub fn get_scsilogical_unit(&self) -> Option<&u16> {
        self.scsilogical_unit.as_ref()
    }

    /// Sets the value of SCSIPort
    pub fn set_scsiport(&mut self, value: u16) {
        self.scsiport = Some(value);
    }

    /// Gets the value of SCSIPort
    pub fn get_scsiport(&self) -> Option<&u16> {
        self.scsiport.as_ref()
    }

    /// Sets the value of SCSITargetId
    pub fn set_scsitarget_id(&mut self, value: u16) {
        self.scsitarget_id = Some(value);
    }

    /// Gets the value of SCSITargetId
    pub fn get_scsitarget_id(&self) -> Option<&u16> {
        self.scsitarget_id.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of TransferRate
    pub fn set_transfer_rate(&mut self, value: f64) {
        self.transfer_rate = Some(value);
    }

    /// Gets the value of TransferRate
    pub fn get_transfer_rate(&self) -> Option<&f64> {
        self.transfer_rate.as_ref()
    }

    /// Sets the value of VolumeName
    pub fn set_volume_name(&mut self, value: String) {
        self.volume_name = Some(value);
    }

    /// Gets the value of VolumeName
    pub fn get_volume_name(&self) -> Option<&String> {
        self.volume_name.as_ref()
    }

    /// Sets the value of VolumeSerialNumber
    pub fn set_volume_serial_number(&mut self, value: String) {
        self.volume_serial_number = Some(value);
    }

    /// Gets the value of VolumeSerialNumber
    pub fn get_volume_serial_number(&self) -> Option<&String> {
        self.volume_serial_number.as_ref()
    }
}

