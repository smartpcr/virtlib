// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_LanguagePackManagement_InstalledLanguages01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_LanguagePackManagement_InstalledLanguages01_01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LanguageFeatures")]
    pub language_features: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Providers")]
    pub providers: Option<i32>,
}

impl MDM_LanguagePackManagement_InstalledLanguages01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            language_features: None,
            parent_id: None,
            providers: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LanguageFeatures
    pub fn set_language_features(&mut self, value: i32) {
        self.language_features = Some(value);
    }

    /// Gets the value of LanguageFeatures
    pub fn get_language_features(&self) -> Option<&i32> {
        self.language_features.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Providers
    pub fn set_providers(&mut self, value: i32) {
        self.providers = Some(value);
    }

    /// Gets the value of Providers
    pub fn get_providers(&self) -> Option<&i32> {
        self.providers.as_ref()
    }
}

