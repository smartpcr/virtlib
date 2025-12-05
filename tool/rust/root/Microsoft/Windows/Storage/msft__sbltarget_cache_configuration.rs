// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SBLTargetCacheConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SBLTargetCacheConfiguration {

/// 
    #[serde(rename = "CacheBehavior")]
    pub cache_behavior: Option<u64>,

/// 
    #[serde(rename = "CachePageSizeinKB")]
    pub cache_page_sizein_kb: Option<u32>,

/// 
    #[serde(rename = "CurrentCacheModeHDD")]
    pub current_cache_mode_hdd: Option<u32>,

/// 
    #[serde(rename = "CurrentCacheModeSSD")]
    pub current_cache_mode_ssd: Option<u32>,

/// 
    #[serde(rename = "CurrentState")]
    pub current_state: Option<u32>,

/// 
    #[serde(rename = "CurrentStateProgress")]
    pub current_state_progress: Option<u64>,

/// 
    #[serde(rename = "CurrentStateProgressMax")]
    pub current_state_progress_max: Option<u64>,

/// 
    #[serde(rename = "DesiredCacheModeHDD")]
    pub desired_cache_mode_hdd: Option<u32>,

/// 
    #[serde(rename = "DesiredCacheModeSSD")]
    pub desired_cache_mode_ssd: Option<u32>,

/// 
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<u32>,

/// 
    #[serde(rename = "FlashMetadataReserveBytes")]
    pub flash_metadata_reserve_bytes: Option<u64>,

/// 
    #[serde(rename = "FlashReservePercent")]
    pub flash_reserve_percent: Option<u32>,

/// 
    #[serde(rename = "Identifier")]
    pub identifier: Option<String>,

/// 
    #[serde(rename = "ProvisioningStage")]
    pub provisioning_stage: Option<u64>,

/// 
    #[serde(rename = "ProvisioningStageMax")]
    pub provisioning_stage_max: Option<u64>,

/// 
    #[serde(rename = "SpacesDirectEnabled")]
    pub spaces_direct_enabled: Option<bool>,
}

