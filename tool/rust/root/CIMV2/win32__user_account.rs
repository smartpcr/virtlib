// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_UserAccount struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_UserAccount {
    #[serde(flatten)]
    pub base: Win32_Account,

/// 
    #[serde(rename = "AccountType")]
    pub account_type: Option<u32>,

/// 
    #[serde(rename = "Disabled")]
    pub disabled: Option<bool>,

/// 
    #[serde(rename = "FullName")]
    pub full_name: Option<String>,

/// 
    #[serde(rename = "Lockout")]
    pub lockout: Option<bool>,

/// 
    #[serde(rename = "PasswordChangeable")]
    pub password_changeable: Option<bool>,

/// 
    #[serde(rename = "PasswordExpires")]
    pub password_expires: Option<bool>,

/// 
    #[serde(rename = "PasswordRequired")]
    pub password_required: Option<bool>,
}

impl Win32_UserAccount {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_Account::new(),
            account_type: None,
            disabled: None,
            full_name: None,
            lockout: None,
            password_changeable: None,
            password_expires: None,
            password_required: None,
        }
    }


    /// Sets the value of AccountType
    pub fn set_account_type(&mut self, value: u32) {
        self.account_type = Some(value);
    }

    /// Gets the value of AccountType
    pub fn get_account_type(&self) -> Option<&u32> {
        self.account_type.as_ref()
    }

    /// Sets the value of Disabled
    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = Some(value);
    }

    /// Gets the value of Disabled
    pub fn get_disabled(&self) -> Option<&bool> {
        self.disabled.as_ref()
    }

    /// Sets the value of FullName
    pub fn set_full_name(&mut self, value: String) {
        self.full_name = Some(value);
    }

    /// Gets the value of FullName
    pub fn get_full_name(&self) -> Option<&String> {
        self.full_name.as_ref()
    }

    /// Sets the value of Lockout
    pub fn set_lockout(&mut self, value: bool) {
        self.lockout = Some(value);
    }

    /// Gets the value of Lockout
    pub fn get_lockout(&self) -> Option<&bool> {
        self.lockout.as_ref()
    }

    /// Sets the value of PasswordChangeable
    pub fn set_password_changeable(&mut self, value: bool) {
        self.password_changeable = Some(value);
    }

    /// Gets the value of PasswordChangeable
    pub fn get_password_changeable(&self) -> Option<&bool> {
        self.password_changeable.as_ref()
    }

    /// Sets the value of PasswordExpires
    pub fn set_password_expires(&mut self, value: bool) {
        self.password_expires = Some(value);
    }

    /// Gets the value of PasswordExpires
    pub fn get_password_expires(&self) -> Option<&bool> {
        self.password_expires.as_ref()
    }

    /// Sets the value of PasswordRequired
    pub fn set_password_required(&mut self, value: bool) {
        self.password_required = Some(value);
    }

    /// Gets the value of PasswordRequired
    pub fn get_password_required(&self) -> Option<&bool> {
        self.password_required.as_ref()
    }

/// 

    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("Rename", &args)

    }

}

