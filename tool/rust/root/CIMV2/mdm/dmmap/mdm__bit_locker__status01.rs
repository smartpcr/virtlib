// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_BitLocker_Status01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_BitLocker_Status01 {

/// 
    #[serde(rename = "DeviceEncryptionStatus")]
    pub device_encryption_status: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RemovableDrivesEncryptionStatus")]
    pub removable_drives_encryption_status: Option<i32>,

/// 
    #[serde(rename = "RotateRecoveryPasswordsRequestID")]
    pub rotate_recovery_passwords_request_id: Option<String>,

/// 
    #[serde(rename = "RotateRecoveryPasswordsStatus")]
    pub rotate_recovery_passwords_status: Option<i32>,
}

impl MDM_BitLocker_Status01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_encryption_status: None,
            instance_id: None,
            parent_id: None,
            removable_drives_encryption_status: None,
            rotate_recovery_passwords_request_id: None,
            rotate_recovery_passwords_status: None,
        }
    }


    /// Sets the value of DeviceEncryptionStatus
    pub fn set_device_encryption_status(&mut self, value: i32) {
        self.device_encryption_status = Some(value);
    }

    /// Gets the value of DeviceEncryptionStatus
    pub fn get_device_encryption_status(&self) -> Option<&i32> {
        self.device_encryption_status.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RemovableDrivesEncryptionStatus
    pub fn set_removable_drives_encryption_status(&mut self, value: i32) {
        self.removable_drives_encryption_status = Some(value);
    }

    /// Gets the value of RemovableDrivesEncryptionStatus
    pub fn get_removable_drives_encryption_status(&self) -> Option<&i32> {
        self.removable_drives_encryption_status.as_ref()
    }

    /// Sets the value of RotateRecoveryPasswordsRequestID
    pub fn set_rotate_recovery_passwords_request_id(&mut self, value: String) {
        self.rotate_recovery_passwords_request_id = Some(value);
    }

    /// Gets the value of RotateRecoveryPasswordsRequestID
    pub fn get_rotate_recovery_passwords_request_id(&self) -> Option<&String> {
        self.rotate_recovery_passwords_request_id.as_ref()
    }

    /// Sets the value of RotateRecoveryPasswordsStatus
    pub fn set_rotate_recovery_passwords_status(&mut self, value: i32) {
        self.rotate_recovery_passwords_status = Some(value);
    }

    /// Gets the value of RotateRecoveryPasswordsStatus
    pub fn get_rotate_recovery_passwords_status(&self) -> Option<&i32> {
        self.rotate_recovery_passwords_status.as_ref()
    }
}

