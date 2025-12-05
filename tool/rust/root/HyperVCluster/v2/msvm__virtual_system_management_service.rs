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


/// Msvm_VirtualSystemManagementService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemManagementService {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemManagementService,
}

impl Msvm_VirtualSystemManagementService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemManagementService::new(),
        }
    }


/// 

    /// * `reference_configuration` - Reference to an instance of class CIM_VirtualSystemSettingData object that is the top level object of a reference virtual system configuration. The reference configuration is used to complement the configuration of the new virtual system if parameters SystemSettings and ResourceSettings did not provide respective information. (CIM_VirtualSystemSettingData)
    /// * `resource_settings` - Array of strings each containing an embedded instance of class CIM_ResourceAllocationSettingData that describes the virtual aspects of a virtual resource to be created in the scope of the new virtual system. (String[])
    /// * `system_settings` - String containing an embedded instance of class CIM_VirtualSystemSettingData that is used to define attributes of the virtual system to be created. (String)

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instance of class CIM_ComputerSystem representing the new virtual systemis presented via association CIM_AffectedJobElementwith property AffectedElement refering to the new instance of class CIM_ComputerSystem and property ElementEffects set to 5 (Create). (CIM_ConcreteJob)
    /// * `resulting_system` - If a virtual computer system is successfully defined, a reference to an instance of class CIM_ComputerSystem that represents the newly defined virtual computer system is returned. (CIM_ComputerSystem)
    /// * `return_value` -  (u32)
    pub fn define_planned_system(&self, system_settings: &String, resource_settings: &Vec<String>, reference_configuration: CIM_VirtualSystemSettingData, resulting_system: &mut CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SystemSettings".to_string(), value: system_settings.into() });
        args.push(MethodParameter { name: "ResourceSettings".to_string(), value: resource_settings.into() });
        args.push(MethodParameter { name: "ReferenceConfiguration".to_string(), value: reference_configuration.into() });

        let result = self.invoke_method_with_job("DefinePlannedSystem", &args)?;
        let job = result.get_value("Job")?;
        let resulting_system = result.get_value("ResultingSystem")?;
        Ok(result.return_value)

    }


