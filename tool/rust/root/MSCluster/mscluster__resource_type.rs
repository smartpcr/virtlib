// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ResourceType struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ResourceType {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "AdminExtensions")]
    pub admin_extensions: Vec<String>,

/// 
    #[serde(rename = "DeadlockTimeout")]
    pub deadlock_timeout: Option<u32>,

/// 
    #[serde(rename = "DeleteRequiresAllNodes")]
    pub delete_requires_all_nodes: Option<bool>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "DllName")]
    pub dll_name: Option<String>,

/// 
    #[serde(rename = "DumpLogQuery")]
    pub dump_log_query: Vec<String>,

/// 
    #[serde(rename = "DumpPolicy")]
    pub dump_policy: Option<u64>,

/// 
    #[serde(rename = "DumpServices")]
    pub dump_services: Vec<String>,

/// 
    #[serde(rename = "EnabledEventLogs")]
    pub enabled_event_logs: Vec<String>,

/// 
    #[serde(rename = "IsAlivePollInterval")]
    pub is_alive_poll_interval: Option<u32>,

/// 
    #[serde(rename = "LocalQuorumCapable")]
    pub local_quorum_capable: Option<bool>,

/// 
    #[serde(rename = "LooksAlivePollInterval")]
    pub looks_alive_poll_interval: Option<u32>,

/// 
    #[serde(rename = "MaximumMonitors")]
    pub maximum_monitors: Option<u32>,

/// 
    #[serde(rename = "PendingTimeout")]
    pub pending_timeout: Option<u32>,

/// 
    #[serde(rename = "PrivateProperties")]
    pub private_properties: Option<MSCluster_Property>,

/// 
    #[serde(rename = "QuorumCapable")]
    pub quorum_capable: Option<bool>,

/// 
    #[serde(rename = "RequiredDependencyClasses")]
    pub required_dependency_classes: Vec<u32>,

/// 
    #[serde(rename = "RequiredDependencyTypes")]
    pub required_dependency_types: Vec<String>,

/// 
    #[serde(rename = "ResourceClass")]
    pub resource_class: Option<u32>,

/// 
    #[serde(rename = "WprProfiles")]
    pub wpr_profiles: Vec<String>,

/// 
    #[serde(rename = "WprStartAfter")]
    pub wpr_start_after: Option<u64>,
}

