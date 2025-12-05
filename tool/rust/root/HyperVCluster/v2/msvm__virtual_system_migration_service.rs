// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;
use Microsoft.Test.Wmi.root.virtualization.v2;


/// Msvm_VirtualSystemMigrationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemMigrationService {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemMigrationService,

/// 
    #[serde(rename = "ActiveStorageMigrationCount")]
    pub active_storage_migration_count: Option<u32>,

/// 
    #[serde(rename = "ActiveVirtualSystemMigrationCount")]
    pub active_virtual_system_migration_count: Option<u32>,

/// 
    #[serde(rename = "MigrationServiceListenerIPAddressList")]
    pub migration_service_listener_ipaddress_list: Vec<String>,
}

impl Msvm_VirtualSystemMigrationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemMigrationService::new(),
            active_storage_migration_count: None,
            active_virtual_system_migration_count: None,
            migration_service_listener_ipaddress_list: Vec::new(),
        }
    }


    /// Sets the value of ActiveStorageMigrationCount
    pub fn set_active_storage_migration_count(&mut self, value: u32) {
        self.active_storage_migration_count = Some(value);
    }

    /// Gets the value of ActiveStorageMigrationCount
    pub fn get_active_storage_migration_count(&self) -> Option<&u32> {
        self.active_storage_migration_count.as_ref()
    }

    /// Sets the value of ActiveVirtualSystemMigrationCount
    pub fn set_active_virtual_system_migration_count(&mut self, value: u32) {
        self.active_virtual_system_migration_count = Some(value);
    }

    /// Gets the value of ActiveVirtualSystemMigrationCount
    pub fn get_active_virtual_system_migration_count(&self) -> Option<&u32> {
        self.active_virtual_system_migration_count.as_ref()
    }

    /// Sets the value of MigrationServiceListenerIPAddressList
    pub fn set_migration_service_listener_ipaddress_list(&mut self, value: Vec<String>) {
        self.migration_service_listener_ipaddress_list = value;
    }

    /// Gets the value of MigrationServiceListenerIPAddressList
    pub fn get_migration_service_listener_ipaddress_list(&self) -> &Vec<String> {
        &self.migration_service_listener_ipaddress_list
    }

/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `destination_host` -  (String)
    /// * `migration_setting_data` -  (String)
    /// * `new_resource_setting_data` -  (String[])
    /// * `new_system_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn check_virtual_system_is_migratable(&self, computer_system: CIM_ComputerSystem, destination_host: &String, migration_setting_data: &String, new_system_setting_data: &String, new_resource_setting_data: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "DestinationHost".to_string(), value: destination_host.into() });
        args.push(MethodParameter { name: "MigrationSettingData".to_string(), value: migration_setting_data.into() });
        args.push(MethodParameter { name: "NewSystemSettingData".to_string(), value: new_system_setting_data.into() });
        args.push(MethodParameter { name: "NewResourceSettingData".to_string(), value: new_resource_setting_data.into() });

        let result = self.invoke_method_with_job("CheckVirtualSystemIsMigratable", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `service_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_service_settings(&self, service_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServiceSettingData".to_string(), value: service_setting_data.into() });

        let result = self.invoke_method_with_job("ModifyServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `network_settings` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn add_network_settings(&self, network_settings: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NetworkSettings".to_string(), value: network_settings.into() });

        let result = self.invoke_method_with_job("AddNetworkSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `network_settings` -  (Msvm_VirtualSystemMigrationNetworkSettingData[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_network_settings(&self, network_settings: &Vec<Msvm_VirtualSystemMigrationNetworkSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NetworkSettings".to_string(), value: network_settings.into() });

        let result = self.invoke_method_with_job("RemoveNetworkSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `network_settings` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_network_settings(&self, network_settings: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NetworkSettings".to_string(), value: network_settings.into() });

        let result = self.invoke_method_with_job("ModifyNetworkSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `compatibility_info` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_system_compatibility_info(&self, computer_system: CIM_ComputerSystem, compatibility_info: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method("GetSystemCompatibilityInfo", &args)?;
        let compatibility_info = result.get_value("CompatibilityInfo")?;
        Ok(result.return_value)

    }


/// 

    /// * `compatibility_info` -  (u8[])

    /// * `reasons` -  (String[])
    /// * `return_value` -  (u32)
    pub fn check_system_compatibility_info(&self, compatibility_info: &Vec<u8>, reasons: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CompatibilityInfo".to_string(), value: compatibility_info.into() });

        let result = self.invoke_method("CheckSystemCompatibilityInfo", &args)?;
        let reasons = result.get_value("Reasons")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `compatibility_vectors` -  (Msvm_CompatibilityVector[])
    /// * `return_value` -  (u32)
    pub fn get_system_compatibility_vectors(&self, computer_system: CIM_ComputerSystem, compatibility_vectors: &mut Vec<Msvm_CompatibilityVector>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method("GetSystemCompatibilityVectors", &args)?;
        let compatibility_vectors = result.get_value("CompatibilityVectors")?;
        Ok(result.return_value)

    }


/// 

    /// * `options` -  (String)

    /// * `compatibility_vectors` -  (Msvm_CompatibilityVector[])
    /// * `return_value` -  (u32)
    pub fn get_processor_feature_limits(&self, options: &String, compatibility_vectors: &mut Vec<Msvm_CompatibilityVector>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });

        let result = self.invoke_method("GetProcessorFeatureLimits", &args)?;
        let compatibility_vectors = result.get_value("CompatibilityVectors")?;
        Ok(result.return_value)

    }

}

impl Msvm_VirtualSystemMigrationService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

