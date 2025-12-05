// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_Node struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_Node {
    #[serde(flatten)]
    pub base: CIM_UnitaryComputerSystem,

/// 
    #[serde(rename = "BuildNumber")]
    pub build_number: Option<u32>,

/// 
    #[serde(rename = "Characteristics")]
    pub characteristics: Option<u32>,

/// 
    #[serde(rename = "CSDVersion")]
    pub csdversion: Option<String>,

/// 
    #[serde(rename = "DetectedCloudPlatform")]
    pub detected_cloud_platform: Option<u32>,

/// 
    #[serde(rename = "DrainErrorCode")]
    pub drain_error_code: Option<u32>,

/// 
    #[serde(rename = "DynamicWeight")]
    pub dynamic_weight: Option<u32>,

/// 
    #[serde(rename = "FailbackErrorCode")]
    pub failback_error_code: Option<u32>,

/// 
    #[serde(rename = "FaultDomain")]
    pub fault_domain: Vec<String>,

/// 
    #[serde(rename = "FaultDomainId")]
    pub fault_domain_id: Option<String>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "HyperthreadingEnabled")]
    pub hyperthreading_enabled: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "MajorVersion")]
    pub major_version: Option<u32>,

/// 
    #[serde(rename = "MinorVersion")]
    pub minor_version: Option<u32>,

/// 
    #[serde(rename = "NeedsPreventQuorum")]
    pub needs_prevent_quorum: Option<u32>,

/// 
    #[serde(rename = "NodeDrainStatus")]
    pub node_drain_status: Option<u32>,

/// 
    #[serde(rename = "NodeDrainTarget")]
    pub node_drain_target: Option<String>,

/// 
    #[serde(rename = "NodeFailbackStatus")]
    pub node_failback_status: Option<u32>,

/// 
    #[serde(rename = "NodeHighestVersion")]
    pub node_highest_version: Option<u32>,

/// 
    #[serde(rename = "NodeInstanceID")]
    pub node_instance_id: Option<String>,

/// 
    #[serde(rename = "NodeLowestVersion")]
    pub node_lowest_version: Option<u32>,

/// 
    #[serde(rename = "NodeWeight")]
    pub node_weight: Option<u32>,

/// 
    #[serde(rename = "PrivateProperties")]
    pub private_properties: Option<MSCluster_Property>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "StatusInformation")]
    pub status_information: Option<u32>,

/// 
    #[serde(rename = "UniqueID")]
    pub unique_id: Option<String>,
}

