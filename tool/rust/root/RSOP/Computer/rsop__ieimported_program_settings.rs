// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEImportedProgramSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEImportedProgramSettings {

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,

/// 
    #[serde(rename = "programSettings")]
    pub program_settings: Option<RSOP_IEProgramSettings>,
}

impl RSOP_IEImportedProgramSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            policy_setting: None,
            program_settings: None,
        }
    }


    /// Sets the value of policySetting
    pub fn set_policy_setting(&mut self, value: RSOP_IEAKPolicySetting) {
        self.policy_setting = Some(value);
    }

    /// Gets the value of policySetting
    pub fn get_policy_setting(&self) -> Option<&RSOP_IEAKPolicySetting> {
        self.policy_setting.as_ref()
    }

    /// Sets the value of programSettings
    pub fn set_program_settings(&mut self, value: RSOP_IEProgramSettings) {
        self.program_settings = Some(value);
    }

    /// Gets the value of programSettings
    pub fn get_program_settings(&self) -> Option<&RSOP_IEProgramSettings> {
        self.program_settings.as_ref()
    }
}

