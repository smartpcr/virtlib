// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProgIDSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProgIDSpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "Parent")]
    pub parent: Option<String>,

/// 
    #[serde(rename = "ProgID")]
    pub prog_id: Option<String>,
}

impl Win32_ProgIDSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            parent: None,
            prog_id: None,
        }
    }


    /// Sets the value of Parent
    pub fn set_parent(&mut self, value: String) {
        self.parent = Some(value);
    }

    /// Gets the value of Parent
    pub fn get_parent(&self) -> Option<&String> {
        self.parent.as_ref()
    }

    /// Sets the value of ProgID
    pub fn set_prog_id(&mut self, value: String) {
        self.prog_id = Some(value);
    }

    /// Gets the value of ProgID
    pub fn get_prog_id(&self) -> Option<&String> {
        self.prog_id.as_ref()
    }
}

