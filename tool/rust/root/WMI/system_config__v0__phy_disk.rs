// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V0_PhyDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V0_PhyDisk {
    #[serde(flatten)]
    pub base: SystemConfig_V0,

/// 
    #[serde(rename = "BootDriveLetter")]
    pub boot_drive_letter: Vec<char>,

/// 
    #[serde(rename = "BytesPerSector")]
    pub bytes_per_sector: Option<u32>,

/// 
    #[serde(rename = "Cylinders")]
    pub cylinders: Option<u64>,

/// 
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Vec<char>,

/// 
    #[serde(rename = "Pad")]
    pub pad: Option<u8>,

/// 
    #[serde(rename = "PartitionCount")]
    pub partition_count: Option<u32>,

/// 
    #[serde(rename = "SCSILun")]
    pub scsilun: Option<u32>,

/// 
    #[serde(rename = "SCSIPath")]
    pub scsipath: Option<u32>,

/// 
    #[serde(rename = "SCSIPort")]
    pub scsiport: Option<u32>,

/// 
    #[serde(rename = "SCSITarget")]
    pub scsitarget: Option<u32>,

/// 
    #[serde(rename = "SectorsPerTrack")]
    pub sectors_per_track: Option<u32>,

/// 
    #[serde(rename = "Spare")]
    pub spare: Vec<char>,

/// 
    #[serde(rename = "TracksPerCylinder")]
    pub tracks_per_cylinder: Option<u32>,

/// 
    #[serde(rename = "WriteCacheEnabled")]
    pub write_cache_enabled: Option<u8>,
}

impl SystemConfig_V0_PhyDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V0::new(),
            boot_drive_letter: Vec::new(),
            bytes_per_sector: None,
            cylinders: None,
            disk_number: None,
            manufacturer: Vec::new(),
            pad: None,
            partition_count: None,
            scsilun: None,
            scsipath: None,
            scsiport: None,
            scsitarget: None,
            sectors_per_track: None,
            spare: Vec::new(),
            tracks_per_cylinder: None,
            write_cache_enabled: None,
        }
    }


    /// Sets the value of BootDriveLetter
    pub fn set_boot_drive_letter(&mut self, value: Vec<char>) {
        self.boot_drive_letter = value;
    }

    /// Gets the value of BootDriveLetter
    pub fn get_boot_drive_letter(&self) -> &Vec<char> {
        &self.boot_drive_letter
    }

    /// Sets the value of BytesPerSector
    pub fn set_bytes_per_sector(&mut self, value: u32) {
        self.bytes_per_sector = Some(value);
    }

    /// Gets the value of BytesPerSector
    pub fn get_bytes_per_sector(&self) -> Option<&u32> {
        self.bytes_per_sector.as_ref()
    }

    /// Sets the value of Cylinders
    pub fn set_cylinders(&mut self, value: u64) {
        self.cylinders = Some(value);
    }

    /// Gets the value of Cylinders
    pub fn get_cylinders(&self) -> Option<&u64> {
        self.cylinders.as_ref()
    }

    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: Vec<char>) {
        self.manufacturer = value;
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> &Vec<char> {
        &self.manufacturer
    }

    /// Sets the value of Pad
    pub fn set_pad(&mut self, value: u8) {
        self.pad = Some(value);
    }

    /// Gets the value of Pad
    pub fn get_pad(&self) -> Option<&u8> {
        self.pad.as_ref()
    }

    /// Sets the value of PartitionCount
    pub fn set_partition_count(&mut self, value: u32) {
        self.partition_count = Some(value);
    }

    /// Gets the value of PartitionCount
    pub fn get_partition_count(&self) -> Option<&u32> {
        self.partition_count.as_ref()
    }

    /// Sets the value of SCSILun
    pub fn set_scsilun(&mut self, value: u32) {
        self.scsilun = Some(value);
    }

    /// Gets the value of SCSILun
    pub fn get_scsilun(&self) -> Option<&u32> {
        self.scsilun.as_ref()
    }

    /// Sets the value of SCSIPath
    pub fn set_scsipath(&mut self, value: u32) {
        self.scsipath = Some(value);
    }

    /// Gets the value of SCSIPath
    pub fn get_scsipath(&self) -> Option<&u32> {
        self.scsipath.as_ref()
    }

    /// Sets the value of SCSIPort
    pub fn set_scsiport(&mut self, value: u32) {
        self.scsiport = Some(value);
    }

    /// Gets the value of SCSIPort
    pub fn get_scsiport(&self) -> Option<&u32> {
        self.scsiport.as_ref()
    }

    /// Sets the value of SCSITarget
    pub fn set_scsitarget(&mut self, value: u32) {
        self.scsitarget = Some(value);
    }

    /// Gets the value of SCSITarget
    pub fn get_scsitarget(&self) -> Option<&u32> {
        self.scsitarget.as_ref()
    }

    /// Sets the value of SectorsPerTrack
    pub fn set_sectors_per_track(&mut self, value: u32) {
        self.sectors_per_track = Some(value);
    }

    /// Gets the value of SectorsPerTrack
    pub fn get_sectors_per_track(&self) -> Option<&u32> {
        self.sectors_per_track.as_ref()
    }

    /// Sets the value of Spare
    pub fn set_spare(&mut self, value: Vec<char>) {
        self.spare = value;
    }

    /// Gets the value of Spare
    pub fn get_spare(&self) -> &Vec<char> {
        &self.spare
    }

    /// Sets the value of TracksPerCylinder
    pub fn set_tracks_per_cylinder(&mut self, value: u32) {
        self.tracks_per_cylinder = Some(value);
    }

    /// Gets the value of TracksPerCylinder
    pub fn get_tracks_per_cylinder(&self) -> Option<&u32> {
        self.tracks_per_cylinder.as_ref()
    }

    /// Sets the value of WriteCacheEnabled
    pub fn set_write_cache_enabled(&mut self, value: u8) {
        self.write_cache_enabled = Some(value);
    }

    /// Gets the value of WriteCacheEnabled
    pub fn get_write_cache_enabled(&self) -> Option<&u8> {
        self.write_cache_enabled.as_ref()
    }
}