impl MSFT_SBLTargetCacheConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cache_behavior: None,
            cache_page_sizein_kb: None,
            current_cache_mode_hdd: None,
            current_cache_mode_ssd: None,
            current_state: None,
            current_state_progress: None,
            current_state_progress_max: None,
            desired_cache_mode_hdd: None,
            desired_cache_mode_ssd: None,
            desired_state: None,
            flash_metadata_reserve_bytes: None,
            flash_reserve_percent: None,
            identifier: None,
            provisioning_stage: None,
            provisioning_stage_max: None,
            spaces_direct_enabled: None,
        }
    }


    /// Sets the value of CacheBehavior
    pub fn set_cache_behavior(&mut self, value: u64) {
        self.cache_behavior = Some(value);
    }

    /// Gets the value of CacheBehavior
    pub fn get_cache_behavior(&self) -> Option<&u64> {
        self.cache_behavior.as_ref()
    }

    /// Sets the value of CachePageSizeinKB
    pub fn set_cache_page_sizein_kb(&mut self, value: u32) {
        self.cache_page_sizein_kb = Some(value);
    }

    /// Gets the value of CachePageSizeinKB
    pub fn get_cache_page_sizein_kb(&self) -> Option<&u32> {
        self.cache_page_sizein_kb.as_ref()
    }

    /// Sets the value of CurrentCacheModeHDD
    pub fn set_current_cache_mode_hdd(&mut self, value: u32) {
        self.current_cache_mode_hdd = Some(value);
    }

    /// Gets the value of CurrentCacheModeHDD
    pub fn get_current_cache_mode_hdd(&self) -> Option<&u32> {
        self.current_cache_mode_hdd.as_ref()
    }

    /// Sets the value of CurrentCacheModeSSD
    pub fn set_current_cache_mode_ssd(&mut self, value: u32) {
        self.current_cache_mode_ssd = Some(value);
    }

    /// Gets the value of CurrentCacheModeSSD
    pub fn get_current_cache_mode_ssd(&self) -> Option<&u32> {
        self.current_cache_mode_ssd.as_ref()
    }

    /// Sets the value of CurrentState
    pub fn set_current_state(&mut self, value: u32) {
        self.current_state = Some(value);
    }

    /// Gets the value of CurrentState
    pub fn get_current_state(&self) -> Option<&u32> {
        self.current_state.as_ref()
    }

    /// Sets the value of CurrentStateProgress
    pub fn set_current_state_progress(&mut self, value: u64) {
        self.current_state_progress = Some(value);
    }

    /// Gets the value of CurrentStateProgress
    pub fn get_current_state_progress(&self) -> Option<&u64> {
        self.current_state_progress.as_ref()
    }

    /// Sets the value of CurrentStateProgressMax
    pub fn set_current_state_progress_max(&mut self, value: u64) {
        self.current_state_progress_max = Some(value);
    }

    /// Gets the value of CurrentStateProgressMax
    pub fn get_current_state_progress_max(&self) -> Option<&u64> {
        self.current_state_progress_max.as_ref()
    }

    /// Sets the value of DesiredCacheModeHDD
    pub fn set_desired_cache_mode_hdd(&mut self, value: u32) {
        self.desired_cache_mode_hdd = Some(value);
    }

    /// Gets the value of DesiredCacheModeHDD
    pub fn get_desired_cache_mode_hdd(&self) -> Option<&u32> {
        self.desired_cache_mode_hdd.as_ref()
    }

    /// Sets the value of DesiredCacheModeSSD
    pub fn set_desired_cache_mode_ssd(&mut self, value: u32) {
        self.desired_cache_mode_ssd = Some(value);
    }

    /// Gets the value of DesiredCacheModeSSD
    pub fn get_desired_cache_mode_ssd(&self) -> Option<&u32> {
        self.desired_cache_mode_ssd.as_ref()
    }

    /// Sets the value of DesiredState
    pub fn set_desired_state(&mut self, value: u32) {
        self.desired_state = Some(value);
    }

    /// Gets the value of DesiredState
    pub fn get_desired_state(&self) -> Option<&u32> {
        self.desired_state.as_ref()
    }

    /// Sets the value of FlashMetadataReserveBytes
    pub fn set_flash_metadata_reserve_bytes(&mut self, value: u64) {
        self.flash_metadata_reserve_bytes = Some(value);
    }

    /// Gets the value of FlashMetadataReserveBytes
    pub fn get_flash_metadata_reserve_bytes(&self) -> Option<&u64> {
        self.flash_metadata_reserve_bytes.as_ref()
    }

    /// Sets the value of FlashReservePercent
    pub fn set_flash_reserve_percent(&mut self, value: u32) {
        self.flash_reserve_percent = Some(value);
    }

    /// Gets the value of FlashReservePercent
    pub fn get_flash_reserve_percent(&self) -> Option<&u32> {
        self.flash_reserve_percent.as_ref()
    }

    /// Sets the value of Identifier
    pub fn set_identifier(&mut self, value: String) {
        self.identifier = Some(value);
    }

    /// Gets the value of Identifier
    pub fn get_identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    /// Sets the value of ProvisioningStage
    pub fn set_provisioning_stage(&mut self, value: u64) {
        self.provisioning_stage = Some(value);
    }

    /// Gets the value of ProvisioningStage
    pub fn get_provisioning_stage(&self) -> Option<&u64> {
        self.provisioning_stage.as_ref()
    }

    /// Sets the value of ProvisioningStageMax
    pub fn set_provisioning_stage_max(&mut self, value: u64) {
        self.provisioning_stage_max = Some(value);
    }

    /// Gets the value of ProvisioningStageMax
    pub fn get_provisioning_stage_max(&self) -> Option<&u64> {
        self.provisioning_stage_max.as_ref()
    }

    /// Sets the value of SpacesDirectEnabled
    pub fn set_spaces_direct_enabled(&mut self, value: bool) {
        self.spaces_direct_enabled = Some(value);
    }

    /// Gets the value of SpacesDirectEnabled
    pub fn get_spaces_direct_enabled(&self) -> Option<&bool> {
        self.spaces_direct_enabled.as_ref()
    }

/// 

    /// * `description` -  (String)
    /// * `disk_guid` -  (String)
    /// * `enclosure_id` -  (String)
    /// * `manufacturer` -  (String)
    /// * `name` -  (String)
    /// * `pool_id` -  (String)
    /// * `product_id` -  (String)
    /// * `serial` -  (String)
    /// * `slot_number` -  (u32)

    /// * `return_value` -  (u32)
    pub fn notify_disk(&self, disk_guid: &String, pool_id: &String, name: &String, description: &String, manufacturer: &String, product_id: &String, serial: &String, slot_number: u32, enclosure_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiskGuid".to_string(), value: disk_guid.into() });
        args.push(MethodParameter { name: "PoolId".to_string(), value: pool_id.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Manufacturer".to_string(), value: manufacturer.into() });
        args.push(MethodParameter { name: "ProductId".to_string(), value: product_id.into() });
        args.push(MethodParameter { name: "Serial".to_string(), value: serial.into() });
        args.push(MethodParameter { name: "SlotNumber".to_string(), value: slot_number.into() });
        args.push(MethodParameter { name: "EnclosureId".to_string(), value: enclosure_id.into() });
        self.invoke_method("NotifyDisk", &args)

    }


/// 

    /// * `description` -  (String)
    /// * `enclosure_guid` -  (String)
    /// * `manufacturer` -  (String)
    /// * `name` -  (String)
    /// * `product_id` -  (String)
    /// * `serial` -  (String)

    /// * `return_value` -  (u32)
    pub fn notify_enclosure(&self, enclosure_guid: &String, name: &String, description: &String, manufacturer: &String, product_id: &String, serial: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EnclosureGuid".to_string(), value: enclosure_guid.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Manufacturer".to_string(), value: manufacturer.into() });
        args.push(MethodParameter { name: "ProductId".to_string(), value: product_id.into() });
        args.push(MethodParameter { name: "Serial".to_string(), value: serial.into() });
        self.invoke_method("NotifyEnclosure", &args)

    }


