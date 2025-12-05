// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DiskDrive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DiskDrive {
    #[serde(flatten)]
    pub base: CIM_DiskDrive,

/// 
    #[serde(rename = "BytesPerSector")]
    pub bytes_per_sector: Option<u32>,

/// 
    #[serde(rename = "FirmwareRevision")]
    pub firmware_revision: Option<String>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "InterfaceType")]
    pub interface_type: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MediaLoaded")]
    pub media_loaded: Option<bool>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "Partitions")]
    pub partitions: Option<u32>,

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
    #[serde(rename = "SectorsPerTrack")]
    pub sectors_per_track: Option<u32>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "Signature")]
    pub signature: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "TotalCylinders")]
    pub total_cylinders: Option<u64>,

/// 
    #[serde(rename = "TotalHeads")]
    pub total_heads: Option<u32>,

/// 
    #[serde(rename = "TotalSectors")]
    pub total_sectors: Option<u64>,

/// 
    #[serde(rename = "TotalTracks")]
    pub total_tracks: Option<u64>,

/// 
    #[serde(rename = "TracksPerCylinder")]
    pub tracks_per_cylinder: Option<u32>,
}

impl Win32_DiskDrive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DiskDrive::new(),
            bytes_per_sector: None,
            firmware_revision: None,
            index: None,
            interface_type: None,
            manufacturer: None,
            media_loaded: None,
            media_type: None,
            model: None,
            partitions: None,
            scsibus: None,
            scsilogical_unit: None,
            scsiport: None,
            scsitarget_id: None,
            sectors_per_track: None,
            serial_number: None,
            signature: None,
            size: None,
            total_cylinders: None,
            total_heads: None,
            total_sectors: None,
            total_tracks: None,
            tracks_per_cylinder: None,
        }
    }


    /// Sets the value of BytesPerSector
    pub fn set_bytes_per_sector(&mut self, value: u32) {
        self.bytes_per_sector = Some(value);
    }

    /// Gets the value of BytesPerSector
    pub fn get_bytes_per_sector(&self) -> Option<&u32> {
        self.bytes_per_sector.as_ref()
    }

    /// Sets the value of FirmwareRevision
    pub fn set_firmware_revision(&mut self, value: String) {
        self.firmware_revision = Some(value);
    }

    /// Gets the value of FirmwareRevision
    pub fn get_firmware_revision(&self) -> Option<&String> {
        self.firmware_revision.as_ref()
    }

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of InterfaceType
    pub fn set_interface_type(&mut self, value: String) {
        self.interface_type = Some(value);
    }

    /// Gets the value of InterfaceType
    pub fn get_interface_type(&self) -> Option<&String> {
        self.interface_type.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
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

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of Partitions
    pub fn set_partitions(&mut self, value: u32) {
        self.partitions = Some(value);
    }

    /// Gets the value of Partitions
    pub fn get_partitions(&self) -> Option<&u32> {
        self.partitions.as_ref()
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

    /// Sets the value of SectorsPerTrack
    pub fn set_sectors_per_track(&mut self, value: u32) {
        self.sectors_per_track = Some(value);
    }

    /// Gets the value of SectorsPerTrack
    pub fn get_sectors_per_track(&self) -> Option<&u32> {
        self.sectors_per_track.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
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

    /// Sets the value of TotalCylinders
    pub fn set_total_cylinders(&mut self, value: u64) {
        self.total_cylinders = Some(value);
    }

    /// Gets the value of TotalCylinders
    pub fn get_total_cylinders(&self) -> Option<&u64> {
        self.total_cylinders.as_ref()
    }

    /// Sets the value of TotalHeads
    pub fn set_total_heads(&mut self, value: u32) {
        self.total_heads = Some(value);
    }

    /// Gets the value of TotalHeads
    pub fn get_total_heads(&self) -> Option<&u32> {
        self.total_heads.as_ref()
    }

    /// Sets the value of TotalSectors
    pub fn set_total_sectors(&mut self, value: u64) {
        self.total_sectors = Some(value);
    }

    /// Gets the value of TotalSectors
    pub fn get_total_sectors(&self) -> Option<&u64> {
        self.total_sectors.as_ref()
    }

    /// Sets the value of TotalTracks
    pub fn set_total_tracks(&mut self, value: u64) {
        self.total_tracks = Some(value);
    }

    /// Gets the value of TotalTracks
    pub fn get_total_tracks(&self) -> Option<&u64> {
        self.total_tracks.as_ref()
    }

    /// Sets the value of TracksPerCylinder
    pub fn set_tracks_per_cylinder(&mut self, value: u32) {
        self.tracks_per_cylinder = Some(value);
    }

    /// Gets the value of TracksPerCylinder
    pub fn get_tracks_per_cylinder(&self) -> Option<&u32> {
        self.tracks_per_cylinder.as_ref()
    }
}

