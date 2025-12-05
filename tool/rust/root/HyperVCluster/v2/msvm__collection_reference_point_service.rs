// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionReferencePointService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionReferencePointService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_CollectionReferencePointService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `collection` -  (Msvm_VirtualSystemCollection)
    /// * `reference_point_settings` -  (String)
    /// * `reference_point_type` -  (CollectionReferencePointService_ReferencePointType)
    /// * `resulting_reference_point_collection` -  (CIM_Collection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `resulting_reference_point_collection` -  (CIM_Collection)
    /// * `return_value` -  (u32)
    pub fn create_reference_point(&self, collection: Msvm_VirtualSystemCollection, reference_point_settings: &String, reference_point_type: CollectionReferencePointService_ReferencePointType, resulting_reference_point_collection: &mut CIM_Collection, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "ReferencePointSettings".to_string(), value: reference_point_settings.into() });
        args.push(MethodParameter { name: "ReferencePointType".to_string(), value: reference_point_type.into() });

        let result = self.invoke_method_with_job("CreateReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        let resulting_reference_point_collection = result.get_value("ResultingReferencePointCollection")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_reference_point_collection` -  (CIM_Collection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn destroy_reference_point(&self, affected_reference_point_collection: CIM_Collection, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedReferencePointCollection".to_string(), value: affected_reference_point_collection.into() });

        let result = self.invoke_method_with_job("DestroyReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_reference_point_collection` -  (CIM_Collection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_associated_data(&self, affected_reference_point_collection: CIM_Collection, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedReferencePointCollection".to_string(), value: affected_reference_point_collection.into() });

        let result = self.invoke_method_with_job("RemoveAssociatedData", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `export_directory` -  (String)
    /// * `export_setting_data` -  (String)
    /// * `reference_point_collection` -  (CIM_Collection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn export_reference_point(&self, reference_point_collection: CIM_Collection, export_directory: &String, export_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReferencePointCollection".to_string(), value: reference_point_collection.into() });
        args.push(MethodParameter { name: "ExportDirectory".to_string(), value: export_directory.into() });
        args.push(MethodParameter { name: "ExportSettingData".to_string(), value: export_setting_data.into() });

        let result = self.invoke_method_with_job("ExportReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_CollectionReferencePointService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

