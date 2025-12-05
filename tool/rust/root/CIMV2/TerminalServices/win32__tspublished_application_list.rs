// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSPublishedApplicationList struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSPublishedApplicationList {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// Whether the Terminal Server restricts remote applications to those on the list.
    #[serde(rename = "Disabled")]
    pub disabled: Option<bool>,

/// Indicates whether the property Disabled is configured by Server (0),Group Policy (1), Default (2) 
    #[serde(rename = "PolicySourceDisabled")]
    pub policy_source_disabled: Option<u32>,
}

impl Win32_TSPublishedApplicationList {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            disabled: None,
            policy_source_disabled: None,
        }
    }


    /// Sets the value of Disabled
    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = Some(value);
    }

    /// Gets the value of Disabled
    pub fn get_disabled(&self) -> Option<&bool> {
        self.disabled.as_ref()
    }

    /// Sets the value of PolicySourceDisabled
    pub fn set_policy_source_disabled(&mut self, value: u32) {
        self.policy_source_disabled = Some(value);
    }

    /// Gets the value of PolicySourceDisabled
    pub fn get_policy_source_disabled(&self) -> Option<&u32> {
        self.policy_source_disabled.as_ref()
    }
}

