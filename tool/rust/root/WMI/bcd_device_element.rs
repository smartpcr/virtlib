// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdDeviceElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdDeviceElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This field contains information about the device.
    #[serde(rename = "Device")]
    pub device: Option<BcdDeviceData>,
}

impl BcdDeviceElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            device: None,
        }
    }


    /// Sets the value of Device
    pub fn set_device(&mut self, value: BcdDeviceData) {
        self.device = Some(value);
    }

    /// Gets the value of Device
    pub fn get_device(&self) -> Option<&BcdDeviceData> {
        self.device.as_ref()
    }
}

