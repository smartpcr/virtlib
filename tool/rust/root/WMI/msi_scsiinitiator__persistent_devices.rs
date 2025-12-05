// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_PersistentDevices struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_PersistentDevices {

/// 
    #[serde(rename = "DevicePath")]
    pub device_path: Option<String>,
}

impl MSiSCSIInitiator_PersistentDevices {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_path: None,
        }
    }


    /// Sets the value of DevicePath
    pub fn set_device_path(&mut self, value: String) {
        self.device_path = Some(value);
    }

    /// Gets the value of DevicePath
    pub fn get_device_path(&self) -> Option<&String> {
        self.device_path.as_ref()
    }
}

