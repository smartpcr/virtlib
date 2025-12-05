// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbShareAccessControlEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbShareAccessControlEntry {

/// 
    #[serde(rename = "AccessControlType")]
    pub access_control_type: Option<SmbShareAccessControlEntry_AccessControlType>,

/// 
    #[serde(rename = "AccessRight")]
    pub access_right: Option<SmbShareAccessControlEntry_AccessRight>,

/// 
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ScopeName")]
    pub scope_name: Option<String>,
}

impl MSFT_SmbShareAccessControlEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_control_type: None,
            access_right: None,
            account_name: None,
            name: None,
            scope_name: None,
        }
    }


    /// Sets the value of AccessControlType
    pub fn set_access_control_type(&mut self, value: SmbShareAccessControlEntry_AccessControlType) {
        self.access_control_type = Some(value);
    }

    /// Gets the value of AccessControlType
    pub fn get_access_control_type(&self) -> Option<&SmbShareAccessControlEntry_AccessControlType> {
        self.access_control_type.as_ref()
    }

    /// Sets the value of AccessRight
    pub fn set_access_right(&mut self, value: SmbShareAccessControlEntry_AccessRight) {
        self.access_right = Some(value);
    }

    /// Gets the value of AccessRight
    pub fn get_access_right(&self) -> Option<&SmbShareAccessControlEntry_AccessRight> {
        self.access_right.as_ref()
    }

    /// Sets the value of AccountName
    pub fn set_account_name(&mut self, value: String) {
        self.account_name = Some(value);
    }

    /// Gets the value of AccountName
    pub fn get_account_name(&self) -> Option<&String> {
        self.account_name.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ScopeName
    pub fn set_scope_name(&mut self, value: String) {
        self.scope_name = Some(value);
    }

    /// Gets the value of ScopeName
    pub fn get_scope_name(&self) -> Option<&String> {
        self.scope_name.as_ref()
    }
}

