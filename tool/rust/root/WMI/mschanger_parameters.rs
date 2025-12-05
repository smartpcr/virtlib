// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSChangerParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSChangerParameters {
    #[serde(flatten)]
    pub base: MSChangerDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MagazineSize")]
    pub magazine_size: Option<u32>,

/// 
    #[serde(rename = "NumberOfCleanerSlots")]
    pub number_of_cleaner_slots: Option<u32>,

/// 
    #[serde(rename = "NumberOfDoors")]
    pub number_of_doors: Option<u32>,

/// 
    #[serde(rename = "NumberOfDrives")]
    pub number_of_drives: Option<u32>,

/// 
    #[serde(rename = "NumberOfIEPorts")]
    pub number_of_ieports: Option<u32>,

/// 
    #[serde(rename = "NumberOfSlots")]
    pub number_of_slots: Option<u32>,

/// 
    #[serde(rename = "NumberOfTransports")]
    pub number_of_transports: Option<u32>,
}

impl MSChangerParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSChangerDriver::new(),
            active: None,
            instance_name: None,
            magazine_size: None,
            number_of_cleaner_slots: None,
            number_of_doors: None,
            number_of_drives: None,
            number_of_ieports: None,
            number_of_slots: None,
            number_of_transports: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MagazineSize
    pub fn set_magazine_size(&mut self, value: u32) {
        self.magazine_size = Some(value);
    }

    /// Gets the value of MagazineSize
    pub fn get_magazine_size(&self) -> Option<&u32> {
        self.magazine_size.as_ref()
    }

    /// Sets the value of NumberOfCleanerSlots
    pub fn set_number_of_cleaner_slots(&mut self, value: u32) {
        self.number_of_cleaner_slots = Some(value);
    }

    /// Gets the value of NumberOfCleanerSlots
    pub fn get_number_of_cleaner_slots(&self) -> Option<&u32> {
        self.number_of_cleaner_slots.as_ref()
    }

    /// Sets the value of NumberOfDoors
    pub fn set_number_of_doors(&mut self, value: u32) {
        self.number_of_doors = Some(value);
    }

    /// Gets the value of NumberOfDoors
    pub fn get_number_of_doors(&self) -> Option<&u32> {
        self.number_of_doors.as_ref()
    }

    /// Sets the value of NumberOfDrives
    pub fn set_number_of_drives(&mut self, value: u32) {
        self.number_of_drives = Some(value);
    }

    /// Gets the value of NumberOfDrives
    pub fn get_number_of_drives(&self) -> Option<&u32> {
        self.number_of_drives.as_ref()
    }

    /// Sets the value of NumberOfIEPorts
    pub fn set_number_of_ieports(&mut self, value: u32) {
        self.number_of_ieports = Some(value);
    }

    /// Gets the value of NumberOfIEPorts
    pub fn get_number_of_ieports(&self) -> Option<&u32> {
        self.number_of_ieports.as_ref()
    }

    /// Sets the value of NumberOfSlots
    pub fn set_number_of_slots(&mut self, value: u32) {
        self.number_of_slots = Some(value);
    }

    /// Gets the value of NumberOfSlots
    pub fn get_number_of_slots(&self) -> Option<&u32> {
        self.number_of_slots.as_ref()
    }

    /// Sets the value of NumberOfTransports
    pub fn set_number_of_transports(&mut self, value: u32) {
        self.number_of_transports = Some(value);
    }

    /// Gets the value of NumberOfTransports
    pub fn get_number_of_transports(&self) -> Option<&u32> {
        self.number_of_transports.as_ref()
    }
}

