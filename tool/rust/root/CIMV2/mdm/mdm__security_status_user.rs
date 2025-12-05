// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_SecurityStatusUser struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_SecurityStatusUser {

/// 
    #[serde(rename = "ConnectedAccountPolicy")]
    pub connected_account_policy: Option<u32>,

/// 
    #[serde(rename = "DeviceEncryptionPolicy")]
    pub device_encryption_policy: Option<u32>,

/// 
    #[serde(rename = "EncryptionStatus")]
    pub encryption_status: Option<u32>,

/// 
    #[serde(rename = "HasConnectedAccount")]
    pub has_connected_account: Option<bool>,

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,

/// 
    #[serde(rename = "PasswordStatus")]
    pub password_status: Option<u32>,
}

impl MDM_SecurityStatusUser {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connected_account_policy: None,
            device_encryption_policy: None,
            encryption_status: None,
            has_connected_account: None,
            key: None,
            password_status: None,
        }
    }


    /// Sets the value of ConnectedAccountPolicy
    pub fn set_connected_account_policy(&mut self, value: u32) {
        self.connected_account_policy = Some(value);
    }

    /// Gets the value of ConnectedAccountPolicy
    pub fn get_connected_account_policy(&self) -> Option<&u32> {
        self.connected_account_policy.as_ref()
    }

    /// Sets the value of DeviceEncryptionPolicy
    pub fn set_device_encryption_policy(&mut self, value: u32) {
        self.device_encryption_policy = Some(value);
    }

    /// Gets the value of DeviceEncryptionPolicy
    pub fn get_device_encryption_policy(&self) -> Option<&u32> {
        self.device_encryption_policy.as_ref()
    }

    /// Sets the value of EncryptionStatus
    pub fn set_encryption_status(&mut self, value: u32) {
        self.encryption_status = Some(value);
    }

    /// Gets the value of EncryptionStatus
    pub fn get_encryption_status(&self) -> Option<&u32> {
        self.encryption_status.as_ref()
    }

    /// Sets the value of HasConnectedAccount
    pub fn set_has_connected_account(&mut self, value: bool) {
        self.has_connected_account = Some(value);
    }

    /// Gets the value of HasConnectedAccount
    pub fn get_has_connected_account(&self) -> Option<&bool> {
        self.has_connected_account.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }

    /// Sets the value of PasswordStatus
    pub fn set_password_status(&mut self, value: u32) {
        self.password_status = Some(value);
    }

    /// Gets the value of PasswordStatus
    pub fn get_password_status(&self) -> Option<&u32> {
        self.password_status.as_ref()
    }
}