impl MSCluster_ResourceType {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            admin_extensions: Vec::new(),
            deadlock_timeout: None,
            delete_requires_all_nodes: None,
            display_name: None,
            dll_name: None,
            dump_log_query: Vec::new(),
            dump_policy: None,
            dump_services: Vec::new(),
            enabled_event_logs: Vec::new(),
            is_alive_poll_interval: None,
            local_quorum_capable: None,
            looks_alive_poll_interval: None,
            maximum_monitors: None,
            pending_timeout: None,
            private_properties: None,
            quorum_capable: None,
            required_dependency_classes: Vec::new(),
            required_dependency_types: Vec::new(),
            resource_class: None,
            wpr_profiles: Vec::new(),
            wpr_start_after: None,
        }
    }


    /// Sets the value of AdminExtensions
    pub fn set_admin_extensions(&mut self, value: Vec<String>) {
        self.admin_extensions = value;
    }

    /// Gets the value of AdminExtensions
    pub fn get_admin_extensions(&self) -> &Vec<String> {
        &self.admin_extensions
    }

    /// Sets the value of DeadlockTimeout
    pub fn set_deadlock_timeout(&mut self, value: u32) {
        self.deadlock_timeout = Some(value);
    }

    /// Gets the value of DeadlockTimeout
    pub fn get_deadlock_timeout(&self) -> Option<&u32> {
        self.deadlock_timeout.as_ref()
    }

    /// Sets the value of DeleteRequiresAllNodes
    pub fn set_delete_requires_all_nodes(&mut self, value: bool) {
        self.delete_requires_all_nodes = Some(value);
    }

    /// Gets the value of DeleteRequiresAllNodes
    pub fn get_delete_requires_all_nodes(&self) -> Option<&bool> {
        self.delete_requires_all_nodes.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of DllName
    pub fn set_dll_name(&mut self, value: String) {
        self.dll_name = Some(value);
    }

    /// Gets the value of DllName
    pub fn get_dll_name(&self) -> Option<&String> {
        self.dll_name.as_ref()
    }

    /// Sets the value of DumpLogQuery
    pub fn set_dump_log_query(&mut self, value: Vec<String>) {
        self.dump_log_query = value;
    }

    /// Gets the value of DumpLogQuery
    pub fn get_dump_log_query(&self) -> &Vec<String> {
        &self.dump_log_query
    }

    /// Sets the value of DumpPolicy
    pub fn set_dump_policy(&mut self, value: u64) {
        self.dump_policy = Some(value);
    }

    /// Gets the value of DumpPolicy
    pub fn get_dump_policy(&self) -> Option<&u64> {
        self.dump_policy.as_ref()
    }

    /// Sets the value of DumpServices
    pub fn set_dump_services(&mut self, value: Vec<String>) {
        self.dump_services = value;
    }

    /// Gets the value of DumpServices
    pub fn get_dump_services(&self) -> &Vec<String> {
        &self.dump_services
    }

    /// Sets the value of EnabledEventLogs
    pub fn set_enabled_event_logs(&mut self, value: Vec<String>) {
        self.enabled_event_logs = value;
    }

    /// Gets the value of EnabledEventLogs
    pub fn get_enabled_event_logs(&self) -> &Vec<String> {
        &self.enabled_event_logs
    }

    /// Sets the value of IsAlivePollInterval
    pub fn set_is_alive_poll_interval(&mut self, value: u32) {
        self.is_alive_poll_interval = Some(value);
    }

    /// Gets the value of IsAlivePollInterval
    pub fn get_is_alive_poll_interval(&self) -> Option<&u32> {
        self.is_alive_poll_interval.as_ref()
    }

    /// Sets the value of LocalQuorumCapable
    pub fn set_local_quorum_capable(&mut self, value: bool) {
        self.local_quorum_capable = Some(value);
    }

    /// Gets the value of LocalQuorumCapable
    pub fn get_local_quorum_capable(&self) -> Option<&bool> {
        self.local_quorum_capable.as_ref()
    }

    /// Sets the value of LooksAlivePollInterval
    pub fn set_looks_alive_poll_interval(&mut self, value: u32) {
        self.looks_alive_poll_interval = Some(value);
    }

    /// Gets the value of LooksAlivePollInterval
    pub fn get_looks_alive_poll_interval(&self) -> Option<&u32> {
        self.looks_alive_poll_interval.as_ref()
    }

    /// Sets the value of MaximumMonitors
    pub fn set_maximum_monitors(&mut self, value: u32) {
        self.maximum_monitors = Some(value);
    }

    /// Gets the value of MaximumMonitors
    pub fn get_maximum_monitors(&self) -> Option<&u32> {
        self.maximum_monitors.as_ref()
    }

    /// Sets the value of PendingTimeout
    pub fn set_pending_timeout(&mut self, value: u32) {
        self.pending_timeout = Some(value);
    }

    /// Gets the value of PendingTimeout
    pub fn get_pending_timeout(&self) -> Option<&u32> {
        self.pending_timeout.as_ref()
    }

    /// Sets the value of PrivateProperties
    pub fn set_private_properties(&mut self, value: MSCluster_Property) {
        self.private_properties = Some(value);
    }

    /// Gets the value of PrivateProperties
    pub fn get_private_properties(&self) -> Option<&MSCluster_Property> {
        self.private_properties.as_ref()
    }

    /// Sets the value of QuorumCapable
    pub fn set_quorum_capable(&mut self, value: bool) {
        self.quorum_capable = Some(value);
    }

    /// Gets the value of QuorumCapable
    pub fn get_quorum_capable(&self) -> Option<&bool> {
        self.quorum_capable.as_ref()
    }

    /// Sets the value of RequiredDependencyClasses
    pub fn set_required_dependency_classes(&mut self, value: Vec<u32>) {
        self.required_dependency_classes = value;
    }

    /// Gets the value of RequiredDependencyClasses
    pub fn get_required_dependency_classes(&self) -> &Vec<u32> {
        &self.required_dependency_classes
    }

    /// Sets the value of RequiredDependencyTypes
    pub fn set_required_dependency_types(&mut self, value: Vec<String>) {
        self.required_dependency_types = value;
    }

    /// Gets the value of RequiredDependencyTypes
    pub fn get_required_dependency_types(&self) -> &Vec<String> {
        &self.required_dependency_types
    }

    /// Sets the value of ResourceClass
    pub fn set_resource_class(&mut self, value: u32) {
        self.resource_class = Some(value);
    }

    /// Gets the value of ResourceClass
    pub fn get_resource_class(&self) -> Option<&u32> {
        self.resource_class.as_ref()
    }

    /// Sets the value of WprProfiles
    pub fn set_wpr_profiles(&mut self, value: Vec<String>) {
        self.wpr_profiles = value;
    }

    /// Gets the value of WprProfiles
    pub fn get_wpr_profiles(&self) -> &Vec<String> {
        &self.wpr_profiles
    }

    /// Sets the value of WprStartAfter
    pub fn set_wpr_start_after(&mut self, value: u64) {
        self.wpr_start_after = Some(value);
    }

    /// Gets the value of WprStartAfter
    pub fn get_wpr_start_after(&self) -> Option<&u64> {
        self.wpr_start_after.as_ref()
    }

/// 

    /// * `display_name` -  (String)
    /// * `dllname` -  (String)
    /// * `is_alive_poll_interval` -  (u32)
    /// * `looks_alive_poll_interval` -  (u32)
    /// * `name` -  (String)
    pub fn create_resource_type(&self, name: &String, display_name: &String, dllname: &String, looks_alive_poll_interval: u32, is_alive_poll_interval: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "DisplayName".to_string(), value: display_name.into() });
        args.push(MethodParameter { name: "DLLName".to_string(), value: dllname.into() });
        args.push(MethodParameter { name: "LooksAlivePollInterval".to_string(), value: looks_alive_poll_interval.into() });
        args.push(MethodParameter { name: "IsAlivePollInterval".to_string(), value: is_alive_poll_interval.into() });
        self.invoke_method("CreateResourceType", &args)

    }


/// 

    /// * `reason` -  (String)
    pub fn delete_resource_type(&self, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("DeleteResourceType", &args)

    }


/// 

    /// * `node_names` -  (String[])
    pub fn get_possible_owners(&self, node_names: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPossibleOwners", &[])?;
        let node_names = result.get_value("NodeNames")?;
        Ok(result.return_value)

    }


/// 

    /// * `control_code` -  (i32)
    /// * `input_buffer` -  (u8[])
    /// * `reason` -  (String)

    /// * `output_buffer` -  (u8[])
    /// * `output_buffer_size` -  (i32)
    pub fn execute_resource_type_control(&self, control_code: i32, input_buffer: &Vec<u8>, output_buffer: &mut Vec<u8>, output_buffer_size: &mut i32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ExecuteResourceTypeControl", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_buffer_size = result.get_value("OutputBufferSize")?;
        Ok(result.return_value)

    }

}

