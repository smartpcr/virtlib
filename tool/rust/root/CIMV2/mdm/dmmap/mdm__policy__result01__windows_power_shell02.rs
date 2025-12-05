// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_WindowsPowerShell02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_WindowsPowerShell02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TurnOnPowerShellScriptBlockLogging")]
    pub turn_on_power_shell_script_block_logging: Option<String>,
}

impl MDM_Policy_Result01_WindowsPowerShell02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            turn_on_power_shell_script_block_logging: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TurnOnPowerShellScriptBlockLogging
    pub fn set_turn_on_power_shell_script_block_logging(&mut self, value: String) {
        self.turn_on_power_shell_script_block_logging = Some(value);
    }

    /// Gets the value of TurnOnPowerShellScriptBlockLogging
    pub fn get_turn_on_power_shell_script_block_logging(&self) -> Option<&String> {
        self.turn_on_power_shell_script_block_logging.as_ref()
    }
}

