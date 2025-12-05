// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SettingsDefineCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SettingsDefineCapabilities {
    #[serde(flatten)]
    pub base: CIM_SettingsDefineCapabilities,

/// 
    #[serde(rename = "SupportStatement")]
    pub support_statement: Option<u16>,
}

impl Msvm_SettingsDefineCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingsDefineCapabilities::new(),
            support_statement: None,
        }
    }


    /// Sets the value of SupportStatement
    pub fn set_support_statement(&mut self, value: u16) {
        self.support_statement = Some(value);
    }

    /// Gets the value of SupportStatement
    pub fn get_support_statement(&self) -> Option<&u16> {
        self.support_statement.as_ref()
    }
}

