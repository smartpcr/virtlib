// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Account struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Account {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "LocalAccount")]
    pub local_account: Option<bool>,

/// 
    #[serde(rename = "SID")]
    pub sid: Option<String>,

/// 
    #[serde(rename = "SIDType")]
    pub sidtype: Option<u8>,
}

impl Win32_Account {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            domain: None,
            local_account: None,
            sid: None,
            sidtype: None,
        }
    }


    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of LocalAccount
    pub fn set_local_account(&mut self, value: bool) {
        self.local_account = Some(value);
    }

    /// Gets the value of LocalAccount
    pub fn get_local_account(&self) -> Option<&bool> {
        self.local_account.as_ref()
    }

    /// Sets the value of SID
    pub fn set_sid(&mut self, value: String) {
        self.sid = Some(value);
    }

    /// Gets the value of SID
    pub fn get_sid(&self) -> Option<&String> {
        self.sid.as_ref()
    }

    /// Sets the value of SIDType
    pub fn set_sidtype(&mut self, value: u8) {
        self.sidtype = Some(value);
    }

    /// Gets the value of SIDType
    pub fn get_sidtype(&self) -> Option<&u8> {
        self.sidtype.as_ref()
    }
}

