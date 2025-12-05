// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnConnectionIPsecConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnConnectionIPsecConfiguration {

/// 
    #[serde(rename = "AuthenticationTransformConstants")]
    pub authentication_transform_constants: Option<u32>,

/// 
    #[serde(rename = "CipherTransformConstants")]
    pub cipher_transform_constants: Option<u32>,

/// 
    #[serde(rename = "DHGroup")]
    pub dhgroup: Option<u32>,

/// 
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<u32>,

/// 
    #[serde(rename = "IntegrityCheckMethod")]
    pub integrity_check_method: Option<u32>,

/// 
    #[serde(rename = "PfsGroup")]
    pub pfs_group: Option<u32>,
}

impl VpnConnectionIPsecConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            authentication_transform_constants: None,
            cipher_transform_constants: None,
            dhgroup: None,
            encryption_method: None,
            integrity_check_method: None,
            pfs_group: None,
        }
    }


    /// Sets the value of AuthenticationTransformConstants
    pub fn set_authentication_transform_constants(&mut self, value: u32) {
        self.authentication_transform_constants = Some(value);
    }

    /// Gets the value of AuthenticationTransformConstants
    pub fn get_authentication_transform_constants(&self) -> Option<&u32> {
        self.authentication_transform_constants.as_ref()
    }

    /// Sets the value of CipherTransformConstants
    pub fn set_cipher_transform_constants(&mut self, value: u32) {
        self.cipher_transform_constants = Some(value);
    }

    /// Gets the value of CipherTransformConstants
    pub fn get_cipher_transform_constants(&self) -> Option<&u32> {
        self.cipher_transform_constants.as_ref()
    }

    /// Sets the value of DHGroup
    pub fn set_dhgroup(&mut self, value: u32) {
        self.dhgroup = Some(value);
    }

    /// Gets the value of DHGroup
    pub fn get_dhgroup(&self) -> Option<&u32> {
        self.dhgroup.as_ref()
    }

    /// Sets the value of EncryptionMethod
    pub fn set_encryption_method(&mut self, value: u32) {
        self.encryption_method = Some(value);
    }

    /// Gets the value of EncryptionMethod
    pub fn get_encryption_method(&self) -> Option<&u32> {
        self.encryption_method.as_ref()
    }

    /// Sets the value of IntegrityCheckMethod
    pub fn set_integrity_check_method(&mut self, value: u32) {
        self.integrity_check_method = Some(value);
    }

    /// Gets the value of IntegrityCheckMethod
    pub fn get_integrity_check_method(&self) -> Option<&u32> {
        self.integrity_check_method.as_ref()
    }

    /// Sets the value of PfsGroup
    pub fn set_pfs_group(&mut self, value: u32) {
        self.pfs_group = Some(value);
    }

    /// Gets the value of PfsGroup
    pub fn get_pfs_group(&self) -> Option<&u32> {
        self.pfs_group.as_ref()
    }
}

