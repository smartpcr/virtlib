// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_WindowsConnectionManager02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_WindowsConnectionManager02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ProhitConnectionToNonDomainNetworksWhenConnectedToDomainAuthenticatedNetwork")]
    pub prohit_connection_to_non_domain_networks_when_connected_to_domain_authenticated_network: Option<String>,
}

impl MDM_Policy_Config01_WindowsConnectionManager02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            prohit_connection_to_non_domain_networks_when_connected_to_domain_authenticated_network: None,
        }
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

    /// Sets the value of ProhitConnectionToNonDomainNetworksWhenConnectedToDomainAuthenticatedNetwork
    pub fn set_prohit_connection_to_non_domain_networks_when_connected_to_domain_authenticated_network(&mut self, value: String) {
        self.prohit_connection_to_non_domain_networks_when_connected_to_domain_authenticated_network = Some(value);
    }

    /// Gets the value of ProhitConnectionToNonDomainNetworksWhenConnectedToDomainAuthenticatedNetwork
    pub fn get_prohit_connection_to_non_domain_networks_when_connected_to_domain_authenticated_network(&self) -> Option<&String> {
        self.prohit_connection_to_non_domain_networks_when_connected_to_domain_authenticated_network.as_ref()
    }
}

