// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_OpticalMedia struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_OpticalMedia {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "BusType")]
    pub bus_type: Option<u16>,

/// 
    #[serde(rename = "BytesPerSector")]
    pub bytes_per_sector: Option<u32>,

/// 
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u16>,

/// 
    #[serde(rename = "DiscStatus")]
    pub disc_status: Option<u16>,

/// 
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u16>,

/// 
    #[serde(rename = "DriveLetter")]
    pub drive_letter: Option<String>,

/// 
    #[serde(rename = "FileSystemName")]
    pub file_system_name: Option<String>,

/// 
    #[serde(rename = "LastSessionStatus")]
    pub last_session_status: Option<u16>,

/// 
    #[serde(rename = "ManufacturerName")]
    pub manufacturer_name: Option<String>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u16>,

/// 
    #[serde(rename = "NextWritableAddress")]
    pub next_writable_address: Option<u64>,

/// 
    #[serde(rename = "NumberOfFreeBlocks")]
    pub number_of_free_blocks: Option<u64>,

/// 
    #[serde(rename = "NumberOfSessions")]
    pub number_of_sessions: Option<u32>,

/// 
    #[serde(rename = "NumberOfTracks")]
    pub number_of_tracks: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "StartingOffset")]
    pub starting_offset: Option<u64>,

/// 
    #[serde(rename = "TotalNumberOfBlocks")]
    pub total_number_of_blocks: Option<u64>,
}

impl SystemConfig_V2_OpticalMedia {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            bus_type: None,
            bytes_per_sector: None,
            device_name: None,
            device_type: None,
            disc_status: None,
            disk_number: None,
            drive_letter: None,
            file_system_name: None,
            last_session_status: None,
            manufacturer_name: None,
            media_type: None,
            next_writable_address: None,
            number_of_free_blocks: None,
            number_of_sessions: None,
            number_of_tracks: None,
            size: None,
            starting_offset: None,
            total_number_of_blocks: None,
        }
    }


    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: u16) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&u16> {
        self.bus_type.as_ref()
    }

    /// Sets the value of BytesPerSector
    pub fn set_bytes_per_sector(&mut self, value: u32) {
        self.bytes_per_sector = Some(value);
    }

    /// Gets the value of BytesPerSector
    pub fn get_bytes_per_sector(&self) -> Option<&u32> {
        self.bytes_per_sector.as_ref()
    }

    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u16) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u16> {
        self.device_type.as_ref()
    }

    /// Sets the value of DiscStatus
    pub fn set_disc_status(&mut self, value: u16) {
        self.disc_status = Some(value);
    }

    /// Gets the value of DiscStatus
    pub fn get_disc_status(&self) -> Option<&u16> {
        self.disc_status.as_ref()
    }

    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u16) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u16> {
        self.disk_number.as_ref()
    }

    /// Sets the value of DriveLetter
    pub fn set_drive_letter(&mut self, value: String) {
        self.drive_letter = Some(value);
    }

    /// Gets the value of DriveLetter
    pub fn get_drive_letter(&self) -> Option<&String> {
        self.drive_letter.as_ref()
    }

    /// Sets the value of FileSystemName
    pub fn set_file_system_name(&mut self, value: String) {
        self.file_system_name = Some(value);
    }

    /// Gets the value of FileSystemName
    pub fn get_file_system_name(&self) -> Option<&String> {
        self.file_system_name.as_ref()
    }

    /// Sets the value of LastSessionStatus
    pub fn set_last_session_status(&mut self, value: u16) {
        self.last_session_status = Some(value);
    }

    /// Gets the value of LastSessionStatus
    pub fn get_last_session_status(&self) -> Option<&u16> {
        self.last_session_status.as_ref()
    }

    /// Sets the value of ManufacturerName
    pub fn set_manufacturer_name(&mut self, value: String) {
        self.manufacturer_name = Some(value);
    }

    /// Gets the value of ManufacturerName
    pub fn get_manufacturer_name(&self) -> Option<&String> {
        self.manufacturer_name.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u16) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u16> {
        self.media_type.as_ref()
    }

    /// Sets the value of NextWritableAddress
    pub fn set_next_writable_address(&mut self, value: u64) {
        self.next_writable_address = Some(value);
    }

    /// Gets the value of NextWritableAddress
    pub fn get_next_writable_address(&self) -> Option<&u64> {
        self.next_writable_address.as_ref()
    }

    /// Sets the value of NumberOfFreeBlocks
    pub fn set_number_of_free_blocks(&mut self, value: u64) {
        self.number_of_free_blocks = Some(value);
    }

    /// Gets the value of NumberOfFreeBlocks
    pub fn get_number_of_free_blocks(&self) -> Option<&u64> {
        self.number_of_free_blocks.as_ref()
    }

    /// Sets the value of NumberOfSessions
    pub fn set_number_of_sessions(&mut self, value: u32) {
        self.number_of_sessions = Some(value);
    }

    /// Gets the value of NumberOfSessions
    pub fn get_number_of_sessions(&self) -> Option<&u32> {
        self.number_of_sessions.as_ref()
    }

    /// Sets the value of NumberOfTracks
    pub fn set_number_of_tracks(&mut self, value: u32) {
        self.number_of_tracks = Some(value);
    }

    /// Gets the value of NumberOfTracks
    pub fn get_number_of_tracks(&self) -> Option<&u32> {
        self.number_of_tracks.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of StartingOffset
    pub fn set_starting_offset(&mut self, value: u64) {
        self.starting_offset = Some(value);
    }

    /// Gets the value of StartingOffset
    pub fn get_starting_offset(&self) -> Option<&u64> {
        self.starting_offset.as_ref()
    }

    /// Sets the value of TotalNumberOfBlocks
    pub fn set_total_number_of_blocks(&mut self, value: u64) {
        self.total_number_of_blocks = Some(value);
    }

    /// Gets the value of TotalNumberOfBlocks
    pub fn get_total_number_of_blocks(&self) -> Option<&u64> {
        self.total_number_of_blocks.as_ref()
    }
}

