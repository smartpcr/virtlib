// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.Security.MicrosoftTpm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __Win32Provider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __Win32Provider {
    #[serde(flatten)]
    pub base: __Provider,

/// 
    #[serde(rename = "ClientLoadableCLSID")]
    pub client_loadable_clsid: Option<String>,

/// 
    #[serde(rename = "CLSID")]
    pub clsid: Option<String>,

/// 
    #[serde(rename = "Concurrency")]
    pub concurrency: Option<i32>,

/// 
    #[serde(rename = "DefaultMachineName")]
    pub default_machine_name: Option<String>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "HostingModel")]
    pub hosting_model: Option<String>,

/// 
    #[serde(rename = "ImpersonationLevel")]
    pub impersonation_level: Option<Win32Provider_ImpersonationLevel>,

/// 
    #[serde(rename = "InitializationReentrancy")]
    pub initialization_reentrancy: Option<Win32Provider_InitializationReentrancy>,

/// 
    #[serde(rename = "InitializationTimeoutInterval")]
    pub initialization_timeout_interval: Option<String>,

/// 
    #[serde(rename = "InitializeAsAdminFirst")]
    pub initialize_as_admin_first: Option<bool>,

/// 
    #[serde(rename = "OperationTimeoutInterval")]
    pub operation_timeout_interval: Option<String>,

/// 
    #[serde(rename = "PerLocaleInitialization")]
    pub per_locale_initialization: Option<bool>,

/// 
    #[serde(rename = "PerUserInitialization")]
    pub per_user_initialization: Option<bool>,

/// 
    #[serde(rename = "Pure")]
    pub pure: Option<bool>,

/// 
    #[serde(rename = "SecurityDescriptor")]
    pub security_descriptor: Option<String>,

/// 
    #[serde(rename = "SupportsExplicitShutdown")]
    pub supports_explicit_shutdown: Option<bool>,

/// 
    #[serde(rename = "SupportsExtendedStatus")]
    pub supports_extended_status: Option<bool>,

/// 
    #[serde(rename = "SupportsQuotas")]
    pub supports_quotas: Option<bool>,

/// 
    #[serde(rename = "SupportsSendStatus")]
    pub supports_send_status: Option<bool>,

/// 
    #[serde(rename = "SupportsShutdown")]
    pub supports_shutdown: Option<bool>,

/// 
    #[serde(rename = "SupportsThrottling")]
    pub supports_throttling: Option<bool>,

/// 
    #[serde(rename = "UnloadTimeout")]
    pub unload_timeout: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<u32>,
}