/// 

    /// * `planned_system` -  (Msvm_PlannedComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn validate_planned_system(&self, planned_system: Msvm_PlannedComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PlannedSystem".to_string(), value: planned_system.into() });

        let result = self.invoke_method_with_job("ValidatePlannedSystem", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `upgrade_setting_data` - String containing an instance of class CIM_SettingData that is used to upgrade the virtual system. (String)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn upgrade_system_version(&self, computer_system: CIM_ComputerSystem, upgrade_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "UpgradeSettingData".to_string(), value: upgrade_setting_data.into() });

        let result = self.invoke_method_with_job("UpgradeSystemVersion", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `generate_new_system_identifier` -  (bool)
    /// * `snapshot_folder` -  (String)
    /// * `system_definition_file` -  (String)

    /// * `imported_system` -  (Msvm_PlannedComputerSystem)
    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn import_system_definition(&self, system_definition_file: &String, snapshot_folder: &String, generate_new_system_identifier: bool, imported_system: &mut Msvm_PlannedComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SystemDefinitionFile".to_string(), value: system_definition_file.into() });
        args.push(MethodParameter { name: "SnapshotFolder".to_string(), value: snapshot_folder.into() });
        args.push(MethodParameter { name: "GenerateNewSystemIdentifier".to_string(), value: generate_new_system_identifier.into() });

        let result = self.invoke_method_with_job("ImportSystemDefinition", &args)?;
        let imported_system = result.get_value("ImportedSystem")?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `planned_system` -  (Msvm_PlannedComputerSystem)
    /// * `snapshot_folder` -  (String)

    /// * `imported_snapshots` -  (Msvm_VirtualSystemSettingData[])
    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn import_snapshot_definitions(&self, planned_system: Msvm_PlannedComputerSystem, snapshot_folder: &String, imported_snapshots: &mut Vec<Msvm_VirtualSystemSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PlannedSystem".to_string(), value: planned_system.into() });
        args.push(MethodParameter { name: "SnapshotFolder".to_string(), value: snapshot_folder.into() });

        let result = self.invoke_method_with_job("ImportSnapshotDefinitions", &args)?;
        let imported_snapshots = result.get_value("ImportedSnapshots")?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `planned_system` -  (Msvm_PlannedComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `resulting_system` -  (CIM_ComputerSystem)
    /// * `return_value` -  (u32)
    pub fn realize_planned_system(&self, planned_system: Msvm_PlannedComputerSystem, resulting_system: &mut CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PlannedSystem".to_string(), value: planned_system.into() });

        let result = self.invoke_method_with_job("RealizePlannedSystem", &args)?;
        let job = result.get_value("Job")?;
        let resulting_system = result.get_value("ResultingSystem")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `export_directory` -  (String)
    /// * `export_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn export_system_definition(&self, computer_system: CIM_ComputerSystem, export_directory: &String, export_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ExportDirectory".to_string(), value: export_directory.into() });
        args.push(MethodParameter { name: "ExportSettingData".to_string(), value: export_setting_data.into() });

        let result = self.invoke_method_with_job("ExportSystemDefinition", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_configuration` - Reference to the affected Ethernet connection. (Msvm_EthernetPortAllocationSettingData)
    /// * `feature_settings` - Array of strings each containing one embedded instance of class Msvm_EthernetSwitchPortFeatureSettingData that describes the feature settings to be added to the connection configuration. (String[])

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instances of class Msvm_EthernetSwitchPortFeatureSettingData representing the added feature settings are available via association Msvm_EthernetPortSettingDataComponent from the instance of class Msvm_EthernetPortAllocationSettingData representing the affected switch port. (CIM_ConcreteJob)
    /// * `resulting_feature_settings` - Array of references to instances of class Msvm_EthernetSwitchPortFeatureSettingData representing the added feature settings. (Msvm_EthernetSwitchPortFeatureSettingData[])
    /// * `return_value` -  (u32)
    pub fn add_feature_settings(&self, affected_configuration: Msvm_EthernetPortAllocationSettingData, feature_settings: &Vec<String>, resulting_feature_settings: &mut Vec<Msvm_EthernetSwitchPortFeatureSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedConfiguration".to_string(), value: affected_configuration.into() });
        args.push(MethodParameter { name: "FeatureSettings".to_string(), value: feature_settings.into() });

        let result = self.invoke_method_with_job("AddFeatureSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_feature_settings = result.get_value("ResultingFeatureSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `feature_settings` - Array of strings each containing an embedded instance of class Msvm_EthernetSwitchPortFeatureSettingData that describes modifications to the current feature settings of an existing Ethernet connection. All instances must have a valid InstanceID in order to identify the feature settings to be modified. (String[])

    /// * `job` - If the operation is long running, then optionally a job be returned. In this case, the instances of class Msvm_EthernetSwitchPortFeatureSettingData representing the modified settings are available via association Msvm_EthernetPortSettingDataComponent from the instance of class Msvm_EthernetPortAllocationSettingData representing the affected switch port. (CIM_ConcreteJob)
    /// * `resulting_feature_settings` - Array of references to instances of class Msvm_EthernetSwitchPortFeatureSettingData representing the modified feature settings. (Msvm_EthernetSwitchPortFeatureSettingData[])
    /// * `return_value` -  (u32)
    pub fn modify_feature_settings(&self, feature_settings: &Vec<String>, resulting_feature_settings: &mut Vec<Msvm_EthernetSwitchPortFeatureSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FeatureSettings".to_string(), value: feature_settings.into() });

        let result = self.invoke_method_with_job("ModifyFeatureSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_feature_settings = result.get_value("ResultingFeatureSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `feature_settings` - Array of references to instances of class Msvm_EthernetSwitchPortFeatureSettingData that are to be removed. (Msvm_EthernetSwitchPortFeatureSettingData[])

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_feature_settings(&self, feature_settings: &Vec<Msvm_EthernetSwitchPortFeatureSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FeatureSettings".to_string(), value: feature_settings.into() });

        let result = self.invoke_method_with_job("RemoveFeatureSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_configuration` - Reference to the affected virtual system configuration. (CIM_VirtualSystemSettingData)
    /// * `boot_source_settings` - Array of strings each containing one embedded instance of class CIM_SettingData that describes the virtual aspects of a virtual resource to be added to the virtual system. (String[])

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instances of class CIM_SettingData representing the added boot source settings are available via association CIM_ConreteComponent from the instance of class CIM_VirtualSystemSettingData representing the affected virtual system configuration. (CIM_ConcreteJob)
    /// * `resulting_boot_source_settings` - Array of references to instances of class Msvm_BootSourceSettingData representing properties of the boot sources. (CIM_SettingData[])
    /// * `return_value` -  (u32)
    pub fn add_boot_source_settings(&self, affected_configuration: CIM_VirtualSystemSettingData, boot_source_settings: &Vec<String>, resulting_boot_source_settings: &mut Vec<CIM_SettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedConfiguration".to_string(), value: affected_configuration.into() });
        args.push(MethodParameter { name: "BootSourceSettings".to_string(), value: boot_source_settings.into() });

        let result = self.invoke_method_with_job("AddBootSourceSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_boot_source_settings = result.get_value("ResultingBootSourceSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_configuration` - Reference to the affected virtual system configuration. (CIM_VirtualSystemSettingData)
    /// * `guest_service_settings` - Array of strings each containing an embedded instance of class CIM_SettingData that describes addition of the virtual aspects of a guest service. All instances must have a valid service ID in order to identify the guest service setting to be added. (String[])

    /// * `job` - If the operation is long running, then optionally a job be returned. In this case, the instances of class CIM_SettingData representing the added guest service settings are available via association CIM_ConreteComponent from the instance of class CIM_VirtualSystemSettingData representing the affected virtual system configuration. (CIM_ConcreteJob)
    /// * `resulting_guest_service_settings` - Array of references to instances of class Cim_SettingData representing virtual aspects of the modified guest services. (CIM_SettingData[])
    /// * `return_value` -  (u32)
    pub fn add_guest_service_settings(&self, affected_configuration: CIM_VirtualSystemSettingData, guest_service_settings: &Vec<String>, resulting_guest_service_settings: &mut Vec<CIM_SettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedConfiguration".to_string(), value: affected_configuration.into() });
        args.push(MethodParameter { name: "GuestServiceSettings".to_string(), value: guest_service_settings.into() });

        let result = self.invoke_method_with_job("AddGuestServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_guest_service_settings = result.get_value("ResultingGuestServiceSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `guest_service_settings` - Array of strings each containing an embedded instance of class CIM_SettingData that describes modifications to the virtual aspects of an existing guest service. All instances must have a valid InstanceID in order to identify the guest service setting to be modified. (String[])

    /// * `job` - If the operation is long running, then optionally a job be returned. In this case, the instances of class CIM_SettingData representing the modified guest service settings are available via association CIM_ConreteComponent from the instance of class CIM_VirtualSystemSettingData representing the affected virtual system configuration. (CIM_ConcreteJob)
    /// * `resulting_guest_service_settings` - Array of references to instances of class Cim_SettingData representing virtual aspects of the modified guest services. (CIM_SettingData[])
    /// * `return_value` -  (u32)
    pub fn modify_guest_service_settings(&self, guest_service_settings: &Vec<String>, resulting_guest_service_settings: &mut Vec<CIM_SettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GuestServiceSettings".to_string(), value: guest_service_settings.into() });

        let result = self.invoke_method_with_job("ModifyGuestServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_guest_service_settings = result.get_value("ResultingGuestServiceSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `boot_source_settings` - Array of references to instances of class CIM_ResourceAllocationSettingData where each instance represents the settings of a boot source within a virtual system configuration that are to be removed. (CIM_SettingData[])

    /// * `job` - If the operation is long running, then optionally a job my be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_boot_source_settings(&self, boot_source_settings: &Vec<CIM_SettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BootSourceSettings".to_string(), value: boot_source_settings.into() });

        let result = self.invoke_method_with_job("RemoveBootSourceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `guest_service_settings` - Array of references to instances of class Cim_SettingData representing virtual aspects of the modified guest services. (CIM_SettingData[])

    /// * `job` - If the operation is long running, then optionally a job my be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_guest_service_settings(&self, guest_service_settings: &Vec<CIM_SettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GuestServiceSettings".to_string(), value: guest_service_settings.into() });

        let result = self.invoke_method_with_job("RemoveGuestServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `height_pixels` -  (u16)
    /// * `target_system` -  (CIM_VirtualSystemSettingData)
    /// * `width_pixels` -  (u16)

    /// * `image_data` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_virtual_system_thumbnail_image(&self, target_system: CIM_VirtualSystemSettingData, width_pixels: u16, height_pixels: u16, image_data: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetSystem".to_string(), value: target_system.into() });
        args.push(MethodParameter { name: "WidthPixels".to_string(), value: width_pixels.into() });
        args.push(MethodParameter { name: "HeightPixels".to_string(), value: height_pixels.into() });

        let result = self.invoke_method("GetVirtualSystemThumbnailImage", &args)?;
        let image_data = result.get_value("ImageData")?;
        Ok(result.return_value)

    }


/// 

    /// * `setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_service_settings(&self, setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SettingData".to_string(), value: setting_data.into() });

        let result = self.invoke_method_with_job("ModifyServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `requested_information` -  (u32[])
    /// * `setting_data` -  (CIM_VirtualSystemSettingData[])

    /// * `return_value` -  (u32)
    /// * `summary_information` -  (Msvm_SummaryInformationBase[])
    pub fn get_summary_information(&self, setting_data: &Vec<CIM_VirtualSystemSettingData>, requested_information: &Vec<u32>, summary_information: &mut Vec<Msvm_SummaryInformationBase>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SettingData".to_string(), value: setting_data.into() });
        args.push(MethodParameter { name: "RequestedInformation".to_string(), value: requested_information.into() });

        let result = self.invoke_method("GetSummaryInformation", &args)?;
        let summary_information = result.get_value("SummaryInformation")?;
        Ok(result.return_value)

    }


/// 

    /// * `definition_files` -  (String[])

    /// * `return_value` -  (u32)
    /// * `summary_information` -  (Msvm_SummaryInformationBase[])
    pub fn get_definition_file_summary_information(&self, definition_files: &Vec<String>, summary_information: &mut Vec<Msvm_SummaryInformationBase>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DefinitionFiles".to_string(), value: definition_files.into() });

        let result = self.invoke_method("GetDefinitionFileSummaryInformation", &args)?;
        let summary_information = result.get_value("SummaryInformation")?;
        Ok(result.return_value)

    }


/// 

    /// * `data_items` -  (String[])
    /// * `target_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn add_kvp_items(&self, target_system: CIM_ComputerSystem, data_items: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetSystem".to_string(), value: target_system.into() });
        args.push(MethodParameter { name: "DataItems".to_string(), value: data_items.into() });

        let result = self.invoke_method_with_job("AddKvpItems", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `data_items` -  (String[])
    /// * `target_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_kvp_items(&self, target_system: CIM_ComputerSystem, data_items: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetSystem".to_string(), value: target_system.into() });
        args.push(MethodParameter { name: "DataItems".to_string(), value: data_items.into() });

        let result = self.invoke_method_with_job("ModifyKvpItems", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `data_items` -  (String[])
    /// * `target_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_kvp_items(&self, target_system: CIM_ComputerSystem, data_items: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetSystem".to_string(), value: target_system.into() });
        args.push(MethodParameter { name: "DataItems".to_string(), value: data_items.into() });

        let result = self.invoke_method_with_job("RemoveKvpItems", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `errors` -  (String[])

    /// * `error_message` -  (String)
    /// * `return_value` -  (u32)
    pub fn format_error(&self, errors: &Vec<String>, error_message: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Errors".to_string(), value: errors.into() });

        let result = self.invoke_method("FormatError", &args)?;
        let error_message = result.get_value("ErrorMessage")?;
        Ok(result.return_value)

    }


/// 

    /// * `setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_disk_merge_settings(&self, setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SettingData".to_string(), value: setting_data.into() });

        let result = self.invoke_method_with_job("ModifyDiskMergeSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `number_of_wwpns` -  (u32)

    /// * `generated_wwpn` -  (String[])
    /// * `return_value` -  (u32)
    pub fn generate_wwpn(&self, number_of_wwpns: u32, generated_wwpn: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NumberOfWwpns".to_string(), value: number_of_wwpns.into() });

        let result = self.invoke_method("GenerateWwpn", &args)?;
        let generated_wwpn = result.get_value("GeneratedWwpn")?;
        Ok(result.return_value)

    }


/// 

    /// * `fc_port_settings` - Array of strings each containing an embedded instance of class Msvm_SyntheticFcPortSettingData that describes settings for synthetic fibre channel ports for virtual machines.All instances must have a valid InstanceID in order to identify the feature settings to be modified. (String[])
    /// * `secret_encoding` -  (VirtualSystemManagementService_SecretEncoding)
    /// * `shared_secret` -  (u8[])

    /// * `return_value` -  (u32)
    pub fn add_fibre_channel_chap(&self, fc_port_settings: &Vec<String>, secret_encoding: VirtualSystemManagementService_SecretEncoding, shared_secret: &Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FcPortSettings".to_string(), value: fc_port_settings.into() });
        args.push(MethodParameter { name: "SecretEncoding".to_string(), value: secret_encoding.into() });
        args.push(MethodParameter { name: "SharedSecret".to_string(), value: shared_secret.into() });
        self.invoke_method("AddFibreChannelChap", &args)

    }


/// 

    /// * `fc_port_settings` - Array of strings each containing an embedded instance of class Msvm_SyntheticFcPortSettingData that describes settings for synthetic fibre channel ports for virtual machines.All instances must have a valid InstanceID in order to identify the feature settings to be modified. (String[])

    /// * `return_value` -  (u32)
    pub fn remove_fibre_channel_chap(&self, fc_port_settings: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FcPortSettings".to_string(), value: fc_port_settings.into() });
        self.invoke_method("RemoveFibreChannelChap", &args)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `network_configuration` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_guest_network_adapter_configuration(&self, computer_system: CIM_ComputerSystem, network_configuration: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "NetworkConfiguration".to_string(), value: network_configuration.into() });

        let result = self.invoke_method_with_job("SetGuestNetworkAdapterConfiguration", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `vssd` -  (CIM_VirtualSystemSettingData)

    /// * `return_value` -  (u32)
    /// * `size` -  (u64)
    pub fn get_size_of_system_files(&self, vssd: CIM_VirtualSystemSettingData, size: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Vssd".to_string(), value: vssd.into() });

        let result = self.invoke_method("GetSizeOfSystemFiles", &args)?;
        let size = result.get_value("Size")?;
        Ok(result.return_value)

    }


/// 

    /// * `current_wwpn` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_current_wwpn_from_generator(&self, current_wwpn: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetCurrentWwpnFromGenerator", &[])?;
        let current_wwpn = result.get_value("CurrentWwpn")?;
        Ok(result.return_value)

    }


/// 

    /// * `isolation_id` -  (u32)
    /// * `is_sender` -  (bool)
    /// * `receiver_ip` -  (String)
    /// * `receiver_mac` -  (String)
    /// * `sender_ip` -  (String)
    /// * `sequence_number` -  (u32)
    /// * `target_network_adapter` - Reference to the Ethernet connection. (Msvm_EthernetPortAllocationSettingData)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    /// * `round_trip_time` - The round trip time for the Ping request. (u32)
    pub fn test_network_connection(&self, target_network_adapter: Msvm_EthernetPortAllocationSettingData, is_sender: bool, sender_ip: &String, receiver_ip: &String, receiver_mac: &String, isolation_id: u32, sequence_number: u32, round_trip_time: &mut u32, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetNetworkAdapter".to_string(), value: target_network_adapter.into() });
        args.push(MethodParameter { name: "IsSender".to_string(), value: is_sender.into() });
        args.push(MethodParameter { name: "SenderIP".to_string(), value: sender_ip.into() });
        args.push(MethodParameter { name: "ReceiverIP".to_string(), value: receiver_ip.into() });
        args.push(MethodParameter { name: "ReceiverMac".to_string(), value: receiver_mac.into() });
        args.push(MethodParameter { name: "IsolationId".to_string(), value: isolation_id.into() });
        args.push(MethodParameter { name: "SequenceNumber".to_string(), value: sequence_number.into() });

        let result = self.invoke_method_with_job("TestNetworkConnection", &args)?;
        let job = result.get_value("Job")?;
        let round_trip_time = result.get_value("RoundTripTime")?;
        Ok(result.return_value)

    }


/// 

    /// * `diagnostic_settings` - An embedded instance of class Msvm_NetworkConnectionDiagnosticSettingData that describes the settings used to diagnose the connectivity. (String)
    /// * `target_network_adapter` - Reference to the Ethernet connection. (Msvm_EthernetPortAllocationSettingData)

    /// * `diagnostic_information` - If successful, this object contains the output of the ping request. This is an embedded instance of Msvm_NetworkConnectionDiagnosticInformation. (String)
    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn diagnose_network_connection(&self, target_network_adapter: Msvm_EthernetPortAllocationSettingData, diagnostic_settings: &String, diagnostic_information: &mut String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetNetworkAdapter".to_string(), value: target_network_adapter.into() });
        args.push(MethodParameter { name: "DiagnosticSettings".to_string(), value: diagnostic_settings.into() });

        let result = self.invoke_method_with_job("DiagnoseNetworkConnection", &args)?;
        let diagnostic_information = result.get_value("DiagnosticInformation")?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `imc_data` -  (u8[])
    /// * `target_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_initial_machine_configuration_data(&self, target_system: CIM_ComputerSystem, imc_data: &Vec<u8>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetSystem".to_string(), value: target_system.into() });
        args.push(MethodParameter { name: "ImcData".to_string(), value: imc_data.into() });

        let result = self.invoke_method_with_job("SetInitialMachineConfigurationData", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_configuration` - Reference to the affected virtual system configuration. (Msvm_VirtualSystemSettingData)
    /// * `component_settings` - Array of strings each containing one embedded instanceof class Msvm_SystemComponentSettingData that describes the virtual aspects of a virtual resource to be added to the virtual system. (String[])

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instances of class Msvm_SystemComponentSettingData representing the added component settings are available via association CIM_ConcreteComponent from the instance of class Msvm_VirtualSystemSettingData representing the affected virtual system configuration. (CIM_ConcreteJob)
    /// * `resulting_component_settings` - Array of references to instances of class Msvm_SystemComponentSettingData representing properties of the resulting components. (Msvm_SystemComponentSettingData[])
    /// * `return_value` -  (u32)
    pub fn add_system_component_settings(&self, affected_configuration: Msvm_VirtualSystemSettingData, component_settings: &Vec<String>, resulting_component_settings: &mut Vec<Msvm_SystemComponentSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedConfiguration".to_string(), value: affected_configuration.into() });
        args.push(MethodParameter { name: "ComponentSettings".to_string(), value: component_settings.into() });

        let result = self.invoke_method_with_job("AddSystemComponentSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_component_settings = result.get_value("ResultingComponentSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `component_settings` - Array of strings each containing an embedded instance of class Msvm_SystemComponentSettingData that describes modifications to the virtual aspects of an existing system component. (String[])

    /// * `job` - If the operation is long running, then optionally a job be returned. In this case, the instances of class Msvm_SystemComponentSettingData representing the modified settings are available via association CIM_ConcreteComponent from the instance of class CIM_VirtualSystemSettingData representing the affected virtual system configuration. (CIM_ConcreteJob)
    /// * `resulting_component_settings` - Array of references to instances of class Msvm_SystemComponentSettingData representing virtual aspects of the modified components. (Msvm_SystemComponentSettingData[])
    /// * `return_value` -  (u32)
    pub fn modify_system_component_settings(&self, component_settings: &Vec<String>, resulting_component_settings: &mut Vec<Msvm_SystemComponentSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComponentSettings".to_string(), value: component_settings.into() });

        let result = self.invoke_method_with_job("ModifySystemComponentSettings", &args)?;
        let job = result.get_value("Job")?;
        let resulting_component_settings = result.get_value("ResultingComponentSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `component_settings` - Array of references to instances of class Msvm_SystemComponentSettingData where each instance represents the settings of a boot source within a virtual system configuration that are to be removed. (Msvm_SystemComponentSettingData[])

    /// * `job` - If the operation is long running, then optionally a job my be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_system_component_settings(&self, component_settings: &Vec<Msvm_SystemComponentSettingData>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComponentSettings".to_string(), value: component_settings.into() });

        let result = self.invoke_method_with_job("RemoveSystemComponentSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_VirtualSystemManagementService {
    /// Gets the related Msvm_VirtualSystemManagementServiceSettingData object(s)
    pub fn get_related__virtual_system_management_service_setting_data(&self) -> Result<Msvm_VirtualSystemManagementServiceSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemManagementServiceSettingData")
    }

    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

