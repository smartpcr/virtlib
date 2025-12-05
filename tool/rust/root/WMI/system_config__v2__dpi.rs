// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_DPI struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_DPI {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "MachineDPI")]
    pub machine_dpi: Option<u32>,

/// 
    #[serde(rename = "UserDPI")]
    pub user_dpi: Option<u32>,
}

impl SystemConfig_V2_DPI {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            machine_dpi: None,
            user_dpi: None,
        }
    }


    /// Sets the value of MachineDPI
    pub fn set_machine_dpi(&mut self, value: u32) {
        self.machine_dpi = Some(value);
    }

    /// Gets the value of MachineDPI
    pub fn get_machine_dpi(&self) -> Option<&u32> {
        self.machine_dpi.as_ref()
    }

    /// Sets the value of UserDPI
    pub fn set_user_dpi(&mut self, value: u32) {
        self.user_dpi = Some(value);
    }

    /// Gets the value of UserDPI
    pub fn get_user_dpi(&self) -> Option<&u32> {
        self.user_dpi.as_ref()
    }
}

