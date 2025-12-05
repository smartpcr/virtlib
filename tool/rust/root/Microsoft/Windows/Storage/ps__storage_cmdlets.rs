// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_StorageCmdlets struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_StorageCmdlets {
}

impl PS_StorageCmdlets {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `guid` -  (String)
    /// * `input_object` -  (MSFT_Disk[])
    /// * `is_offline` -  (bool)
    /// * `is_read_only` -  (bool)
    /// * `number` -  (u32)
    /// * `partition_style` -  (u16)
    /// * `path` -  (String)
    /// * `signature` -  (u32)
    /// * `unique_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_disk(&self, input_object: &Vec<MSFT_Disk>, unique_id: &String, path: &String, number: u32, partition_style: u16, is_read_only: bool, is_offline: bool, signature: u32, guid: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Number".to_string(), value: number.into() });
        args.push(MethodParameter { name: "PartitionStyle".to_string(), value: partition_style.into() });
        args.push(MethodParameter { name: "IsReadOnly".to_string(), value: is_read_only.into() });
        args.push(MethodParameter { name: "IsOffline".to_string(), value: is_offline.into() });
        args.push(MethodParameter { name: "Signature".to_string(), value: signature.into() });
        args.push(MethodParameter { name: "Guid".to_string(), value: guid.into() });
        self.invoke_method("SetDisk", &args)

    }


/// 

    /// * `dedup_mode` -  (u32)
    /// * `drive_letter` -  (char)
    /// * `file_system_label` -  (String)
    /// * `input_object` -  (MSFT_Volume[])
    /// * `new_file_system_label` -  (String)
    /// * `path` -  (String)
    /// * `unique_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_volume(&self, input_object: &Vec<MSFT_Volume>, unique_id: &String, path: &String, file_system_label: &String, drive_letter: char, new_file_system_label: &String, dedup_mode: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "FileSystemLabel".to_string(), value: file_system_label.into() });
        args.push(MethodParameter { name: "DriveLetter".to_string(), value: drive_letter.into() });
        args.push(MethodParameter { name: "NewFileSystemLabel".to_string(), value: new_file_system_label.into() });
        args.push(MethodParameter { name: "DedupMode".to_string(), value: dedup_mode.into() });
        self.invoke_method("SetVolume", &args)

    }


/// 

    /// * `disk_id` -  (String)
    /// * `disk_number` -  (u32)
    /// * `drive_letter` -  (char)
    /// * `gpt_type` -  (String)
    /// * `input_object` -  (MSFT_Partition[])
    /// * `is_active` -  (bool)
    /// * `is_dax` -  (bool)
    /// * `is_hidden` -  (bool)
    /// * `is_offline` -  (bool)
    /// * `is_read_only` -  (bool)
    /// * `is_shadow_copy` -  (bool)
    /// * `mbr_type` -  (u16)
    /// * `new_drive_letter` -  (char)
    /// * `no_default_drive_letter` -  (bool)
    /// * `offset` -  (u64)
    /// * `partition_number` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_partition(&self, input_object: &Vec<MSFT_Partition>, disk_id: &String, offset: u64, disk_number: u32, partition_number: u32, drive_letter: char, new_drive_letter: char, is_offline: bool, is_read_only: bool, no_default_drive_letter: bool, is_active: bool, is_hidden: bool, is_shadow_copy: bool, is_dax: bool, mbr_type: u16, gpt_type: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "DiskId".to_string(), value: disk_id.into() });
        args.push(MethodParameter { name: "Offset".to_string(), value: offset.into() });
        args.push(MethodParameter { name: "DiskNumber".to_string(), value: disk_number.into() });
        args.push(MethodParameter { name: "PartitionNumber".to_string(), value: partition_number.into() });
        args.push(MethodParameter { name: "DriveLetter".to_string(), value: drive_letter.into() });
        args.push(MethodParameter { name: "NewDriveLetter".to_string(), value: new_drive_letter.into() });
        args.push(MethodParameter { name: "IsOffline".to_string(), value: is_offline.into() });
        args.push(MethodParameter { name: "IsReadOnly".to_string(), value: is_read_only.into() });
        args.push(MethodParameter { name: "NoDefaultDriveLetter".to_string(), value: no_default_drive_letter.into() });
        args.push(MethodParameter { name: "IsActive".to_string(), value: is_active.into() });
        args.push(MethodParameter { name: "IsHidden".to_string(), value: is_hidden.into() });
        args.push(MethodParameter { name: "IsShadowCopy".to_string(), value: is_shadow_copy.into() });
        args.push(MethodParameter { name: "IsDAX".to_string(), value: is_dax.into() });
        args.push(MethodParameter { name: "MbrType".to_string(), value: mbr_type.into() });
        args.push(MethodParameter { name: "GptType".to_string(), value: gpt_type.into() });
        self.invoke_method("SetPartition", &args)

    }


