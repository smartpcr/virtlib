// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdDeviceLocateStringData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdDeviceLocateStringData {
    #[serde(flatten)]
    pub base: BcdDeviceLocateData,

/// This provides the locate device element.
    #[serde(rename = "Path")]
    pub path: Option<String>,
}

impl BcdDeviceLocateStringData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdDeviceLocateData::new(),
            path: None,
        }
    }


    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

