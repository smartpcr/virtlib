// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSAccount struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSAccount {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,

/// 
    #[serde(rename = "AuditFail")]
    pub audit_fail: Option<u32>,

/// 
    #[serde(rename = "AuditSuccess")]
    pub audit_success: Option<u32>,

/// 
    #[serde(rename = "PermissionsAllowed")]
    pub permissions_allowed: Option<u32>,

/// 
    #[serde(rename = "PermissionsDenied")]
    pub permissions_denied: Option<u32>,

/// 
    #[serde(rename = "SID")]
    pub sid: Option<String>,
}

impl Win32_TSAccount {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            account_name: None,
            audit_fail: None,
            audit_success: None,
            permissions_allowed: None,
            permissions_denied: None,
            sid: None,
        }
    }


    /// Sets the value of AccountName
    pub fn set_account_name(&mut self, value: String) {
        self.account_name = Some(value);
    }

    /// Gets the value of AccountName
    pub fn get_account_name(&self) -> Option<&String> {
        self.account_name.as_ref()
    }

    /// Sets the value of AuditFail
    pub fn set_audit_fail(&mut self, value: u32) {
        self.audit_fail = Some(value);
    }

    /// Gets the value of AuditFail
    pub fn get_audit_fail(&self) -> Option<&u32> {
        self.audit_fail.as_ref()
    }

    /// Sets the value of AuditSuccess
    pub fn set_audit_success(&mut self, value: u32) {
        self.audit_success = Some(value);
    }

    /// Gets the value of AuditSuccess
    pub fn get_audit_success(&self) -> Option<&u32> {
        self.audit_success.as_ref()
    }

    /// Sets the value of PermissionsAllowed
    pub fn set_permissions_allowed(&mut self, value: u32) {
        self.permissions_allowed = Some(value);
    }

    /// Gets the value of PermissionsAllowed
    pub fn get_permissions_allowed(&self) -> Option<&u32> {
        self.permissions_allowed.as_ref()
    }

    /// Sets the value of PermissionsDenied
    pub fn set_permissions_denied(&mut self, value: u32) {
        self.permissions_denied = Some(value);
    }

    /// Gets the value of PermissionsDenied
    pub fn get_permissions_denied(&self) -> Option<&u32> {
        self.permissions_denied.as_ref()
    }

    /// Sets the value of SID
    pub fn set_sid(&mut self, value: String) {
        self.sid = Some(value);
    }

    /// Gets the value of SID
    pub fn get_sid(&self) -> Option<&String> {
        self.sid.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn delete(&self) -> Result<(), WmiError> {
        self.invoke_method("Delete", &[])

    }


/// 

    /// * `allow` -  (bool)
    /// * `permission_mask` -  (u32)

    /// * `return_value` -  (u32)
    pub fn modify_permissions(&self, permission_mask: u32, allow: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PermissionMask".to_string(), value: permission_mask.into() });
        args.push(MethodParameter { name: "Allow".to_string(), value: allow.into() });
        self.invoke_method("ModifyPermissions", &args)

    }


/// 

    /// * `permission_mask` -  (u32)
    /// * `success` -  (bool)

    /// * `return_value` -  (u32)
    pub fn modify_audit_permissions(&self, permission_mask: u32, success: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PermissionMask".to_string(), value: permission_mask.into() });
        args.push(MethodParameter { name: "Success".to_string(), value: success.into() });
        self.invoke_method("ModifyAuditPermissions", &args)

    }

}

