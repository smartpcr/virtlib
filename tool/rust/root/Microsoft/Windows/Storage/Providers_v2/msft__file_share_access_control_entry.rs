// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileShareAccessControlEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileShareAccessControlEntry {

/// Denotes the access type ( Allow, Deny ).
    #[serde(rename = "AccessControlType")]
    pub access_control_type: Option<FileShareAccessControlEntry_AccessControlType>,

/// Denotes the access right.
    #[serde(rename = "AccessRight")]
    pub access_right: Option<FileShareAccessControlEntry_AccessRight>,

/// The name of the account to which the access right is granted.
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,
}

impl MSFT_FileShareAccessControlEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_control_type: None,
            access_right: None,
            account_name: None,
        }
    }


    /// Sets the value of AccessControlType
    pub fn set_access_control_type(&mut self, value: FileShareAccessControlEntry_AccessControlType) {
        self.access_control_type = Some(value);
    }

    /// Gets the value of AccessControlType
    pub fn get_access_control_type(&self) -> Option<&FileShareAccessControlEntry_AccessControlType> {
        self.access_control_type.as_ref()
    }

    /// Sets the value of AccessRight
    pub fn set_access_right(&mut self, value: FileShareAccessControlEntry_AccessRight) {
        self.access_right = Some(value);
    }

    /// Gets the value of AccessRight
    pub fn get_access_right(&self) -> Option<&FileShareAccessControlEntry_AccessRight> {
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
}

