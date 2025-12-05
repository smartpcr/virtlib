// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualSystemMigrationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualSystemMigrationService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl CIM_VirtualSystemMigrationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// Method to move, migrate or relocate a virtual system to a target host specified by a network name or IP address.
/// Return code description:
/// 0: Success: Virtual system was migrated.
/// 1: Error: Method not supported by implementation.
/// 2: Error: Virtual system migration failed for unspecified reasons.
/// 3: Error: Virtual system migration time out; the virtual system state is unknown.
/// 4: Error: One or more parameters are formally invalid. For example, the value of the DestinationHost parameter could have been specified in an unsupported format.
/// 5: Error: The source virtual system, the source host system or the target host system are in a state that does allow initiation of the requested virtual system migration; this may be a temporary condition.
/// 6: Error: One or more input parameters are incompatible as a set, or with respect to the target host. For example the value of the MigrationNewSettingData parameter contains properties that are not supported by the target host system identified by the value of the DestinationHost parameter. Note: The MigrateVirtualSystemToHost( ) methods is intended as a transitional solution only until modelling of cluster support is available.

    /// * `computer_system` - Source virtual computer system to be migrated. (CIM_ComputerSystem)
    /// * `destination_host` - Target host system for the migration. Acceptable formats for this parameter are conveyed through values of elements of the DestinationHostFormatsSupported[ ] array property in the instance of the CIM_VirtualSystemMigrationCapabilities that is associated through the CIM_ElementCapabilities assocation. (String)
    /// * `migration_setting_data` - String containing an embedded instance of the CIM_VirtualSystemMigrationSettingData class representing migration settings applicable to the migration operation. (String)
    /// * `new_resource_setting_data` - Array of strings each containing an embedded instance of the CIM_ResourceAllocationSettingData class representing new properties applicable to virtual resources in the scope of the virtual system after it is migrated. (String[])
    /// * `new_system_setting_data` - String containing an embedded instance of the CIM_VirtualSystemSettingData class representing new properties applicable to the virtual system after it is migrated. (String)

    /// * `job` - If operation is long running then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn migrate_virtual_system_to_host(&self, computer_system: CIM_ComputerSystem, destination_host: &String, migration_setting_data: &String, new_system_setting_data: &String, new_resource_setting_data: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "DestinationHost".to_string(), value: destination_host.into() });
        args.push(MethodParameter { name: "MigrationSettingData".to_string(), value: migration_setting_data.into() });
        args.push(MethodParameter { name: "NewSystemSettingData".to_string(), value: new_system_setting_data.into() });
        args.push(MethodParameter { name: "NewResourceSettingData".to_string(), value: new_resource_setting_data.into() });

        let result = self.invoke_method_with_job("MigrateVirtualSystemToHost", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Method to move, migrate or relocate a virtual system to a target system.
/// Return code description:
/// 0: Success: Virtual system was migrated
/// 1: Error: Method not supported by implementation
/// 2: Error: Virtual system migration failed for unspecified reasons
/// 3: Error: Virtual system migration time out; the virtual system state is unknown
/// 4: Error: One or more parameters are formally invalid For example, the value of the Destination System parameter does not contain a valid object path
/// 5: Error: The source virtual system, the source host system or the target host system are in a state that does allow initiation of the requested virtual system migration; this may be a temporary condition.
/// 6: Error: One or more input parameters are incompatible as a set, or with respect to the target host. For example the value of the MigrationNewSettingData parameter contains properties that are not supported by the target host system identified by the value of the DestinationSystem parameter.

    /// * `computer_system` - Source virtual computer system to be migrated. (CIM_ComputerSystem)
    /// * `destination_system` - Destination host system whereto migrate the virtual system. (CIM_System)
    /// * `migration_setting_data` - String containing an embedded instance of the CIM_VirtualSystemMigrationSettingData class representing migration settings applicable to the migration operation. (String)
    /// * `new_resource_setting_data` - Array of strings each containing an embedded instance of the CIM_ResourceAllocationSettingData class representing new properties applicable to virtual resources in the scope of the virtual system after it is migrated. (String[])
    /// * `new_system_setting_data` - String containing an embedded instance of the CIM_VirtualSystemSettingData class representing new properties applicable to the virtual system after it is migrated. (String)

    /// * `job` - If operation is long running then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `new_computer_system` - Reference to an instance of the CIM_ComputerSystem class representing the virtual computer system after it has been migrated. (CIM_ComputerSystem)
    /// * `return_value` -  (u32)
    pub fn migrate_virtual_system_to_system(&self, computer_system: CIM_ComputerSystem, destination_system: CIM_System, migration_setting_data: &String, new_system_setting_data: &String, new_resource_setting_data: &Vec<String>, new_computer_system: &mut CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "DestinationSystem".to_string(), value: destination_system.into() });
        args.push(MethodParameter { name: "MigrationSettingData".to_string(), value: migration_setting_data.into() });
        args.push(MethodParameter { name: "NewSystemSettingData".to_string(), value: new_system_setting_data.into() });
        args.push(MethodParameter { name: "NewResourceSettingData".to_string(), value: new_resource_setting_data.into() });

        let result = self.invoke_method_with_job("MigrateVirtualSystemToSystem", &args)?;
        let job = result.get_value("Job")?;
        let new_computer_system = result.get_value("NewComputerSystem")?;
        Ok(result.return_value)

    }


