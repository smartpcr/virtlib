// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ClusterService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ClusterService {

/// 
    #[serde(rename = "ElementName")]
    pub element_name: Option<String>,
}

impl MSCluster_ClusterService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            element_name: None,
        }
    }


    /// Sets the value of ElementName
    pub fn set_element_name(&mut self, value: String) {
        self.element_name = Some(value);
    }

    /// Gets the value of ElementName
    pub fn get_element_name(&self) -> Option<&String> {
        self.element_name.as_ref()
    }

/// 

    /// * `what_if` -  (bool)

    /// * `return_value` -  (u32)
    pub fn update_functional_level(&self, what_if: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "WhatIf".to_string(), value: what_if.into() });
        self.invoke_method("UpdateFunctionalLevel", &args)

    }


/// 

    /// * `is_ready` -  (bool)
    /// * `return_value` -  (u32)
    pub fn cluster_is_ready_for_upgrade(&self, is_ready: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("ClusterIsReadyForUpgrade", &[])?;
        let is_ready = result.get_value("isReady")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn enable_health(&self) -> Result<(), WmiError> {
        self.invoke_method("EnableHealth", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable_health(&self) -> Result<(), WmiError> {
        self.invoke_method("DisableHealth", &[])

    }


/// 

    /// * `providers` -  (String[])

    /// * `return_value` -  (u32)
    pub fn add_health_providers(&self, providers: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Providers".to_string(), value: providers.into() });
        self.invoke_method("AddHealthProviders", &args)

    }


/// 

    /// * `providers` -  (String[])

    /// * `return_value` -  (u32)
    pub fn remove_health_providers(&self, providers: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Providers".to_string(), value: providers.into() });
        self.invoke_method("RemoveHealthProviders", &args)

    }


/// 

    /// * `fault_domains` -  (u32)
    /// * `local_disk_size` -  (u32)
    /// * `reserved_nodes` -  (f64)
    /// * `reserve_spare_node` -  (bool)
    /// * `update_domains` -  (u32)
    /// * `version` -  (u32)
    /// * `vm_cpu_reservation` -  (u32)
    /// * `vm_flags` -  (u32)
    /// * `vm_memory` -  (u32)
    /// * `vm_virtual_core_count` -  (u32)

    /// * `max_num_of_vms_in_cluster` -  (u32)
    /// * `max_num_of_vms_in_node` -  (u32)
    /// * `placement_score_flags` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_placement_score(&self, vm_memory: u32, vm_virtual_core_count: u32, vm_cpu_reservation: u32, vm_flags: u32, reserved_nodes: f64, max_num_of_vms_in_cluster: &mut u32, max_num_of_vms_in_node: &mut u32, placement_score_flags: &mut u32, version: Option<u32>, local_disk_size: Option<u32>, update_domains: Option<u32>, fault_domains: Option<u32>, reserve_spare_node: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmMemory".to_string(), value: vm_memory.into() });
        args.push(MethodParameter { name: "VmVirtualCoreCount".to_string(), value: vm_virtual_core_count.into() });
        args.push(MethodParameter { name: "VmCpuReservation".to_string(), value: vm_cpu_reservation.into() });
        args.push(MethodParameter { name: "VmFlags".to_string(), value: vm_flags.into() });
        args.push(MethodParameter { name: "ReservedNodes".to_string(), value: reserved_nodes.into() });
        if let Some(val) = version {
            args.push(MethodParameter { name: "Version".to_string(), value: val.into() });
        }
        if let Some(val) = local_disk_size {
            args.push(MethodParameter { name: "LocalDiskSize".to_string(), value: val.into() });
        }
        if let Some(val) = update_domains {
            args.push(MethodParameter { name: "UpdateDomains".to_string(), value: val.into() });
        }
        if let Some(val) = fault_domains {
            args.push(MethodParameter { name: "FaultDomains".to_string(), value: val.into() });
        }
        if let Some(val) = reserve_spare_node {
            args.push(MethodParameter { name: "ReserveSpareNode".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetPlacementScore", &args)?;
        let max_num_of_vms_in_cluster = result.get_value("MaxNumOfVMsInCluster")?;
        let max_num_of_vms_in_node = result.get_value("MaxNumOfVMsInNode")?;
        let placement_score_flags = result.get_value("PlacementScoreFlags")?;
        Ok(result.return_value)

    }


/// 

    /// * `fault_domains` -  (u32)
    /// * `local_disk_size` -  (u32)
    /// * `reserved_nodes` -  (f64)
    /// * `reserve_spare_node` -  (bool)
    /// * `update_domains` -  (u32)
    /// * `version` -  (u32)
    /// * `vm_cpu_reservation` -  (u32)
    /// * `vm_flags` -  (u32)
    /// * `vm_memory` -  (u32)
    /// * `vm_queue_pair_count` -  (u32)
    /// * `vm_virtual_core_count` -  (u32)
    /// * `vm_virtual_function_count` -  (u32)

    /// * `max_num_of_vms_in_cluster` -  (u32)
    /// * `max_num_of_vms_in_node` -  (u32)
    /// * `placement_score_flags` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_placement_score_ex(&self, vm_memory: u32, vm_virtual_core_count: u32, vm_cpu_reservation: u32, vm_virtual_function_count: u32, vm_queue_pair_count: u32, vm_flags: u32, reserved_nodes: f64, max_num_of_vms_in_cluster: &mut u32, max_num_of_vms_in_node: &mut u32, placement_score_flags: &mut u32, version: Option<u32>, local_disk_size: Option<u32>, update_domains: Option<u32>, fault_domains: Option<u32>, reserve_spare_node: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmMemory".to_string(), value: vm_memory.into() });
        args.push(MethodParameter { name: "VmVirtualCoreCount".to_string(), value: vm_virtual_core_count.into() });
        args.push(MethodParameter { name: "VmCpuReservation".to_string(), value: vm_cpu_reservation.into() });
        args.push(MethodParameter { name: "VmVirtualFunctionCount".to_string(), value: vm_virtual_function_count.into() });
        args.push(MethodParameter { name: "VmQueuePairCount".to_string(), value: vm_queue_pair_count.into() });
        args.push(MethodParameter { name: "VmFlags".to_string(), value: vm_flags.into() });
        args.push(MethodParameter { name: "ReservedNodes".to_string(), value: reserved_nodes.into() });
        if let Some(val) = version {
            args.push(MethodParameter { name: "Version".to_string(), value: val.into() });
        }
        if let Some(val) = local_disk_size {
            args.push(MethodParameter { name: "LocalDiskSize".to_string(), value: val.into() });
        }
        if let Some(val) = update_domains {
            args.push(MethodParameter { name: "UpdateDomains".to_string(), value: val.into() });
        }
        if let Some(val) = fault_domains {
            args.push(MethodParameter { name: "FaultDomains".to_string(), value: val.into() });
        }
        if let Some(val) = reserve_spare_node {
            args.push(MethodParameter { name: "ReserveSpareNode".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetPlacementScoreEx", &args)?;
        let max_num_of_vms_in_cluster = result.get_value("MaxNumOfVMsInCluster")?;
        let max_num_of_vms_in_node = result.get_value("MaxNumOfVMsInNode")?;
        let placement_score_flags = result.get_value("PlacementScoreFlags")?;
        Ok(result.return_value)

    }


/// 

    /// * `availability_set_name` -  (String)
    /// * `local_disk_size` -  (u32)
    /// * `reservation_id` -  (String)
    /// * `time_span` -  (u32)
    /// * `version` -  (u32)
    /// * `vm_cpu_reservation` -  (u32)
    /// * `vm_flags` -  (u32)
    /// * `vm_memory` -  (u32)
    /// * `vm_virtual_core_count` -  (u32)

    /// * `node_id` -  (u32)
    /// * `return_value` -  (u32)
    pub fn create_vm_reservation(&self, vm_memory: u32, vm_virtual_core_count: u32, vm_cpu_reservation: u32, vm_flags: u32, time_span: u32, reservation_id: &String, node_id: &mut u32, version: Option<u32>, local_disk_size: Option<u32>, availability_set_name: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmMemory".to_string(), value: vm_memory.into() });
        args.push(MethodParameter { name: "VmVirtualCoreCount".to_string(), value: vm_virtual_core_count.into() });
        args.push(MethodParameter { name: "VmCpuReservation".to_string(), value: vm_cpu_reservation.into() });
        args.push(MethodParameter { name: "VmFlags".to_string(), value: vm_flags.into() });
        args.push(MethodParameter { name: "TimeSpan".to_string(), value: time_span.into() });
        args.push(MethodParameter { name: "ReservationId".to_string(), value: reservation_id.into() });
        if let Some(val) = version {
            args.push(MethodParameter { name: "Version".to_string(), value: val.into() });
        }
        if let Some(val) = local_disk_size {
            args.push(MethodParameter { name: "LocalDiskSize".to_string(), value: val.into() });
        }
        if let Some(val) = availability_set_name {
            args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CreateVmReservation", &args)?;
        let node_id = result.get_value("NodeId")?;
        Ok(result.return_value)

    }


/// 

    /// * `availability_set_name` -  (String)
    /// * `fault_domain` -  (u32)
    /// * `local_disk_size` -  (u32)
    /// * `reservation_id` -  (String)
    /// * `reserved` -  (u64)
    /// * `time_span` -  (u32)
    /// * `update_domain` -  (u32)
    /// * `version` -  (u32)
    /// * `vm_cpu_reservation` -  (u32)
    /// * `vm_flags` -  (u32)
    /// * `vm_memory` -  (u32)
    /// * `vm_virtual_core_count` -  (u32)

    /// * `node_id` -  (u32)
    /// * `return_value` -  (u32)
    pub fn create_vm_reservation_ex(&self, vm_memory: u32, vm_virtual_core_count: u32, vm_cpu_reservation: u32, vm_flags: u32, time_span: u32, reservation_id: &String, node_id: &mut u32, version: Option<u32>, local_disk_size: Option<u32>, availability_set_name: &Option<String>, fault_domain: Option<u32>, update_domain: Option<u32>, reserved: Option<u64>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmMemory".to_string(), value: vm_memory.into() });
        args.push(MethodParameter { name: "VmVirtualCoreCount".to_string(), value: vm_virtual_core_count.into() });
        args.push(MethodParameter { name: "VmCpuReservation".to_string(), value: vm_cpu_reservation.into() });
        args.push(MethodParameter { name: "VmFlags".to_string(), value: vm_flags.into() });
        args.push(MethodParameter { name: "TimeSpan".to_string(), value: time_span.into() });
        args.push(MethodParameter { name: "ReservationId".to_string(), value: reservation_id.into() });
        if let Some(val) = version {
            args.push(MethodParameter { name: "Version".to_string(), value: val.into() });
        }
        if let Some(val) = local_disk_size {
            args.push(MethodParameter { name: "LocalDiskSize".to_string(), value: val.into() });
        }
        if let Some(val) = availability_set_name {
            args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: val.into() });
        }
        if let Some(val) = fault_domain {
            args.push(MethodParameter { name: "FaultDomain".to_string(), value: val.into() });
        }
        if let Some(val) = update_domain {
            args.push(MethodParameter { name: "UpdateDomain".to_string(), value: val.into() });
        }
        if let Some(val) = reserved {
            args.push(MethodParameter { name: "Reserved".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CreateVmReservationEx", &args)?;
        let node_id = result.get_value("NodeId")?;
        Ok(result.return_value)

    }


/// 

    /// * `availability_set_name` -  (String)
    /// * `fault_domain` -  (u32)
    /// * `local_disk_size` -  (u32)
    /// * `reservation_id` -  (String)
    /// * `reserved` -  (u64)
    /// * `time_span` -  (u32)
    /// * `update_domain` -  (u32)
    /// * `version` -  (u32)
    /// * `vm_cpu_reservation` -  (u32)
    /// * `vm_flags` -  (u32)
    /// * `vm_memory` -  (u32)
    /// * `vm_queue_pair_count` -  (u32)
    /// * `vm_virtual_core_count` -  (u32)
    /// * `vm_virtual_function_count` -  (u32)

    /// * `node_id` -  (u32)
    /// * `return_value` -  (u32)
    pub fn create_vm_reservation_ex2(&self, vm_memory: u32, vm_virtual_core_count: u32, vm_cpu_reservation: u32, vm_virtual_function_count: u32, vm_queue_pair_count: u32, vm_flags: u32, time_span: u32, reservation_id: &String, node_id: &mut u32, version: Option<u32>, local_disk_size: Option<u32>, availability_set_name: &Option<String>, fault_domain: Option<u32>, update_domain: Option<u32>, reserved: Option<u64>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmMemory".to_string(), value: vm_memory.into() });
        args.push(MethodParameter { name: "VmVirtualCoreCount".to_string(), value: vm_virtual_core_count.into() });
        args.push(MethodParameter { name: "VmCpuReservation".to_string(), value: vm_cpu_reservation.into() });
        args.push(MethodParameter { name: "VmVirtualFunctionCount".to_string(), value: vm_virtual_function_count.into() });
        args.push(MethodParameter { name: "VmQueuePairCount".to_string(), value: vm_queue_pair_count.into() });
        args.push(MethodParameter { name: "VmFlags".to_string(), value: vm_flags.into() });
        args.push(MethodParameter { name: "TimeSpan".to_string(), value: time_span.into() });
        args.push(MethodParameter { name: "ReservationId".to_string(), value: reservation_id.into() });
        if let Some(val) = version {
            args.push(MethodParameter { name: "Version".to_string(), value: val.into() });
        }
        if let Some(val) = local_disk_size {
            args.push(MethodParameter { name: "LocalDiskSize".to_string(), value: val.into() });
        }
        if let Some(val) = availability_set_name {
            args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: val.into() });
        }
        if let Some(val) = fault_domain {
            args.push(MethodParameter { name: "FaultDomain".to_string(), value: val.into() });
        }
        if let Some(val) = update_domain {
            args.push(MethodParameter { name: "UpdateDomain".to_string(), value: val.into() });
        }
        if let Some(val) = reserved {
            args.push(MethodParameter { name: "Reserved".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CreateVmReservationEx2", &args)?;
        let node_id = result.get_value("NodeId")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `reservation_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_vm_reservation(&self, reservation_id: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReservationId".to_string(), value: reservation_id.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveVmReservation", &args)

    }


/// 

    /// * `version` -  (u32)
    /// * `vm_cpu_reservation` -  (u32)
    /// * `vm_flags` -  (u32)
    /// * `vm_memory` -  (u32)
    /// * `vm_resource_name` -  (String)
    /// * `vm_virtual_core_count` -  (u32)

    /// * `node_id` -  (u32)
    /// * `return_value` -  (u32)
    pub fn change_vmsettings(&self, vm_resource_name: &String, vm_memory: u32, vm_virtual_core_count: u32, vm_cpu_reservation: u32, vm_flags: u32, node_id: &mut u32, version: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmResourceName".to_string(), value: vm_resource_name.into() });
        args.push(MethodParameter { name: "VmMemory".to_string(), value: vm_memory.into() });
        args.push(MethodParameter { name: "VmVirtualCoreCount".to_string(), value: vm_virtual_core_count.into() });
        args.push(MethodParameter { name: "VmCpuReservation".to_string(), value: vm_cpu_reservation.into() });
        args.push(MethodParameter { name: "VmFlags".to_string(), value: vm_flags.into() });
        if let Some(val) = version {
            args.push(MethodParameter { name: "Version".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ChangeVMSettings", &args)?;
        let node_id = result.get_value("NodeId")?;
        Ok(result.return_value)

    }


/// 

    /// * `account_key` -  (String)
    /// * `account_name` -  (String)
    /// * `cloud_witness_name` -  (String)
    /// * `container_name` -  (String)
    /// * `endpoint_info` -  (String)
    /// * `managed_identity` -  (bool)
    /// * `sastoken` -  (String)

    /// * `return_value` -  (u32)
    pub fn create_cloud_witness(&self, account_name: &String, account_key: &String, sastoken: &String, container_name: &String, managed_identity: bool, endpoint_info: &String, cloud_witness_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountName".to_string(), value: account_name.into() });
        args.push(MethodParameter { name: "AccountKey".to_string(), value: account_key.into() });
        args.push(MethodParameter { name: "SASToken".to_string(), value: sastoken.into() });
        args.push(MethodParameter { name: "ContainerName".to_string(), value: container_name.into() });
        args.push(MethodParameter { name: "ManagedIdentity".to_string(), value: managed_identity.into() });
        args.push(MethodParameter { name: "EndpointInfo".to_string(), value: endpoint_info.into() });
        args.push(MethodParameter { name: "CloudWitnessName".to_string(), value: cloud_witness_name.into() });
        self.invoke_method("CreateCloudWitness", &args)

    }


/// 

    /// * `account_key` -  (String)
    /// * `container_name` -  (String)
    /// * `sastoken` -  (String)

    /// * `return_value` -  (u32)
    pub fn update_cloud_witness_key(&self, account_key: &String, sastoken: &String, container_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountKey".to_string(), value: account_key.into() });
        args.push(MethodParameter { name: "SASToken".to_string(), value: sastoken.into() });
        args.push(MethodParameter { name: "ContainerName".to_string(), value: container_name.into() });
        self.invoke_method("UpdateCloudWitnessKey", &args)

    }


/// 

    /// * `current_node_name` -  (String)
    /// * `replacement_node_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn node_replacement(&self, current_node_name: &String, replacement_node_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CurrentNodeName".to_string(), value: current_node_name.into() });
        args.push(MethodParameter { name: "ReplacementNodeName".to_string(), value: replacement_node_name.into() });
        self.invoke_method("NodeReplacement", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn force_flush_database(&self) -> Result<(), WmiError> {
        self.invoke_method("ForceFlushDatabase", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn create_cluster_set(&self) -> Result<(), WmiError> {
        self.invoke_method("CreateClusterSet", &[])

    }


/// 

    /// * `flags` -  (u32)
    /// * `force` -  (bool)
    /// * `remove_file_server` -  (bool)

    /// * `return_value` -  (u32)
    pub fn remove_cluster_set(&self, force: bool, remove_file_server: bool, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "RemoveFileServer".to_string(), value: remove_file_server.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveClusterSet", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `force` -  (bool)
    /// * `remove_file_server` -  (bool)

    /// * `return_value` -  (u32)
    pub fn remove_cluster_set_worker(&self, force: bool, remove_file_server: bool, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "RemoveFileServer".to_string(), value: remove_file_server.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveClusterSetWorker", &args)

    }


/// 

    /// * `cert_data` -  (u8[])
    /// * `flags` -  (u32)
    /// * `id` -  (u32)
    /// * `infra_sofsname` -  (String)
    /// * `key_data` -  (u8[])
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn setup_cluster_set_worker(&self, name: &String, infra_sofsname: &String, id: u32, cert_data: &Vec<u8>, key_data: &Vec<u8>, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "InfraSOFSName".to_string(), value: infra_sofsname.into() });
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        args.push(MethodParameter { name: "CertData".to_string(), value: cert_data.into() });
        args.push(MethodParameter { name: "KeyData".to_string(), value: key_data.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("SetupClusterSetWorker", &args)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `supported_versions` -  (MSCluster_NodeSupportedVersion[])
    pub fn get_node_supported_versions(&self, supported_versions: &mut Vec<MSCluster_NodeSupportedVersion>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetNodeSupportedVersions", &[])?;
        let supported_versions = result.get_value("SupportedVersions")?;
        Ok(result.return_value)

    }


/// 

    /// * `exclusion_type` -  (u32)
    /// * `exclusion_value` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_cluster_excluded_adapter(&self, exclusion_type: u32, exclusion_value: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ExclusionType".to_string(), value: exclusion_type.into() });
        args.push(MethodParameter { name: "ExclusionValue".to_string(), value: exclusion_value.into() });
        self.invoke_method("SetClusterExcludedAdapter", &args)

    }


/// 

    /// * `exclusion_type` -  (u32)
    /// * `exclusion_value` -  (String[])

    /// * `return_value` -  (u32)
    pub fn add_cluster_excluded_adapter(&self, exclusion_type: u32, exclusion_value: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ExclusionType".to_string(), value: exclusion_type.into() });
        args.push(MethodParameter { name: "ExclusionValue".to_string(), value: exclusion_value.into() });
        self.invoke_method("AddClusterExcludedAdapter", &args)

    }


/// 

    /// * `exclusion_type` -  (u32)
    /// * `exclusion_value` -  (String[])

    /// * `return_value` -  (u32)
    pub fn remove_cluster_excluded_adapter(&self, exclusion_type: u32, exclusion_value: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ExclusionType".to_string(), value: exclusion_type.into() });
        args.push(MethodParameter { name: "ExclusionValue".to_string(), value: exclusion_value.into() });
        self.invoke_method("RemoveClusterExcludedAdapter", &args)

    }


/// 

    /// * `exclusion_type` -  (u32)

    /// * `excluded_adapters` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_cluster_excluded_adapter(&self, exclusion_type: u32, excluded_adapters: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ExclusionType".to_string(), value: exclusion_type.into() });

        let result = self.invoke_method("GetClusterExcludedAdapter", &args)?;
        let excluded_adapters = result.get_value("ExcludedAdapters")?;
        Ok(result.return_value)

    }

}

