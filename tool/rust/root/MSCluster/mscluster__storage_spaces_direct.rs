// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_StorageSpacesDirect struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_StorageSpacesDirect {

/// 
    #[serde(rename = "CacheDeviceModel")]
    pub cache_device_model: Vec<String>,

/// 
    #[serde(rename = "CacheMetadataReserveBytes")]
    pub cache_metadata_reserve_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheModeHDD")]
    pub cache_mode_hdd: Option<u32>,

/// 
    #[serde(rename = "CacheModeSSD")]
    pub cache_mode_ssd: Option<u32>,

/// 
    #[serde(rename = "CachePageSizeKBytes")]
    pub cache_page_size_kbytes: Option<u32>,

/// 
    #[serde(rename = "CacheState")]
    pub cache_state: Option<u32>,

/// 
    #[serde(rename = "EnableReportName")]
    pub enable_report_name: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Node")]
    pub node: Option<String>,

/// 
    #[serde(rename = "ScmUse")]
    pub scm_use: Option<u32>,

/// 
    #[serde(rename = "SedProtectionState")]
    pub sed_protection_state: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "UseSedExclusively")]
    pub use_sed_exclusively: Option<bool>,
}

impl MSCluster_StorageSpacesDirect {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cache_device_model: Vec::new(),
            cache_metadata_reserve_bytes: None,
            cache_mode_hdd: None,
            cache_mode_ssd: None,
            cache_page_size_kbytes: None,
            cache_state: None,
            enable_report_name: None,
            name: None,
            node: None,
            scm_use: None,
            sed_protection_state: None,
            state: None,
            use_sed_exclusively: None,
        }
    }


    /// Sets the value of CacheDeviceModel
    pub fn set_cache_device_model(&mut self, value: Vec<String>) {
        self.cache_device_model = value;
    }

    /// Gets the value of CacheDeviceModel
    pub fn get_cache_device_model(&self) -> &Vec<String> {
        &self.cache_device_model
    }

    /// Sets the value of CacheMetadataReserveBytes
    pub fn set_cache_metadata_reserve_bytes(&mut self, value: u64) {
        self.cache_metadata_reserve_bytes = Some(value);
    }

    /// Gets the value of CacheMetadataReserveBytes
    pub fn get_cache_metadata_reserve_bytes(&self) -> Option<&u64> {
        self.cache_metadata_reserve_bytes.as_ref()
    }

    /// Sets the value of CacheModeHDD
    pub fn set_cache_mode_hdd(&mut self, value: u32) {
        self.cache_mode_hdd = Some(value);
    }

    /// Gets the value of CacheModeHDD
    pub fn get_cache_mode_hdd(&self) -> Option<&u32> {
        self.cache_mode_hdd.as_ref()
    }

    /// Sets the value of CacheModeSSD
    pub fn set_cache_mode_ssd(&mut self, value: u32) {
        self.cache_mode_ssd = Some(value);
    }

    /// Gets the value of CacheModeSSD
    pub fn get_cache_mode_ssd(&self) -> Option<&u32> {
        self.cache_mode_ssd.as_ref()
    }

    /// Sets the value of CachePageSizeKBytes
    pub fn set_cache_page_size_kbytes(&mut self, value: u32) {
        self.cache_page_size_kbytes = Some(value);
    }

    /// Gets the value of CachePageSizeKBytes
    pub fn get_cache_page_size_kbytes(&self) -> Option<&u32> {
        self.cache_page_size_kbytes.as_ref()
    }

    /// Sets the value of CacheState
    pub fn set_cache_state(&mut self, value: u32) {
        self.cache_state = Some(value);
    }

    /// Gets the value of CacheState
    pub fn get_cache_state(&self) -> Option<&u32> {
        self.cache_state.as_ref()
    }

    /// Sets the value of EnableReportName
    pub fn set_enable_report_name(&mut self, value: String) {
        self.enable_report_name = Some(value);
    }

    /// Gets the value of EnableReportName
    pub fn get_enable_report_name(&self) -> Option<&String> {
        self.enable_report_name.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Node
    pub fn set_node(&mut self, value: String) {
        self.node = Some(value);
    }

    /// Gets the value of Node
    pub fn get_node(&self) -> Option<&String> {
        self.node.as_ref()
    }

    /// Sets the value of ScmUse
    pub fn set_scm_use(&mut self, value: u32) {
        self.scm_use = Some(value);
    }

    /// Gets the value of ScmUse
    pub fn get_scm_use(&self) -> Option<&u32> {
        self.scm_use.as_ref()
    }

    /// Sets the value of SedProtectionState
    pub fn set_sed_protection_state(&mut self, value: u32) {
        self.sed_protection_state = Some(value);
    }

    /// Gets the value of SedProtectionState
    pub fn get_sed_protection_state(&self) -> Option<&u32> {
        self.sed_protection_state.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of UseSedExclusively
    pub fn set_use_sed_exclusively(&mut self, value: bool) {
        self.use_sed_exclusively = Some(value);
    }

    /// Gets the value of UseSedExclusively
    pub fn get_use_sed_exclusively(&self) -> Option<&bool> {
        self.use_sed_exclusively.as_ref()
    }

/// 

    /// * `auto_config` -  (bool)
    /// * `bus_types_to_use` -  (u16[])
    /// * `cache_device_model` -  (String[])
    /// * `cache_metadata_reserve_bytes` -  (u64)
    /// * `cache_page_size_kbytes` -  (u32)
    /// * `cache_state` -  (u32)
    /// * `collect_performance_history` -  (bool)
    /// * `pool_friendly_name` -  (String)
    /// * `sed_protection_state` -  (u32)
    /// * `skip_eligibility_checks` -  (bool)
    /// * `use_sed_exclusively` -  (bool)
    /// * `xml` -  (String)

    /// * `return_value` -  (u32)
    /// * `storage_spaces_direct` -  (MSCluster_StorageSpacesDirect)
    pub fn enable_storage_spaces_direct(&self, cache_state: u32, cache_metadata_reserve_bytes: u64, xml: &String, cache_device_model: &Vec<String>, auto_config: bool, cache_page_size_kbytes: u32, pool_friendly_name: &String, skip_eligibility_checks: bool, collect_performance_history: bool, bus_types_to_use: &Vec<u16>, use_sed_exclusively: bool, sed_protection_state: u32, storage_spaces_direct: &mut MSCluster_StorageSpacesDirect) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CacheState".to_string(), value: cache_state.into() });
        args.push(MethodParameter { name: "CacheMetadataReserveBytes".to_string(), value: cache_metadata_reserve_bytes.into() });
        args.push(MethodParameter { name: "XML".to_string(), value: xml.into() });
        args.push(MethodParameter { name: "CacheDeviceModel".to_string(), value: cache_device_model.into() });
        args.push(MethodParameter { name: "AutoConfig".to_string(), value: auto_config.into() });
        args.push(MethodParameter { name: "CachePageSizeKBytes".to_string(), value: cache_page_size_kbytes.into() });
        args.push(MethodParameter { name: "PoolFriendlyName".to_string(), value: pool_friendly_name.into() });
        args.push(MethodParameter { name: "SkipEligibilityChecks".to_string(), value: skip_eligibility_checks.into() });
        args.push(MethodParameter { name: "CollectPerformanceHistory".to_string(), value: collect_performance_history.into() });
        args.push(MethodParameter { name: "BusTypesToUse".to_string(), value: bus_types_to_use.into() });
        args.push(MethodParameter { name: "UseSedExclusively".to_string(), value: use_sed_exclusively.into() });
        args.push(MethodParameter { name: "SedProtectionState".to_string(), value: sed_protection_state.into() });

        let result = self.invoke_method("EnableStorageSpacesDirect", &args)?;
        let storage_spaces_direct = result.get_value("StorageSpacesDirect")?;
        Ok(result.return_value)

    }


/// 

    /// * `cleanup_cache` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable_storage_spaces_direct(&self, cleanup_cache: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CleanupCache".to_string(), value: cleanup_cache.into() });
        self.invoke_method("DisableStorageSpacesDirect", &args)

    }


