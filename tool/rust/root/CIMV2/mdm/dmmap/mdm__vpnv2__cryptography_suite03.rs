// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_CryptographySuite03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_CryptographySuite03 {

/// 
    #[serde(rename = "AuthenticationTransformConstants")]
    pub authentication_transform_constants: Option<String>,

/// 
    #[serde(rename = "CipherTransformConstants")]
    pub cipher_transform_constants: Option<String>,

/// 
    #[serde(rename = "DHGroup")]
    pub dhgroup: Option<String>,

/// 
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IntegrityCheckMethod")]
    pub integrity_check_method: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PfsGroup")]
    pub pfs_group: Option<String>,
}

impl MDM_VPNv2_CryptographySuite03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            authentication_transform_constants: None,
            cipher_transform_constants: None,
            dhgroup: None,
            encryption_method: None,
            instance_id: None,
            integrity_check_method: None,
            parent_id: None,
            pfs_group: None,
        }
    }


    /// Sets the value of AuthenticationTransformConstants
    pub fn set_authentication_transform_constants(&mut self, value: String) {
        self.authentication_transform_constants = Some(value);
    }

    /// Gets the value of AuthenticationTransformConstants
    pub fn get_authentication_transform_constants(&self) -> Option<&String> {
        self.authentication_transform_constants.as_ref()
    }

    /// Sets the value of CipherTransformConstants
    pub fn set_cipher_transform_constants(&mut self, value: String) {
        self.cipher_transform_constants = Some(value);
    }

    /// Gets the value of CipherTransformConstants
    pub fn get_cipher_transform_constants(&self) -> Option<&String> {
        self.cipher_transform_constants.as_ref()
    }

    /// Sets the value of DHGroup
    pub fn set_dhgroup(&mut self, value: String) {
        self.dhgroup = Some(value);
    }

    /// Gets the value of DHGroup
    pub fn get_dhgroup(&self) -> Option<&String> {
        self.dhgroup.as_ref()
    }

    /// Sets the value of EncryptionMethod
    pub fn set_encryption_method(&mut self, value: String) {
        self.encryption_method = Some(value);
    }

    /// Gets the value of EncryptionMethod
    pub fn get_encryption_method(&self) -> Option<&String> {
        self.encryption_method.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IntegrityCheckMethod
    pub fn set_integrity_check_method(&mut self, value: String) {
        self.integrity_check_method = Some(value);
    }

    /// Gets the value of IntegrityCheckMethod
    pub fn get_integrity_check_method(&self) -> Option<&String> {
        self.integrity_check_method.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PfsGroup
    pub fn set_pfs_group(&mut self, value: String) {
        self.pfs_group = Some(value);
    }

    /// Gets the value of PfsGroup
    pub fn get_pfs_group(&self) -> Option<&String> {
        self.pfs_group.as_ref()
    }
}

