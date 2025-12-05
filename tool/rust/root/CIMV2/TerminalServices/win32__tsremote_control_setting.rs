// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSRemoteControlSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSRemoteControlSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "LevelOfControl")]
    pub level_of_control: Option<u32>,

/// 
    #[serde(rename = "PolicySourceLevelOfControl")]
    pub policy_source_level_of_control: Option<u32>,

/// 
    #[serde(rename = "RemoteControlPolicy")]
    pub remote_control_policy: Option<u32>,
}

impl Win32_TSRemoteControlSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            level_of_control: None,
            policy_source_level_of_control: None,
            remote_control_policy: None,
        }
    }


    /// Sets the value of LevelOfControl
    pub fn set_level_of_control(&mut self, value: u32) {
        self.level_of_control = Some(value);
    }

    /// Gets the value of LevelOfControl
    pub fn get_level_of_control(&self) -> Option<&u32> {
        self.level_of_control.as_ref()
    }

    /// Sets the value of PolicySourceLevelOfControl
    pub fn set_policy_source_level_of_control(&mut self, value: u32) {
        self.policy_source_level_of_control = Some(value);
    }

    /// Gets the value of PolicySourceLevelOfControl
    pub fn get_policy_source_level_of_control(&self) -> Option<&u32> {
        self.policy_source_level_of_control.as_ref()
    }

    /// Sets the value of RemoteControlPolicy
    pub fn set_remote_control_policy(&mut self, value: u32) {
        self.remote_control_policy = Some(value);
    }

    /// Gets the value of RemoteControlPolicy
    pub fn get_remote_control_policy(&self) -> Option<&u32> {
        self.remote_control_policy.as_ref()
    }

/// 

    /// * `level_of_control` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remote_control(&self, level_of_control: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LevelOfControl".to_string(), value: level_of_control.into() });
        self.invoke_method("RemoteControl", &args)

    }

}

