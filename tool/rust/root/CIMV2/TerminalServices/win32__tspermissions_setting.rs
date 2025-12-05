// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSPermissionsSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSPermissionsSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "DenyAdminPermissionForCustomization")]
    pub deny_admin_permission_for_customization: Option<u32>,

/// 
    #[serde(rename = "PolicySourceDenyAdminPermissionForCustomization")]
    pub policy_source_deny_admin_permission_for_customization: Option<u32>,

/// 
    #[serde(rename = "StringSecurityDescriptor")]
    pub string_security_descriptor: Option<String>,
}

impl Win32_TSPermissionsSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            deny_admin_permission_for_customization: None,
            policy_source_deny_admin_permission_for_customization: None,
            string_security_descriptor: None,
        }
    }


    /// Sets the value of DenyAdminPermissionForCustomization
    pub fn set_deny_admin_permission_for_customization(&mut self, value: u32) {
        self.deny_admin_permission_for_customization = Some(value);
    }

    /// Gets the value of DenyAdminPermissionForCustomization
    pub fn get_deny_admin_permission_for_customization(&self) -> Option<&u32> {
        self.deny_admin_permission_for_customization.as_ref()
    }

    /// Sets the value of PolicySourceDenyAdminPermissionForCustomization
    pub fn set_policy_source_deny_admin_permission_for_customization(&mut self, value: u32) {
        self.policy_source_deny_admin_permission_for_customization = Some(value);
    }

    /// Gets the value of PolicySourceDenyAdminPermissionForCustomization
    pub fn get_policy_source_deny_admin_permission_for_customization(&self) -> Option<&u32> {
        self.policy_source_deny_admin_permission_for_customization.as_ref()
    }

    /// Sets the value of StringSecurityDescriptor
    pub fn set_string_security_descriptor(&mut self, value: String) {
        self.string_security_descriptor = Some(value);
    }

    /// Gets the value of StringSecurityDescriptor
    pub fn get_string_security_descriptor(&self) -> Option<&String> {
        self.string_security_descriptor.as_ref()
    }

/// 

    /// * `account_name` -  (String)
    /// * `permission_pre_set` -  (u32)

    /// * `return_value` -  (u32)
    pub fn add_account(&self, account_name: &String, permission_pre_set: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountName".to_string(), value: account_name.into() });
        args.push(MethodParameter { name: "PermissionPreSet".to_string(), value: permission_pre_set.into() });
        self.invoke_method("AddAccount", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn restore_defaults(&self) -> Result<(), WmiError> {
        self.invoke_method("RestoreDefaults", &[])

    }

}

