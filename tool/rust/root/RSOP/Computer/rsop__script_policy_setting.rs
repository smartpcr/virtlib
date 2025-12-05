// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_ScriptPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_ScriptPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "psScriptOrder")]
    pub ps_script_order: Option<u32>,

/// 
    #[serde(rename = "scriptList")]
    pub script_list: Vec<RSOP_ScriptCmd>,

/// 
    #[serde(rename = "scriptOrder")]
    pub script_order: Option<u32>,

/// 
    #[serde(rename = "scriptType")]
    pub script_type: Option<u32>,
}

impl RSOP_ScriptPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            ps_script_order: None,
            script_list: Vec::new(),
            script_order: None,
            script_type: None,
        }
    }


    /// Sets the value of psScriptOrder
    pub fn set_ps_script_order(&mut self, value: u32) {
        self.ps_script_order = Some(value);
    }

    /// Gets the value of psScriptOrder
    pub fn get_ps_script_order(&self) -> Option<&u32> {
        self.ps_script_order.as_ref()
    }

    /// Sets the value of scriptList
    pub fn set_script_list(&mut self, value: Vec<RSOP_ScriptCmd>) {
        self.script_list = value;
    }

    /// Gets the value of scriptList
    pub fn get_script_list(&self) -> &Vec<RSOP_ScriptCmd> {
        &self.script_list
    }

    /// Sets the value of scriptOrder
    pub fn set_script_order(&mut self, value: u32) {
        self.script_order = Some(value);
    }

    /// Gets the value of scriptOrder
    pub fn get_script_order(&self) -> Option<&u32> {
        self.script_order.as_ref()
    }

    /// Sets the value of scriptType
    pub fn set_script_type(&mut self, value: u32) {
        self.script_type = Some(value);
    }

    /// Gets the value of scriptType
    pub fn get_script_type(&self) -> Option<&u32> {
        self.script_type.as_ref()
    }
}

