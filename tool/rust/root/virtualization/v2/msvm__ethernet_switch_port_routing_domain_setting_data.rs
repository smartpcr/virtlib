// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortRoutingDomainSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortRoutingDomainSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "IsolationIdList")]
    pub isolation_id_list: Vec<u32>,

/// 
    #[serde(rename = "IsolationIdNameList")]
    pub isolation_id_name_list: Vec<String>,

/// 
    #[serde(rename = "RoutingDomainGuid")]
    pub routing_domain_guid: Option<String>,

/// 
    #[serde(rename = "RoutingDomainName")]
    pub routing_domain_name: Option<String>,
}

impl Msvm_EthernetSwitchPortRoutingDomainSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            isolation_id_list: Vec::new(),
            isolation_id_name_list: Vec::new(),
            routing_domain_guid: None,
            routing_domain_name: None,
        }
    }


    /// Sets the value of IsolationIdList
    pub fn set_isolation_id_list(&mut self, value: Vec<u32>) {
        self.isolation_id_list = value;
    }

    /// Gets the value of IsolationIdList
    pub fn get_isolation_id_list(&self) -> &Vec<u32> {
        &self.isolation_id_list
    }

    /// Sets the value of IsolationIdNameList
    pub fn set_isolation_id_name_list(&mut self, value: Vec<String>) {
        self.isolation_id_name_list = value;
    }

    /// Gets the value of IsolationIdNameList
    pub fn get_isolation_id_name_list(&self) -> &Vec<String> {
        &self.isolation_id_name_list
    }

    /// Sets the value of RoutingDomainGuid
    pub fn set_routing_domain_guid(&mut self, value: String) {
        self.routing_domain_guid = Some(value);
    }

    /// Gets the value of RoutingDomainGuid
    pub fn get_routing_domain_guid(&self) -> Option<&String> {
        self.routing_domain_guid.as_ref()
    }

    /// Sets the value of RoutingDomainName
    pub fn set_routing_domain_name(&mut self, value: String) {
        self.routing_domain_name = Some(value);
    }

    /// Gets the value of RoutingDomainName
    pub fn get_routing_domain_name(&self) -> Option<&String> {
        self.routing_domain_name.as_ref()
    }
}

impl Msvm_EthernetSwitchPortRoutingDomainSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

