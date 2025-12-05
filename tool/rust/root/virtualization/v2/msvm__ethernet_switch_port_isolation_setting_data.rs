// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortIsolationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortIsolationSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "AllowUntaggedTraffic")]
    pub allow_untagged_traffic: Option<bool>,

/// 
    #[serde(rename = "DefaultIsolationId")]
    pub default_isolation_id: Option<u32>,

/// 
    #[serde(rename = "EnableMultiTenantStack")]
    pub enable_multi_tenant_stack: Option<bool>,

/// 
    #[serde(rename = "IsolationMode")]
    pub isolation_mode: Option<u32>,
}

impl Msvm_EthernetSwitchPortIsolationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            allow_untagged_traffic: None,
            default_isolation_id: None,
            enable_multi_tenant_stack: None,
            isolation_mode: None,
        }
    }


    /// Sets the value of AllowUntaggedTraffic
    pub fn set_allow_untagged_traffic(&mut self, value: bool) {
        self.allow_untagged_traffic = Some(value);
    }

    /// Gets the value of AllowUntaggedTraffic
    pub fn get_allow_untagged_traffic(&self) -> Option<&bool> {
        self.allow_untagged_traffic.as_ref()
    }

    /// Sets the value of DefaultIsolationId
    pub fn set_default_isolation_id(&mut self, value: u32) {
        self.default_isolation_id = Some(value);
    }

    /// Gets the value of DefaultIsolationId
    pub fn get_default_isolation_id(&self) -> Option<&u32> {
        self.default_isolation_id.as_ref()
    }

    /// Sets the value of EnableMultiTenantStack
    pub fn set_enable_multi_tenant_stack(&mut self, value: bool) {
        self.enable_multi_tenant_stack = Some(value);
    }

    /// Gets the value of EnableMultiTenantStack
    pub fn get_enable_multi_tenant_stack(&self) -> Option<&bool> {
        self.enable_multi_tenant_stack.as_ref()
    }

    /// Sets the value of IsolationMode
    pub fn set_isolation_mode(&mut self, value: u32) {
        self.isolation_mode = Some(value);
    }

    /// Gets the value of IsolationMode
    pub fn get_isolation_mode(&self) -> Option<&u32> {
        self.isolation_mode.as_ref()
    }
}

impl Msvm_EthernetSwitchPortIsolationSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

