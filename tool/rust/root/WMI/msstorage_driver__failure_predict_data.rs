// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_FailurePredictData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_FailurePredictData {
    #[serde(flatten)]
    pub base: MSStorageDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Length")]
    pub length: Option<u32>,

/// 
    #[serde(rename = "VendorSpecific")]
    pub vendor_specific: Vec<u8>,
}

impl MSStorageDriver_FailurePredictData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSStorageDriver::new(),
            active: None,
            instance_name: None,
            length: None,
            vendor_specific: Vec::new(),
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

    /// Sets the value of Length
    pub fn set_length(&mut self, value: u32) {
        self.length = Some(value);
    }

    /// Gets the value of Length
    pub fn get_length(&self) -> Option<&u32> {
        self.length.as_ref()
    }

    /// Sets the value of VendorSpecific
    pub fn set_vendor_specific(&mut self, value: Vec<u8>) {
        self.vendor_specific = value;
    }

    /// Gets the value of VendorSpecific
    pub fn get_vendor_specific(&self) -> &Vec<u8> {
        &self.vendor_specific
    }
}

