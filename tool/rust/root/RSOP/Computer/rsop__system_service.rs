// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_SystemService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_SystemService {
    #[serde(flatten)]
    pub base: RSOP_SecuritySettings,

/// 
    #[serde(rename = "SDDLString")]
    pub sddlstring: Option<String>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,

/// 
    #[serde(rename = "StartupMode")]
    pub startup_mode: Option<SystemService_StartupMode>,
}

impl RSOP_SystemService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_SecuritySettings::new(),
            sddlstring: None,
            service: None,
            startup_mode: None,
        }
    }


    /// Sets the value of SDDLString
    pub fn set_sddlstring(&mut self, value: String) {
        self.sddlstring = Some(value);
    }

    /// Gets the value of SDDLString
    pub fn get_sddlstring(&self) -> Option<&String> {
        self.sddlstring.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }

    /// Sets the value of StartupMode
    pub fn set_startup_mode(&mut self, value: SystemService_StartupMode) {
        self.startup_mode = Some(value);
    }

    /// Gets the value of StartupMode
    pub fn get_startup_mode(&self) -> Option<&SystemService_StartupMode> {
        self.startup_mode.as_ref()
    }
}

