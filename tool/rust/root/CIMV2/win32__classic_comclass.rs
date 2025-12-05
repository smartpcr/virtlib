// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ClassicCOMClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ClassicCOMClass {
    #[serde(flatten)]
    pub base: Win32_COMClass,

/// 
    #[serde(rename = "ComponentId")]
    pub component_id: Option<String>,
}

impl Win32_ClassicCOMClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_COMClass::new(),
            component_id: None,
        }
    }


    /// Sets the value of ComponentId
    pub fn set_component_id(&mut self, value: String) {
        self.component_id = Some(value);
    }

    /// Gets the value of ComponentId
    pub fn get_component_id(&self) -> Option<&String> {
        self.component_id.as_ref()
    }
}

