// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemReferencePointService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemReferencePointService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_VirtualSystemReferencePointService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// Creates a reference point of a virtual system.

    /// * `affected_system` - Reference to the affected virtual system. (Msvm_ComputerSystem)
    /// * `reference_point_settings` - Parameter settings. (String)
    /// * `reference_point_type` - Requested reference point type: Log based: Based on Hyper-V replica log tracking. RCT based: Based on Resilient Change Tracking of virtual disks. (VirtualSystemReferencePointService_ReferencePointType)
    /// * `resulting_reference_point` - Resulting virtual system reference point (Msvm_VirtualSystemReferencePoint)

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instance of the Msvm_VirtualSystemReferencePoint class representing the new virtual system reference point is presented via the CIM_AffectedJobElement association with the value of the AffectedElement property referring to the new instance of the Msvm_VirtualSystemReferencePoint class representing the virtual system reference point and the value of the ElementEffects set to 5 (Create). (CIM_ConcreteJob)
    /// * `resulting_reference_point` - Resulting virtual system reference point (Msvm_VirtualSystemReferencePoint)
    /// * `return_value` -  (u32)
    pub fn create_reference_point(&self, affected_system: Msvm_ComputerSystem, reference_point_settings: &String, reference_point_type: VirtualSystemReferencePointService_ReferencePointType, resulting_reference_point: &mut Msvm_VirtualSystemReferencePoint, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSystem".to_string(), value: affected_system.into() });
        args.push(MethodParameter { name: "ReferencePointSettings".to_string(), value: reference_point_settings.into() });
        args.push(MethodParameter { name: "ReferencePointType".to_string(), value: reference_point_type.into() });

        let result = self.invoke_method_with_job("CreateReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        let resulting_reference_point = result.get_value("ResultingReferencePoint")?;
        Ok(result.return_value)

    }


/// 

    /// * `export_directory` -  (String)
    /// * `export_setting_data` -  (String)
    /// * `reference_point` -  (Msvm_VirtualSystemReferencePoint)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn export_reference_point(&self, reference_point: Msvm_VirtualSystemReferencePoint, export_directory: &String, export_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReferencePoint".to_string(), value: reference_point.into() });
        args.push(MethodParameter { name: "ExportDirectory".to_string(), value: export_directory.into() });
        args.push(MethodParameter { name: "ExportSettingData".to_string(), value: export_setting_data.into() });

        let result = self.invoke_method_with_job("ExportReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_reference_point` -  (Msvm_VirtualSystemReferencePoint)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn destroy_reference_point(&self, affected_reference_point: Msvm_VirtualSystemReferencePoint, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedReferencePoint".to_string(), value: affected_reference_point.into() });

        let result = self.invoke_method_with_job("DestroyReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_reference_point` -  (Msvm_VirtualSystemReferencePoint)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_associated_data(&self, affected_reference_point: Msvm_VirtualSystemReferencePoint, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedReferencePoint".to_string(), value: affected_reference_point.into() });

        let result = self.invoke_method_with_job("RemoveAssociatedData", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_system` - Reference to the affected virtual system. (Msvm_ComputerSystem)
    /// * `config_file_path` -  (String)
    /// * `runtime_state_file_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn import_reference_point_metadata(&self, affected_system: Msvm_ComputerSystem, config_file_path: &String, runtime_state_file_path: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSystem".to_string(), value: affected_system.into() });
        args.push(MethodParameter { name: "ConfigFilePath".to_string(), value: config_file_path.into() });
        args.push(MethodParameter { name: "RuntimeStateFilePath".to_string(), value: runtime_state_file_path.into() });

        let result = self.invoke_method_with_job("ImportReferencePointMetadata", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_VirtualSystemReferencePointService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Vec<Msvm_ComputerSystem>, WmiError> {
        self.get_all_related("Msvm_ComputerSystem")
    }

}