/// 

    /// * `description` -  (String)
    /// * `friendly_name` -  (String)
    /// * `input_object` -  (MSFT_PhysicalDisk[])
    /// * `is_hidden` -  (bool)
    /// * `media_type` -  (u16)
    /// * `new_friendly_name` -  (String)
    /// * `storage_enclosure_id` -  (String)
    /// * `storage_scale_unit_id` -  (String)
    /// * `unique_id` -  (String)
    /// * `usage` -  (u16)

    /// * `return_value` -  (u32)
    pub fn set_physical_disk(&self, input_object: &Vec<MSFT_PhysicalDisk>, unique_id: &String, friendly_name: &String, new_friendly_name: &String, description: &String, usage: u16, media_type: u16, storage_enclosure_id: &String, storage_scale_unit_id: &String, is_hidden: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "NewFriendlyName".to_string(), value: new_friendly_name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "StorageEnclosureId".to_string(), value: storage_enclosure_id.into() });
        args.push(MethodParameter { name: "StorageScaleUnitId".to_string(), value: storage_scale_unit_id.into() });
        args.push(MethodParameter { name: "IsHidden".to_string(), value: is_hidden.into() });
        self.invoke_method("SetPhysicalDisk", &args)

    }


/// 

    /// * `auto_write_cache_size` -  (bool)
    /// * `clear_on_deallocate` -  (bool)
    /// * `enclosure_aware_default` -  (bool)
    /// * `fault_domain_awareness_default` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `input_object` -  (MSFT_StoragePool[])
    /// * `is_power_protected` -  (bool)
    /// * `is_read_only` -  (bool)
    /// * `media_type_default` -  (u16)
    /// * `name` -  (String)
    /// * `new_friendly_name` -  (String)
    /// * `other_usage_description` -  (String)
    /// * `provisioning_type_default` -  (u16)
    /// * `repair_policy` -  (u16)
    /// * `resiliency_setting_name_default` -  (String)
    /// * `retire_missing_physical_disks` -  (u16)
    /// * `thin_provisioning_alert_thresholds` -  (u16[])
    /// * `unique_id` -  (String)
    /// * `usage` -  (u16)
    /// * `write_cache_size_default` -  (u64)

    /// * `return_value` -  (u32)
    pub fn set_storage_pool(&self, input_object: &Vec<MSFT_StoragePool>, unique_id: &String, name: &String, friendly_name: &String, new_friendly_name: &String, usage: u16, other_usage_description: &String, provisioning_type_default: u16, media_type_default: u16, resiliency_setting_name_default: &String, enclosure_aware_default: bool, fault_domain_awareness_default: u16, write_cache_size_default: u64, auto_write_cache_size: bool, is_read_only: bool, clear_on_deallocate: bool, is_power_protected: bool, repair_policy: u16, retire_missing_physical_disks: u16, thin_provisioning_alert_thresholds: &Vec<u16>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "NewFriendlyName".to_string(), value: new_friendly_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "ProvisioningTypeDefault".to_string(), value: provisioning_type_default.into() });
        args.push(MethodParameter { name: "MediaTypeDefault".to_string(), value: media_type_default.into() });
        args.push(MethodParameter { name: "ResiliencySettingNameDefault".to_string(), value: resiliency_setting_name_default.into() });
        args.push(MethodParameter { name: "EnclosureAwareDefault".to_string(), value: enclosure_aware_default.into() });
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });
        args.push(MethodParameter { name: "WriteCacheSizeDefault".to_string(), value: write_cache_size_default.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });
        args.push(MethodParameter { name: "IsReadOnly".to_string(), value: is_read_only.into() });
        args.push(MethodParameter { name: "ClearOnDeallocate".to_string(), value: clear_on_deallocate.into() });
        args.push(MethodParameter { name: "IsPowerProtected".to_string(), value: is_power_protected.into() });
        args.push(MethodParameter { name: "RepairPolicy".to_string(), value: repair_policy.into() });
        args.push(MethodParameter { name: "RetireMissingPhysicalDisks".to_string(), value: retire_missing_physical_disks.into() });
        args.push(MethodParameter { name: "ThinProvisioningAlertThresholds".to_string(), value: thin_provisioning_alert_thresholds.into() });
        self.invoke_method("SetStoragePool", &args)

    }


