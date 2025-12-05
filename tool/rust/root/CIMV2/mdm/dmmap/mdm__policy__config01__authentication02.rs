// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Authentication02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Authentication02 {

/// 
    #[serde(rename = "AllowAadPasswordReset")]
    pub allow_aad_password_reset: Option<i32>,

/// 
    #[serde(rename = "AllowFastReconnect")]
    pub allow_fast_reconnect: Option<i32>,

/// 
    #[serde(rename = "AllowSecondaryAuthenticationDevice")]
    pub allow_secondary_authentication_device: Option<i32>,

/// 
    #[serde(rename = "ConfigureWebcamAccessDomainNames")]
    pub configure_webcam_access_domain_names: Option<String>,

/// 
    #[serde(rename = "ConfigureWebSignInAllowedUrls")]
    pub configure_web_sign_in_allowed_urls: Option<String>,

/// 
    #[serde(rename = "EnableFastFirstSignIn")]
    pub enable_fast_first_sign_in: Option<i32>,

/// 
    #[serde(rename = "EnableWebSignIn")]
    pub enable_web_sign_in: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreferredAadTenantDomainName")]
    pub preferred_aad_tenant_domain_name: Option<String>,
}

impl MDM_Policy_Config01_Authentication02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_aad_password_reset: None,
            allow_fast_reconnect: None,
            allow_secondary_authentication_device: None,
            configure_webcam_access_domain_names: None,
            configure_web_sign_in_allowed_urls: None,
            enable_fast_first_sign_in: None,
            enable_web_sign_in: None,
            instance_id: None,
            parent_id: None,
            preferred_aad_tenant_domain_name: None,
        }
    }


    /// Sets the value of AllowAadPasswordReset
    pub fn set_allow_aad_password_reset(&mut self, value: i32) {
        self.allow_aad_password_reset = Some(value);
    }

    /// Gets the value of AllowAadPasswordReset
    pub fn get_allow_aad_password_reset(&self) -> Option<&i32> {
        self.allow_aad_password_reset.as_ref()
    }

    /// Sets the value of AllowFastReconnect
    pub fn set_allow_fast_reconnect(&mut self, value: i32) {
        self.allow_fast_reconnect = Some(value);
    }

    /// Gets the value of AllowFastReconnect
    pub fn get_allow_fast_reconnect(&self) -> Option<&i32> {
        self.allow_fast_reconnect.as_ref()
    }

    /// Sets the value of AllowSecondaryAuthenticationDevice
    pub fn set_allow_secondary_authentication_device(&mut self, value: i32) {
        self.allow_secondary_authentication_device = Some(value);
    }

    /// Gets the value of AllowSecondaryAuthenticationDevice
    pub fn get_allow_secondary_authentication_device(&self) -> Option<&i32> {
        self.allow_secondary_authentication_device.as_ref()
    }

    /// Sets the value of ConfigureWebcamAccessDomainNames
    pub fn set_configure_webcam_access_domain_names(&mut self, value: String) {
        self.configure_webcam_access_domain_names = Some(value);
    }

    /// Gets the value of ConfigureWebcamAccessDomainNames
    pub fn get_configure_webcam_access_domain_names(&self) -> Option<&String> {
        self.configure_webcam_access_domain_names.as_ref()
    }

    /// Sets the value of ConfigureWebSignInAllowedUrls
    pub fn set_configure_web_sign_in_allowed_urls(&mut self, value: String) {
        self.configure_web_sign_in_allowed_urls = Some(value);
    }

    /// Gets the value of ConfigureWebSignInAllowedUrls
    pub fn get_configure_web_sign_in_allowed_urls(&self) -> Option<&String> {
        self.configure_web_sign_in_allowed_urls.as_ref()
    }

    /// Sets the value of EnableFastFirstSignIn
    pub fn set_enable_fast_first_sign_in(&mut self, value: i32) {
        self.enable_fast_first_sign_in = Some(value);
    }

    /// Gets the value of EnableFastFirstSignIn
    pub fn get_enable_fast_first_sign_in(&self) -> Option<&i32> {
        self.enable_fast_first_sign_in.as_ref()
    }

    /// Sets the value of EnableWebSignIn
    pub fn set_enable_web_sign_in(&mut self, value: i32) {
        self.enable_web_sign_in = Some(value);
    }

    /// Gets the value of EnableWebSignIn
    pub fn get_enable_web_sign_in(&self) -> Option<&i32> {
        self.enable_web_sign_in.as_ref()
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

    /// Sets the value of PreferredAadTenantDomainName
    pub fn set_preferred_aad_tenant_domain_name(&mut self, value: String) {
        self.preferred_aad_tenant_domain_name = Some(value);
    }

    /// Gets the value of PreferredAadTenantDomainName
    pub fn get_preferred_aad_tenant_domain_name(&self) -> Option<&String> {
        self.preferred_aad_tenant_domain_name.as_ref()
    }
}

