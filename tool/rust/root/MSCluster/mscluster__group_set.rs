// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_GroupSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_GroupSet {

/// 
    #[serde(rename = "ClusterNodeObjectReturnedFrom")]
    pub cluster_node_object_returned_from: Option<String>,

/// 
    #[serde(rename = "FaultDomains")]
    pub fault_domains: Option<u32>,

/// 
    #[serde(rename = "GroupNames")]
    pub group_names: Vec<String>,

/// 
    #[serde(rename = "IsAvailabilitySet")]
    pub is_availability_set: Option<bool>,

/// 
    #[serde(rename = "IsGlobal")]
    pub is_global: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NodeDomainInfo")]
    pub node_domain_info: Vec<String>,

/// 
    #[serde(rename = "ProviderNames")]
    pub provider_names: Vec<String>,

/// 
    #[serde(rename = "ReserveSpareNode")]
    pub reserve_spare_node: Option<bool>,

/// 
    #[serde(rename = "StartupCount")]
    pub startup_count: Option<u32>,

/// 
    #[serde(rename = "StartupDelay")]
    pub startup_delay: Option<u32>,

/// 
    #[serde(rename = "StartupDelayTrigger")]
    pub startup_delay_trigger: Option<u32>,

/// 
    #[serde(rename = "StatusInformation")]
    pub status_information: Option<u64>,

/// 
    #[serde(rename = "UpdateDomains")]
    pub update_domains: Option<u32>,
}

