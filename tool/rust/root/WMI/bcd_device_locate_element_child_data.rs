// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdDeviceLocateElementChildData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdDeviceLocateElementChildData {
    #[serde(flatten)]
    pub base: BcdDeviceLocateData,

/// This provides the locate device element.
    #[serde(rename = "Element")]
    pub element: Option<u32>,

/// This is the parent device of this locate device.
    #[serde(rename = "Parent")]
    pub parent: Option<BcdDeviceData>,
}

impl BcdDeviceLocateElementChildData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdDeviceLocateData::new(),
            element: None,
            parent: None,
        }
    }


    /// Sets the value of Element
    pub fn set_element(&mut self, value: u32) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&u32> {
        self.element.as_ref()
    }

    /// Sets the value of Parent
    pub fn set_parent(&mut self, value: BcdDeviceData) {
        self.parent = Some(value);
    }

    /// Gets the value of Parent
    pub fn get_parent(&self) -> Option<&BcdDeviceData> {
        self.parent.as_ref()
    }
}

