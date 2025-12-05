// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSDiskDriver_Geometry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSDiskDriver_Geometry {
    #[serde(flatten)]
    pub base: MSDiskDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BytesPerSector")]
    pub bytes_per_sector: Option<u32>,

/// 
    #[serde(rename = "Cylinders")]
    pub cylinders: Option<i64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u32>,

/// 
    #[serde(rename = "SectorsPerTrack")]
    pub sectors_per_track: Option<u32>,

/// 
    #[serde(rename = "TracksPerCylinder")]
    pub tracks_per_cylinder: Option<u32>,
}

impl MSDiskDriver_Geometry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSDiskDriver::new(),
            active: None,
            bytes_per_sector: None,
            cylinders: None,
            instance_name: None,
            media_type: None,
            sectors_per_track: None,
            tracks_per_cylinder: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
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
    pub fn set_cylinders(&mut self, value: i64) {
        self.cylinders = Some(value);
    }

    /// Gets the value of Cylinders
    pub fn get_cylinders(&self) -> Option<&i64> {
        self.cylinders.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u32) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u32> {
        self.media_type.as_ref()
    }

    /// Sets the value of SectorsPerTrack
    pub fn set_sectors_per_track(&mut self, value: u32) {
        self.sectors_per_track = Some(value);
    }

    /// Gets the value of SectorsPerTrack
    pub fn get_sectors_per_track(&self) -> Option<&u32> {
        self.sectors_per_track.as_ref()
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