impl MSCluster_GroupSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_node_object_returned_from: None,
            fault_domains: None,
            group_names: Vec::new(),
            is_availability_set: None,
            is_global: None,
            name: None,
            node_domain_info: Vec::new(),
            provider_names: Vec::new(),
            reserve_spare_node: None,
            startup_count: None,
            startup_delay: None,
            startup_delay_trigger: None,
            status_information: None,
            update_domains: None,
        }
    }


    /// Sets the value of ClusterNodeObjectReturnedFrom
    pub fn set_cluster_node_object_returned_from(&mut self, value: String) {
        self.cluster_node_object_returned_from = Some(value);
    }

    /// Gets the value of ClusterNodeObjectReturnedFrom
    pub fn get_cluster_node_object_returned_from(&self) -> Option<&String> {
        self.cluster_node_object_returned_from.as_ref()
    }

    /// Sets the value of FaultDomains
    pub fn set_fault_domains(&mut self, value: u32) {
        self.fault_domains = Some(value);
    }

    /// Gets the value of FaultDomains
    pub fn get_fault_domains(&self) -> Option<&u32> {
        self.fault_domains.as_ref()
    }

    /// Sets the value of GroupNames
    pub fn set_group_names(&mut self, value: Vec<String>) {
        self.group_names = value;
    }

    /// Gets the value of GroupNames
    pub fn get_group_names(&self) -> &Vec<String> {
        &self.group_names
    }

    /// Sets the value of IsAvailabilitySet
    pub fn set_is_availability_set(&mut self, value: bool) {
        self.is_availability_set = Some(value);
    }

    /// Gets the value of IsAvailabilitySet
    pub fn get_is_availability_set(&self) -> Option<&bool> {
        self.is_availability_set.as_ref()
    }

    /// Sets the value of IsGlobal
    pub fn set_is_global(&mut self, value: bool) {
        self.is_global = Some(value);
    }

    /// Gets the value of IsGlobal
    pub fn get_is_global(&self) -> Option<&bool> {
        self.is_global.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NodeDomainInfo
    pub fn set_node_domain_info(&mut self, value: Vec<String>) {
        self.node_domain_info = value;
    }

    /// Gets the value of NodeDomainInfo
    pub fn get_node_domain_info(&self) -> &Vec<String> {
        &self.node_domain_info
    }

    /// Sets the value of ProviderNames
    pub fn set_provider_names(&mut self, value: Vec<String>) {
        self.provider_names = value;
    }

    /// Gets the value of ProviderNames
    pub fn get_provider_names(&self) -> &Vec<String> {
        &self.provider_names
    }

    /// Sets the value of ReserveSpareNode
    pub fn set_reserve_spare_node(&mut self, value: bool) {
        self.reserve_spare_node = Some(value);
    }

    /// Gets the value of ReserveSpareNode
    pub fn get_reserve_spare_node(&self) -> Option<&bool> {
        self.reserve_spare_node.as_ref()
    }

    /// Sets the value of StartupCount
    pub fn set_startup_count(&mut self, value: u32) {
        self.startup_count = Some(value);
    }

    /// Gets the value of StartupCount
    pub fn get_startup_count(&self) -> Option<&u32> {
        self.startup_count.as_ref()
    }

    /// Sets the value of StartupDelay
    pub fn set_startup_delay(&mut self, value: u32) {
        self.startup_delay = Some(value);
    }

    /// Gets the value of StartupDelay
    pub fn get_startup_delay(&self) -> Option<&u32> {
        self.startup_delay.as_ref()
    }

    /// Sets the value of StartupDelayTrigger
    pub fn set_startup_delay_trigger(&mut self, value: u32) {
        self.startup_delay_trigger = Some(value);
    }

    /// Gets the value of StartupDelayTrigger
    pub fn get_startup_delay_trigger(&self) -> Option<&u32> {
        self.startup_delay_trigger.as_ref()
    }

    /// Sets the value of StatusInformation
    pub fn set_status_information(&mut self, value: u64) {
        self.status_information = Some(value);
    }

    /// Gets the value of StatusInformation
    pub fn get_status_information(&self) -> Option<&u64> {
        self.status_information.as_ref()
    }

    /// Sets the value of UpdateDomains
    pub fn set_update_domains(&mut self, value: u32) {
        self.update_domains = Some(value);
    }

    /// Gets the value of UpdateDomains
    pub fn get_update_domains(&self) -> Option<&u32> {
        self.update_domains.as_ref()
    }

/// 

    /// * `group` -  (String[])
    /// * `name` -  (String)
    /// * `provider` -  (String[])

    /// * `created_set` -  (MSCluster_GroupSet)
    /// * `return_value` -  (u32)
    pub fn create_set(&self, name: &String, group: &Vec<String>, provider: &Vec<String>, created_set: &mut MSCluster_GroupSet) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        args.push(MethodParameter { name: "provider".to_string(), value: provider.into() });

        let result = self.invoke_method("CreateSet", &args)?;
        let created_set = result.get_value("CreatedSet")?;
        Ok(result.return_value)

    }


/// 

    /// * `fault_domains` -  (u32)
    /// * `group` -  (String[])
    /// * `name` -  (String)
    /// * `reserve_spare_node` -  (bool)
    /// * `update_domains` -  (u32)

    /// * `created_set` -  (MSCluster_GroupSet)
    /// * `return_value` -  (u32)
    pub fn create_availability_set(&self, name: &String, group: &Vec<String>, update_domains: u32, fault_domains: u32, reserve_spare_node: bool, created_set: &mut MSCluster_GroupSet) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        args.push(MethodParameter { name: "UpdateDomains".to_string(), value: update_domains.into() });
        args.push(MethodParameter { name: "FaultDomains".to_string(), value: fault_domains.into() });
        args.push(MethodParameter { name: "ReserveSpareNode".to_string(), value: reserve_spare_node.into() });

        let result = self.invoke_method("CreateAvailabilitySet", &args)?;
        let created_set = result.get_value("CreatedSet")?;
        Ok(result.return_value)

    }


