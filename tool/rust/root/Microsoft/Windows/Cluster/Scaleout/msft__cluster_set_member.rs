// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetMember struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetMember {

/// 
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u64>,

/// 
    #[serde(rename = "NetworkPrefixes")]
    pub network_prefixes: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,

/// 
    #[serde(rename = "TopologyLabel")]
    pub topology_label: Option<String>,

/// 
    #[serde(rename = "WorkloadCount")]
    pub workload_count: Option<u32>,
}

impl MSFT_ClusterSetMember {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_name: None,
            id: None,
            network_prefixes: None,
            state: None,
            tags: Vec::new(),
            topology_label: None,
            workload_count: None,
        }
    }


    /// Sets the value of ClusterName
    pub fn set_cluster_name(&mut self, value: String) {
        self.cluster_name = Some(value);
    }

    /// Gets the value of ClusterName
    pub fn get_cluster_name(&self) -> Option<&String> {
        self.cluster_name.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u64) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    /// Sets the value of NetworkPrefixes
    pub fn set_network_prefixes(&mut self, value: String) {
        self.network_prefixes = Some(value);
    }

    /// Gets the value of NetworkPrefixes
    pub fn get_network_prefixes(&self) -> Option<&String> {
        self.network_prefixes.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of Tags
    pub fn set_tags(&mut self, value: Vec<String>) {
        self.tags = value;
    }

    /// Gets the value of Tags
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    /// Sets the value of TopologyLabel
    pub fn set_topology_label(&mut self, value: String) {
        self.topology_label = Some(value);
    }

    /// Gets the value of TopologyLabel
    pub fn get_topology_label(&self) -> Option<&String> {
        self.topology_label.as_ref()
    }

    /// Sets the value of WorkloadCount
    pub fn set_workload_count(&mut self, value: u32) {
        self.workload_count = Some(value);
    }

    /// Gets the value of WorkloadCount
    pub fn get_workload_count(&self) -> Option<&u32> {
        self.workload_count.as_ref()
    }

/// 

    /// * `flags` -  (u32)
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn evict(&self, force: bool, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("Evict", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `key_type` -  (u32)

    /// * `key` -  (MSFT_ClusterSetKey)
    /// * `return_value` -  (u32)
    pub fn get_key(&self, key_type: u32, flags: u32, key: &mut MSFT_ClusterSetKey) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyType".to_string(), value: key_type.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("GetKey", &args)?;
        let key = result.get_value("key")?;
        Ok(result.return_value)

    }


/// 

    /// * `availability_set_name` -  (String)
    /// * `id` -  (String)
    /// * `name` -  (String)
    /// * `vm_config` -  (String)

    /// * `return_value` -  (u32)
    /// * `vm` -  (MSFT_ClusterSetVM)
    pub fn cluster_local_vm(&self, vm_config: &String, id: &String, name: &String, availability_set_name: &String, vm: &mut MSFT_ClusterSetVM) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "vmConfig".to_string(), value: vm_config.into() });
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: availability_set_name.into() });

        let result = self.invoke_method("ClusterLocalVm", &args)?;
        let vm = result.get_value("VM")?;
        Ok(result.return_value)

    }


/// 

    /// * `tag` -  (String[])

    /// * `return_value` -  (u32)
    pub fn add_tags(&self, tag: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Tag".to_string(), value: tag.into() });
        self.invoke_method("AddTags", &args)

    }


/// 

    /// * `tag` -  (String[])

    /// * `return_value` -  (u32)
    pub fn remove_tags(&self, tag: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Tag".to_string(), value: tag.into() });
        self.invoke_method("RemoveTags", &args)

    }


/// 

    /// * `availability_set_name` -  (String)
    /// * `register_all` -  (bool)
    /// * `vmid` -  (String)
    /// * `vmname` -  (String)

    /// * `return_value` -  (u32)
    /// * `vm` -  (MSFT_ClusterSetVM[])
    pub fn add_vm(&self, vmid: &String, vmname: &String, register_all: bool, availability_set_name: &String, vm: &mut Vec<MSFT_ClusterSetVM>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VMId".to_string(), value: vmid.into() });
        args.push(MethodParameter { name: "VMName".to_string(), value: vmname.into() });
        args.push(MethodParameter { name: "RegisterAll".to_string(), value: register_all.into() });
        args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: availability_set_name.into() });

        let result = self.invoke_method("AddVM", &args)?;
        let vm = result.get_value("VM")?;
        Ok(result.return_value)

    }

}

