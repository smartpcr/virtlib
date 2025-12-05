// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Registry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Registry {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CurrentSize")]
    pub current_size: Option<u32>,

/// 
    #[serde(rename = "MaximumSize")]
    pub maximum_size: Option<u32>,

/// 
    #[serde(rename = "ProposedSize")]
    pub proposed_size: Option<u32>,
}

impl Win32_Registry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            current_size: None,
            maximum_size: None,
            proposed_size: None,
        }
    }


    /// Sets the value of CurrentSize
    pub fn set_current_size(&mut self, value: u32) {
        self.current_size = Some(value);
    }

    /// Gets the value of CurrentSize
    pub fn get_current_size(&self) -> Option<&u32> {
        self.current_size.as_ref()
    }

    /// Sets the value of MaximumSize
    pub fn set_maximum_size(&mut self, value: u32) {
        self.maximum_size = Some(value);
    }

    /// Gets the value of MaximumSize
    pub fn get_maximum_size(&self) -> Option<&u32> {
        self.maximum_size.as_ref()
    }

    /// Sets the value of ProposedSize
    pub fn set_proposed_size(&mut self, value: u32) {
        self.proposed_size = Some(value);
    }

    /// Gets the value of ProposedSize
    pub fn get_proposed_size(&self) -> Option<&u32> {
        self.proposed_size.as_ref()
    }
}

