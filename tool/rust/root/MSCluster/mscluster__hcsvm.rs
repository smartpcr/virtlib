// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_HCSVM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_HCSVM {

/// 
    #[serde(rename = "CpuCount")]
    pub cpu_count: Option<u32>,

/// 
    #[serde(rename = "ExtendedVmConfiguration")]
    pub extended_vm_configuration: Option<String>,

/// 
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

/// 
    #[serde(rename = "MemorySizeInMb")]
    pub memory_size_in_mb: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NetworkEndpointId")]
    pub network_endpoint_id: Option<String>,

/// 
    #[serde(rename = "OfflineAction")]
    pub offline_action: Option<u32>,

/// 
    #[serde(rename = "ResState")]
    pub res_state: Option<String>,

/// 
    #[serde(rename = "SwitchName")]
    pub switch_name: Option<String>,

/// 
    #[serde(rename = "VhdPath")]
    pub vhd_path: Option<String>,

/// 
    #[serde(rename = "VmName")]
    pub vm_name: Option<String>,
}

impl MSCluster_HCSVM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cpu_count: None,
            extended_vm_configuration: None,
            mac_address: None,
            memory_size_in_mb: None,
            name: None,
            network_endpoint_id: None,
            offline_action: None,
            res_state: None,
            switch_name: None,
            vhd_path: None,
            vm_name: None,
        }
    }


    /// Sets the value of CpuCount
    pub fn set_cpu_count(&mut self, value: u32) {
        self.cpu_count = Some(value);
    }

    /// Gets the value of CpuCount
    pub fn get_cpu_count(&self) -> Option<&u32> {
        self.cpu_count.as_ref()
    }

    /// Sets the value of ExtendedVmConfiguration
    pub fn set_extended_vm_configuration(&mut self, value: String) {
        self.extended_vm_configuration = Some(value);
    }

    /// Gets the value of ExtendedVmConfiguration
    pub fn get_extended_vm_configuration(&self) -> Option<&String> {
        self.extended_vm_configuration.as_ref()
    }

    /// Sets the value of MacAddress
    pub fn set_mac_address(&mut self, value: String) {
        self.mac_address = Some(value);
    }

    /// Gets the value of MacAddress
    pub fn get_mac_address(&self) -> Option<&String> {
        self.mac_address.as_ref()
    }

    /// Sets the value of MemorySizeInMb
    pub fn set_memory_size_in_mb(&mut self, value: u32) {
        self.memory_size_in_mb = Some(value);
    }

    /// Gets the value of MemorySizeInMb
    pub fn get_memory_size_in_mb(&self) -> Option<&u32> {
        self.memory_size_in_mb.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NetworkEndpointId
    pub fn set_network_endpoint_id(&mut self, value: String) {
        self.network_endpoint_id = Some(value);
    }

    /// Gets the value of NetworkEndpointId
    pub fn get_network_endpoint_id(&self) -> Option<&String> {
        self.network_endpoint_id.as_ref()
    }

    /// Sets the value of OfflineAction
    pub fn set_offline_action(&mut self, value: u32) {
        self.offline_action = Some(value);
    }

    /// Gets the value of OfflineAction
    pub fn get_offline_action(&self) -> Option<&u32> {
        self.offline_action.as_ref()
    }

    /// Sets the value of ResState
    pub fn set_res_state(&mut self, value: String) {
        self.res_state = Some(value);
    }

    /// Gets the value of ResState
    pub fn get_res_state(&self) -> Option<&String> {
        self.res_state.as_ref()
    }

    /// Sets the value of SwitchName
    pub fn set_switch_name(&mut self, value: String) {
        self.switch_name = Some(value);
    }

    /// Gets the value of SwitchName
    pub fn get_switch_name(&self) -> Option<&String> {
        self.switch_name.as_ref()
    }

    /// Sets the value of VhdPath
    pub fn set_vhd_path(&mut self, value: String) {
        self.vhd_path = Some(value);
    }

    /// Gets the value of VhdPath
    pub fn get_vhd_path(&self) -> Option<&String> {
        self.vhd_path.as_ref()
    }

    /// Sets the value of VmName
    pub fn set_vm_name(&mut self, value: String) {
        self.vm_name = Some(value);
    }

    /// Gets the value of VmName
    pub fn get_vm_name(&self) -> Option<&String> {
        self.vm_name.as_ref()
    }

/// 

    /// * `cpu_count` -  (u32)
    /// * `extended_vm_configuration` -  (String)
    /// * `memory_size_in_mb` -  (u32)
    /// * `name` -  (String)
    /// * `offline_action` -  (u32)
    /// * `switch_name` -  (String)
    /// * `vhd_path` -  (String)
    /// * `vm_name` -  (String)

    /// * `created_hcsvmcluster` -  (MSCluster_HCSVM)
    /// * `return_value` -  (u32)
    pub fn new_cluster_hcsvm(&self, name: &String, switch_name: &String, extended_vm_configuration: &String, memory_size_in_mb: u32, cpu_count: u32, vhd_path: &String, vm_name: &String, offline_action: u32, created_hcsvmcluster: &mut MSCluster_HCSVM) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "SwitchName".to_string(), value: switch_name.into() });
        args.push(MethodParameter { name: "ExtendedVmConfiguration".to_string(), value: extended_vm_configuration.into() });
        args.push(MethodParameter { name: "MemorySizeInMb".to_string(), value: memory_size_in_mb.into() });
        args.push(MethodParameter { name: "CpuCount".to_string(), value: cpu_count.into() });
        args.push(MethodParameter { name: "VhdPath".to_string(), value: vhd_path.into() });
        args.push(MethodParameter { name: "VmName".to_string(), value: vm_name.into() });
        args.push(MethodParameter { name: "OfflineAction".to_string(), value: offline_action.into() });

        let result = self.invoke_method("NewClusterHCSVM", &args)?;
        let created_hcsvmcluster = result.get_value("CreatedHCSVMCluster")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_cluster_hcsvm(&self, name: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("RemoveClusterHCSVM", &args)

    }


/// 

    /// * `extended_vm_configuration` -  (String)
    /// * `name` -  (String)
    /// * `new_name` -  (String)
    /// * `offline_action` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_cluster_hcsvm(&self, name: &String, new_name: &String, extended_vm_configuration: &String, offline_action: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "ExtendedVmConfiguration".to_string(), value: extended_vm_configuration.into() });
        args.push(MethodParameter { name: "OfflineAction".to_string(), value: offline_action.into() });
        self.invoke_method("SetClusterHCSVM", &args)

    }


/// 

    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn start_cluster_hcsvm(&self, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("StartClusterHCSVM", &args)

    }


/// 

    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn stop_cluster_hcsvm(&self, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("StopClusterHCSVM", &args)

    }

}

