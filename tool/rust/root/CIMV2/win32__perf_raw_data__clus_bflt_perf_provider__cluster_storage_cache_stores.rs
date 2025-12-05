// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageCacheStores struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageCacheStores {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BindingsActive")]
    pub bindings_active: Option<u64>,

/// 
    #[serde(rename = "BindingsEnabled")]
    pub bindings_enabled: Option<u64>,

/// 
    #[serde(rename = "CachePages")]
    pub cache_pages: Option<u64>,

/// 
    #[serde(rename = "CachePagesBytes")]
    pub cache_pages_bytes: Option<u64>,

/// 
    #[serde(rename = "CachePagesDirty")]
    pub cache_pages_dirty: Option<u64>,

/// 
    #[serde(rename = "CachePagesFree")]
    pub cache_pages_free: Option<u64>,

/// 
    #[serde(rename = "CachePagesStandBy")]
    pub cache_pages_stand_by: Option<u64>,

/// 
    #[serde(rename = "CachePagesStandByL0")]
    pub cache_pages_stand_by_l0: Option<u64>,

/// 
    #[serde(rename = "CachePagesStandByL1")]
    pub cache_pages_stand_by_l1: Option<u64>,

/// 
    #[serde(rename = "CachePagesStandByL2")]
    pub cache_pages_stand_by_l2: Option<u64>,

/// 
    #[serde(rename = "CachePagesStandByOldestL1")]
    pub cache_pages_stand_by_oldest_l1: Option<u64>,

/// 
    #[serde(rename = "CacheStores")]
    pub cache_stores: Option<u64>,

/// 
    #[serde(rename = "CacheUsageEfficiencyPercent")]
    pub cache_usage_efficiency_percent: Option<u64>,

/// 
    #[serde(rename = "CacheUsageEfficiencyPercent_Base")]
    pub cache_usage_efficiency_percent__base: Option<u64>,

/// 
    #[serde(rename = "CacheUsagePercent")]
    pub cache_usage_percent: Option<u64>,

/// 
    #[serde(rename = "CacheUsagePercent_Base")]
    pub cache_usage_percent__base: Option<u64>,

/// 
    #[serde(rename = "DestageBytes")]
    pub destage_bytes: Option<u64>,

/// 
    #[serde(rename = "DestageBytesPersec")]
    pub destage_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DestagedAtLowPriPercent")]
    pub destaged_at_low_pri_percent: Option<u64>,

/// 
    #[serde(rename = "DestagedAtLowPriPercent_Base")]
    pub destaged_at_low_pri_percent__base: Option<u64>,

/// 
    #[serde(rename = "DestagedAtNormalPriPercent")]
    pub destaged_at_normal_pri_percent: Option<u64>,

/// 
    #[serde(rename = "DestagedAtNormalPriPercent_Base")]
    pub destaged_at_normal_pri_percent__base: Option<u64>,

/// 
    #[serde(rename = "DestageTransfers")]
    pub destage_transfers: Option<u64>,

/// 
    #[serde(rename = "DestageTransfersPersec")]
    pub destage_transfers_persec: Option<u64>,

/// 
    #[serde(rename = "DevicesBlocked")]
    pub devices_blocked: Option<u64>,

/// 
    #[serde(rename = "DevicesHybrid")]
    pub devices_hybrid: Option<u64>,

/// 
    #[serde(rename = "DevicesMaintenance")]
    pub devices_maintenance: Option<u64>,

/// 
    #[serde(rename = "DevicesNotConfigured")]
    pub devices_not_configured: Option<u64>,

/// 
    #[serde(rename = "DevicesOrphan")]
    pub devices_orphan: Option<u64>,

/// 
    #[serde(rename = "MultiPageFragments")]
    pub multi_page_fragments: Option<u64>,

/// 
    #[serde(rename = "MultiPageFragmentsRate")]
    pub multi_page_fragments_rate: Option<u64>,