impl __Win32Provider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Provider::new(),
            client_loadable_clsid: None,
            clsid: None,
            concurrency: None,
            default_machine_name: None,
            enabled: None,
            hosting_model: None,
            impersonation_level: None,
            initialization_reentrancy: None,
            initialization_timeout_interval: None,
            initialize_as_admin_first: None,
            operation_timeout_interval: None,
            per_locale_initialization: None,
            per_user_initialization: None,
            pure: None,
            security_descriptor: None,
            supports_explicit_shutdown: None,
            supports_extended_status: None,
            supports_quotas: None,
            supports_send_status: None,
            supports_shutdown: None,
            supports_throttling: None,
            unload_timeout: None,
            version: None,
        }
    }


    /// Sets the value of ClientLoadableCLSID
    pub fn set_client_loadable_clsid(&mut self, value: String) {
        self.client_loadable_clsid = Some(value);
    }

    /// Gets the value of ClientLoadableCLSID
    pub fn get_client_loadable_clsid(&self) -> Option<&String> {
        self.client_loadable_clsid.as_ref()
    }

    /// Sets the value of CLSID
    pub fn set_clsid(&mut self, value: String) {
        self.clsid = Some(value);
    }

    /// Gets the value of CLSID
    pub fn get_clsid(&self) -> Option<&String> {
        self.clsid.as_ref()
    }

    /// Sets the value of Concurrency
    pub fn set_concurrency(&mut self, value: i32) {
        self.concurrency = Some(value);
    }

    /// Gets the value of Concurrency
    pub fn get_concurrency(&self) -> Option<&i32> {
        self.concurrency.as_ref()
    }

    /// Sets the value of DefaultMachineName
    pub fn set_default_machine_name(&mut self, value: String) {
        self.default_machine_name = Some(value);
    }

    /// Gets the value of DefaultMachineName
    pub fn get_default_machine_name(&self) -> Option<&String> {
        self.default_machine_name.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of HostingModel
    pub fn set_hosting_model(&mut self, value: String) {
        self.hosting_model = Some(value);
    }

    /// Gets the value of HostingModel
    pub fn get_hosting_model(&self) -> Option<&String> {
        self.hosting_model.as_ref()
    }

    /// Sets the value of ImpersonationLevel
    pub fn set_impersonation_level(&mut self, value: Win32Provider_ImpersonationLevel) {
        self.impersonation_level = Some(value);
    }

    /// Gets the value of ImpersonationLevel
    pub fn get_impersonation_level(&self) -> Option<&Win32Provider_ImpersonationLevel> {
        self.impersonation_level.as_ref()
    }

    /// Sets the value of InitializationReentrancy
    pub fn set_initialization_reentrancy(&mut self, value: Win32Provider_InitializationReentrancy) {
        self.initialization_reentrancy = Some(value);
    }

    /// Gets the value of InitializationReentrancy
    pub fn get_initialization_reentrancy(&self) -> Option<&Win32Provider_InitializationReentrancy> {
        self.initialization_reentrancy.as_ref()
    }

    /// Sets the value of InitializationTimeoutInterval
    pub fn set_initialization_timeout_interval(&mut self, value: String) {
        self.initialization_timeout_interval = Some(value);
    }

    /// Gets the value of InitializationTimeoutInterval
    pub fn get_initialization_timeout_interval(&self) -> Option<&String> {
        self.initialization_timeout_interval.as_ref()
    }

    /// Sets the value of InitializeAsAdminFirst
    pub fn set_initialize_as_admin_first(&mut self, value: bool) {
        self.initialize_as_admin_first = Some(value);
    }

    /// Gets the value of InitializeAsAdminFirst
    pub fn get_initialize_as_admin_first(&self) -> Option<&bool> {
        self.initialize_as_admin_first.as_ref()
    }

    /// Sets the value of OperationTimeoutInterval
    pub fn set_operation_timeout_interval(&mut self, value: String) {
        self.operation_timeout_interval = Some(value);
    }

    /// Gets the value of OperationTimeoutInterval
    pub fn get_operation_timeout_interval(&self) -> Option<&String> {
        self.operation_timeout_interval.as_ref()
    }

    /// Sets the value of PerLocaleInitialization
    pub fn set_per_locale_initialization(&mut self, value: bool) {
        self.per_locale_initialization = Some(value);
    }

    /// Gets the value of PerLocaleInitialization
    pub fn get_per_locale_initialization(&self) -> Option<&bool> {
        self.per_locale_initialization.as_ref()
    }

    /// Sets the value of PerUserInitialization
    pub fn set_per_user_initialization(&mut self, value: bool) {
        self.per_user_initialization = Some(value);
    }

    /// Gets the value of PerUserInitialization
    pub fn get_per_user_initialization(&self) -> Option<&bool> {
        self.per_user_initialization.as_ref()
    }

    /// Sets the value of Pure
    pub fn set_pure(&mut self, value: bool) {
        self.pure = Some(value);
    }

    /// Gets the value of Pure
    pub fn get_pure(&self) -> Option<&bool> {
        self.pure.as_ref()
    }

    /// Sets the value of SecurityDescriptor
    pub fn set_security_descriptor(&mut self, value: String) {
        self.security_descriptor = Some(value);
    }

    /// Gets the value of SecurityDescriptor
    pub fn get_security_descriptor(&self) -> Option<&String> {
        self.security_descriptor.as_ref()
    }

    /// Sets the value of SupportsExplicitShutdown
    pub fn set_supports_explicit_shutdown(&mut self, value: bool) {
        self.supports_explicit_shutdown = Some(value);
    }

    /// Gets the value of SupportsExplicitShutdown
    pub fn get_supports_explicit_shutdown(&self) -> Option<&bool> {
        self.supports_explicit_shutdown.as_ref()
    }

    /// Sets the value of SupportsExtendedStatus
    pub fn set_supports_extended_status(&mut self, value: bool) {
        self.supports_extended_status = Some(value);
    }

    /// Gets the value of SupportsExtendedStatus
    pub fn get_supports_extended_status(&self) -> Option<&bool> {
        self.supports_extended_status.as_ref()
    }

    /// Sets the value of SupportsQuotas
    pub fn set_supports_quotas(&mut self, value: bool) {
        self.supports_quotas = Some(value);
    }

    /// Gets the value of SupportsQuotas
    pub fn get_supports_quotas(&self) -> Option<&bool> {
        self.supports_quotas.as_ref()
    }

    /// Sets the value of SupportsSendStatus
    pub fn set_supports_send_status(&mut self, value: bool) {
        self.supports_send_status = Some(value);
    }

    /// Gets the value of SupportsSendStatus
    pub fn get_supports_send_status(&self) -> Option<&bool> {
        self.supports_send_status.as_ref()
    }

    /// Sets the value of SupportsShutdown
    pub fn set_supports_shutdown(&mut self, value: bool) {
        self.supports_shutdown = Some(value);
    }

    /// Gets the value of SupportsShutdown
    pub fn get_supports_shutdown(&self) -> Option<&bool> {
        self.supports_shutdown.as_ref()
    }

    /// Sets the value of SupportsThrottling
    pub fn set_supports_throttling(&mut self, value: bool) {
        self.supports_throttling = Some(value);
    }

    /// Gets the value of SupportsThrottling
    pub fn get_supports_throttling(&self) -> Option<&bool> {
        self.supports_throttling.as_ref()
    }

    /// Sets the value of UnloadTimeout
    pub fn set_unload_timeout(&mut self, value: String) {
        self.unload_timeout = Some(value);
    }

    /// Gets the value of UnloadTimeout
    pub fn get_unload_timeout(&self) -> Option<&String> {
        self.unload_timeout.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: u32) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&u32> {
        self.version.as_ref()
    }
}

