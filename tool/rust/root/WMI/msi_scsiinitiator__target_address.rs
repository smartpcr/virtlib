// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_TargetAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_TargetAddress {

/// 
    #[serde(rename = "OSBusNumber")]
    pub osbus_number: Option<u32>,

/// 
    #[serde(rename = "OSDeviceName")]
    pub osdevice_name: Option<String>,

/// 
    #[serde(rename = "OSLunNumber")]
    pub oslun_number: Option<u32>,

/// 
    #[serde(rename = "OSTargetNumber")]
    pub ostarget_number: Option<u32>,
}

impl MSiSCSIInitiator_TargetAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            osbus_number: None,
            osdevice_name: None,
            oslun_number: None,
            ostarget_number: None,
        }
    }


    /// Sets the value of OSBusNumber
    pub fn set_osbus_number(&mut self, value: u32) {
        self.osbus_number = Some(value);
    }

    /// Gets the value of OSBusNumber
    pub fn get_osbus_number(&self) -> Option<&u32> {
        self.osbus_number.as_ref()
    }

    /// Sets the value of OSDeviceName
    pub fn set_osdevice_name(&mut self, value: String) {
        self.osdevice_name = Some(value);
    }

    /// Gets the value of OSDeviceName
    pub fn get_osdevice_name(&self) -> Option<&String> {
        self.osdevice_name.as_ref()
    }

    /// Sets the value of OSLunNumber
    pub fn set_oslun_number(&mut self, value: u32) {
        self.oslun_number = Some(value);
    }

    /// Gets the value of OSLunNumber
    pub fn get_oslun_number(&self) -> Option<&u32> {
        self.oslun_number.as_ref()
    }

    /// Sets the value of OSTargetNumber
    pub fn set_ostarget_number(&mut self, value: u32) {
        self.ostarget_number = Some(value);
    }

    /// Gets the value of OSTargetNumber
    pub fn get_ostarget_number(&self) -> Option<&u32> {
        self.ostarget_number.as_ref()
    }
}