/// 
    #[serde(rename = "MultiPageFragmentsRate_Base")]
    pub multi_page_fragments_rate__base: Option<u32>,

/// 
    #[serde(rename = "MultiPageReMap")]
    pub multi_page_re_map: Option<u64>,

/// 
    #[serde(rename = "PageHit")]
    pub page_hit: Option<u64>,

/// 
    #[serde(rename = "PageHitPersec")]
    pub page_hit_persec: Option<u64>,

/// 
    #[serde(rename = "PageReMap")]
    pub page_re_map: Option<u64>,

/// 
    #[serde(rename = "PageReMapPersec")]
    pub page_re_map_persec: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsMedia")]
    pub read_errors_media: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsTimeout")]
    pub read_errors_timeout: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsTotal")]
    pub read_errors_total: Option<u64>,

/// 
    #[serde(rename = "UpdateBytes")]
    pub update_bytes: Option<u64>,

/// 
    #[serde(rename = "UpdateBytesPersec")]
    pub update_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "UpdatesCritical")]
    pub updates_critical: Option<u64>,

/// 
    #[serde(rename = "UpdatesCriticalLogFull")]
    pub updates_critical_log_full: Option<u64>,

/// 
    #[serde(rename = "UpdatesCriticalPersec")]
    pub updates_critical_persec: Option<u64>,

/// 
    #[serde(rename = "UpdatesNonCritical")]
    pub updates_non_critical: Option<u64>,

/// 
    #[serde(rename = "UpdatesNonCriticalLogFull")]
    pub updates_non_critical_log_full: Option<u64>,

/// 
    #[serde(rename = "UpdatesNonCriticalPersec")]
    pub updates_non_critical_persec: Option<u64>,

/// 
    #[serde(rename = "UpdatesNotCommitted")]
    pub updates_not_committed: Option<u64>,

/// 
    #[serde(rename = "UpdateTransfers")]
    pub update_transfers: Option<u64>,

/// 
    #[serde(rename = "UpdateTransfersPersec")]
    pub update_transfers_persec: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsMedia")]
    pub write_errors_media: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsTimeout")]
    pub write_errors_timeout: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsTotal")]
    pub write_errors_total: Option<u64>,
}

