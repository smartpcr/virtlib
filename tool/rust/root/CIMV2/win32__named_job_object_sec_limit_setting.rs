// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NamedJobObjectSecLimitSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NamedJobObjectSecLimitSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "PrivilegesToDelete")]
    pub privileges_to_delete: Option<Win32_TokenPrivileges>,

/// 
    #[serde(rename = "RestrictedSIDs")]
    pub restricted_sids: Option<Win32_TokenGroups>,

/// 
    #[serde(rename = "SecurityLimitFlags")]
    pub security_limit_flags: Option<u32>,

/// 
    #[serde(rename = "SIDsToDisable")]
    pub sids_to_disable: Option<Win32_TokenGroups>,
}

impl Win32_NamedJobObjectSecLimitSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            privileges_to_delete: None,
            restricted_sids: None,
            security_limit_flags: None,
            sids_to_disable: None,
        }
    }


    /// Sets the value of PrivilegesToDelete
    pub fn set_privileges_to_delete(&mut self, value: Win32_TokenPrivileges) {
        self.privileges_to_delete = Some(value);
    }

    /// Gets the value of PrivilegesToDelete
    pub fn get_privileges_to_delete(&self) -> Option<&Win32_TokenPrivileges> {
        self.privileges_to_delete.as_ref()
    }

    /// Sets the value of RestrictedSIDs
    pub fn set_restricted_sids(&mut self, value: Win32_TokenGroups) {
        self.restricted_sids = Some(value);
    }

    /// Gets the value of RestrictedSIDs
    pub fn get_restricted_sids(&self) -> Option<&Win32_TokenGroups> {
        self.restricted_sids.as_ref()
    }

    /// Sets the value of SecurityLimitFlags
    pub fn set_security_limit_flags(&mut self, value: u32) {
        self.security_limit_flags = Some(value);
    }

    /// Gets the value of SecurityLimitFlags
    pub fn get_security_limit_flags(&self) -> Option<&u32> {
        self.security_limit_flags.as_ref()
    }

    /// Sets the value of SIDsToDisable
    pub fn set_sids_to_disable(&mut self, value: Win32_TokenGroups) {
        self.sids_to_disable = Some(value);
    }

    /// Gets the value of SIDsToDisable
    pub fn get_sids_to_disable(&self) -> Option<&Win32_TokenGroups> {
        self.sids_to_disable.as_ref()
    }
}

