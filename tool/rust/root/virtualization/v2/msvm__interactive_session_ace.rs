// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_InteractiveSessionACE struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_InteractiveSessionACE {

/// 
    #[serde(rename = "AccessType")]
    pub access_type: Option<u16>,

/// 
    #[serde(rename = "Trustee")]
    pub trustee: Option<String>,
}

impl Msvm_InteractiveSessionACE {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_type: None,
            trustee: None,
        }
    }


    /// Sets the value of AccessType
    pub fn set_access_type(&mut self, value: u16) {
        self.access_type = Some(value);
    }

    /// Gets the value of AccessType
    pub fn get_access_type(&self) -> Option<&u16> {
        self.access_type.as_ref()
    }

    /// Sets the value of Trustee
    pub fn set_trustee(&mut self, value: String) {
        self.trustee = Some(value);
    }

    /// Gets the value of Trustee
    pub fn get_trustee(&self) -> Option<&String> {
        self.trustee.as_ref()
    }
}

