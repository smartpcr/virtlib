// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualEthernetSwitchManagementService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualEthernetSwitchManagementService {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemManagementService,
}

impl Msvm_VirtualEthernetSwitchManagementService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemManagementService::new(),
        }
    }


/// 

    /// * `affected_configuration` - Reference to the affected Ethernet switch port or Ethernet Switch configuration. (CIM_SettingData)
    /// * `feature_settings` - Array of strings each containing one embedded instance of class Msvm_FeatureSettingData that describes the feature settings to be added to the switch port configuration. (String[])

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instances of class Msvm_FeatureSettingData representing the added feature settings are available via association Msvm_EthernetPortSettingDataComponent from the instance of class Msvm_EthernetPortAllocationSettingData representing the affected switch port. (CIM_ConcreteJob)
    /// * `resulting_feature_settings` - Array of references to instances of class Msvm_FeatureSettingData representing the added feature settings. (Msvm_FeatureSettingData[])
    /// * `return_value` -  (u32)
    pub fn add_feature_settings(&self, affected_configuration: CIM_SettingData, feature_settings: &Vec<String>, resulting_feature_settings: &mut Vec<Msvm_FeatureSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedConfiguration".to_string(), value: affected_configuration.into() });
        args.push(MethodParameter { name: "FeatureSettings".to_string(), value: feature_settings.into() });

        let result = self.invoke_method_with_job("AddFeatureSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_feature_settings = result.get_value("ResultingFeatureSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `feature_settings` - Array of strings each containing an embedded instance of class Msvm_FeatureSettingData that describes modifications to the current feature settings of an existing Ethernet switch port. All instances must have a valid InstanceID in order to identify the feature settings to be modified. (String[])

    /// * `job` - If the operation is long running, then optionally a job be returned. In this case, the instances of class Msvm_FeatureSettingData representing the modified settings are available via association Msvm_EthernetPortSettingDataComponent from the instance of class Msvm_EthernetPortAllocationSettingData representing the affected switch port. (CIM_ConcreteJob)
    /// * `resulting_feature_settings` - Array of references to instances of class Msvm_FeatureSettingData representing the modified feature settings. (Msvm_FeatureSettingData[])
    /// * `return_value` -  (u32)
    pub fn modify_feature_settings(&self, feature_settings: &Vec<String>, resulting_feature_settings: &mut Vec<Msvm_FeatureSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FeatureSettings".to_string(), value: feature_settings.into() });

        let result = self.invoke_method_with_job("ModifyFeatureSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_feature_settings = result.get_value("ResultingFeatureSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `feature_settings` - Array of references to instances of class Msvm_FeatureSettingData that are to be removed. (Msvm_FeatureSettingData[])

    /// * `job` - If the operation is long running, then optionally a job my be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_feature_settings(&self, feature_settings: &Vec<Msvm_FeatureSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FeatureSettings".to_string(), value: feature_settings.into() });

        let result = self.invoke_method_with_job("RemoveFeatureSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_VirtualEthernetSwitchManagementService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_VirtualEthernetSwitchManagementCapabilities object(s)
    pub fn get_related__virtual_ethernet_switch_management_capabilities(&self) -> Result<Msvm_VirtualEthernetSwitchManagementCapabilities, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitchManagementCapabilities")
    }

}

