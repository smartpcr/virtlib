// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdDeviceLocateData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdDeviceLocateData {
    #[serde(flatten)]
    pub base: BcdDeviceData,

/// This provides the locate device type.
    #[serde(rename = "Type")]
    pub type: Option<BcdDeviceLocateData_Type>,
}

impl BcdDeviceLocateData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdDeviceData::new(),
            type: None,
        }
    }


    /// Sets the value of Type
    pub fn set_type(&mut self, value: BcdDeviceLocateData_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&BcdDeviceLocateData_Type> {
        self.type.as_ref()
    }
}

