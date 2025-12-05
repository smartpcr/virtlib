// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_ApplicationDefaults02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_ApplicationDefaults02 {

/// 
    #[serde(rename = "DefaultAssociationsConfiguration")]
    pub default_associations_configuration: Option<String>,

/// 
    #[serde(rename = "EnableAppUriHandlers")]
    pub enable_app_uri_handlers: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_ApplicationDefaults02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            default_associations_configuration: None,
            enable_app_uri_handlers: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of DefaultAssociationsConfiguration
    pub fn set_default_associations_configuration(&mut self, value: String) {
        self.default_associations_configuration = Some(value);
    }

    /// Gets the value of DefaultAssociationsConfiguration
    pub fn get_default_associations_configuration(&self) -> Option<&String> {
        self.default_associations_configuration.as_ref()
    }

    /// Sets the value of EnableAppUriHandlers
    pub fn set_enable_app_uri_handlers(&mut self, value: i32) {
        self.enable_app_uri_handlers = Some(value);
    }

    /// Gets the value of EnableAppUriHandlers
    pub fn get_enable_app_uri_handlers(&self) -> Option<&i32> {
        self.enable_app_uri_handlers.as_ref()
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
}

