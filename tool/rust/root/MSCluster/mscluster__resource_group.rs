// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ResourceGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ResourceGroup {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "AntiAffinityClassNames")]
    pub anti_affinity_class_names: Vec<String>,

/// 
    #[serde(rename = "AutoFailbackType")]
    pub auto_failback_type: Option<u32>,

/// 
    #[serde(rename = "CCFEpoch")]
    pub ccfepoch: Option<u64>,

/// 
    #[serde(rename = "CCFEpochHigh")]
    pub ccfepoch_high: Option<u64>,

/// 
    #[serde(rename = "ColdStartSetting")]
    pub cold_start_setting: Option<u32>,

/// 
    #[serde(rename = "DefaultOwner")]
    pub default_owner: Option<u32>,

/// 
    #[serde(rename = "FailbackWindowEnd")]
    pub failback_window_end: Option<i32>,

/// 
    #[serde(rename = "FailbackWindowStart")]
    pub failback_window_start: Option<i32>,

/// 
    #[serde(rename = "FailoverPeriod")]
    pub failover_period: Option<u32>,

/// 
    #[serde(rename = "FailoverThreshold")]
    pub failover_threshold: Option<u32>,

/// 
    #[serde(rename = "FaultDomain")]
    pub fault_domain: Option<u32>,

/// 
    #[serde(rename = "GroupType")]
    pub group_type: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IsCore")]
    pub is_core: Option<bool>,

/// 
    #[serde(rename = "LockedFromMoving")]
    pub locked_from_moving: Option<u32>,

/// 
    #[serde(rename = "OwnerNode")]
    pub owner_node: Option<String>,

/// 
    #[serde(rename = "PersistentState")]
    pub persistent_state: Option<bool>,

/// 
    #[serde(rename = "PlacementOptions")]
    pub placement_options: Option<u32>,

/// 
    #[serde(rename = "PreferredSite")]
    pub preferred_site: Vec<String>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "PrivateProperties")]
    pub private_properties: Option<MSCluster_Property>,

/// 
    #[serde(rename = "ResiliencyPeriod")]
    pub resiliency_period: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "StatusInformation")]
    pub status_information: Option<u64>,

/// 
    #[serde(rename = "UpdateDomain")]
    pub update_domain: Option<u32>,
}

