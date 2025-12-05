// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemSnapshotService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemSnapshotService {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemSnapshotService,
}

impl Msvm_VirtualSystemSnapshotService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemSnapshotService::new(),
        }
    }


/// 

    /// * `snapshot_setting_data` -  (CIM_VirtualSystemSettingData)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn destroy_snapshot_tree(&self, snapshot_setting_data: CIM_VirtualSystemSettingData, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SnapshotSettingData".to_string(), value: snapshot_setting_data.into() });

        let result = self.invoke_method_with_job("DestroySnapshotTree", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `snapshot_setting_data` -  (CIM_VirtualSystemSettingData)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn clear_snapshot_state(&self, snapshot_setting_data: CIM_VirtualSystemSettingData, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SnapshotSettingData".to_string(), value: snapshot_setting_data.into() });

        let result = self.invoke_method_with_job("ClearSnapshotState", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Convert an existing virtual system snapshot to a reference point. The snapshot gets deleted as a side effect. Only recovery snapshots can be converted to reference points.

    /// * `affected_snapshot` - Reference to the affected virtual system snapshot. (CIM_VirtualSystemSettingData)
    /// * `reference_point_settings` - Parameter settings. (String)
    /// * `resulting_reference_point` - Resulting virtual system reference point (Msvm_VirtualSystemReferencePoint)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `resulting_reference_point` - Resulting virtual system reference point (Msvm_VirtualSystemReferencePoint)
    /// * `return_value` -  (u32)
    pub fn convert_to_reference_point(&self, affected_snapshot: CIM_VirtualSystemSettingData, reference_point_settings: &String, resulting_reference_point: &mut Msvm_VirtualSystemReferencePoint, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSnapshot".to_string(), value: affected_snapshot.into() });
        args.push(MethodParameter { name: "ReferencePointSettings".to_string(), value: reference_point_settings.into() });

        let result = self.invoke_method_with_job("ConvertToReferencePoint", &args)?;
        let job = result.get_value("Job")?;
        let resulting_reference_point = result.get_value("ResultingReferencePoint")?;
        Ok(result.return_value)

    }

}

impl Msvm_VirtualSystemSnapshotService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Vec<Msvm_ComputerSystem>, WmiError> {
        self.get_all_related("Msvm_ComputerSystem")
    }

}

