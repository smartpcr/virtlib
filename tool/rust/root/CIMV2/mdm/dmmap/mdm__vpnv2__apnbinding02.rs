// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_APNBinding02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_APNBinding02 {

/// 
    #[serde(rename = "AccessPointName")]
    pub access_point_name: Option<String>,

/// 
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IsCompressionEnabled")]
    pub is_compression_enabled: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "ProviderId")]
    pub provider_id: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl MDM_VPNv2_APNBinding02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_point_name: None,
            authentication_type: None,
            instance_id: None,
            is_compression_enabled: None,
            parent_id: None,
            password: None,
            provider_id: None,
            user_name: None,
        }
    }


    /// Sets the value of AccessPointName
    pub fn set_access_point_name(&mut self, value: String) {
        self.access_point_name = Some(value);
    }

    /// Gets the value of AccessPointName
    pub fn get_access_point_name(&self) -> Option<&String> {
        self.access_point_name.as_ref()
    }

    /// Sets the value of AuthenticationType
    pub fn set_authentication_type(&mut self, value: String) {
        self.authentication_type = Some(value);
    }

    /// Gets the value of AuthenticationType
    pub fn get_authentication_type(&self) -> Option<&String> {
        self.authentication_type.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IsCompressionEnabled
    pub fn set_is_compression_enabled(&mut self, value: bool) {
        self.is_compression_enabled = Some(value);
    }

    /// Gets the value of IsCompressionEnabled
    pub fn get_is_compression_enabled(&self) -> Option<&bool> {
        self.is_compression_enabled.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Sets the value of ProviderId
    pub fn set_provider_id(&mut self, value: String) {
        self.provider_id = Some(value);
    }

    /// Gets the value of ProviderId
    pub fn get_provider_id(&self) -> Option<&String> {
        self.provider_id.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