/// 

    /// * `cache_mode_hdd` -  (u32)
    /// * `cache_mode_ssd` -  (u32)
    /// * `cache_state` -  (u32)
    /// * `nodes` -  (String[])
    /// * `scm_use` -  (u32)
    /// * `sed_protection_state` -  (u32)
    /// * `skip_eligibility_checks` -  (bool)
    /// * `use_sed_exclusively` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_storage_spaces_direct(&self, cache_state: u32, cache_mode_hdd: u32, cache_mode_ssd: u32, skip_eligibility_checks: bool, scm_use: u32, nodes: &Vec<String>, use_sed_exclusively: bool, sed_protection_state: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CacheState".to_string(), value: cache_state.into() });
        args.push(MethodParameter { name: "CacheModeHdd".to_string(), value: cache_mode_hdd.into() });
        args.push(MethodParameter { name: "CacheModeSsd".to_string(), value: cache_mode_ssd.into() });
        args.push(MethodParameter { name: "SkipEligibilityChecks".to_string(), value: skip_eligibility_checks.into() });
        args.push(MethodParameter { name: "ScmUse".to_string(), value: scm_use.into() });
        args.push(MethodParameter { name: "Nodes".to_string(), value: nodes.into() });
        args.push(MethodParameter { name: "UseSedExclusively".to_string(), value: use_sed_exclusively.into() });
        args.push(MethodParameter { name: "SedProtectionState".to_string(), value: sed_protection_state.into() });
        self.invoke_method("SetStorageSpacesDirect", &args)

    }