impl Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageCacheStores {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bindings_active: None,
            bindings_enabled: None,
            cache_pages: None,
            cache_pages_bytes: None,
            cache_pages_dirty: None,
            cache_pages_free: None,
            cache_pages_stand_by: None,
            cache_pages_stand_by_l0: None,
            cache_pages_stand_by_l1: None,
            cache_pages_stand_by_l2: None,
            cache_pages_stand_by_oldest_l1: None,
            cache_stores: None,
            cache_usage_efficiency_percent: None,
            cache_usage_efficiency_percent__base: None,
            cache_usage_percent: None,
            cache_usage_percent__base: None,
            destage_bytes: None,
            destage_bytes_persec: None,
            destaged_at_low_pri_percent: None,
            destaged_at_low_pri_percent__base: None,
            destaged_at_normal_pri_percent: None,
            destaged_at_normal_pri_percent__base: None,
            destage_transfers: None,
            destage_transfers_persec: None,
            devices_blocked: None,
            devices_hybrid: None,
            devices_maintenance: None,
            devices_not_configured: None,
            devices_orphan: None,
            multi_page_fragments: None,
            multi_page_fragments_rate: None,
            multi_page_fragments_rate__base: None,
            multi_page_re_map: None,
            page_hit: None,
            page_hit_persec: None,
            page_re_map: None,
            page_re_map_persec: None,
            read_errors_media: None,
            read_errors_timeout: None,
            read_errors_total: None,
            update_bytes: None,
            update_bytes_persec: None,
            updates_critical: None,
            updates_critical_log_full: None,
            updates_critical_persec: None,
            updates_non_critical: None,
            updates_non_critical_log_full: None,
            updates_non_critical_persec: None,
            updates_not_committed: None,
            update_transfers: None,
            update_transfers_persec: None,
            write_errors_media: None,
            write_errors_timeout: None,
            write_errors_total: None,
        }
    }


    /// Sets the value of BindingsActive
    pub fn set_bindings_active(&mut self, value: u64) {
        self.bindings_active = Some(value);
    }

    /// Gets the value of BindingsActive
    pub fn get_bindings_active(&self) -> Option<&u64> {
        self.bindings_active.as_ref()
    }

    /// Sets the value of BindingsEnabled
    pub fn set_bindings_enabled(&mut self, value: u64) {
        self.bindings_enabled = Some(value);
    }

    /// Gets the value of BindingsEnabled
    pub fn get_bindings_enabled(&self) -> Option<&u64> {
        self.bindings_enabled.as_ref()
    }

    /// Sets the value of CachePages
    pub fn set_cache_pages(&mut self, value: u64) {
        self.cache_pages = Some(value);
    }

    /// Gets the value of CachePages
    pub fn get_cache_pages(&self) -> Option<&u64> {
        self.cache_pages.as_ref()
    }

    /// Sets the value of CachePagesBytes
    pub fn set_cache_pages_bytes(&mut self, value: u64) {
        self.cache_pages_bytes = Some(value);
    }

    /// Gets the value of CachePagesBytes
    pub fn get_cache_pages_bytes(&self) -> Option<&u64> {
        self.cache_pages_bytes.as_ref()
    }

    /// Sets the value of CachePagesDirty
    pub fn set_cache_pages_dirty(&mut self, value: u64) {
        self.cache_pages_dirty = Some(value);
    }

    /// Gets the value of CachePagesDirty
    pub fn get_cache_pages_dirty(&self) -> Option<&u64> {
        self.cache_pages_dirty.as_ref()
    }

    /// Sets the value of CachePagesFree
    pub fn set_cache_pages_free(&mut self, value: u64) {
        self.cache_pages_free = Some(value);
    }

    /// Gets the value of CachePagesFree
    pub fn get_cache_pages_free(&self) -> Option<&u64> {
        self.cache_pages_free.as_ref()
    }

    /// Sets the value of CachePagesStandBy
    pub fn set_cache_pages_stand_by(&mut self, value: u64) {
        self.cache_pages_stand_by = Some(value);
    }

    /// Gets the value of CachePagesStandBy
    pub fn get_cache_pages_stand_by(&self) -> Option<&u64> {
        self.cache_pages_stand_by.as_ref()
    }

    /// Sets the value of CachePagesStandByL0
    pub fn set_cache_pages_stand_by_l0(&mut self, value: u64) {
        self.cache_pages_stand_by_l0 = Some(value);
    }

    /// Gets the value of CachePagesStandByL0
    pub fn get_cache_pages_stand_by_l0(&self) -> Option<&u64> {
        self.cache_pages_stand_by_l0.as_ref()
    }

    /// Sets the value of CachePagesStandByL1
    pub fn set_cache_pages_stand_by_l1(&mut self, value: u64) {
        self.cache_pages_stand_by_l1 = Some(value);
    }

    /// Gets the value of CachePagesStandByL1
    pub fn get_cache_pages_stand_by_l1(&self) -> Option<&u64> {
        self.cache_pages_stand_by_l1.as_ref()
    }

    /// Sets the value of CachePagesStandByL2
    pub fn set_cache_pages_stand_by_l2(&mut self, value: u64) {
        self.cache_pages_stand_by_l2 = Some(value);
    }

    /// Gets the value of CachePagesStandByL2
    pub fn get_cache_pages_stand_by_l2(&self) -> Option<&u64> {
        self.cache_pages_stand_by_l2.as_ref()
    }

    /// Sets the value of CachePagesStandByOldestL1
    pub fn set_cache_pages_stand_by_oldest_l1(&mut self, value: u64) {
        self.cache_pages_stand_by_oldest_l1 = Some(value);
    }

    /// Gets the value of CachePagesStandByOldestL1
    pub fn get_cache_pages_stand_by_oldest_l1(&self) -> Option<&u64> {
        self.cache_pages_stand_by_oldest_l1.as_ref()
    }

    /// Sets the value of CacheStores
    pub fn set_cache_stores(&mut self, value: u64) {
        self.cache_stores = Some(value);
    }

    /// Gets the value of CacheStores
    pub fn get_cache_stores(&self) -> Option<&u64> {
        self.cache_stores.as_ref()
    }

    /// Sets the value of CacheUsageEfficiencyPercent
    pub fn set_cache_usage_efficiency_percent(&mut self, value: u64) {
        self.cache_usage_efficiency_percent = Some(value);
    }

    /// Gets the value of CacheUsageEfficiencyPercent
    pub fn get_cache_usage_efficiency_percent(&self) -> Option<&u64> {
        self.cache_usage_efficiency_percent.as_ref()
    }

    /// Sets the value of CacheUsageEfficiencyPercent_Base
    pub fn set_cache_usage_efficiency_percent__base(&mut self, value: u64) {
        self.cache_usage_efficiency_percent__base = Some(value);
    }

    /// Gets the value of CacheUsageEfficiencyPercent_Base
    pub fn get_cache_usage_efficiency_percent__base(&self) -> Option<&u64> {
        self.cache_usage_efficiency_percent__base.as_ref()
    }

    /// Sets the value of CacheUsagePercent
    pub fn set_cache_usage_percent(&mut self, value: u64) {
        self.cache_usage_percent = Some(value);
    }

    /// Gets the value of CacheUsagePercent
    pub fn get_cache_usage_percent(&self) -> Option<&u64> {
        self.cache_usage_percent.as_ref()
    }

    /// Sets the value of CacheUsagePercent_Base
    pub fn set_cache_usage_percent__base(&mut self, value: u64) {
        self.cache_usage_percent__base = Some(value);
    }

    /// Gets the value of CacheUsagePercent_Base
    pub fn get_cache_usage_percent__base(&self) -> Option<&u64> {
        self.cache_usage_percent__base.as_ref()
    }

    /// Sets the value of DestageBytes
    pub fn set_destage_bytes(&mut self, value: u64) {
        self.destage_bytes = Some(value);
    }

    /// Gets the value of DestageBytes
    pub fn get_destage_bytes(&self) -> Option<&u64> {
        self.destage_bytes.as_ref()
    }

    /// Sets the value of DestageBytesPersec
    pub fn set_destage_bytes_persec(&mut self, value: u64) {
        self.destage_bytes_persec = Some(value);
    }

    /// Gets the value of DestageBytesPersec
    pub fn get_destage_bytes_persec(&self) -> Option<&u64> {
        self.destage_bytes_persec.as_ref()
    }

    /// Sets the value of DestagedAtLowPriPercent
    pub fn set_destaged_at_low_pri_percent(&mut self, value: u64) {
        self.destaged_at_low_pri_percent = Some(value);
    }

    /// Gets the value of DestagedAtLowPriPercent
    pub fn get_destaged_at_low_pri_percent(&self) -> Option<&u64> {
        self.destaged_at_low_pri_percent.as_ref()
    }

    /// Sets the value of DestagedAtLowPriPercent_Base
    pub fn set_destaged_at_low_pri_percent__base(&mut self, value: u64) {
        self.destaged_at_low_pri_percent__base = Some(value);
    }

    /// Gets the value of DestagedAtLowPriPercent_Base
    pub fn get_destaged_at_low_pri_percent__base(&self) -> Option<&u64> {
        self.destaged_at_low_pri_percent__base.as_ref()
    }

    /// Sets the value of DestagedAtNormalPriPercent
    pub fn set_destaged_at_normal_pri_percent(&mut self, value: u64) {
        self.destaged_at_normal_pri_percent = Some(value);
    }

    /// Gets the value of DestagedAtNormalPriPercent
    pub fn get_destaged_at_normal_pri_percent(&self) -> Option<&u64> {
        self.destaged_at_normal_pri_percent.as_ref()
    }

    /// Sets the value of DestagedAtNormalPriPercent_Base
    pub fn set_destaged_at_normal_pri_percent__base(&mut self, value: u64) {
        self.destaged_at_normal_pri_percent__base = Some(value);
    }

    /// Gets the value of DestagedAtNormalPriPercent_Base
    pub fn get_destaged_at_normal_pri_percent__base(&self) -> Option<&u64> {
        self.destaged_at_normal_pri_percent__base.as_ref()
    }

    /// Sets the value of DestageTransfers
    pub fn set_destage_transfers(&mut self, value: u64) {
        self.destage_transfers = Some(value);
    }

    /// Gets the value of DestageTransfers
    pub fn get_destage_transfers(&self) -> Option<&u64> {
        self.destage_transfers.as_ref()
    }

    /// Sets the value of DestageTransfersPersec
    pub fn set_destage_transfers_persec(&mut self, value: u64) {
        self.destage_transfers_persec = Some(value);
    }

    /// Gets the value of DestageTransfersPersec
    pub fn get_destage_transfers_persec(&self) -> Option<&u64> {
        self.destage_transfers_persec.as_ref()
    }

    /// Sets the value of DevicesBlocked
    pub fn set_devices_blocked(&mut self, value: u64) {
        self.devices_blocked = Some(value);
    }

    /// Gets the value of DevicesBlocked
    pub fn get_devices_blocked(&self) -> Option<&u64> {
        self.devices_blocked.as_ref()
    }

    /// Sets the value of DevicesHybrid
    pub fn set_devices_hybrid(&mut self, value: u64) {
        self.devices_hybrid = Some(value);
    }

    /// Gets the value of DevicesHybrid
    pub fn get_devices_hybrid(&self) -> Option<&u64> {
        self.devices_hybrid.as_ref()
    }

    /// Sets the value of DevicesMaintenance
    pub fn set_devices_maintenance(&mut self, value: u64) {
        self.devices_maintenance = Some(value);
    }

    /// Gets the value of DevicesMaintenance
    pub fn get_devices_maintenance(&self) -> Option<&u64> {
        self.devices_maintenance.as_ref()
    }

    /// Sets the value of DevicesNotConfigured
    pub fn set_devices_not_configured(&mut self, value: u64) {
        self.devices_not_configured = Some(value);
    }

    /// Gets the value of DevicesNotConfigured
    pub fn get_devices_not_configured(&self) -> Option<&u64> {
        self.devices_not_configured.as_ref()
    }

    /// Sets the value of DevicesOrphan
    pub fn set_devices_orphan(&mut self, value: u64) {
        self.devices_orphan = Some(value);
    }

    /// Gets the value of DevicesOrphan
    pub fn get_devices_orphan(&self) -> Option<&u64> {
        self.devices_orphan.as_ref()
    }

    /// Sets the value of MultiPageFragments
    pub fn set_multi_page_fragments(&mut self, value: u64) {
        self.multi_page_fragments = Some(value);
    }

    /// Gets the value of MultiPageFragments
    pub fn get_multi_page_fragments(&self) -> Option<&u64> {
        self.multi_page_fragments.as_ref()
    }

    /// Sets the value of MultiPageFragmentsRate
    pub fn set_multi_page_fragments_rate(&mut self, value: u64) {
        self.multi_page_fragments_rate = Some(value);
    }

    /// Gets the value of MultiPageFragmentsRate
    pub fn get_multi_page_fragments_rate(&self) -> Option<&u64> {
        self.multi_page_fragments_rate.as_ref()
    }

    /// Sets the value of MultiPageFragmentsRate_Base
    pub fn set_multi_page_fragments_rate__base(&mut self, value: u32) {
        self.multi_page_fragments_rate__base = Some(value);
    }

    /// Gets the value of MultiPageFragmentsRate_Base
    pub fn get_multi_page_fragments_rate__base(&self) -> Option<&u32> {
        self.multi_page_fragments_rate__base.as_ref()
    }

    /// Sets the value of MultiPageReMap
    pub fn set_multi_page_re_map(&mut self, value: u64) {
        self.multi_page_re_map = Some(value);
    }

    /// Gets the value of MultiPageReMap
    pub fn get_multi_page_re_map(&self) -> Option<&u64> {
        self.multi_page_re_map.as_ref()
    }

    /// Sets the value of PageHit
    pub fn set_page_hit(&mut self, value: u64) {
        self.page_hit = Some(value);
    }

    /// Gets the value of PageHit
    pub fn get_page_hit(&self) -> Option<&u64> {
        self.page_hit.as_ref()
    }

    /// Sets the value of PageHitPersec
    pub fn set_page_hit_persec(&mut self, value: u64) {
        self.page_hit_persec = Some(value);
    }

    /// Gets the value of PageHitPersec
    pub fn get_page_hit_persec(&self) -> Option<&u64> {
        self.page_hit_persec.as_ref()
    }

    /// Sets the value of PageReMap
    pub fn set_page_re_map(&mut self, value: u64) {
        self.page_re_map = Some(value);
    }

    /// Gets the value of PageReMap
    pub fn get_page_re_map(&self) -> Option<&u64> {
        self.page_re_map.as_ref()
    }

    /// Sets the value of PageReMapPersec
    pub fn set_page_re_map_persec(&mut self, value: u64) {
        self.page_re_map_persec = Some(value);
    }

    /// Gets the value of PageReMapPersec
    pub fn get_page_re_map_persec(&self) -> Option<&u64> {
        self.page_re_map_persec.as_ref()
    }

    /// Sets the value of ReadErrorsMedia
    pub fn set_read_errors_media(&mut self, value: u64) {
        self.read_errors_media = Some(value);
    }

    /// Gets the value of ReadErrorsMedia
    pub fn get_read_errors_media(&self) -> Option<&u64> {
        self.read_errors_media.as_ref()
    }

    /// Sets the value of ReadErrorsTimeout
    pub fn set_read_errors_timeout(&mut self, value: u64) {
        self.read_errors_timeout = Some(value);
    }

    /// Gets the value of ReadErrorsTimeout
    pub fn get_read_errors_timeout(&self) -> Option<&u64> {
        self.read_errors_timeout.as_ref()
    }

    /// Sets the value of ReadErrorsTotal
    pub fn set_read_errors_total(&mut self, value: u64) {
        self.read_errors_total = Some(value);
    }

    /// Gets the value of ReadErrorsTotal
    pub fn get_read_errors_total(&self) -> Option<&u64> {
        self.read_errors_total.as_ref()
    }

    /// Sets the value of UpdateBytes
    pub fn set_update_bytes(&mut self, value: u64) {
        self.update_bytes = Some(value);
    }

    /// Gets the value of UpdateBytes
    pub fn get_update_bytes(&self) -> Option<&u64> {
        self.update_bytes.as_ref()
    }

    /// Sets the value of UpdateBytesPersec
    pub fn set_update_bytes_persec(&mut self, value: u64) {
        self.update_bytes_persec = Some(value);
    }

    /// Gets the value of UpdateBytesPersec
    pub fn get_update_bytes_persec(&self) -> Option<&u64> {
        self.update_bytes_persec.as_ref()
    }

    /// Sets the value of UpdatesCritical
    pub fn set_updates_critical(&mut self, value: u64) {
        self.updates_critical = Some(value);
    }

    /// Gets the value of UpdatesCritical
    pub fn get_updates_critical(&self) -> Option<&u64> {
        self.updates_critical.as_ref()
    }

    /// Sets the value of UpdatesCriticalLogFull
    pub fn set_updates_critical_log_full(&mut self, value: u64) {
        self.updates_critical_log_full = Some(value);
    }

    /// Gets the value of UpdatesCriticalLogFull
    pub fn get_updates_critical_log_full(&self) -> Option<&u64> {
        self.updates_critical_log_full.as_ref()
    }

    /// Sets the value of UpdatesCriticalPersec
    pub fn set_updates_critical_persec(&mut self, value: u64) {
        self.updates_critical_persec = Some(value);
    }

    /// Gets the value of UpdatesCriticalPersec
    pub fn get_updates_critical_persec(&self) -> Option<&u64> {
        self.updates_critical_persec.as_ref()
    }

    /// Sets the value of UpdatesNonCritical
    pub fn set_updates_non_critical(&mut self, value: u64) {
        self.updates_non_critical = Some(value);
    }

    /// Gets the value of UpdatesNonCritical
    pub fn get_updates_non_critical(&self) -> Option<&u64> {
        self.updates_non_critical.as_ref()
    }

    /// Sets the value of UpdatesNonCriticalLogFull
    pub fn set_updates_non_critical_log_full(&mut self, value: u64) {
        self.updates_non_critical_log_full = Some(value);
    }

    /// Gets the value of UpdatesNonCriticalLogFull
    pub fn get_updates_non_critical_log_full(&self) -> Option<&u64> {
        self.updates_non_critical_log_full.as_ref()
    }

    /// Sets the value of UpdatesNonCriticalPersec
    pub fn set_updates_non_critical_persec(&mut self, value: u64) {
        self.updates_non_critical_persec = Some(value);
    }

    /// Gets the value of UpdatesNonCriticalPersec
    pub fn get_updates_non_critical_persec(&self) -> Option<&u64> {
        self.updates_non_critical_persec.as_ref()
    }

    /// Sets the value of UpdatesNotCommitted
    pub fn set_updates_not_committed(&mut self, value: u64) {
        self.updates_not_committed = Some(value);
    }

    /// Gets the value of UpdatesNotCommitted
    pub fn get_updates_not_committed(&self) -> Option<&u64> {
        self.updates_not_committed.as_ref()
    }

    /// Sets the value of UpdateTransfers
    pub fn set_update_transfers(&mut self, value: u64) {
        self.update_transfers = Some(value);
    }

    /// Gets the value of UpdateTransfers
    pub fn get_update_transfers(&self) -> Option<&u64> {
        self.update_transfers.as_ref()
    }

    /// Sets the value of UpdateTransfersPersec
    pub fn set_update_transfers_persec(&mut self, value: u64) {
        self.update_transfers_persec = Some(value);
    }

    /// Gets the value of UpdateTransfersPersec
    pub fn get_update_transfers_persec(&self) -> Option<&u64> {
        self.update_transfers_persec.as_ref()
    }

    /// Sets the value of WriteErrorsMedia
    pub fn set_write_errors_media(&mut self, value: u64) {
        self.write_errors_media = Some(value);
    }

    /// Gets the value of WriteErrorsMedia
    pub fn get_write_errors_media(&self) -> Option<&u64> {
        self.write_errors_media.as_ref()
    }

    /// Sets the value of WriteErrorsTimeout
    pub fn set_write_errors_timeout(&mut self, value: u64) {
        self.write_errors_timeout = Some(value);
    }

    /// Gets the value of WriteErrorsTimeout
    pub fn get_write_errors_timeout(&self) -> Option<&u64> {
        self.write_errors_timeout.as_ref()
    }

    /// Sets the value of WriteErrorsTotal
    pub fn set_write_errors_total(&mut self, value: u64) {
        self.write_errors_total = Some(value);
    }

    /// Gets the value of WriteErrorsTotal
    pub fn get_write_errors_total(&self) -> Option<&u64> {
        self.write_errors_total.as_ref()
    }
}

