// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_DiscoveryConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_DiscoveryConfig {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// TRUE if adapter should perform automatic discovery of iSNS server.
    #[serde(rename = "AutomaticiSNSDiscovery")]
    pub automatici_snsdiscovery: Option<bool>,

/// Default initiator name for registering with iSNS.
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// If AutomaticiSNSDiscovery is FALSE then this contains the fixed addresses of iSNS servers
    #[serde(rename = "iSNSServer")]
    pub i_snsserver: Option<ISCSI_IP_Address>,

/// TRUE if adapter should perform target discovery via iSNS.
    #[serde(rename = "PerformiSNSDiscovery")]
    pub performi_snsdiscovery: Option<bool>,

/// TRUE if adapter should perform target discovery via SLP.
    #[serde(rename = "PerformSLPDiscovery")]
    pub perform_slpdiscovery: Option<bool>,
}

impl MSiSCSI_DiscoveryConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            automatici_snsdiscovery: None,
            initiator_name: None,
            instance_name: None,
            i_snsserver: None,
            performi_snsdiscovery: None,
            perform_slpdiscovery: None,
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

    /// Sets the value of AutomaticiSNSDiscovery
    pub fn set_automatici_snsdiscovery(&mut self, value: bool) {
        self.automatici_snsdiscovery = Some(value);
    }

    /// Gets the value of AutomaticiSNSDiscovery
    pub fn get_automatici_snsdiscovery(&self) -> Option<&bool> {
        self.automatici_snsdiscovery.as_ref()
    }

    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of iSNSServer
    pub fn set_i_snsserver(&mut self, value: ISCSI_IP_Address) {
        self.i_snsserver = Some(value);
    }

    /// Gets the value of iSNSServer
    pub fn get_i_snsserver(&self) -> Option<&ISCSI_IP_Address> {
        self.i_snsserver.as_ref()
    }

    /// Sets the value of PerformiSNSDiscovery
    pub fn set_performi_snsdiscovery(&mut self, value: bool) {
        self.performi_snsdiscovery = Some(value);
    }

    /// Gets the value of PerformiSNSDiscovery
    pub fn get_performi_snsdiscovery(&self) -> Option<&bool> {
        self.performi_snsdiscovery.as_ref()
    }

    /// Sets the value of PerformSLPDiscovery
    pub fn set_perform_slpdiscovery(&mut self, value: bool) {
        self.perform_slpdiscovery = Some(value);
    }

    /// Gets the value of PerformSLPDiscovery
    pub fn get_perform_slpdiscovery(&self) -> Option<&bool> {
        self.perform_slpdiscovery.as_ref()
    }
}

