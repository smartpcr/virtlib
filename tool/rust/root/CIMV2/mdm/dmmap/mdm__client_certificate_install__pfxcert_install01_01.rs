// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ClientCertificateInstall_PFXCertInstall01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ClientCertificateInstall_PFXCertInstall01_01 {

/// 
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "KeyLocation")]
    pub key_location: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PFXCertBlob")]
    pub pfxcert_blob: Option<String>,

/// 
    #[serde(rename = "PFXCertPassword")]
    pub pfxcert_password: Option<String>,

/// 
    #[serde(rename = "PFXCertPasswordEncryptionStore")]
    pub pfxcert_password_encryption_store: Option<String>,

/// 
    #[serde(rename = "PFXCertPasswordEncryptionType")]
    pub pfxcert_password_encryption_type: Option<i32>,

/// 
    #[serde(rename = "PFXKeyExportable")]
    pub pfxkey_exportable: Option<bool>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,

/// 
    #[serde(rename = "Thumbprint")]
    pub thumbprint: Option<String>,
}

impl MDM_ClientCertificateInstall_PFXCertInstall01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            container_name: None,
            instance_id: None,
            key_location: None,
            parent_id: None,
            pfxcert_blob: None,
            pfxcert_password: None,
            pfxcert_password_encryption_store: None,
            pfxcert_password_encryption_type: None,
            pfxkey_exportable: None,
            status: None,
            thumbprint: None,
        }
    }


    /// Sets the value of ContainerName
    pub fn set_container_name(&mut self, value: String) {
        self.container_name = Some(value);
    }

    /// Gets the value of ContainerName
    pub fn get_container_name(&self) -> Option<&String> {
        self.container_name.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of KeyLocation
    pub fn set_key_location(&mut self, value: i32) {
        self.key_location = Some(value);
    }

    /// Gets the value of KeyLocation
    pub fn get_key_location(&self) -> Option<&i32> {
        self.key_location.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PFXCertBlob
    pub fn set_pfxcert_blob(&mut self, value: String) {
        self.pfxcert_blob = Some(value);
    }

    /// Gets the value of PFXCertBlob
    pub fn get_pfxcert_blob(&self) -> Option<&String> {
        self.pfxcert_blob.as_ref()
    }

    /// Sets the value of PFXCertPassword
    pub fn set_pfxcert_password(&mut self, value: String) {
        self.pfxcert_password = Some(value);
    }

    /// Gets the value of PFXCertPassword
    pub fn get_pfxcert_password(&self) -> Option<&String> {
        self.pfxcert_password.as_ref()
    }

    /// Sets the value of PFXCertPasswordEncryptionStore
    pub fn set_pfxcert_password_encryption_store(&mut self, value: String) {
        self.pfxcert_password_encryption_store = Some(value);
    }

    /// Gets the value of PFXCertPasswordEncryptionStore
    pub fn get_pfxcert_password_encryption_store(&self) -> Option<&String> {
        self.pfxcert_password_encryption_store.as_ref()
    }

    /// Sets the value of PFXCertPasswordEncryptionType
    pub fn set_pfxcert_password_encryption_type(&mut self, value: i32) {
        self.pfxcert_password_encryption_type = Some(value);
    }

    /// Gets the value of PFXCertPasswordEncryptionType
    pub fn get_pfxcert_password_encryption_type(&self) -> Option<&i32> {
        self.pfxcert_password_encryption_type.as_ref()
    }

    /// Sets the value of PFXKeyExportable
    pub fn set_pfxkey_exportable(&mut self, value: bool) {
        self.pfxkey_exportable = Some(value);
    }

    /// Gets the value of PFXKeyExportable
    pub fn get_pfxkey_exportable(&self) -> Option<&bool> {
        self.pfxkey_exportable.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

    /// Sets the value of Thumbprint
    pub fn set_thumbprint(&mut self, value: String) {
        self.thumbprint = Some(value);
    }

    /// Gets the value of Thumbprint
    pub fn get_thumbprint(&self) -> Option<&String> {
        self.thumbprint.as_ref()
    }
}

