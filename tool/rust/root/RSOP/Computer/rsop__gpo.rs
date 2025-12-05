// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_GPO struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_GPO {

/// 
    #[serde(rename = "accessDenied")]
    pub access_denied: Option<bool>,

/// 
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "extensionIds")]
    pub extension_ids: Vec<String>,

/// 
    #[serde(rename = "fileSystemPath")]
    pub file_system_path: Option<String>,

/// 
    #[serde(rename = "filterAllowed")]
    pub filter_allowed: Option<bool>,

/// 
    #[serde(rename = "filterId")]
    pub filter_id: Option<String>,

/// 
    #[serde(rename = "guidName")]
    pub guid_name: Option<String>,

/// 
    #[serde(rename = "id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "securityDescriptor")]
    pub security_descriptor: Vec<u8>,

/// 
    #[serde(rename = "version")]
    pub version: Option<u32>,
}

impl RSOP_GPO {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_denied: None,
            enabled: None,
            extension_ids: Vec::new(),
            file_system_path: None,
            filter_allowed: None,
            filter_id: None,
            guid_name: None,
            id: None,
            name: None,
            security_descriptor: Vec::new(),
            version: None,
        }
    }


    /// Sets the value of accessDenied
    pub fn set_access_denied(&mut self, value: bool) {
        self.access_denied = Some(value);
    }

    /// Gets the value of accessDenied
    pub fn get_access_denied(&self) -> Option<&bool> {
        self.access_denied.as_ref()
    }

    /// Sets the value of enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of extensionIds
    pub fn set_extension_ids(&mut self, value: Vec<String>) {
        self.extension_ids = value;
    }

    /// Gets the value of extensionIds
    pub fn get_extension_ids(&self) -> &Vec<String> {
        &self.extension_ids
    }

    /// Sets the value of fileSystemPath
    pub fn set_file_system_path(&mut self, value: String) {
        self.file_system_path = Some(value);
    }

    /// Gets the value of fileSystemPath
    pub fn get_file_system_path(&self) -> Option<&String> {
        self.file_system_path.as_ref()
    }

    /// Sets the value of filterAllowed
    pub fn set_filter_allowed(&mut self, value: bool) {
        self.filter_allowed = Some(value);
    }

    /// Gets the value of filterAllowed
    pub fn get_filter_allowed(&self) -> Option<&bool> {
        self.filter_allowed.as_ref()
    }

    /// Sets the value of filterId
    pub fn set_filter_id(&mut self, value: String) {
        self.filter_id = Some(value);
    }

    /// Gets the value of filterId
    pub fn get_filter_id(&self) -> Option<&String> {
        self.filter_id.as_ref()
    }

    /// Sets the value of guidName
    pub fn set_guid_name(&mut self, value: String) {
        self.guid_name = Some(value);
    }

    /// Gets the value of guidName
    pub fn get_guid_name(&self) -> Option<&String> {
        self.guid_name.as_ref()
    }

    /// Sets the value of id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of securityDescriptor
    pub fn set_security_descriptor(&mut self, value: Vec<u8>) {
        self.security_descriptor = value;
    }

    /// Gets the value of securityDescriptor
    pub fn get_security_descriptor(&self) -> &Vec<u8> {
        &self.security_descriptor
    }

    /// Sets the value of version
    pub fn set_version(&mut self, value: u32) {
        self.version = Some(value);
    }

    /// Gets the value of version
    pub fn get_version(&self) -> Option<&u32> {
        self.version.as_ref()
    }
}

