// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NTLogEventUser struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NTLogEventUser {

/// 
    #[serde(rename = "Record")]
    pub record: Option<Win32_NTLogEvent>,

/// 
    #[serde(rename = "User")]
    pub user: Option<Win32_UserAccount>,
}

impl Win32_NTLogEventUser {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            record: None,
            user: None,
        }
    }


    /// Sets the value of Record
    pub fn set_record(&mut self, value: Win32_NTLogEvent) {
        self.record = Some(value);
    }

    /// Gets the value of Record
    pub fn get_record(&self) -> Option<&Win32_NTLogEvent> {
        self.record.as_ref()
    }

    /// Sets the value of User
    pub fn set_user(&mut self, value: Win32_UserAccount) {
        self.user = Some(value);
    }

    /// Gets the value of User
    pub fn get_user(&self) -> Option<&Win32_UserAccount> {
        self.user.as_ref()
    }
}

