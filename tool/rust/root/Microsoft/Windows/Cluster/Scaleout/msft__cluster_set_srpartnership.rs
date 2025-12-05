// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetSRPartnership struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetSRPartnership {

/// 
    #[serde(rename = "DestinationClusterId")]
    pub destination_cluster_id: Option<u64>,

/// 
    #[serde(rename = "DestinationClusterName")]
    pub destination_cluster_name: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PartnershipId")]
    pub partnership_id: Option<String>,

/// 
    #[serde(rename = "SourceClusterId")]
    pub source_cluster_id: Option<u64>,

/// 
    #[serde(rename = "SourceClusterName")]
    pub source_cluster_name: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSFT_ClusterSetSRPartnership {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            destination_cluster_id: None,
            destination_cluster_name: None,
            id: None,
            name: None,
            partnership_id: None,
            source_cluster_id: None,
            source_cluster_name: None,
            state: None,
        }
    }


    /// Sets the value of DestinationClusterId
    pub fn set_destination_cluster_id(&mut self, value: u64) {
        self.destination_cluster_id = Some(value);
    }

    /// Gets the value of DestinationClusterId
    pub fn get_destination_cluster_id(&self) -> Option<&u64> {
        self.destination_cluster_id.as_ref()
    }

    /// Sets the value of DestinationClusterName
    pub fn set_destination_cluster_name(&mut self, value: String) {
        self.destination_cluster_name = Some(value);
    }

    /// Gets the value of DestinationClusterName
    pub fn get_destination_cluster_name(&self) -> Option<&String> {
        self.destination_cluster_name.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u64) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of PartnershipId
    pub fn set_partnership_id(&mut self, value: String) {
        self.partnership_id = Some(value);
    }

    /// Gets the value of PartnershipId
    pub fn get_partnership_id(&self) -> Option<&String> {
        self.partnership_id.as_ref()
    }

    /// Sets the value of SourceClusterId
    pub fn set_source_cluster_id(&mut self, value: u64) {
        self.source_cluster_id = Some(value);
    }

    /// Gets the value of SourceClusterId
    pub fn get_source_cluster_id(&self) -> Option<&u64> {
        self.source_cluster_id.as_ref()
    }

    /// Sets the value of SourceClusterName
    pub fn set_source_cluster_name(&mut self, value: String) {
        self.source_cluster_name = Some(value);
    }

    /// Gets the value of SourceClusterName
    pub fn get_source_cluster_name(&self) -> Option<&String> {
        self.source_cluster_name.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_srpartnership(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveSRPartnership", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_srpartnership_properties(&self, name: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("SetSRPartnershipProperties", &args)

    }

}

