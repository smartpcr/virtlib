// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEESCPrivacySettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEESCPrivacySettings {
    #[serde(flatten)]
    pub base: RSOP_IEPrivacySettings,

/// 
    #[serde(rename = "EscEnabled")]
    pub esc_enabled: Option<bool>,
}

impl RSOP_IEESCPrivacySettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_IEPrivacySettings::new(),
            esc_enabled: None,
        }
    }


    /// Sets the value of EscEnabled
    pub fn set_esc_enabled(&mut self, value: bool) {
        self.esc_enabled = Some(value);
    }

    /// Gets the value of EscEnabled
    pub fn get_esc_enabled(&self) -> Option<&bool> {
        self.esc_enabled.as_ref()
    }
}

