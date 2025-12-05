// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualSystemManagementService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualSystemManagementService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl CIM_VirtualSystemManagementService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// Adds resources to a virtual system configuration
/// .When applied to a "state" virtual system configuration, as a side effect resources are added to the active virtual system.

    /// * `affected_configuration` - Reference to the affected virtual system configuration. (CIM_VirtualSystemSettingData)
    /// * `resource_settings` - Array of strings each containing one embedded instance of class CIM_ResourceAllocationSettingData that describes the virtual aspects of a virtual resource to be added to the virtual system. (String[])

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instances of class CIM_ResourceAllocationSettingData representing the added resource settings are available via association CIM_ConreteComponent from the instance of class CIM_VirtualSystemSettingData representing the affected virtual system configuration. (CIM_ConcreteJob)
    /// * `resulting_resource_settings` - Array of references to instances of class CIM_ResourceAllocationSettingData representing virtual aspects of the added virtual resources. (CIM_ResourceAllocationSettingData[])
    /// * `return_value` -  (u32)
    pub fn add_resource_settings(&self, affected_configuration: CIM_VirtualSystemSettingData, resource_settings: &Vec<String>, resulting_resource_settings: &mut Vec<CIM_ResourceAllocationSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedConfiguration".to_string(), value: affected_configuration.into() });
        args.push(MethodParameter { name: "ResourceSettings".to_string(), value: resource_settings.into() });

        let result = self.invoke_method_with_job("AddResourceSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_resource_settings = result.get_value("ResultingResourceSettings")?;
        Ok(result.return_value)

    }


/// Defines a virtual system.
/// Input that is not completely specified may be filled out with default values.

    /// * `reference_configuration` - Reference to an instance of class CIM_VirtualSystemSettingData object that is the top level object of a reference virtual system configuration. The reference configuration is used to complement the configuration of the new virtual system if parameters SystemSettings and ResourceSettings did not provide respective information. (CIM_VirtualSystemSettingData)
    /// * `resource_settings` - Array of strings each containing an embedded instance of class CIM_ResourceAllocationSettingData that describes the virtual aspects of a virtual resource to be created in the scope of the new virtual system. (String[])
    /// * `system_settings` - String containing an embedded instance of class CIM_VirtualSystemSettingData that is used to define attributes of the virtual system to be created. (String)

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instance of class CIM_ComputerSystem representing the new virtual systemis presented via association CIM_AffectedJobElementwith property AffectedElement refering to the new instance of class CIM_ComputerSystem and property ElementEffects set to 5 (Create). (CIM_ConcreteJob)
    /// * `resulting_system` - If a virtual computer system is successfully defined, a reference to an instance of class CIM_ComputerSystem that represents the newly defined virtual computer system is returned. (CIM_ComputerSystem)
    /// * `return_value` -  (u32)
    pub fn define_system(&self, system_settings: &String, resource_settings: &Vec<String>, reference_configuration: CIM_VirtualSystemSettingData, resulting_system: &mut CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SystemSettings".to_string(), value: system_settings.into() });
        args.push(MethodParameter { name: "ResourceSettings".to_string(), value: resource_settings.into() });
        args.push(MethodParameter { name: "ReferenceConfiguration".to_string(), value: reference_configuration.into() });

        let result = self.invoke_method_with_job("DefineSystem", &args)?;
        let job = result.get_value("Job")?;
        let resulting_system = result.get_value("ResultingSystem")?;
        Ok(result.return_value)

    }


/// Destroys a virtual system.
/// The referenced virtual system is destroyed, including any elements scoped by it. Virtual resources are returned to their resource pools, which may imply the destruction of those resources (implementation dependent). If the virtual system is active when the operation is invoked, it is first deactivated and then destroyed. If snapshots were created from the virtual system, these are destroyed as well.

    /// * `affected_system` - Reference to an instance of class CIM_ComputerSystem representing the virtual computer system that it to be destroyed. (CIM_ComputerSystem)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn destroy_system(&self, affected_system: CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSystem".to_string(), value: affected_system.into() });

        let result = self.invoke_method_with_job("DestroySystem", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Modifies virtual resource settings.
/// When applied to parts of a "current" virtual system configuration, as a side effect resources of the active virtual system may be modified.

    /// * `resource_settings` - Array of strings each containing an embedded instance of class CIM_ResourceAllocationSettingData that describes modifications to the virtual aspects of an existing virtual resource. All instances must have a valid InstanceID in order to identify the virtual resource setting to be modified. (String[])

    /// * `job` - If the operation is long running, then optionally a job be returned. In this case, the instances of class CIM_ResourceAllocationSettingData representing the modified resource settings are available via association CIM_ConreteComponent from the instance of class CIM_VirtualSystemSettingData representing the affected virtual system configuration. (CIM_ConcreteJob)
    /// * `resulting_resource_settings` - Array of references to instances of class Cim_ResourceAllocationSettingData representing virtual aspects of the modified virtual resources. (CIM_ResourceAllocationSettingData[])
    /// * `return_value` -  (u32)
    pub fn modify_resource_settings(&self, resource_settings: &Vec<String>, resulting_resource_settings: &mut Vec<CIM_ResourceAllocationSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResourceSettings".to_string(), value: resource_settings.into() });

        let result = self.invoke_method_with_job("ModifyResourceSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_resource_settings = result.get_value("ResultingResourceSettings")?;
        Ok(result.return_value)

    }


/// Modifies virtual system settings.
/// When applied to the system settings of a "current" virtual system configuration, as a side effect the virtual system instance may be modified.

    /// * `system_settings` - String containing an instance of class CIM_VirtualSystemSettingData that is used to modify the settings of the virtual system. The instance must have a valid InstanceID in order to identify the virtual system setting to be modified. (String)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_system_settings(&self, system_settings: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SystemSettings".to_string(), value: system_settings.into() });

        let result = self.invoke_method_with_job("ModifySystemSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Removes virtual resource settings from a virtual system configuration.
/// When applied to parts of a "current" virtual system configuration, as a side effect resources of the active virtual system may be removed.

    /// * `resource_settings` - Array of references to instances of class CIM_ResourceAllocationSettingData where each instance represents the settings of a virtual resource within a virtual system configuration that are to be removed. (CIM_ResourceAllocationSettingData[])

    /// * `job` - If the operation is long running, then optionally a job my be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_resource_settings(&self, resource_settings: &Vec<CIM_ResourceAllocationSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResourceSettings".to_string(), value: resource_settings.into() });

        let result = self.invoke_method_with_job("RemoveResourceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

