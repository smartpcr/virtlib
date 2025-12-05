// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdDeviceData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdDeviceData {

/// This represents the additional options for the element.
    #[serde(rename = "AdditionalOptions")]
    pub additional_options: Option<String>,

/// This identifies the type of device element. This value dictates whether this is a file device element or a partition device element.
    #[serde(rename = "DeviceType")]
    pub device_type: Option<BcdDeviceData_DeviceType>,
}

impl BcdDeviceData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            additional_options: None,
            device_type: None,
        }
    }


    /// Sets the value of AdditionalOptions
    pub fn set_additional_options(&mut self, value: String) {
        self.additional_options = Some(value);
    }

    /// Gets the value of AdditionalOptions
    pub fn get_additional_options(&self) -> Option<&String> {
        self.additional_options.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: BcdDeviceData_DeviceType) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&BcdDeviceData_DeviceType> {
        self.device_type.as_ref()
    }
}

