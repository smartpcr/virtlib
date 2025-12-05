// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_TerminalConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_TerminalConnection {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// 
    #[serde(rename = "ConnectionID")]
    pub connection_id: Option<String>,
}

impl Msvm_TerminalConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            connection_id: None,
        }
    }


    /// Sets the value of ConnectionID
    pub fn set_connection_id(&mut self, value: String) {
        self.connection_id = Some(value);
    }

    /// Gets the value of ConnectionID
    pub fn get_connection_id(&self) -> Option<&String> {
        self.connection_id.as_ref()
    }
}

