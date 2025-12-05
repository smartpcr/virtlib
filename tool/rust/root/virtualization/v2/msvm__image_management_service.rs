// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ImageManagementService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ImageManagementService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_ImageManagementService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `virtual_disk_setting_data` - String containing an embedded instance of class Msvm_VirtualHardDiskSettingData that is used to define attributes of the virtual hard disk to be set. (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_virtual_hard_disk_setting_data(&self, virtual_disk_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualDiskSettingData".to_string(), value: virtual_disk_setting_data.into() });

        let result = self.invoke_method_with_job("SetVirtualHardDiskSettingData", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `virtual_disk_setting_data` - String containing an embedded instance of class Msvm_VirtualHardDiskSettingData that is used to define attributes of the virtual hard disk to be created. (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn create_virtual_hard_disk(&self, virtual_disk_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualDiskSettingData".to_string(), value: virtual_disk_setting_data.into() });

        let result = self.invoke_method_with_job("CreateVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `child_path` -  (String)
    /// * `ignore_idmismatch` -  (bool)
    /// * `leaf_path` -  (String)
    /// * `parent_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_parent_virtual_hard_disk(&self, child_path: &String, parent_path: &String, leaf_path: &String, ignore_idmismatch: bool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ChildPath".to_string(), value: child_path.into() });
        args.push(MethodParameter { name: "ParentPath".to_string(), value: parent_path.into() });
        args.push(MethodParameter { name: "LeafPath".to_string(), value: leaf_path.into() });
        args.push(MethodParameter { name: "IgnoreIDMismatch".to_string(), value: ignore_idmismatch.into() });

        let result = self.invoke_method_with_job("SetParentVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn create_virtual_floppy_disk(&self, path: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });

        let result = self.invoke_method_with_job("CreateVirtualFloppyDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `destination_path` -  (String)
    /// * `source_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn merge_virtual_hard_disk(&self, source_path: &String, destination_path: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourcePath".to_string(), value: source_path.into() });
        args.push(MethodParameter { name: "DestinationPath".to_string(), value: destination_path.into() });

        let result = self.invoke_method_with_job("MergeVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `mode` -  (u16)
    /// * `path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn compact_virtual_hard_disk(&self, path: &String, mode: u16, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Mode".to_string(), value: mode.into() });

        let result = self.invoke_method_with_job("CompactVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `max_internal_size` -  (u64)
    /// * `path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn resize_virtual_hard_disk(&self, path: &String, max_internal_size: u64, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "MaxInternalSize".to_string(), value: max_internal_size.into() });

        let result = self.invoke_method_with_job("ResizeVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `source_path` -  (String)
    /// * `virtual_disk_setting_data` - String containing an embedded instance of class Msvm_VirtualHardDiskSettingData that is used to define attributes of the virtual hard disk to be created. (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn convert_virtual_hard_disk(&self, source_path: &String, virtual_disk_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourcePath".to_string(), value: source_path.into() });
        args.push(MethodParameter { name: "VirtualDiskSettingData".to_string(), value: virtual_disk_setting_data.into() });

        let result = self.invoke_method_with_job("ConvertVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    /// * `setting_data` -  (String)
    pub fn get_virtual_hard_disk_setting_data(&self, path: &String, setting_data: &mut String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });

        let result = self.invoke_method_with_job("GetVirtualHardDiskSettingData", &args)?;
        let job = result.get_value("Job")?;
        let setting_data = result.get_value("SettingData")?;
        Ok(result.return_value)

    }


/// 

    /// * `path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    /// * `state` -  (String)
    pub fn get_virtual_hard_disk_state(&self, path: &String, state: &mut String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });

        let result = self.invoke_method_with_job("GetVirtualHardDiskState", &args)?;
        let job = result.get_value("Job")?;
        let state = result.get_value("State")?;
        Ok(result.return_value)

    }


/// 

    /// * `assign_drive_letter` -  (bool)
    /// * `path` -  (String)
    /// * `read_only` -  (bool)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn attach_virtual_hard_disk(&self, path: &String, assign_drive_letter: bool, read_only: bool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "AssignDriveLetter".to_string(), value: assign_drive_letter.into() });
        args.push(MethodParameter { name: "ReadOnly".to_string(), value: read_only.into() });

        let result = self.invoke_method_with_job("AttachVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn validate_virtual_hard_disk(&self, path: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });

        let result = self.invoke_method_with_job("ValidateVirtualHardDisk", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn validate_persistent_reservation_support(&self, path: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });

        let result = self.invoke_method_with_job("ValidatePersistentReservationSupport", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `additional_information` -  (u32[])
    /// * `vhdset_path` -  (String)

    /// * `information` -  (String)
    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn get_vhdset_information(&self, vhdset_path: &String, additional_information: &Vec<u32>, information: &mut String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VHDSetPath".to_string(), value: vhdset_path.into() });
        args.push(MethodParameter { name: "AdditionalInformation".to_string(), value: additional_information.into() });

        let result = self.invoke_method_with_job("GetVHDSetInformation", &args)?;
        let information = result.get_value("Information")?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `additional_information` -  (u32[])
    /// * `snapshot_ids` -  (String[])
    /// * `vhdset_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    /// * `snapshot_information` -  (String[])
    pub fn get_vhdsnapshot_information(&self, vhdset_path: &String, snapshot_ids: &Vec<String>, additional_information: &Vec<u32>, snapshot_information: &mut Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VHDSetPath".to_string(), value: vhdset_path.into() });
        args.push(MethodParameter { name: "SnapshotIds".to_string(), value: snapshot_ids.into() });
        args.push(MethodParameter { name: "AdditionalInformation".to_string(), value: additional_information.into() });

        let result = self.invoke_method_with_job("GetVHDSnapshotInformation", &args)?;
        let job = result.get_value("Job")?;
        let snapshot_information = result.get_value("SnapshotInformation")?;
        Ok(result.return_value)

    }


/// 

    /// * `information` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_vhdsnapshot_information(&self, information: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Information".to_string(), value: information.into() });

        let result = self.invoke_method_with_job("SetVHDSnapshotInformation", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `persist_reference_snapshot` -  (bool)
    /// * `snapshot_id` -  (String)
    /// * `vhdset_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn delete_vhdsnapshot(&self, vhdset_path: &String, snapshot_id: &String, persist_reference_snapshot: bool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VHDSetPath".to_string(), value: vhdset_path.into() });
        args.push(MethodParameter { name: "SnapshotId".to_string(), value: snapshot_id.into() });
        args.push(MethodParameter { name: "PersistReferenceSnapshot".to_string(), value: persist_reference_snapshot.into() });

        let result = self.invoke_method_with_job("DeleteVHDSnapshot", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `virtual_hard_disk_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn convert_virtual_hard_disk_to_vhdset(&self, virtual_hard_disk_path: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualHardDiskPath".to_string(), value: virtual_hard_disk_path.into() });

        let result = self.invoke_method_with_job("ConvertVirtualHardDiskToVHDSet", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `vhdset_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn optimize_vhdset(&self, vhdset_path: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VHDSetPath".to_string(), value: vhdset_path.into() });

        let result = self.invoke_method_with_job("OptimizeVHDSet", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `byte_length` -  (u64)
    /// * `byte_offset` -  (u64)
    /// * `limit_id` -  (String)
    /// * `path` -  (String)
    /// * `target_snapshot_id` -  (String)

    /// * `changed_byte_lengths` -  (u64[])
    /// * `changed_byte_offsets` -  (u64[])
    /// * `job` -  (CIM_ConcreteJob)
    /// * `processed_byte_length` -  (u64)
    /// * `return_value` -  (u32)
    pub fn get_virtual_disk_changes(&self, path: &String, limit_id: &String, target_snapshot_id: &String, byte_offset: u64, byte_length: u64, processed_byte_length: &mut u64, changed_byte_offsets: &mut Vec<u64>, changed_byte_lengths: &mut Vec<u64>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "LimitId".to_string(), value: limit_id.into() });
        args.push(MethodParameter { name: "TargetSnapshotId".to_string(), value: target_snapshot_id.into() });
        args.push(MethodParameter { name: "ByteOffset".to_string(), value: byte_offset.into() });
        args.push(MethodParameter { name: "ByteLength".to_string(), value: byte_length.into() });

        let result = self.invoke_method_with_job("GetVirtualDiskChanges", &args)?;
        let changed_byte_lengths = result.get_value("ChangedByteLengths")?;
        let changed_byte_offsets = result.get_value("ChangedByteOffsets")?;
        let job = result.get_value("Job")?;
        let processed_byte_length = result.get_value("ProcessedByteLength")?;
        Ok(result.return_value)

    }


/// 

    /// * `criterion_type` -  (u16)
    /// * `selection_criterion` -  (String)

    /// * `image` -  (Msvm_MountedStorageImage)
    /// * `return_value` -  (u32)
    pub fn find_mounted_storage_image_instance(&self, selection_criterion: &String, criterion_type: u16, image: &mut Msvm_MountedStorageImage) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SelectionCriterion".to_string(), value: selection_criterion.into() });
        args.push(MethodParameter { name: "CriterionType".to_string(), value: criterion_type.into() });

        let result = self.invoke_method("FindMountedStorageImageInstance", &args)?;
        let image = result.get_value("Image")?;
        Ok(result.return_value)

    }

}

impl Msvm_ImageManagementService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