/// 

    /// * `access` -  (u16)
    /// * `allocation_unit_size` -  (u64)
    /// * `column_isolation` -  (u16)
    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `input_object` -  (MSFT_VirtualDisk[])
    /// * `interleave` -  (u64)
    /// * `is_manual_attach` -  (bool)
    /// * `max_io_bandwidth` -  (u64)
    /// * `max_iops` -  (u64)
    /// * `media_type` -  (u16)
    /// * `name` -  (String)
    /// * `new_friendly_name` -  (String)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `other_usage_description` -  (String)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `resiliency_setting_name` -  (String)
    /// * `storage_node_name` -  (String)
    /// * `unique_id` -  (String)
    /// * `usage` -  (u16)

    /// * `return_value` -  (u32)
    pub fn set_virtual_disk(&self, input_object: &Vec<MSFT_VirtualDisk>, unique_id: &String, name: &String, friendly_name: &String, new_friendly_name: &String, usage: u16, other_usage_description: &String, is_manual_attach: bool, storage_node_name: &String, access: u16, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, fault_domain_awareness: u16, column_isolation: u16, resiliency_setting_name: &String, physical_disk_redundancy: u16, number_of_data_copies: u16, number_of_groups: u16, number_of_columns: u16, interleave: u64, max_iops: u64, max_io_bandwidth: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "NewFriendlyName".to_string(), value: new_friendly_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "IsManualAttach".to_string(), value: is_manual_attach.into() });
        args.push(MethodParameter { name: "StorageNodeName".to_string(), value: storage_node_name.into() });
        args.push(MethodParameter { name: "Access".to_string(), value: access.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "MaxIops".to_string(), value: max_iops.into() });
        args.push(MethodParameter { name: "MaxIoBandwidth".to_string(), value: max_io_bandwidth.into() });
        self.invoke_method("SetVirtualDisk", &args)

    }


/// 

    /// * `allocation_unit_size` -  (u64)
    /// * `column_isolation` -  (u16)
    /// * `description` -  (String)
    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `input_object` -  (MSFT_StorageTier[])
    /// * `interleave` -  (u64)
    /// * `media_type` -  (u16)
    /// * `new_friendly_name` -  (String)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `resiliency_setting_name` -  (String)
    /// * `unique_id` -  (String)
    /// * `usage` -  (u16)

    /// * `return_value` -  (u32)
    pub fn set_storage_tier(&self, input_object: &Vec<MSFT_StorageTier>, unique_id: &String, friendly_name: &String, new_friendly_name: &String, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, fault_domain_awareness: u16, column_isolation: u16, resiliency_setting_name: &String, usage: u16, physical_disk_redundancy: u16, number_of_data_copies: u16, number_of_groups: u16, number_of_columns: u16, interleave: u64, description: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "NewFriendlyName".to_string(), value: new_friendly_name.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        self.invoke_method("SetStorageTier", &args)

    }


/// 

    /// * `automatic_clustering_enabled` -  (bool)
    /// * `description` -  (String)
    /// * `fault_domain_awareness_default` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `input_object` -  (MSFT_StorageSubSystem[])
    /// * `name` -  (String)
    /// * `unique_id` -  (String)
    /// * `virtual_disk_repair_enabled` -  (bool)
    /// * `virtual_disk_repair_queue_depth` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_storage_sub_system(&self, input_object: &Vec<MSFT_StorageSubSystem>, unique_id: &String, name: &String, friendly_name: &String, description: &String, automatic_clustering_enabled: bool, virtual_disk_repair_enabled: bool, virtual_disk_repair_queue_depth: u32, fault_domain_awareness_default: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "AutomaticClusteringEnabled".to_string(), value: automatic_clustering_enabled.into() });
        args.push(MethodParameter { name: "VirtualDiskRepairEnabled".to_string(), value: virtual_disk_repair_enabled.into() });
        args.push(MethodParameter { name: "VirtualDiskRepairQueueDepth".to_string(), value: virtual_disk_repair_queue_depth.into() });
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });
        self.invoke_method("SetStorageSubSystem", &args)

    }


