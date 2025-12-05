// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DCOMApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DCOMApplication {
    #[serde(flatten)]
    pub base: Win32_COMApplication,

/// 
    #[serde(rename = "AppID")]
    pub app_id: Option<String>,
}

impl Win32_DCOMApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_COMApplication::new(),
            app_id: None,
        }
    }


    /// Sets the value of AppID
    pub fn set_app_id(&mut self, value: String) {
        self.app_id = Some(value);
    }

    /// Gets the value of AppID
    pub fn get_app_id(&self) -> Option<&String> {
        self.app_id.as_ref()
    }
}

