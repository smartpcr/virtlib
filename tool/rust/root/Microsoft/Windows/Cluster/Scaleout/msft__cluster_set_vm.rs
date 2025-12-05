// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetVM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetVM {

/// 
    #[serde(rename = "AvailabilitySet")]
    pub availability_set: Option<u64>,

/// 
    #[serde(rename = "CheckHeartBeat")]
    pub check_heart_beat: Option<bool>,

/// 
    #[serde(rename = "DefaultMoveType")]
    pub default_move_type: Option<u32>,

/// 
    #[serde(rename = "FaultDomain")]
    pub fault_domain: Option<u64>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u64>,

/// 
    #[serde(rename = "MemberId")]
    pub member_id: Option<u64>,

/// 
    #[serde(rename = "MemberName")]
    pub member_name: Option<String>,

/// 
    #[serde(rename = "NodeId")]
    pub node_id: Option<u64>,

/// 
    #[serde(rename = "NodeName")]
    pub node_name: Option<String>,

/// 
    #[serde(rename = "OfflineAction")]
    pub offline_action: Option<u32>,

/// 
    #[serde(rename = "PlacementCondition")]
    pub placement_condition: Option<String>,

/// 
    #[serde(rename = "StartMemory")]
    pub start_memory: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "UpdateDomain")]
    pub update_domain: Option<u64>,

/// 
    #[serde(rename = "VMId")]
    pub vmid: Option<String>,

/// 
    #[serde(rename = "VMName")]
    pub vmname: Option<String>,
}

impl MSFT_ClusterSetVM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            availability_set: None,
            check_heart_beat: None,
            default_move_type: None,
            fault_domain: None,
            id: None,
            member_id: None,
            member_name: None,
            node_id: None,
            node_name: None,
            offline_action: None,
            placement_condition: None,
            start_memory: None,
            state: None,
            update_domain: None,
            vmid: None,
            vmname: None,
        }
    }


    /// Sets the value of AvailabilitySet
    pub fn set_availability_set(&mut self, value: u64) {
        self.availability_set = Some(value);
    }

    /// Gets the value of AvailabilitySet
    pub fn get_availability_set(&self) -> Option<&u64> {
        self.availability_set.as_ref()
    }

    /// Sets the value of CheckHeartBeat
    pub fn set_check_heart_beat(&mut self, value: bool) {
        self.check_heart_beat = Some(value);
    }

    /// Gets the value of CheckHeartBeat
    pub fn get_check_heart_beat(&self) -> Option<&bool> {
        self.check_heart_beat.as_ref()
    }

    /// Sets the value of DefaultMoveType
    pub fn set_default_move_type(&mut self, value: u32) {
        self.default_move_type = Some(value);
    }

    /// Gets the value of DefaultMoveType
    pub fn get_default_move_type(&self) -> Option<&u32> {
        self.default_move_type.as_ref()
    }

    /// Sets the value of FaultDomain
    pub fn set_fault_domain(&mut self, value: u64) {
        self.fault_domain = Some(value);
    }

    /// Gets the value of FaultDomain
    pub fn get_fault_domain(&self) -> Option<&u64> {
        self.fault_domain.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u64) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    /// Sets the value of MemberId
    pub fn set_member_id(&mut self, value: u64) {
        self.member_id = Some(value);
    }

    /// Gets the value of MemberId
    pub fn get_member_id(&self) -> Option<&u64> {
        self.member_id.as_ref()
    }

    /// Sets the value of MemberName
    pub fn set_member_name(&mut self, value: String) {
        self.member_name = Some(value);
    }

    /// Gets the value of MemberName
    pub fn get_member_name(&self) -> Option<&String> {
        self.member_name.as_ref()
    }

    /// Sets the value of NodeId
    pub fn set_node_id(&mut self, value: u64) {
        self.node_id = Some(value);
    }

    /// Gets the value of NodeId
    pub fn get_node_id(&self) -> Option<&u64> {
        self.node_id.as_ref()
    }

    /// Sets the value of NodeName
    pub fn set_node_name(&mut self, value: String) {
        self.node_name = Some(value);
    }

    /// Gets the value of NodeName
    pub fn get_node_name(&self) -> Option<&String> {
        self.node_name.as_ref()
    }

    /// Sets the value of OfflineAction
    pub fn set_offline_action(&mut self, value: u32) {
        self.offline_action = Some(value);
    }

    /// Gets the value of OfflineAction
    pub fn get_offline_action(&self) -> Option<&u32> {
        self.offline_action.as_ref()
    }

    /// Sets the value of PlacementCondition
    pub fn set_placement_condition(&mut self, value: String) {
        self.placement_condition = Some(value);
    }

    /// Gets the value of PlacementCondition
    pub fn get_placement_condition(&self) -> Option<&String> {
        self.placement_condition.as_ref()
    }

    /// Sets the value of StartMemory
    pub fn set_start_memory(&mut self, value: u32) {
        self.start_memory = Some(value);
    }

    /// Gets the value of StartMemory
    pub fn get_start_memory(&self) -> Option<&u32> {
        self.start_memory.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of UpdateDomain
    pub fn set_update_domain(&mut self, value: u64) {
        self.update_domain = Some(value);
    }

    /// Gets the value of UpdateDomain
    pub fn get_update_domain(&self) -> Option<&u64> {
        self.update_domain.as_ref()
    }

    /// Sets the value of VMId
    pub fn set_vmid(&mut self, value: String) {
        self.vmid = Some(value);
    }

    /// Gets the value of VMId
    pub fn get_vmid(&self) -> Option<&String> {
        self.vmid.as_ref()
    }

    /// Sets the value of VMName
    pub fn set_vmname(&mut self, value: String) {
        self.vmname = Some(value);
    }

    /// Gets the value of VMName
    pub fn get_vmname(&self) -> Option<&String> {
        self.vmname.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn online(&self) -> Result<(), WmiError> {
        self.invoke_method("Online", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn offline(&self) -> Result<(), WmiError> {
        self.invoke_method("Offline", &[])

    }


/// 

    /// * `move_type` -  (u32)
    /// * `node` -  (String)

    /// * `return_value` -  (u32)
    pub fn move(&self, node: &String, move_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Node".to_string(), value: node.into() });
        args.push(MethodParameter { name: "MoveType".to_string(), value: move_type.into() });
        self.invoke_method("Move", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn destroy_vm(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("DestroyVm", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn uncluster_vm(&self, force: bool, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("UnclusterVm", &args)

    }


/// 

    /// * `availability_set_name` -  (String)
    /// * `placement_condition` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_vm_properties(&self, availability_set_name: &String, placement_condition: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: availability_set_name.into() });
        args.push(MethodParameter { name: "PlacementCondition".to_string(), value: placement_condition.into() });
        self.invoke_method("SetVmProperties", &args)

    }

}

