// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HBAScsiID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HBAScsiID {

/// 
    #[serde(rename = "OSDeviceName")]
    pub osdevice_name: Vec<u16>,

/// 
    #[serde(rename = "ScsiBusNumber")]
    pub scsi_bus_number: Option<u32>,

/// 
    #[serde(rename = "ScsiOSLun")]
    pub scsi_oslun: Option<u32>,

/// 
    #[serde(rename = "ScsiTargetNumber")]
    pub scsi_target_number: Option<u32>,
}

impl HBAScsiID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            osdevice_name: Vec::new(),
            scsi_bus_number: None,
            scsi_oslun: None,
            scsi_target_number: None,
        }
    }


    /// Sets the value of OSDeviceName
    pub fn set_osdevice_name(&mut self, value: Vec<u16>) {
        self.osdevice_name = value;
    }

    /// Gets the value of OSDeviceName
    pub fn get_osdevice_name(&self) -> &Vec<u16> {
        &self.osdevice_name
    }

    /// Sets the value of ScsiBusNumber
    pub fn set_scsi_bus_number(&mut self, value: u32) {
        self.scsi_bus_number = Some(value);
    }

    /// Gets the value of ScsiBusNumber
    pub fn get_scsi_bus_number(&self) -> Option<&u32> {
        self.scsi_bus_number.as_ref()
    }

    /// Sets the value of ScsiOSLun
    pub fn set_scsi_oslun(&mut self, value: u32) {
        self.scsi_oslun = Some(value);
    }

    /// Gets the value of ScsiOSLun
    pub fn get_scsi_oslun(&self) -> Option<&u32> {
        self.scsi_oslun.as_ref()
    }

    /// Sets the value of ScsiTargetNumber
    pub fn set_scsi_target_number(&mut self, value: u32) {
        self.scsi_target_number = Some(value);
    }

    /// Gets the value of ScsiTargetNumber
    pub fn get_scsi_target_number(&self) -> Option<&u32> {
        self.scsi_target_number.as_ref()
    }
}

