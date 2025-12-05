// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TypeLibraryAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TypeLibraryAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "Cost")]
    pub cost: Option<u32>,

/// 
    #[serde(rename = "Language")]
    pub language: Option<u16>,

/// 
    #[serde(rename = "LibID")]
    pub lib_id: Option<String>,
}

impl Win32_TypeLibraryAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            cost: None,
            language: None,
            lib_id: None,
        }
    }


    /// Sets the value of Cost
    pub fn set_cost(&mut self, value: u32) {
        self.cost = Some(value);
    }

    /// Gets the value of Cost
    pub fn get_cost(&self) -> Option<&u32> {
        self.cost.as_ref()
    }

    /// Sets the value of Language
    pub fn set_language(&mut self, value: u16) {
        self.language = Some(value);
    }

    /// Gets the value of Language
    pub fn get_language(&self) -> Option<&u16> {
        self.language.as_ref()
    }

    /// Sets the value of LibID
    pub fn set_lib_id(&mut self, value: String) {
        self.lib_id = Some(value);
    }

    /// Gets the value of LibID
    pub fn get_lib_id(&self) -> Option<&String> {
        self.lib_id.as_ref()
    }
}

