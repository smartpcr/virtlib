// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Certificate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Certificate {

/// 
    #[serde(rename = "Blob")]
    pub blob: Option<String>,

/// 
    #[serde(rename = "IsInstalled")]
    pub is_installed: Option<bool>,

/// 
    #[serde(rename = "StoreLocation")]
    pub store_location: Option<u8>,

/// 
    #[serde(rename = "StoreName")]
    pub store_name: Option<String>,

/// 
    #[serde(rename = "Thumbprint")]
    pub thumbprint: Option<String>,
}

impl MDM_Certificate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            blob: None,
            is_installed: None,
            store_location: None,
            store_name: None,
            thumbprint: None,
        }
    }


    /// Sets the value of Blob
    pub fn set_blob(&mut self, value: String) {
        self.blob = Some(value);
    }

    /// Gets the value of Blob
    pub fn get_blob(&self) -> Option<&String> {
        self.blob.as_ref()
    }

    /// Sets the value of IsInstalled
    pub fn set_is_installed(&mut self, value: bool) {
        self.is_installed = Some(value);
    }

    /// Gets the value of IsInstalled
    pub fn get_is_installed(&self) -> Option<&bool> {
        self.is_installed.as_ref()
    }

    /// Sets the value of StoreLocation
    pub fn set_store_location(&mut self, value: u8) {
        self.store_location = Some(value);
    }

    /// Gets the value of StoreLocation
    pub fn get_store_location(&self) -> Option<&u8> {
        self.store_location.as_ref()
    }

    /// Sets the value of StoreName
    pub fn set_store_name(&mut self, value: String) {
        self.store_name = Some(value);
    }

    /// Gets the value of StoreName
    pub fn get_store_name(&self) -> Option<&String> {
        self.store_name.as_ref()
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

