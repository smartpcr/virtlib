// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_TargetMappings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_TargetMappings {

/// 
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,

/// 
    #[serde(rename = "LUNList")]
    pub lunlist: Vec<MSiSCSIInitiator_LUNList>,

/// 
    #[serde(rename = "OSBusNumber")]
    pub osbus_number: Option<u32>,

/// 
    #[serde(rename = "OSDeviceName")]
    pub osdevice_name: Option<String>,

/// 
    #[serde(rename = "OSTargetNumber")]
    pub ostarget_number: Option<u32>,

/// 
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,
}

impl MSiSCSIInitiator_TargetMappings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_name: None,
            lunlist: Vec::new(),
            osbus_number: None,
            osdevice_name: None,
            ostarget_number: None,
            target_name: None,
        }
    }


    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }

    /// Sets the value of LUNList
    pub fn set_lunlist(&mut self, value: Vec<MSiSCSIInitiator_LUNList>) {
        self.lunlist = value;
    }

    /// Gets the value of LUNList
    pub fn get_lunlist(&self) -> &Vec<MSiSCSIInitiator_LUNList> {
        &self.lunlist
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

    /// Sets the value of OSTargetNumber
    pub fn set_ostarget_number(&mut self, value: u32) {
        self.ostarget_number = Some(value);
    }

    /// Gets the value of OSTargetNumber
    pub fn get_ostarget_number(&self) -> Option<&u32> {
        self.ostarget_number.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }
}