impl MSCluster_Node {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_UnitaryComputerSystem::new(),
            build_number: None,
            characteristics: None,
            csdversion: None,
            detected_cloud_platform: None,
            drain_error_code: None,
            dynamic_weight: None,
            failback_error_code: None,
            fault_domain: Vec::new(),
            fault_domain_id: None,
            flags: None,
            hyperthreading_enabled: None,
            id: None,
            major_version: None,
            minor_version: None,
            needs_prevent_quorum: None,
            node_drain_status: None,
            node_drain_target: None,
            node_failback_status: None,
            node_highest_version: None,
            node_instance_id: None,
            node_lowest_version: None,
            node_weight: None,
            private_properties: None,
            state: None,
            status_information: None,
            unique_id: None,
        }
    }


    /// Sets the value of BuildNumber
    pub fn set_build_number(&mut self, value: u32) {
        self.build_number = Some(value);
    }

    /// Gets the value of BuildNumber
    pub fn get_build_number(&self) -> Option<&u32> {
        self.build_number.as_ref()
    }

    /// Sets the value of Characteristics
    pub fn set_characteristics(&mut self, value: u32) {
        self.characteristics = Some(value);
    }

    /// Gets the value of Characteristics
    pub fn get_characteristics(&self) -> Option<&u32> {
        self.characteristics.as_ref()
    }

    /// Sets the value of CSDVersion
    pub fn set_csdversion(&mut self, value: String) {
        self.csdversion = Some(value);
    }

    /// Gets the value of CSDVersion
    pub fn get_csdversion(&self) -> Option<&String> {
        self.csdversion.as_ref()
    }

    /// Sets the value of DetectedCloudPlatform
    pub fn set_detected_cloud_platform(&mut self, value: u32) {
        self.detected_cloud_platform = Some(value);
    }

    /// Gets the value of DetectedCloudPlatform
    pub fn get_detected_cloud_platform(&self) -> Option<&u32> {
        self.detected_cloud_platform.as_ref()
    }

    /// Sets the value of DrainErrorCode
    pub fn set_drain_error_code(&mut self, value: u32) {
        self.drain_error_code = Some(value);
    }

    /// Gets the value of DrainErrorCode
    pub fn get_drain_error_code(&self) -> Option<&u32> {
        self.drain_error_code.as_ref()
    }

    /// Sets the value of DynamicWeight
    pub fn set_dynamic_weight(&mut self, value: u32) {
        self.dynamic_weight = Some(value);
    }

    /// Gets the value of DynamicWeight
    pub fn get_dynamic_weight(&self) -> Option<&u32> {
        self.dynamic_weight.as_ref()
    }

    /// Sets the value of FailbackErrorCode
    pub fn set_failback_error_code(&mut self, value: u32) {
        self.failback_error_code = Some(value);
    }

    /// Gets the value of FailbackErrorCode
    pub fn get_failback_error_code(&self) -> Option<&u32> {
        self.failback_error_code.as_ref()
    }

    /// Sets the value of FaultDomain
    pub fn set_fault_domain(&mut self, value: Vec<String>) {
        self.fault_domain = value;
    }

    /// Gets the value of FaultDomain
    pub fn get_fault_domain(&self) -> &Vec<String> {
        &self.fault_domain
    }

    /// Sets the value of FaultDomainId
    pub fn set_fault_domain_id(&mut self, value: String) {
        self.fault_domain_id = Some(value);
    }

    /// Gets the value of FaultDomainId
    pub fn get_fault_domain_id(&self) -> Option<&String> {
        self.fault_domain_id.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of HyperthreadingEnabled
    pub fn set_hyperthreading_enabled(&mut self, value: u32) {
        self.hyperthreading_enabled = Some(value);
    }

    /// Gets the value of HyperthreadingEnabled
    pub fn get_hyperthreading_enabled(&self) -> Option<&u32> {
        self.hyperthreading_enabled.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of MajorVersion
    pub fn set_major_version(&mut self, value: u32) {
        self.major_version = Some(value);
    }

    /// Gets the value of MajorVersion
    pub fn get_major_version(&self) -> Option<&u32> {
        self.major_version.as_ref()
    }

    /// Sets the value of MinorVersion
    pub fn set_minor_version(&mut self, value: u32) {
        self.minor_version = Some(value);
    }

    /// Gets the value of MinorVersion
    pub fn get_minor_version(&self) -> Option<&u32> {
        self.minor_version.as_ref()
    }

    /// Sets the value of NeedsPreventQuorum
    pub fn set_needs_prevent_quorum(&mut self, value: u32) {
        self.needs_prevent_quorum = Some(value);
    }

    /// Gets the value of NeedsPreventQuorum
    pub fn get_needs_prevent_quorum(&self) -> Option<&u32> {
        self.needs_prevent_quorum.as_ref()
    }

    /// Sets the value of NodeDrainStatus
    pub fn set_node_drain_status(&mut self, value: u32) {
        self.node_drain_status = Some(value);
    }

    /// Gets the value of NodeDrainStatus
    pub fn get_node_drain_status(&self) -> Option<&u32> {
        self.node_drain_status.as_ref()
    }

    /// Sets the value of NodeDrainTarget
    pub fn set_node_drain_target(&mut self, value: String) {
        self.node_drain_target = Some(value);
    }

    /// Gets the value of NodeDrainTarget
    pub fn get_node_drain_target(&self) -> Option<&String> {
        self.node_drain_target.as_ref()
    }

    /// Sets the value of NodeFailbackStatus
    pub fn set_node_failback_status(&mut self, value: u32) {
        self.node_failback_status = Some(value);
    }

    /// Gets the value of NodeFailbackStatus
    pub fn get_node_failback_status(&self) -> Option<&u32> {
        self.node_failback_status.as_ref()
    }

    /// Sets the value of NodeHighestVersion
    pub fn set_node_highest_version(&mut self, value: u32) {
        self.node_highest_version = Some(value);
    }

    /// Gets the value of NodeHighestVersion
    pub fn get_node_highest_version(&self) -> Option<&u32> {
        self.node_highest_version.as_ref()
    }

    /// Sets the value of NodeInstanceID
    pub fn set_node_instance_id(&mut self, value: String) {
        self.node_instance_id = Some(value);
    }

    /// Gets the value of NodeInstanceID
    pub fn get_node_instance_id(&self) -> Option<&String> {
        self.node_instance_id.as_ref()
    }

    /// Sets the value of NodeLowestVersion
    pub fn set_node_lowest_version(&mut self, value: u32) {
        self.node_lowest_version = Some(value);
    }

    /// Gets the value of NodeLowestVersion
    pub fn get_node_lowest_version(&self) -> Option<&u32> {
        self.node_lowest_version.as_ref()
    }

    /// Sets the value of NodeWeight
    pub fn set_node_weight(&mut self, value: u32) {
        self.node_weight = Some(value);
    }

    /// Gets the value of NodeWeight
    pub fn get_node_weight(&self) -> Option<&u32> {
        self.node_weight.as_ref()
    }

    /// Sets the value of PrivateProperties
    pub fn set_private_properties(&mut self, value: MSCluster_Property) {
        self.private_properties = Some(value);
    }

    /// Gets the value of PrivateProperties
    pub fn get_private_properties(&self) -> Option<&MSCluster_Property> {
        self.private_properties.as_ref()
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
    pub fn set_status_information(&mut self, value: u32) {
        self.status_information = Some(value);
    }

    /// Gets the value of StatusInformation
    pub fn get_status_information(&self) -> Option<&u32> {
        self.status_information.as_ref()
    }

    /// Sets the value of UniqueID
    pub fn set_unique_id(&mut self, value: String) {
        self.unique_id = Some(value);
    }

    /// Gets the value of UniqueID
    pub fn get_unique_id(&self) -> Option<&String> {
        self.unique_id.as_ref()
    }

/// 

    /// * `drain_type` -  (u32)
    /// * `reason` -  (String)
    /// * `target_node` -  (String)
    pub fn pause(&self, drain_type: u32, target_node: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DrainType".to_string(), value: drain_type.into() });
        args.push(MethodParameter { name: "TargetNode".to_string(), value: target_node.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("Pause", &args)

    }


/// 

    /// * `failback_type` -  (u32)
    /// * `reason` -  (String)
    pub fn resume(&self, failback_type: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FailbackType".to_string(), value: failback_type.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("Resume", &args)

    }


/// 

    /// * `reason` -  (String)

    /// * `return_value` -  (bool)
    pub fn will_evict_lose_quorum(&self, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("WillEvictLoseQuorum", &args)

    }


/// 

    /// * `reason` -  (String)

    /// * `return_value` -  (bool)
    pub fn will_offline_lose_quorum(&self, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("WillOfflineLoseQuorum", &args)

    }


/// 

    /// * `control_code` -  (i32)
    /// * `input_buffer` -  (u8[])
    /// * `reason` -  (String)

    /// * `output_buffer` -  (u8[])
    /// * `output_buffer_size` -  (i32)
    pub fn execute_node_control(&self, control_code: i32, input_buffer: &Vec<u8>, output_buffer: &mut Vec<u8>, output_buffer_size: &mut i32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ExecuteNodeControl", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_buffer_size = result.get_value("OutputBufferSize")?;
        Ok(result.return_value)

    }

}

