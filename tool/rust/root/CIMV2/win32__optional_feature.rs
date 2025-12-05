// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OptionalFeature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OptionalFeature {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "InstallState")]
    pub install_state: Option<u32>,
}

impl Win32_OptionalFeature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            install_state: None,
        }
    }


    /// Sets the value of InstallState
    pub fn set_install_state(&mut self, value: u32) {
        self.install_state = Some(value);
    }

    /// Gets the value of InstallState
    pub fn get_install_state(&self) -> Option<&u32> {
        self.install_state.as_ref()
    }
}

