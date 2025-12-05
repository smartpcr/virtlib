// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Bid2Etw_RowsetHelper_1_Trace_TextA struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bid2Etw_RowsetHelper_1_Trace_TextA {
    #[serde(flatten)]
    pub base: Bid2Etw_RowsetHelper_1_Trace,

/// Module ID
    #[serde(rename = "ModID")]
    pub mod_id: Option<u32>,

/// Text StringA
    #[serde(rename = "msgStr")]
    pub msg_str: Option<serde_json::Value>,
}

impl Bid2Etw_RowsetHelper_1_Trace_TextA {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Bid2Etw_RowsetHelper_1_Trace::new(),
            mod_id: None,
            msg_str: None,
        }
    }


    /// Sets the value of ModID
    pub fn set_mod_id(&mut self, value: u32) {
        self.mod_id = Some(value);
    }

    /// Gets the value of ModID
    pub fn get_mod_id(&self) -> Option<&u32> {
        self.mod_id.as_ref()
    }

    /// Sets the value of msgStr
    pub fn set_msg_str(&mut self, value: serde_json::Value) {
        self.msg_str = Some(value);
    }

    /// Gets the value of msgStr
    pub fn get_msg_str(&self) -> Option<&serde_json::Value> {
        self.msg_str.as_ref()
    }
}