/// 

    /// * `node` -  (String)

    /// * `return_value` -  (u32)
    /// * `the_storage_spaces_direct` -  (MSCluster_StorageSpacesDirect)
    pub fn get_storage_spaces_direct(&self, node: &String, the_storage_spaces_direct: &mut MSCluster_StorageSpacesDirect) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Node".to_string(), value: node.into() });

        let result = self.invoke_method("GetStorageSpacesDirect", &args)?;
        let the_storage_spaces_direct = result.get_value("TheStorageSpacesDirect")?;
        Ok(result.return_value)

    }


/// 

    /// * `n_nodes_in_site` -  (u32)
    /// * `pool_has_non_cache_hdd` -  (bool)
    /// * `pool_has_non_cache_scm` -  (bool)
    /// * `pool_has_non_cache_ssd` -  (bool)
    /// * `pool_unique_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn post_create_pool(&self, pool_unique_id: &String, n_nodes_in_site: u32, pool_has_non_cache_hdd: bool, pool_has_non_cache_ssd: bool, pool_has_non_cache_scm: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PoolUniqueId".to_string(), value: pool_unique_id.into() });
        args.push(MethodParameter { name: "nNodesInSite".to_string(), value: n_nodes_in_site.into() });
        args.push(MethodParameter { name: "PoolHasNonCacheHdd".to_string(), value: pool_has_non_cache_hdd.into() });
        args.push(MethodParameter { name: "PoolHasNonCacheSsd".to_string(), value: pool_has_non_cache_ssd.into() });
        args.push(MethodParameter { name: "PoolHasNonCacheScm".to_string(), value: pool_has_non_cache_scm.into() });
        self.invoke_method("PostCreatePool", &args)

    }


/// 

    /// * `disable_storage_maintenance_mode` -  (bool)
    /// * `node` -  (String)
    /// * `recover_unbound_drives` -  (bool)
    /// * `skip_disk_recovery` -  (bool)

    /// * `return_value` -  (u32)
    pub fn repair_storage_spaces_direct(&self, skip_disk_recovery: bool, disable_storage_maintenance_mode: bool, recover_unbound_drives: bool, node: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SkipDiskRecovery".to_string(), value: skip_disk_recovery.into() });
        args.push(MethodParameter { name: "DisableStorageMaintenanceMode".to_string(), value: disable_storage_maintenance_mode.into() });
        args.push(MethodParameter { name: "RecoverUnboundDrives".to_string(), value: recover_unbound_drives.into() });
        args.push(MethodParameter { name: "Node".to_string(), value: node.into() });
        self.invoke_method("RepairStorageSpacesDirect", &args)

    }


/// 

    /// * `cache_usage` -  (u32)
    /// * `can_be_claimed` -  (bool)
    /// * `physical_disk_ids` -  (String[])
    /// * `reset` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_storage_spaces_direct_disk(&self, can_be_claimed: bool, reset: bool, cache_usage: u32, physical_disk_ids: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CanBeClaimed".to_string(), value: can_be_claimed.into() });
        args.push(MethodParameter { name: "Reset".to_string(), value: reset.into() });
        args.push(MethodParameter { name: "CacheUsage".to_string(), value: cache_usage.into() });
        args.push(MethodParameter { name: "PhysicalDiskIds".to_string(), value: physical_disk_ids.into() });
        self.invoke_method("SetStorageSpacesDirectDisk", &args)

    }


/// 

    /// * `can_be_claimed` -  (bool)

    /// * `physical_disk_ids` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_storage_spaces_direct_disk(&self, can_be_claimed: bool, physical_disk_ids: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CanBeClaimed".to_string(), value: can_be_claimed.into() });

        let result = self.invoke_method("GetStorageSpacesDirectDisk", &args)?;
        let physical_disk_ids = result.get_value("PhysicalDiskIds")?;
        Ok(result.return_value)

    }

}