/// 

    /// * `contained_group` -  (String)
    /// * `dependent_group` -  (String)
    /// * `name` -  (String)
    /// * `provider` -  (String)

    /// * `return_value` -  (u32)
    /// * `sets` -  (MSCluster_GroupSet[])
    pub fn get_set_from(&self, contained_group: &String, name: &String, provider: &String, dependent_group: &String, sets: &mut Vec<MSCluster_GroupSet>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ContainedGroup".to_string(), value: contained_group.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "provider".to_string(), value: provider.into() });
        args.push(MethodParameter { name: "DependentGroup".to_string(), value: dependent_group.into() });

        let result = self.invoke_method("GetSetFrom", &args)?;
        let sets = result.get_value("Sets")?;
        Ok(result.return_value)

    }


/// 

    /// * `provider` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_set_provider(&self, provider: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "provider".to_string(), value: provider.into() });
        self.invoke_method("AddSetProvider", &args)

    }


/// 

    /// * `provider` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_set_provider(&self, provider: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "provider".to_string(), value: provider.into() });
        self.invoke_method("RemoveSetProvider", &args)

    }


/// 

    /// * `group` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_group_to_set(&self, group: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        self.invoke_method("AddGroupToSet", &args)

    }


/// 

    /// * `fault_domain` -  (u32)
    /// * `group` -  (String)
    /// * `reserved` -  (u64)
    /// * `update_domain` -  (u32)

    /// * `return_value` -  (u32)
    pub fn add_group_to_set_ex(&self, group: &String, fault_domain: u32, update_domain: u32, reserved: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        args.push(MethodParameter { name: "FaultDomain".to_string(), value: fault_domain.into() });
        args.push(MethodParameter { name: "UpdateDomain".to_string(), value: update_domain.into() });
        args.push(MethodParameter { name: "Reserved".to_string(), value: reserved.into() });
        self.invoke_method("AddGroupToSetEx", &args)

    }


/// 

    /// * `group` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_group_from_set(&self, group: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        self.invoke_method("RemoveGroupFromSet", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn remove(&self) -> Result<(), WmiError> {
        self.invoke_method("Remove", &[])

    }


/// 

    /// * `is_global` -  (bool)
    /// * `startup_count` -  (u32)
    /// * `startup_delay` -  (u32)
    /// * `startup_delay_trigger` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_set(&self, startup_delay_trigger: u32, startup_count: u32, is_global: bool, startup_delay: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StartupDelayTrigger".to_string(), value: startup_delay_trigger.into() });
        args.push(MethodParameter { name: "StartupCount".to_string(), value: startup_count.into() });
        args.push(MethodParameter { name: "IsGlobal".to_string(), value: is_global.into() });
        args.push(MethodParameter { name: "StartupDelay".to_string(), value: startup_delay.into() });
        self.invoke_method("SetSet", &args)

    }


/// 

    /// * `dependent_group` -  (String)
    /// * `provider_group` -  (String)

    /// * `groups` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_groups(&self, dependent_group: &String, provider_group: &String, groups: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DependentGroup".to_string(), value: dependent_group.into() });
        args.push(MethodParameter { name: "ProviderGroup".to_string(), value: provider_group.into() });

        let result = self.invoke_method("GetGroups", &args)?;
        let groups = result.get_value("Groups")?;
        Ok(result.return_value)

    }


/// 

    /// * `group` -  (String)
    /// * `provider` -  (String)
    /// * `provider_group` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_provider_for_group(&self, group: &String, provider_group: &String, provider: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        args.push(MethodParameter { name: "ProviderGroup".to_string(), value: provider_group.into() });
        args.push(MethodParameter { name: "provider".to_string(), value: provider.into() });
        self.invoke_method("RemoveProviderForGroup", &args)

    }


/// 

    /// * `group` -  (String)
    /// * `provider` -  (String)
    /// * `provider_group` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_provider_for_group(&self, group: &String, provider_group: &String, provider: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        args.push(MethodParameter { name: "ProviderGroup".to_string(), value: provider_group.into() });
        args.push(MethodParameter { name: "provider".to_string(), value: provider.into() });
        self.invoke_method("AddProviderForGroup", &args)

    }

}