/// 

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `storage_pool` -  (MSFT_StoragePool)
    /// * `storage_pool_friendly_name` -  (String)
    /// * `storage_pool_name` -  (String)
    /// * `storage_pool_unique_id` -  (String)
    /// * `usage` -  (u16)
    /// * `virtual_disk` -  (MSFT_VirtualDisk)
    /// * `virtual_disk_friendly_name` -  (String)
    /// * `virtual_disk_name` -  (String)
    /// * `virtual_disk_unique_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_physical_disk(&self, storage_pool: MSFT_StoragePool, storage_pool_unique_id: &String, storage_pool_name: &String, storage_pool_friendly_name: &String, virtual_disk: MSFT_VirtualDisk, virtual_disk_unique_id: &String, virtual_disk_name: &String, virtual_disk_friendly_name: &String, physical_disks: &Vec<MSFT_PhysicalDisk>, usage: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StoragePool".to_string(), value: storage_pool.into() });
        args.push(MethodParameter { name: "StoragePoolUniqueId".to_string(), value: storage_pool_unique_id.into() });
        args.push(MethodParameter { name: "StoragePoolName".to_string(), value: storage_pool_name.into() });
        args.push(MethodParameter { name: "StoragePoolFriendlyName".to_string(), value: storage_pool_friendly_name.into() });
        args.push(MethodParameter { name: "VirtualDisk".to_string(), value: virtual_disk.into() });
        args.push(MethodParameter { name: "VirtualDiskUniqueId".to_string(), value: virtual_disk_unique_id.into() });
        args.push(MethodParameter { name: "VirtualDiskName".to_string(), value: virtual_disk_name.into() });
        args.push(MethodParameter { name: "VirtualDiskFriendlyName".to_string(), value: virtual_disk_friendly_name.into() });
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        self.invoke_method("AddPhysicalDisk", &args)

    }


/// 

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `storage_pool` -  (MSFT_StoragePool)
    /// * `storage_pool_friendly_name` -  (String)
    /// * `storage_pool_name` -  (String)
    /// * `storage_pool_unique_id` -  (String)
    /// * `virtual_disk` -  (MSFT_VirtualDisk)
    /// * `virtual_disk_friendly_name` -  (String)
    /// * `virtual_disk_name` -  (String)
    /// * `virtual_disk_unique_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_physical_disk(&self, storage_pool: MSFT_StoragePool, storage_pool_unique_id: &String, storage_pool_name: &String, storage_pool_friendly_name: &String, virtual_disk: MSFT_VirtualDisk, virtual_disk_unique_id: &String, virtual_disk_name: &String, virtual_disk_friendly_name: &String, physical_disks: &Vec<MSFT_PhysicalDisk>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StoragePool".to_string(), value: storage_pool.into() });
        args.push(MethodParameter { name: "StoragePoolUniqueId".to_string(), value: storage_pool_unique_id.into() });
        args.push(MethodParameter { name: "StoragePoolName".to_string(), value: storage_pool_name.into() });
        args.push(MethodParameter { name: "StoragePoolFriendlyName".to_string(), value: storage_pool_friendly_name.into() });
        args.push(MethodParameter { name: "VirtualDisk".to_string(), value: virtual_disk.into() });
        args.push(MethodParameter { name: "VirtualDiskUniqueId".to_string(), value: virtual_disk_unique_id.into() });
        args.push(MethodParameter { name: "VirtualDiskName".to_string(), value: virtual_disk_name.into() });
        args.push(MethodParameter { name: "VirtualDiskFriendlyName".to_string(), value: virtual_disk_friendly_name.into() });
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        self.invoke_method("RemovePhysicalDisk", &args)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn launch_provider_host(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("LaunchProviderHost", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)
    /// * `encrypt_data` -  (bool)
    /// * `input_object` -  (MSFT_FileShare[])
    /// * `name` -  (String)
    /// * `unique_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_file_share(&self, input_object: &Vec<MSFT_FileShare>, unique_id: &String, name: &String, description: &String, encrypt_data: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "UniqueId".to_string(), value: unique_id.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "EncryptData".to_string(), value: encrypt_data.into() });
        self.invoke_method("SetFileShare", &args)

    }


