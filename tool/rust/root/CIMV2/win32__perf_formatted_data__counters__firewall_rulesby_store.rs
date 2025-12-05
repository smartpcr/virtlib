// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_FirewallRulesbyStore struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_FirewallRulesbyStore {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AppIsolationStore")]
    pub app_isolation_store: Option<u64>,

/// 
    #[serde(rename = "DefaultsStore")]
    pub defaults_store: Option<u64>,

/// 
    #[serde(rename = "DynamicStore")]
    pub dynamic_store: Option<u64>,

/// 
    #[serde(rename = "GPOStore")]
    pub gpostore: Option<u64>,

/// 
    #[serde(rename = "GroupPolicyRSoPStore")]
    pub group_policy_rso_pstore: Option<u64>,

/// 
    #[serde(rename = "InterfaceIsolationDynamicStore")]
    pub interface_isolation_dynamic_store: Option<u64>,

/// 
    #[serde(rename = "InterfaceIsolationStore")]
    pub interface_isolation_store: Option<u64>,

/// 
    #[serde(rename = "LocalStore")]
    pub local_store: Option<u64>,

/// 
    #[serde(rename = "MDMStore")]
    pub mdmstore: Option<u64>,

/// 
    #[serde(rename = "TenantRestrictionsStore")]
    pub tenant_restrictions_store: Option<u64>,

/// 
    #[serde(rename = "Total")]
    pub total: Option<u64>,

/// 
    #[serde(rename = "WSHConfigurableStore")]
    pub wshconfigurable_store: Option<u64>,

/// 
    #[serde(rename = "WSHStaticStore")]
    pub wshstatic_store: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_FirewallRulesbyStore {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            app_isolation_store: None,
            defaults_store: None,
            dynamic_store: None,
            gpostore: None,
            group_policy_rso_pstore: None,
            interface_isolation_dynamic_store: None,
            interface_isolation_store: None,
            local_store: None,
            mdmstore: None,
            tenant_restrictions_store: None,
            total: None,
            wshconfigurable_store: None,
            wshstatic_store: None,
        }
    }


    /// Sets the value of AppIsolationStore
    pub fn set_app_isolation_store(&mut self, value: u64) {
        self.app_isolation_store = Some(value);
    }

    /// Gets the value of AppIsolationStore
    pub fn get_app_isolation_store(&self) -> Option<&u64> {
        self.app_isolation_store.as_ref()
    }

    /// Sets the value of DefaultsStore
    pub fn set_defaults_store(&mut self, value: u64) {
        self.defaults_store = Some(value);
    }

    /// Gets the value of DefaultsStore
    pub fn get_defaults_store(&self) -> Option<&u64> {
        self.defaults_store.as_ref()
    }

    /// Sets the value of DynamicStore
    pub fn set_dynamic_store(&mut self, value: u64) {
        self.dynamic_store = Some(value);
    }

    /// Gets the value of DynamicStore
    pub fn get_dynamic_store(&self) -> Option<&u64> {
        self.dynamic_store.as_ref()
    }

    /// Sets the value of GPOStore
    pub fn set_gpostore(&mut self, value: u64) {
        self.gpostore = Some(value);
    }

    /// Gets the value of GPOStore
    pub fn get_gpostore(&self) -> Option<&u64> {
        self.gpostore.as_ref()
    }

    /// Sets the value of GroupPolicyRSoPStore
    pub fn set_group_policy_rso_pstore(&mut self, value: u64) {
        self.group_policy_rso_pstore = Some(value);
    }

    /// Gets the value of GroupPolicyRSoPStore
    pub fn get_group_policy_rso_pstore(&self) -> Option<&u64> {
        self.group_policy_rso_pstore.as_ref()
    }

    /// Sets the value of InterfaceIsolationDynamicStore
    pub fn set_interface_isolation_dynamic_store(&mut self, value: u64) {
        self.interface_isolation_dynamic_store = Some(value);
    }

    /// Gets the value of InterfaceIsolationDynamicStore
    pub fn get_interface_isolation_dynamic_store(&self) -> Option<&u64> {
        self.interface_isolation_dynamic_store.as_ref()
    }

    /// Sets the value of InterfaceIsolationStore
    pub fn set_interface_isolation_store(&mut self, value: u64) {
        self.interface_isolation_store = Some(value);
    }

    /// Gets the value of InterfaceIsolationStore
    pub fn get_interface_isolation_store(&self) -> Option<&u64> {
        self.interface_isolation_store.as_ref()
    }

    /// Sets the value of LocalStore
    pub fn set_local_store(&mut self, value: u64) {
        self.local_store = Some(value);
    }

    /// Gets the value of LocalStore
    pub fn get_local_store(&self) -> Option<&u64> {
        self.local_store.as_ref()
    }

    /// Sets the value of MDMStore
    pub fn set_mdmstore(&mut self, value: u64) {
        self.mdmstore = Some(value);
    }

    /// Gets the value of MDMStore
    pub fn get_mdmstore(&self) -> Option<&u64> {
        self.mdmstore.as_ref()
    }

    /// Sets the value of TenantRestrictionsStore
    pub fn set_tenant_restrictions_store(&mut self, value: u64) {
        self.tenant_restrictions_store = Some(value);
    }

    /// Gets the value of TenantRestrictionsStore
    pub fn get_tenant_restrictions_store(&self) -> Option<&u64> {
        self.tenant_restrictions_store.as_ref()
    }

    /// Sets the value of Total
    pub fn set_total(&mut self, value: u64) {
        self.total = Some(value);
    }

    /// Gets the value of Total
    pub fn get_total(&self) -> Option<&u64> {
        self.total.as_ref()
    }

    /// Sets the value of WSHConfigurableStore
    pub fn set_wshconfigurable_store(&mut self, value: u64) {
        self.wshconfigurable_store = Some(value);
    }

    /// Gets the value of WSHConfigurableStore
    pub fn get_wshconfigurable_store(&self) -> Option<&u64> {
        self.wshconfigurable_store.as_ref()
    }

    /// Sets the value of WSHStaticStore
    pub fn set_wshstatic_store(&mut self, value: u64) {
        self.wshstatic_store = Some(value);
    }

    /// Gets the value of WSHStaticStore
    pub fn get_wshstatic_store(&self) -> Option<&u64> {
        self.wshstatic_store.as_ref()
    }
}