/// 

    /// * `disk_guid` -  (String)
    /// * `state_change` -  (u32)

    /// * `return_value` -  (u32)
    pub fn notify_disk_state_change(&self, disk_guid: &String, state_change: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiskGuid".to_string(), value: disk_guid.into() });
        args.push(MethodParameter { name: "StateChange".to_string(), value: state_change.into() });
        self.invoke_method("NotifyDiskStateChange", &args)

    }


/// 

    /// * `disk_guid` -  (String)
    /// * `use_for_storage_spaces_direct` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_disk_usage(&self, disk_guid: &String, use_for_storage_spaces_direct: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiskGuid".to_string(), value: disk_guid.into() });
        args.push(MethodParameter { name: "UseForStorageSpacesDirect".to_string(), value: use_for_storage_spaces_direct.into() });
        self.invoke_method("SetDiskUsage", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn start_optimize(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("StartOptimize", &args)

    }


/// 

    /// * `cache_state` -  (u32)

    /// * `return_value` -  (u32)
    pub fn check_system_supports_cache_state(&self, cache_state: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CacheState".to_string(), value: cache_state.into() });
        self.invoke_method("CheckSystemSupportsCacheState", &args)

    }


/// 

    /// * `cache_state` -  (u32)
    /// * `disk_guid` -  (String)

    /// * `return_value` -  (u32)
    pub fn check_disk_supports_cache_state(&self, disk_guid: &String, cache_state: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiskGuid".to_string(), value: disk_guid.into() });
        args.push(MethodParameter { name: "CacheState".to_string(), value: cache_state.into() });
        self.invoke_method("CheckDiskSupportsCacheState", &args)

    }


/// 

    /// * `cache_state` -  (u32)

    /// * `disk_guids` -  (String[])
    /// * `disk_numbers` -  (u32[])
    /// * `return_value` -  (u32)
    /// * `support_statuses` -  (u32[])
    pub fn check_all_disks_support_cache(&self, cache_state: u32, disk_guids: &mut Vec<String>, disk_numbers: &mut Vec<u32>, support_statuses: &mut Vec<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CacheState".to_string(), value: cache_state.into() });

        let result = self.invoke_method("CheckAllDisksSupportCache", &args)?;
        let disk_guids = result.get_value("DiskGuids")?;
        let disk_numbers = result.get_value("DiskNumbers")?;
        let support_statuses = result.get_value("SupportStatuses")?;
        Ok(result.return_value)

    }


/// 

    /// * `disk_guid` -  (String)

    /// * `bound_disk_guids` -  (String[])
    /// * `return_value` -  (u32)
    pub fn query_bound_devices(&self, disk_guid: &String, bound_disk_guids: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiskGuid".to_string(), value: disk_guid.into() });

        let result = self.invoke_method("QueryBoundDevices", &args)?;
        let bound_disk_guids = result.get_value("BoundDiskGuids")?;
        Ok(result.return_value)

    }


/// 

    /// * `cache_mode` -  (u32)
    /// * `disk_guid` -  (String)
    /// * `flags` -  (u32)
    /// * `force` -  (bool)
    /// * `originator` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_disk_cache_mode(&self, disk_guid: &String, cache_mode: u32, flags: u32, originator: u32, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiskGuid".to_string(), value: disk_guid.into() });
        args.push(MethodParameter { name: "CacheMode".to_string(), value: cache_mode.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "Originator".to_string(), value: originator.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("SetDiskCacheMode", &args)

    }


/// 

    /// * `cache_hint` -  (u32)
    /// * `disk_guid` -  (String)
    /// * `flags` -  (u32)
    /// * `originator` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_disk_cache_hint(&self, disk_guid: &String, cache_hint: u32, flags: u32, originator: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiskGuid".to_string(), value: disk_guid.into() });
        args.push(MethodParameter { name: "CacheHint".to_string(), value: cache_hint.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "Originator".to_string(), value: originator.into() });
        self.invoke_method("SetDiskCacheHint", &args)

    }

}

