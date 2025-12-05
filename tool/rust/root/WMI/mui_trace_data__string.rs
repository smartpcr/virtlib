// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MuiTraceData_String struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MuiTraceData_String {
    #[serde(flatten)]
    pub base: MuiTraceData,

/// 
    #[serde(rename = "MuiLoadString")]
    pub mui_load_string: Option<String>,
}

impl MuiTraceData_String {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MuiTraceData::new(),
            mui_load_string: None,
        }
    }


    /// Sets the value of MuiLoadString
    pub fn set_mui_load_string(&mut self, value: String) {
        self.mui_load_string = Some(value);
    }

    /// Gets the value of MuiLoadString
    pub fn get_mui_load_string(&self) -> Option<&String> {
        self.mui_load_string.as_ref()
    }
}

