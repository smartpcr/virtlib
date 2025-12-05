// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualSystemSnapshotService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualSystemSnapshotService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl CIM_VirtualSystemSnapshotService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// Creates a snapshot of a virtual system.

    /// * `affected_system` - Reference to the affected virtual system. (CIM_ComputerSystem)
    /// * `resulting_snapshot` - Resulting virtual system snapshot (CIM_VirtualSystemSettingData)
    /// * `snapshot_settings` - Parameter settings. (String)
    /// * `snapshot_type` - Requested snapshot type: Full Snapshot: Complete snapshot of the virtual system. Disk Snapshot: Snapshot of virtual system disks. (VirtualSystemSnapshotService_SnapshotType)

    /// * `job` - If the operation is long running, then optionally a job may be returned. In this case, the instance of the CIM_VirtualSystemSettingData class representing the new virtual system snapshot is presented via the CIM_AffectedJobElement association with the value of the AffectedElement property referring to the new instance of the CIM_VirtualSystemSettingData class representing the virtual system snapshot and and the value of the ElementEffects set to 5 (Create). (CIM_ConcreteJob)
    /// * `resulting_snapshot` - Resulting virtual system snapshot (CIM_VirtualSystemSettingData)
    /// * `return_value` -  (u32)
    pub fn create_snapshot(&self, affected_system: CIM_ComputerSystem, snapshot_settings: &String, snapshot_type: VirtualSystemSnapshotService_SnapshotType, resulting_snapshot: &mut CIM_VirtualSystemSettingData, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSystem".to_string(), value: affected_system.into() });
        args.push(MethodParameter { name: "SnapshotSettings".to_string(), value: snapshot_settings.into() });
        args.push(MethodParameter { name: "SnapshotType".to_string(), value: snapshot_type.into() });

        let result = self.invoke_method_with_job("CreateSnapshot", &args)?;
        let job = result.get_value("Job")?;
        let resulting_snapshot = result.get_value("ResultingSnapshot")?;
        Ok(result.return_value)

    }


/// Destroy an existing virtual system snapshot.This method may as a side effect destroy other snapshots that are dependent on the affected snapshot.

    /// * `affected_snapshot` - Reference to the affected virtual system snapshot. (CIM_VirtualSystemSettingData)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn destroy_snapshot(&self, affected_snapshot: CIM_VirtualSystemSettingData, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AffectedSnapshot".to_string(), value: affected_snapshot.into() });

        let result = self.invoke_method_with_job("DestroySnapshot", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Apply a virtual system snapshot to the virtual system that it was created from.

    /// * `snapshot` - Reference to the virtual system snapshot. (CIM_VirtualSystemSettingData)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn apply_snapshot(&self, snapshot: CIM_VirtualSystemSettingData, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Snapshot".to_string(), value: snapshot.into() });

        let result = self.invoke_method_with_job("ApplySnapshot", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

