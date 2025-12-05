// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_InitiatorLoginStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_InitiatorLoginStatistics {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "LoginAcceptRsps")]
    pub login_accept_rsps: Option<u32>,

/// 
    #[serde(rename = "LoginAuthenticateFails")]
    pub login_authenticate_fails: Option<u32>,

/// 
    #[serde(rename = "LoginAuthFailRsps")]
    pub login_auth_fail_rsps: Option<u32>,

/// 
    #[serde(rename = "LoginFailures")]
    pub login_failures: Option<u32>,

/// 
    #[serde(rename = "LoginNegotiateFails")]
    pub login_negotiate_fails: Option<u32>,

/// 
    #[serde(rename = "LoginOtherFailRsps")]
    pub login_other_fail_rsps: Option<u32>,

/// 
    #[serde(rename = "LoginRedirectRsps")]
    pub login_redirect_rsps: Option<u32>,

/// 
    #[serde(rename = "LogoutNormals")]
    pub logout_normals: Option<u32>,

/// 
    #[serde(rename = "LogoutOtherCodes")]
    pub logout_other_codes: Option<u32>,

/// 
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,
}

impl MSiSCSI_InitiatorLoginStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active: None,
            instance_name: None,
            login_accept_rsps: None,
            login_authenticate_fails: None,
            login_auth_fail_rsps: None,
            login_failures: None,
            login_negotiate_fails: None,
            login_other_fail_rsps: None,
            login_redirect_rsps: None,
            logout_normals: None,
            logout_other_codes: None,
            unique_adapter_id: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LoginAcceptRsps
    pub fn set_login_accept_rsps(&mut self, value: u32) {
        self.login_accept_rsps = Some(value);
    }

    /// Gets the value of LoginAcceptRsps
    pub fn get_login_accept_rsps(&self) -> Option<&u32> {
        self.login_accept_rsps.as_ref()
    }

    /// Sets the value of LoginAuthenticateFails
    pub fn set_login_authenticate_fails(&mut self, value: u32) {
        self.login_authenticate_fails = Some(value);
    }

    /// Gets the value of LoginAuthenticateFails
    pub fn get_login_authenticate_fails(&self) -> Option<&u32> {
        self.login_authenticate_fails.as_ref()
    }

    /// Sets the value of LoginAuthFailRsps
    pub fn set_login_auth_fail_rsps(&mut self, value: u32) {
        self.login_auth_fail_rsps = Some(value);
    }

    /// Gets the value of LoginAuthFailRsps
    pub fn get_login_auth_fail_rsps(&self) -> Option<&u32> {
        self.login_auth_fail_rsps.as_ref()
    }

    /// Sets the value of LoginFailures
    pub fn set_login_failures(&mut self, value: u32) {
        self.login_failures = Some(value);
    }

    /// Gets the value of LoginFailures
    pub fn get_login_failures(&self) -> Option<&u32> {
        self.login_failures.as_ref()
    }

    /// Sets the value of LoginNegotiateFails
    pub fn set_login_negotiate_fails(&mut self, value: u32) {
        self.login_negotiate_fails = Some(value);
    }

    /// Gets the value of LoginNegotiateFails
    pub fn get_login_negotiate_fails(&self) -> Option<&u32> {
        self.login_negotiate_fails.as_ref()
    }

    /// Sets the value of LoginOtherFailRsps
    pub fn set_login_other_fail_rsps(&mut self, value: u32) {
        self.login_other_fail_rsps = Some(value);
    }

    /// Gets the value of LoginOtherFailRsps
    pub fn get_login_other_fail_rsps(&self) -> Option<&u32> {
        self.login_other_fail_rsps.as_ref()
    }

    /// Sets the value of LoginRedirectRsps
    pub fn set_login_redirect_rsps(&mut self, value: u32) {
        self.login_redirect_rsps = Some(value);
    }

    /// Gets the value of LoginRedirectRsps
    pub fn get_login_redirect_rsps(&self) -> Option<&u32> {
        self.login_redirect_rsps.as_ref()
    }

    /// Sets the value of LogoutNormals
    pub fn set_logout_normals(&mut self, value: u32) {
        self.logout_normals = Some(value);
    }

    /// Gets the value of LogoutNormals
    pub fn get_logout_normals(&self) -> Option<&u32> {
        self.logout_normals.as_ref()
    }

    /// Sets the value of LogoutOtherCodes
    pub fn set_logout_other_codes(&mut self, value: u32) {
        self.logout_other_codes = Some(value);
    }

    /// Gets the value of LogoutOtherCodes
    pub fn get_logout_other_codes(&self) -> Option<&u32> {
        self.logout_other_codes.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }
}

