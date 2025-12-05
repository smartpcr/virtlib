// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_UserPrivilegeRight struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_UserPrivilegeRight {
    #[serde(flatten)]
    pub base: RSOP_SecuritySettings,

/// 
    #[serde(rename = "AccountList")]
    pub account_list: Vec<String>,

/// 
    #[serde(rename = "UserRight")]
    pub user_right: Option<String>,
}

impl RSOP_UserPrivilegeRight {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_SecuritySettings::new(),
            account_list: Vec::new(),
            user_right: None,
        }
    }


    /// Sets the value of AccountList
    pub fn set_account_list(&mut self, value: Vec<String>) {
        self.account_list = value;
    }

    /// Gets the value of AccountList
    pub fn get_account_list(&self) -> &Vec<String> {
        &self.account_list
    }

    /// Sets the value of UserRight
    pub fn set_user_right(&mut self, value: String) {
        self.user_right = Some(value);
    }

    /// Gets the value of UserRight
    pub fn get_user_right(&self) -> Option<&String> {
        self.user_right.as_ref()
    }
}

