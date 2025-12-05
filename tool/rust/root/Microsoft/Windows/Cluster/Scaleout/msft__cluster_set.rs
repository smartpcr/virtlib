// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSet {

/// 
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,

/// 
    #[serde(rename = "EvacuationMoveThreshold")]
    pub evacuation_move_threshold: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u64>,

/// 
    #[serde(rename = "MigrationExcludeNetworks")]
    pub migration_exclude_networks: Option<String>,

/// 
    #[serde(rename = "MigrationNetworkOrder")]
    pub migration_network_order: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NamespaceRoot")]
    pub namespace_root: Option<String>,

/// 
    #[serde(rename = "VMFailoverMode")]
    pub vmfailover_mode: Option<u32>,
}

impl MSFT_ClusterSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_name: None,
            evacuation_move_threshold: None,
            id: None,
            migration_exclude_networks: None,
            migration_network_order: None,
            name: None,
            namespace_root: None,
            vmfailover_mode: None,
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

    /// Sets the value of EvacuationMoveThreshold
    pub fn set_evacuation_move_threshold(&mut self, value: u32) {
        self.evacuation_move_threshold = Some(value);
    }

    /// Gets the value of EvacuationMoveThreshold
    pub fn get_evacuation_move_threshold(&self) -> Option<&u32> {
        self.evacuation_move_threshold.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u64) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    /// Sets the value of MigrationExcludeNetworks
    pub fn set_migration_exclude_networks(&mut self, value: String) {
        self.migration_exclude_networks = Some(value);
    }

    /// Gets the value of MigrationExcludeNetworks
    pub fn get_migration_exclude_networks(&self) -> Option<&String> {
        self.migration_exclude_networks.as_ref()
    }

    /// Sets the value of MigrationNetworkOrder
    pub fn set_migration_network_order(&mut self, value: String) {
        self.migration_network_order = Some(value);
    }

    /// Gets the value of MigrationNetworkOrder
    pub fn get_migration_network_order(&self) -> Option<&String> {
        self.migration_network_order.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NamespaceRoot
    pub fn set_namespace_root(&mut self, value: String) {
        self.namespace_root = Some(value);
    }

    /// Gets the value of NamespaceRoot
    pub fn get_namespace_root(&self) -> Option<&String> {
        self.namespace_root.as_ref()
    }

    /// Sets the value of VMFailoverMode
    pub fn set_vmfailover_mode(&mut self, value: u32) {
        self.vmfailover_mode = Some(value);
    }

    /// Gets the value of VMFailoverMode
    pub fn get_vmfailover_mode(&self) -> Option<&u32> {
        self.vmfailover_mode.as_ref()
    }

/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)

    /// * `member` -  (MSFT_ClusterSetMember)
    /// * `return_value` -  (u32)
    pub fn add_member(&self, name: &String, flags: u32, member: &mut MSFT_ClusterSetMember) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("AddMember", &args)?;
        let member = result.get_value("Member")?;
        Ok(result.return_value)

    }


/// 

    /// * `vm_config` -  (String)
    /// * `vm_host` -  (String)
    /// * `vm_id` -  (String)
    /// * `vm_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `vm` -  (MSFT_ClusterSetVM)
    pub fn add_vm(&self, vm_config: &String, vm_id: &String, vm_name: &String, vm_host: &String, vm: &mut MSFT_ClusterSetVM) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "vmConfig".to_string(), value: vm_config.into() });
        args.push(MethodParameter { name: "vmId".to_string(), value: vm_id.into() });
        args.push(MethodParameter { name: "vmName".to_string(), value: vm_name.into() });
        args.push(MethodParameter { name: "vmHost".to_string(), value: vm_host.into() });

        let result = self.invoke_method("AddVm", &args)?;
        let vm = result.get_value("VM")?;
        Ok(result.return_value)

    }


/// 

    /// * `availability_set_name` -  (String)
    /// * `local_disk_size` -  (u32)
    /// * `placement_condition` -  (String)
    /// * `version` -  (u32)
    /// * `vm_cpu_reservation` -  (u32)
    /// * `vm_flags` -  (u32)
    /// * `vm_memory` -  (u32)
    /// * `vm_virtual_core_count` -  (u32)

    /// * `node` -  (MSFT_ClusterSetNode)
    /// * `return_value` -  (u32)
    pub fn get_optimal_node_for_vm(&self, vm_memory: u32, vm_virtual_core_count: u32, vm_cpu_reservation: u32, vm_flags: u32, version: u32, local_disk_size: u32, placement_condition: &String, availability_set_name: &String, node: &mut MSFT_ClusterSetNode) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmMemory".to_string(), value: vm_memory.into() });
        args.push(MethodParameter { name: "VmVirtualCoreCount".to_string(), value: vm_virtual_core_count.into() });
        args.push(MethodParameter { name: "VmCpuReservation".to_string(), value: vm_cpu_reservation.into() });
        args.push(MethodParameter { name: "VmFlags".to_string(), value: vm_flags.into() });
        args.push(MethodParameter { name: "Version".to_string(), value: version.into() });
        args.push(MethodParameter { name: "LocalDiskSize".to_string(), value: local_disk_size.into() });
        args.push(MethodParameter { name: "PlacementCondition".to_string(), value: placement_condition.into() });
        args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: availability_set_name.into() });

        let result = self.invoke_method("GetOptimalNodeForVm", &args)?;
        let node = result.get_value("Node")?;
        Ok(result.return_value)

    }


/// 

    /// * `operation` -  (String)
    /// * `param1` -  (String)
    /// * `param2` -  (String)
    /// * `param3` -  (String)

    /// * `outparam1` -  (String)
    /// * `return_value` -  (u32)
    pub fn do_op(&self, operation: &String, param1: &String, param2: &String, param3: &String, outparam1: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "operation".to_string(), value: operation.into() });
        args.push(MethodParameter { name: "param1".to_string(), value: param1.into() });
        args.push(MethodParameter { name: "param2".to_string(), value: param2.into() });
        args.push(MethodParameter { name: "param3".to_string(), value: param3.into() });

        let result = self.invoke_method("DoOp", &args)?;
        let outparam1 = result.get_value("outparam1")?;
        Ok(result.return_value)

    }


/// 

    /// * `namespace_root` -  (String)
    /// * `vmfailover_mode` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set(&self, namespace_root: &String, vmfailover_mode: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NamespaceRoot".to_string(), value: namespace_root.into() });
        args.push(MethodParameter { name: "VMFailoverMode".to_string(), value: vmfailover_mode.into() });
        self.invoke_method("Set", &args)

    }


/// 

    /// * `destination_cluster_name` -  (String)
    /// * `destination_rgname` -  (String)
    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `source_cluster_name` -  (String)
    /// * `source_rgname` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_srpartnership(&self, name: &String, source_cluster_name: &String, source_rgname: &String, destination_cluster_name: &String, destination_rgname: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "SourceClusterName".to_string(), value: source_cluster_name.into() });
        args.push(MethodParameter { name: "SourceRGName".to_string(), value: source_rgname.into() });
        args.push(MethodParameter { name: "DestinationClusterName".to_string(), value: destination_cluster_name.into() });
        args.push(MethodParameter { name: "DestinationRGName".to_string(), value: destination_rgname.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("AddSRPartnership", &args)

    }

}