impl MSCluster_ResourceGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            anti_affinity_class_names: Vec::new(),
            auto_failback_type: None,
            ccfepoch: None,
            ccfepoch_high: None,
            cold_start_setting: None,
            default_owner: None,
            failback_window_end: None,
            failback_window_start: None,
            failover_period: None,
            failover_threshold: None,
            fault_domain: None,
            group_type: None,
            id: None,
            is_core: None,
            locked_from_moving: None,
            owner_node: None,
            persistent_state: None,
            placement_options: None,
            preferred_site: Vec::new(),
            priority: None,
            private_properties: None,
            resiliency_period: None,
            state: None,
            status_information: None,
            update_domain: None,
        }
    }


    /// Sets the value of AntiAffinityClassNames
    pub fn set_anti_affinity_class_names(&mut self, value: Vec<String>) {
        self.anti_affinity_class_names = value;
    }

    /// Gets the value of AntiAffinityClassNames
    pub fn get_anti_affinity_class_names(&self) -> &Vec<String> {
        &self.anti_affinity_class_names
    }

    /// Sets the value of AutoFailbackType
    pub fn set_auto_failback_type(&mut self, value: u32) {
        self.auto_failback_type = Some(value);
    }

    /// Gets the value of AutoFailbackType
    pub fn get_auto_failback_type(&self) -> Option<&u32> {
        self.auto_failback_type.as_ref()
    }

    /// Sets the value of CCFEpoch
    pub fn set_ccfepoch(&mut self, value: u64) {
        self.ccfepoch = Some(value);
    }

    /// Gets the value of CCFEpoch
    pub fn get_ccfepoch(&self) -> Option<&u64> {
        self.ccfepoch.as_ref()
    }

    /// Sets the value of CCFEpochHigh
    pub fn set_ccfepoch_high(&mut self, value: u64) {
        self.ccfepoch_high = Some(value);
    }

    /// Gets the value of CCFEpochHigh
    pub fn get_ccfepoch_high(&self) -> Option<&u64> {
        self.ccfepoch_high.as_ref()
    }

    /// Sets the value of ColdStartSetting
    pub fn set_cold_start_setting(&mut self, value: u32) {
        self.cold_start_setting = Some(value);
    }

    /// Gets the value of ColdStartSetting
    pub fn get_cold_start_setting(&self) -> Option<&u32> {
        self.cold_start_setting.as_ref()
    }

    /// Sets the value of DefaultOwner
    pub fn set_default_owner(&mut self, value: u32) {
        self.default_owner = Some(value);
    }

    /// Gets the value of DefaultOwner
    pub fn get_default_owner(&self) -> Option<&u32> {
        self.default_owner.as_ref()
    }

    /// Sets the value of FailbackWindowEnd
    pub fn set_failback_window_end(&mut self, value: i32) {
        self.failback_window_end = Some(value);
    }

    /// Gets the value of FailbackWindowEnd
    pub fn get_failback_window_end(&self) -> Option<&i32> {
        self.failback_window_end.as_ref()
    }

    /// Sets the value of FailbackWindowStart
    pub fn set_failback_window_start(&mut self, value: i32) {
        self.failback_window_start = Some(value);
    }

    /// Gets the value of FailbackWindowStart
    pub fn get_failback_window_start(&self) -> Option<&i32> {
        self.failback_window_start.as_ref()
    }

    /// Sets the value of FailoverPeriod
    pub fn set_failover_period(&mut self, value: u32) {
        self.failover_period = Some(value);
    }

    /// Gets the value of FailoverPeriod
    pub fn get_failover_period(&self) -> Option<&u32> {
        self.failover_period.as_ref()
    }

    /// Sets the value of FailoverThreshold
    pub fn set_failover_threshold(&mut self, value: u32) {
        self.failover_threshold = Some(value);
    }

    /// Gets the value of FailoverThreshold
    pub fn get_failover_threshold(&self) -> Option<&u32> {
        self.failover_threshold.as_ref()
    }

    /// Sets the value of FaultDomain
    pub fn set_fault_domain(&mut self, value: u32) {
        self.fault_domain = Some(value);
    }

    /// Gets the value of FaultDomain
    pub fn get_fault_domain(&self) -> Option<&u32> {
        self.fault_domain.as_ref()
    }

    /// Sets the value of GroupType
    pub fn set_group_type(&mut self, value: u32) {
        self.group_type = Some(value);
    }

    /// Gets the value of GroupType
    pub fn get_group_type(&self) -> Option<&u32> {
        self.group_type.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IsCore
    pub fn set_is_core(&mut self, value: bool) {
        self.is_core = Some(value);
    }

    /// Gets the value of IsCore
    pub fn get_is_core(&self) -> Option<&bool> {
        self.is_core.as_ref()
    }

    /// Sets the value of LockedFromMoving
    pub fn set_locked_from_moving(&mut self, value: u32) {
        self.locked_from_moving = Some(value);
    }

    /// Gets the value of LockedFromMoving
    pub fn get_locked_from_moving(&self) -> Option<&u32> {
        self.locked_from_moving.as_ref()
    }

    /// Sets the value of OwnerNode
    pub fn set_owner_node(&mut self, value: String) {
        self.owner_node = Some(value);
    }

    /// Gets the value of OwnerNode
    pub fn get_owner_node(&self) -> Option<&String> {
        self.owner_node.as_ref()
    }

    /// Sets the value of PersistentState
    pub fn set_persistent_state(&mut self, value: bool) {
        self.persistent_state = Some(value);
    }

    /// Gets the value of PersistentState
    pub fn get_persistent_state(&self) -> Option<&bool> {
        self.persistent_state.as_ref()
    }

    /// Sets the value of PlacementOptions
    pub fn set_placement_options(&mut self, value: u32) {
        self.placement_options = Some(value);
    }

    /// Gets the value of PlacementOptions
    pub fn get_placement_options(&self) -> Option<&u32> {
        self.placement_options.as_ref()
    }

    /// Sets the value of PreferredSite
    pub fn set_preferred_site(&mut self, value: Vec<String>) {
        self.preferred_site = value;
    }

    /// Gets the value of PreferredSite
    pub fn get_preferred_site(&self) -> &Vec<String> {
        &self.preferred_site
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of PrivateProperties
    pub fn set_private_properties(&mut self, value: MSCluster_Property) {
        self.private_properties = Some(value);
    }

    /// Gets the value of PrivateProperties
    pub fn get_private_properties(&self) -> Option<&MSCluster_Property> {
        self.private_properties.as_ref()
    }

    /// Sets the value of ResiliencyPeriod
    pub fn set_resiliency_period(&mut self, value: u32) {
        self.resiliency_period = Some(value);
    }

    /// Gets the value of ResiliencyPeriod
    pub fn get_resiliency_period(&self) -> Option<&u32> {
        self.resiliency_period.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of StatusInformation
    pub fn set_status_information(&mut self, value: u64) {
        self.status_information = Some(value);
    }

    /// Gets the value of StatusInformation
    pub fn get_status_information(&self) -> Option<&u64> {
        self.status_information.as_ref()
    }

    /// Sets the value of UpdateDomain
    pub fn set_update_domain(&mut self, value: u32) {
        self.update_domain = Some(value);
    }

    /// Gets the value of UpdateDomain
    pub fn get_update_domain(&self) -> Option<&u32> {
        self.update_domain.as_ref()
    }

/// 

    /// * `flags` -  (u32)
    /// * `reason` -  (String)
    /// * `time_out` -  (u32)
    pub fn bring_online(&self, time_out: u32, flags: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeOut".to_string(), value: time_out.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("BringOnline", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `parameters` -  (MSCluster_Property)
    /// * `reason` -  (String)
    /// * `time_out` -  (u32)
    pub fn take_offline(&self, time_out: u32, parameters: MSCluster_Property, flags: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeOut".to_string(), value: time_out.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("TakeOffline", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `parameters` -  (u8[])
    /// * `reason` -  (String)
    /// * `time_out` -  (u32)
    pub fn take_offline_params(&self, time_out: u32, parameters: &Vec<u8>, flags: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeOut".to_string(), value: time_out.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("TakeOfflineParams", &args)

    }


/// 

    /// * `node_name` -  (String)
    /// * `time_out` -  (u32)
    pub fn move_to_new_node(&self, node_name: &String, time_out: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        args.push(MethodParameter { name: "TimeOut".to_string(), value: time_out.into() });
        self.invoke_method("MoveToNewNode", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `node_name` -  (String)
    /// * `parameters` -  (MSCluster_Property)
    /// * `reason` -  (String)
    pub fn move_to_new_node_ex(&self, node_name: &String, parameters: MSCluster_Property, flags: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("MoveToNewNodeEx", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `node_name` -  (String)
    /// * `parameters` -  (u8[])
    /// * `reason` -  (String)
    pub fn move_to_new_node_params(&self, node_name: &String, parameters: &Vec<u8>, flags: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("MoveToNewNodeParams", &args)

    }


/// 

    /// * `flags` -  (u32)
    pub fn cancel_operation(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("CancelOperation", &args)

    }


/// 

    /// * `new_name` -  (String)
    /// * `reason` -  (String)
    pub fn rename(&self, new_name: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `group_name` -  (String)
    /// * `group_type` -  (u32)
    /// * `id` -  (String)

    /// * `id` -  (String)
    pub fn create_group(&self, group_name: &String, group_type: u32, id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GroupName".to_string(), value: group_name.into() });
        args.push(MethodParameter { name: "GroupType".to_string(), value: group_type.into() });

        let result = self.invoke_method("CreateGroup", &args)?;
        let id = result.get_value("Id")?;
        Ok(result.return_value)

    }


/// 

    /// * `reason` -  (String)
    pub fn delete_group(&self, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("DeleteGroup", &args)

    }


/// 

    /// * `options` -  (u32)
    /// * `reason` -  (String)
    pub fn destroy_group(&self, options: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("DestroyGroup", &args)

    }


/// 

    /// * `node_names` -  (String[])
    /// * `reason` -  (String)
    pub fn set_preferred_owners(&self, node_names: &Vec<String>, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeNames".to_string(), value: node_names.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("SetPreferredOwners", &args)

    }


/// 

    /// * `node_names` -  (String[])
    pub fn get_preferred_owners(&self, node_names: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPreferredOwners", &[])?;
        let node_names = result.get_value("NodeNames")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn get_group_type(&self) -> Result<(), WmiError> {
        self.invoke_method("GetGroupType", &[])

    }


/// 

    /// * `group_type` -  (u32)
    pub fn set_group_type(&self, group_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GroupType".to_string(), value: group_type.into() });
        self.invoke_method("SetGroupType", &args)

    }


/// 

    /// * `control_code` -  (i32)
    /// * `input_buffer` -  (u8[])
    /// * `reason` -  (String)

    /// * `output_buffer` -  (u8[])
    /// * `output_buffer_size` -  (i32)
    pub fn execute_group_control(&self, control_code: i32, input_buffer: &Vec<u8>, output_buffer: &mut Vec<u8>, output_buffer_size: &mut i32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ExecuteGroupControl", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_buffer_size = result.get_value("OutputBufferSize")?;
        Ok(result.return_value)

    }

}