/// 

    /// * `access_path` -  (String)
    /// * `allocation_unit_size` -  (u32)
    /// * `disk` -  (MSFT_Disk)
    /// * `disk_number` -  (u32)
    /// * `disk_path` -  (String)
    /// * `disk_unique_id` -  (String)
    /// * `file_server` -  (MSFT_FileServer)
    /// * `file_system` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `number_of_columns` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `read_cache_size` -  (u64)
    /// * `resiliency_setting_name` -  (String)
    /// * `run_as_job` -  (bool)
    /// * `size` -  (u64)
    /// * `storage_pool` -  (MSFT_StoragePool)
    /// * `storage_pool_friendly_name` -  (String)
    /// * `storage_pool_name` -  (String)
    /// * `storage_pool_unique_id` -  (String)
    /// * `storage_tiers` -  (MSFT_StorageTier[])
    /// * `storage_tier_sizes` -  (u64[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_volume` -  (MSFT_Volume[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_volume(&self, storage_pool: MSFT_StoragePool, storage_pool_unique_id: &String, storage_pool_name: &String, storage_pool_friendly_name: &String, disk: MSFT_Disk, disk_number: u32, disk_path: &String, disk_unique_id: &String, friendly_name: &String, size: u64, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, provisioning_type: u16, resiliency_setting_name: &String, physical_disk_redundancy: u16, number_of_columns: u16, file_system: u16, access_path: &String, allocation_unit_size: u32, read_cache_size: u64, file_server: MSFT_FileServer, created_volume: &mut Vec<MSFT_Volume>, run_as_job: Option<bool>, created_storage_job: &mut Option<MSFT_StorageJob>, extended_status: &mut Option<MSFT_StorageExtendedStatus>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StoragePool".to_string(), value: storage_pool.into() });
        args.push(MethodParameter { name: "StoragePoolUniqueId".to_string(), value: storage_pool_unique_id.into() });
        args.push(MethodParameter { name: "StoragePoolName".to_string(), value: storage_pool_name.into() });
        args.push(MethodParameter { name: "StoragePoolFriendlyName".to_string(), value: storage_pool_friendly_name.into() });
        args.push(MethodParameter { name: "Disk".to_string(), value: disk.into() });
        args.push(MethodParameter { name: "DiskNumber".to_string(), value: disk_number.into() });
        args.push(MethodParameter { name: "DiskPath".to_string(), value: disk_path.into() });
        args.push(MethodParameter { name: "DiskUniqueId".to_string(), value: disk_unique_id.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "StorageTiers".to_string(), value: storage_tiers.into() });
        args.push(MethodParameter { name: "StorageTierSizes".to_string(), value: storage_tier_sizes.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });
        args.push(MethodParameter { name: "AccessPath".to_string(), value: access_path.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "ReadCacheSize".to_string(), value: read_cache_size.into() });
        args.push(MethodParameter { name: "FileServer".to_string(), value: file_server.into() });
        if let Some(val) = run_as_job {
            args.push(MethodParameter { name: "RunAsJob".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CreateVolume", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_volume = result.get_value("CreatedVolume")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `disk` -  (MSFT_Disk)
    /// * `physical_disk` -  (MSFT_PhysicalDisk)

    /// * `return_value` -  (u32)
    /// * `storage_reliability_counter` -  (MSFT_StorageReliabilityCounter)
    pub fn get_storage_reliability_counter(&self, physical_disk: MSFT_PhysicalDisk, disk: MSFT_Disk, storage_reliability_counter: &mut MSFT_StorageReliabilityCounter) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PhysicalDisk".to_string(), value: physical_disk.into() });
        args.push(MethodParameter { name: "Disk".to_string(), value: disk.into() });

        let result = self.invoke_method("GetStorageReliabilityCounter", &args)?;
        let storage_reliability_counter = result.get_value("StorageReliabilityCounter")?;
        Ok(result.return_value)

    }

}

