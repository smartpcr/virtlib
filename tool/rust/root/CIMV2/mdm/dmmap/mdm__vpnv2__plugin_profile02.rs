// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_PluginProfile02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_PluginProfile02 {

/// 
    #[serde(rename = "CustomConfiguration")]
    pub custom_configuration: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PluginPackageFamilyName")]
    pub plugin_package_family_name: Option<String>,

/// 
    #[serde(rename = "ServerUrlList")]
    pub server_url_list: Option<String>,
}

impl MDM_VPNv2_PluginProfile02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            custom_configuration: None,
            instance_id: None,
            parent_id: None,
            plugin_package_family_name: None,
            server_url_list: None,
        }
    }


    /// Sets the value of CustomConfiguration
    pub fn set_custom_configuration(&mut self, value: String) {
        self.custom_configuration = Some(value);
    }

    /// Gets the value of CustomConfiguration
    pub fn get_custom_configuration(&self) -> Option<&String> {
        self.custom_configuration.as_ref()
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

    /// Sets the value of PluginPackageFamilyName
    pub fn set_plugin_package_family_name(&mut self, value: String) {
        self.plugin_package_family_name = Some(value);
    }

    /// Gets the value of PluginPackageFamilyName
    pub fn get_plugin_package_family_name(&self) -> Option<&String> {
        self.plugin_package_family_name.as_ref()
    }

    /// Sets the value of ServerUrlList
    pub fn set_server_url_list(&mut self, value: String) {
        self.server_url_list = Some(value);
    }

    /// Gets the value of ServerUrlList
    pub fn get_server_url_list(&self) -> Option<&String> {
        self.server_url_list.as_ref()
    }
}