/// Method to perform a pre-check to determine whether a virtual system is likely to be successfully migrated to a target host specified by a network name or IP address. This method does not guarantee that a subsequent migration will always succeed, due to dynamic resource availability.
/// Return code description:
/// 0: Success: Check performed; result reported through the value of the [Out] IsMigratable parameter.
/// 1: Error: Method not supported by implementation. No result reported through the value of the [Out] IsMigratable parameter.
/// 2: Error: Check failed for unspecified reasons. No result reported through the value of the [Out] IsMigratable parameter.
/// 3: Error: Check timed out. No result reported through the value of the [Out] IsMigratable parameter.
/// 4: Error: One or more parameters are formally invalid. For example, the value of the DestinationHost parameter could have been specified in an unsupported format.
/// No result reported through the value of the [Out] IsMigratable parameter.
/// 5: Error: The source virtual system, the source host system or the target host system are in a state that does allow initiation of the requested virtual system migration; this may be a temporary condition. No result reported reported through the value of the [Out] IsMigratable parameter.
/// 6: Error: One or more input parameters are incompatible as a set, or with respect to the target host. For example the value of the MigrationNewSettingData parameter contains properties that are not supported by the target host system identified by the value of the DestinationHost parameter. No result reported through the value of the [Out] IsMigratable parameter.
/// Note: The CheckVirtualSystemIsMigratableToHost( ) method is intended as a transitional solution only until modelling of cluster support is available.

    /// * `computer_system` - Source virtual computer system to be migrated. (CIM_ComputerSystem)
    /// * `destination_host` - Target host system for the migration. Acceptable formats for this parameter are conveyed through values of elements of the DestinationHostFormatsSupported[ ] array property in the instance of the CIM_VirtualSystemMigrationCapabilities that is associated through the CIM_ElementCapabilities assocation. (String)
    /// * `migration_setting_data` - String containing an embedded instance of the CIM_VirtualSystemMigrationSettingData class representing migration settings applicable to the migration operation. (String)
    /// * `new_resource_setting_data` - Array of strings each containing an embedded instance of the CIM_ResourceAllocationSettingData class representing new properties applicable to virtual resources in the scope of the virtual system after it is migrated. (String[])
    /// * `new_system_setting_data` - String containing an embedded instance of the CIM_VirtualSystemSettingData class representing new properties applicable to the virtual system after it is migrated. (String)

    /// * `is_migratable` - The migration check result indicating whether or not the virtual system can be successfully migrated. (bool)
    /// * `return_value` -  (u32)
    pub fn check_virtual_system_is_migratable_to_host(&self, computer_system: CIM_ComputerSystem, destination_host: &String, migration_setting_data: &String, new_system_setting_data: &String, new_resource_setting_data: &Vec<String>, is_migratable: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "DestinationHost".to_string(), value: destination_host.into() });
        args.push(MethodParameter { name: "MigrationSettingData".to_string(), value: migration_setting_data.into() });
        args.push(MethodParameter { name: "NewSystemSettingData".to_string(), value: new_system_setting_data.into() });
        args.push(MethodParameter { name: "NewResourceSettingData".to_string(), value: new_resource_setting_data.into() });

        let result = self.invoke_method("CheckVirtualSystemIsMigratableToHost", &args)?;
        let is_migratable = result.get_value("IsMigratable")?;
        Ok(result.return_value)

    }


