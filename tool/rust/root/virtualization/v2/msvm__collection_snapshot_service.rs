// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionSnapshotService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionSnapshotService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_CollectionSnapshotService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `resulting_snapshot_collection` -  (CIM_Collection)
    /// * `snapshot_settings` -  (String)
    /// * `snapshot_type` -  (CollectionSnapshotService_SnapshotType)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `resulting_snapshot_collection` -  (CIM_Collection)
    /// * `return_value` -  (u32)
    pub fn create_snapshot(&self, collection: CIM_CollectionOfMSEs, snapshot_settings: &String, snapshot_type: CollectionSnapshotService_SnapshotType, resulting_snapshot_collection: &mut CIM_Collection, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "SnapshotSettings".to_string(), value: snapshot_settings.into() });
        args.push(MethodParameter { name: "SnapshotType".to_string(), value: snapshot_type.into() });

        let result = self.invoke_method_with_job("CreateSnapshot", &args)?;
        let job = result.get_value("Job")?;
        let resulting_snapshot_collection = result.get_value("ResultingSnapshotCollection")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_snapshot_collection` -  (CIM_Collection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn destroy_snapshot(&self, affected_snapshot_collection: CIM_Collection, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSnapshotCollection".to_string(), value: affected_snapshot_collection.into() });

        let result = self.invoke_method_with_job("DestroySnapshot", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `export_directory` -  (String)
    /// * `export_setting_data` -  (String)
    /// * `snapshot_collection` -  (CIM_Collection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn export_snapshot(&self, snapshot_collection: CIM_Collection, export_directory: &String, export_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SnapshotCollection".to_string(), value: snapshot_collection.into() });
        args.push(MethodParameter { name: "ExportDirectory".to_string(), value: export_directory.into() });
        args.push(MethodParameter { name: "ExportSettingData".to_string(), value: export_setting_data.into() });

        let result = self.invoke_method_with_job("ExportSnapshot", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `snapshot_collection` -  (CIM_Collection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn apply_snapshot(&self, snapshot_collection: CIM_Collection, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SnapshotCollection".to_string(), value: snapshot_collection.into() });

        let result = self.invoke_method_with_job("ApplySnapshot", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `affected_snapshot_collection` -  (Msvm_SnapshotCollection)
    /// * `resulting_reference_point_collection` -  (Msvm_ReferencePointCollection)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `resulting_reference_point_collection` -  (Msvm_ReferencePointCollection)
    /// * `return_value` -  (u32)
    pub fn convert_to_reference_point(&self, affected_snapshot_collection: Msvm_SnapshotCollection, resulting_reference_point_collection: &mut Msvm_ReferencePointCollection, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSnapshotCollection".to_string(), value: affected_snapshot_collection.into() });

        let result = self.invoke_method_with_job("ConvertToReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        let resulting_reference_point_collection = result.get_value("ResultingReferencePointCollection")?;
        Ok(result.return_value)

    }

}

impl Msvm_CollectionSnapshotService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