/// Method to perform a pre-check to determine whether a virtual system is likely to be successfully migrated to a target system. This method does not guarantee that a subsequent migration will always succeed, due to dynamic resource availability. Return code description:
/// 0: Success: Check performed; result reported through the value of the [Out] IsMigratable parameter.
/// 1: Error: Method not supported by implementation. No result reported through the value of the [Out] IsMigratable parameter.
/// 2: Error: Check failed for unspecified reasons. No result reported through the value of the [Out] IsMigratable parameter.
/// 3: Error: Check timed out. No result reported through the value of the [Out] IsMigratable parameter.
/// 4: Error: One or more parameters are formally invalid. For example, the value of the DestinationSystem parameter does not comprise a valid object path. No result reported through the value of the [Out] IsMigratable parameter.
/// 5: Error: The source virtual system, the source host system or the target host system are in a state that does allow initiation of the requested virtual system migration; this may be a temporary condition. No result reported reported through the value of the [Out] IsMigratable parameter.
/// 6: Error: One or more input parameters are incompatible as a set, or with respect to the target host. For example the value of the NewSettingData parameter contains properties that are not supported by the target host system identified by the value of the DestinationSystem parameter. No result reported through the value of the [Out] IsMigratable parameter.

    /// * `computer_system` - Source virtual computer system to be migrated. (CIM_ComputerSystem)
    /// * `destination_system` - Destination system onto which to migrate the virtual system. (CIM_System)
    /// * `migration_setting_data` - String containing an embedded instance of the CIM_VirtualSystemMigrationSettingData class representing migration settings applicable to the migration operation. (String)
    /// * `new_resource_setting_data` - Array of strings each containing an embedded instance of the CIM_ResourceAllocationSettingData class representing new properties applicable to virtual resources in the scope of the virtual system after it is migrated. (String[])
    /// * `new_system_setting_data` - String containing an embedded instance of the CIM_VirtualSystemSettingData class representing new properties applicable to the virtual system after it is migrated. (String)

    /// * `is_migratable` - The migration check result indicating whether or not the virtual system can be successfully migrated. (bool)
    /// * `return_value` -  (u32)
    pub fn check_virtual_system_is_migratable_to_system(&self, computer_system: CIM_ComputerSystem, destination_system: CIM_System, migration_setting_data: &String, new_system_setting_data: &String, new_resource_setting_data: &Vec<String>, is_migratable: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "DestinationSystem".to_string(), value: destination_system.into() });
        args.push(MethodParameter { name: "MigrationSettingData".to_string(), value: migration_setting_data.into() });
        args.push(MethodParameter { name: "NewSystemSettingData".to_string(), value: new_system_setting_data.into() });
        args.push(MethodParameter { name: "NewResourceSettingData".to_string(), value: new_resource_setting_data.into() });

        let result = self.invoke_method("CheckVirtualSystemIsMigratableToSystem", &args)?;
        let is_migratable = result.get_value("IsMigratable")?;
        Ok(result.return_value)

    }

}

