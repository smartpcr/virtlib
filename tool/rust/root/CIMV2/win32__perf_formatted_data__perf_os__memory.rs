// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_PerfOS_Memory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_PerfOS_Memory {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AvailableBytes")]
    pub available_bytes: Option<u64>,

/// 
    #[serde(rename = "AvailableKBytes")]
    pub available_kbytes: Option<u64>,

/// 
    #[serde(rename = "AvailableMBytes")]
    pub available_mbytes: Option<u64>,

/// 
    #[serde(rename = "CacheBytes")]
    pub cache_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheBytesPeak")]
    pub cache_bytes_peak: Option<u64>,

/// 
    #[serde(rename = "CacheFaultsPersec")]
    pub cache_faults_persec: Option<u32>,

/// 
    #[serde(rename = "CommitLimit")]
    pub commit_limit: Option<u64>,

/// 
    #[serde(rename = "CommittedBytes")]
    pub committed_bytes: Option<u64>,

/// 
    #[serde(rename = "DemandZeroFaultsPersec")]
    pub demand_zero_faults_persec: Option<u32>,

/// 
    #[serde(rename = "FreeAndZeroPageListBytes")]
    pub free_and_zero_page_list_bytes: Option<u64>,

/// 
    #[serde(rename = "FreeSystemPageTableEntries")]
    pub free_system_page_table_entries: Option<u32>,

/// 
    #[serde(rename = "LongTermAverageStandbyCacheLifetimes")]
    pub long_term_average_standby_cache_lifetimes: Option<u32>,

/// 
    #[serde(rename = "ModifiedPageListBytes")]
    pub modified_page_list_bytes: Option<u64>,

/// 
    #[serde(rename = "PageFaultsPersec")]
    pub page_faults_persec: Option<u32>,

/// 
    #[serde(rename = "PageReadsPersec")]
    pub page_reads_persec: Option<u32>,

/// 
    #[serde(rename = "PagesInputPersec")]
    pub pages_input_persec: Option<u32>,

/// 
    #[serde(rename = "PagesOutputPersec")]
    pub pages_output_persec: Option<u32>,

/// 
    #[serde(rename = "PagesPersec")]
    pub pages_persec: Option<u32>,

/// 
    #[serde(rename = "PageWritesPersec")]
    pub page_writes_persec: Option<u32>,

/// 
    #[serde(rename = "PercentCommittedBytesInUse")]
    pub percent_committed_bytes_in_use: Option<u32>,

/// 
    #[serde(rename = "PoolNonpagedAllocs")]
    pub pool_nonpaged_allocs: Option<u32>,

/// 
    #[serde(rename = "PoolNonpagedBytes")]
    pub pool_nonpaged_bytes: Option<u64>,

/// 
    #[serde(rename = "PoolPagedAllocs")]
    pub pool_paged_allocs: Option<u32>,

/// 
    #[serde(rename = "PoolPagedBytes")]
    pub pool_paged_bytes: Option<u64>,

/// 
    #[serde(rename = "PoolPagedResidentBytes")]
    pub pool_paged_resident_bytes: Option<u64>,

/// 
    #[serde(rename = "StandbyCacheCoreBytes")]
    pub standby_cache_core_bytes: Option<u64>,

/// 
    #[serde(rename = "StandbyCacheNormalPriorityBytes")]
    pub standby_cache_normal_priority_bytes: Option<u64>,

/// 
    #[serde(rename = "StandbyCacheReserveBytes")]
    pub standby_cache_reserve_bytes: Option<u64>,

/// 
    #[serde(rename = "SystemCacheResidentBytes")]
    pub system_cache_resident_bytes: Option<u64>,

/// 
    #[serde(rename = "SystemCodeResidentBytes")]
    pub system_code_resident_bytes: Option<u64>,

/// 
    #[serde(rename = "SystemCodeTotalBytes")]
    pub system_code_total_bytes: Option<u64>,

/// 
    #[serde(rename = "SystemDriverResidentBytes")]
    pub system_driver_resident_bytes: Option<u64>,

/// 
    #[serde(rename = "SystemDriverTotalBytes")]
    pub system_driver_total_bytes: Option<u64>,

/// 
    #[serde(rename = "TransitionFaultsPersec")]
    pub transition_faults_persec: Option<u32>,

/// 
    #[serde(rename = "TransitionPagesRePurposedPersec")]
    pub transition_pages_re_purposed_persec: Option<u32>,

/// 
    #[serde(rename = "WriteCopiesPersec")]
    pub write_copies_persec: Option<u32>,
}

impl Win32_PerfFormattedData_PerfOS_Memory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            available_bytes: None,
            available_kbytes: None,
            available_mbytes: None,
            cache_bytes: None,
            cache_bytes_peak: None,
            cache_faults_persec: None,
            commit_limit: None,
            committed_bytes: None,
            demand_zero_faults_persec: None,
            free_and_zero_page_list_bytes: None,
            free_system_page_table_entries: None,
            long_term_average_standby_cache_lifetimes: None,
            modified_page_list_bytes: None,
            page_faults_persec: None,
            page_reads_persec: None,
            pages_input_persec: None,
            pages_output_persec: None,
            pages_persec: None,
            page_writes_persec: None,
            percent_committed_bytes_in_use: None,
            pool_nonpaged_allocs: None,
            pool_nonpaged_bytes: None,
            pool_paged_allocs: None,
            pool_paged_bytes: None,
            pool_paged_resident_bytes: None,
            standby_cache_core_bytes: None,
            standby_cache_normal_priority_bytes: None,
            standby_cache_reserve_bytes: None,
            system_cache_resident_bytes: None,
            system_code_resident_bytes: None,
            system_code_total_bytes: None,
            system_driver_resident_bytes: None,
            system_driver_total_bytes: None,
            transition_faults_persec: None,
            transition_pages_re_purposed_persec: None,
            write_copies_persec: None,
        }
    }


    /// Sets the value of AvailableBytes
    pub fn set_available_bytes(&mut self, value: u64) {
        self.available_bytes = Some(value);
    }

    /// Gets the value of AvailableBytes
    pub fn get_available_bytes(&self) -> Option<&u64> {
        self.available_bytes.as_ref()
    }

    /// Sets the value of AvailableKBytes
    pub fn set_available_kbytes(&mut self, value: u64) {
        self.available_kbytes = Some(value);
    }

    /// Gets the value of AvailableKBytes
    pub fn get_available_kbytes(&self) -> Option<&u64> {
        self.available_kbytes.as_ref()
    }

    /// Sets the value of AvailableMBytes
    pub fn set_available_mbytes(&mut self, value: u64) {
        self.available_mbytes = Some(value);
    }

    /// Gets the value of AvailableMBytes
    pub fn get_available_mbytes(&self) -> Option<&u64> {
        self.available_mbytes.as_ref()
    }

    /// Sets the value of CacheBytes
    pub fn set_cache_bytes(&mut self, value: u64) {
        self.cache_bytes = Some(value);
    }

    /// Gets the value of CacheBytes
    pub fn get_cache_bytes(&self) -> Option<&u64> {
        self.cache_bytes.as_ref()
    }

    /// Sets the value of CacheBytesPeak
    pub fn set_cache_bytes_peak(&mut self, value: u64) {
        self.cache_bytes_peak = Some(value);
    }

    /// Gets the value of CacheBytesPeak
    pub fn get_cache_bytes_peak(&self) -> Option<&u64> {
        self.cache_bytes_peak.as_ref()
    }

    /// Sets the value of CacheFaultsPersec
    pub fn set_cache_faults_persec(&mut self, value: u32) {
        self.cache_faults_persec = Some(value);
    }

    /// Gets the value of CacheFaultsPersec
    pub fn get_cache_faults_persec(&self) -> Option<&u32> {
        self.cache_faults_persec.as_ref()
    }

    /// Sets the value of CommitLimit
    pub fn set_commit_limit(&mut self, value: u64) {
        self.commit_limit = Some(value);
    }

    /// Gets the value of CommitLimit
    pub fn get_commit_limit(&self) -> Option<&u64> {
        self.commit_limit.as_ref()
    }

    /// Sets the value of CommittedBytes
    pub fn set_committed_bytes(&mut self, value: u64) {
        self.committed_bytes = Some(value);
    }

    /// Gets the value of CommittedBytes
    pub fn get_committed_bytes(&self) -> Option<&u64> {
        self.committed_bytes.as_ref()
    }

    /// Sets the value of DemandZeroFaultsPersec
    pub fn set_demand_zero_faults_persec(&mut self, value: u32) {
        self.demand_zero_faults_persec = Some(value);
    }

    /// Gets the value of DemandZeroFaultsPersec
    pub fn get_demand_zero_faults_persec(&self) -> Option<&u32> {
        self.demand_zero_faults_persec.as_ref()
    }

    /// Sets the value of FreeAndZeroPageListBytes
    pub fn set_free_and_zero_page_list_bytes(&mut self, value: u64) {
        self.free_and_zero_page_list_bytes = Some(value);
    }

    /// Gets the value of FreeAndZeroPageListBytes
    pub fn get_free_and_zero_page_list_bytes(&self) -> Option<&u64> {
        self.free_and_zero_page_list_bytes.as_ref()
    }

    /// Sets the value of FreeSystemPageTableEntries
    pub fn set_free_system_page_table_entries(&mut self, value: u32) {
        self.free_system_page_table_entries = Some(value);
    }

    /// Gets the value of FreeSystemPageTableEntries
    pub fn get_free_system_page_table_entries(&self) -> Option<&u32> {
        self.free_system_page_table_entries.as_ref()
    }

    /// Sets the value of LongTermAverageStandbyCacheLifetimes
    pub fn set_long_term_average_standby_cache_lifetimes(&mut self, value: u32) {
        self.long_term_average_standby_cache_lifetimes = Some(value);
    }

    /// Gets the value of LongTermAverageStandbyCacheLifetimes
    pub fn get_long_term_average_standby_cache_lifetimes(&self) -> Option<&u32> {
        self.long_term_average_standby_cache_lifetimes.as_ref()
    }

    /// Sets the value of ModifiedPageListBytes
    pub fn set_modified_page_list_bytes(&mut self, value: u64) {
        self.modified_page_list_bytes = Some(value);
    }

    /// Gets the value of ModifiedPageListBytes
    pub fn get_modified_page_list_bytes(&self) -> Option<&u64> {
        self.modified_page_list_bytes.as_ref()
    }

    /// Sets the value of PageFaultsPersec
    pub fn set_page_faults_persec(&mut self, value: u32) {
        self.page_faults_persec = Some(value);
    }

    /// Gets the value of PageFaultsPersec
    pub fn get_page_faults_persec(&self) -> Option<&u32> {
        self.page_faults_persec.as_ref()
    }

    /// Sets the value of PageReadsPersec
    pub fn set_page_reads_persec(&mut self, value: u32) {
        self.page_reads_persec = Some(value);
    }

    /// Gets the value of PageReadsPersec
    pub fn get_page_reads_persec(&self) -> Option<&u32> {
        self.page_reads_persec.as_ref()
    }

    /// Sets the value of PagesInputPersec
    pub fn set_pages_input_persec(&mut self, value: u32) {
        self.pages_input_persec = Some(value);
    }

    /// Gets the value of PagesInputPersec
    pub fn get_pages_input_persec(&self) -> Option<&u32> {
        self.pages_input_persec.as_ref()
    }

    /// Sets the value of PagesOutputPersec
    pub fn set_pages_output_persec(&mut self, value: u32) {
        self.pages_output_persec = Some(value);
    }

    /// Gets the value of PagesOutputPersec
    pub fn get_pages_output_persec(&self) -> Option<&u32> {
        self.pages_output_persec.as_ref()
    }

    /// Sets the value of PagesPersec
    pub fn set_pages_persec(&mut self, value: u32) {
        self.pages_persec = Some(value);
    }

    /// Gets the value of PagesPersec
    pub fn get_pages_persec(&self) -> Option<&u32> {
        self.pages_persec.as_ref()
    }

    /// Sets the value of PageWritesPersec
    pub fn set_page_writes_persec(&mut self, value: u32) {
        self.page_writes_persec = Some(value);
    }

    /// Gets the value of PageWritesPersec
    pub fn get_page_writes_persec(&self) -> Option<&u32> {
        self.page_writes_persec.as_ref()
    }

    /// Sets the value of PercentCommittedBytesInUse
    pub fn set_percent_committed_bytes_in_use(&mut self, value: u32) {
        self.percent_committed_bytes_in_use = Some(value);
    }

    /// Gets the value of PercentCommittedBytesInUse
    pub fn get_percent_committed_bytes_in_use(&self) -> Option<&u32> {
        self.percent_committed_bytes_in_use.as_ref()
    }

    /// Sets the value of PoolNonpagedAllocs
    pub fn set_pool_nonpaged_allocs(&mut self, value: u32) {
        self.pool_nonpaged_allocs = Some(value);
    }

    /// Gets the value of PoolNonpagedAllocs
    pub fn get_pool_nonpaged_allocs(&self) -> Option<&u32> {
        self.pool_nonpaged_allocs.as_ref()
    }

    /// Sets the value of PoolNonpagedBytes
    pub fn set_pool_nonpaged_bytes(&mut self, value: u64) {
        self.pool_nonpaged_bytes = Some(value);
    }

    /// Gets the value of PoolNonpagedBytes
    pub fn get_pool_nonpaged_bytes(&self) -> Option<&u64> {
        self.pool_nonpaged_bytes.as_ref()
    }

    /// Sets the value of PoolPagedAllocs
    pub fn set_pool_paged_allocs(&mut self, value: u32) {
        self.pool_paged_allocs = Some(value);
    }

    /// Gets the value of PoolPagedAllocs
    pub fn get_pool_paged_allocs(&self) -> Option<&u32> {
        self.pool_paged_allocs.as_ref()
    }

    /// Sets the value of PoolPagedBytes
    pub fn set_pool_paged_bytes(&mut self, value: u64) {
        self.pool_paged_bytes = Some(value);
    }

    /// Gets the value of PoolPagedBytes
    pub fn get_pool_paged_bytes(&self) -> Option<&u64> {
        self.pool_paged_bytes.as_ref()
    }

    /// Sets the value of PoolPagedResidentBytes
    pub fn set_pool_paged_resident_bytes(&mut self, value: u64) {
        self.pool_paged_resident_bytes = Some(value);
    }

    /// Gets the value of PoolPagedResidentBytes
    pub fn get_pool_paged_resident_bytes(&self) -> Option<&u64> {
        self.pool_paged_resident_bytes.as_ref()
    }

    /// Sets the value of StandbyCacheCoreBytes
    pub fn set_standby_cache_core_bytes(&mut self, value: u64) {
        self.standby_cache_core_bytes = Some(value);
    }

    /// Gets the value of StandbyCacheCoreBytes
    pub fn get_standby_cache_core_bytes(&self) -> Option<&u64> {
        self.standby_cache_core_bytes.as_ref()
    }

    /// Sets the value of StandbyCacheNormalPriorityBytes
    pub fn set_standby_cache_normal_priority_bytes(&mut self, value: u64) {
        self.standby_cache_normal_priority_bytes = Some(value);
    }

    /// Gets the value of StandbyCacheNormalPriorityBytes
    pub fn get_standby_cache_normal_priority_bytes(&self) -> Option<&u64> {
        self.standby_cache_normal_priority_bytes.as_ref()
    }

    /// Sets the value of StandbyCacheReserveBytes
    pub fn set_standby_cache_reserve_bytes(&mut self, value: u64) {
        self.standby_cache_reserve_bytes = Some(value);
    }

    /// Gets the value of StandbyCacheReserveBytes
    pub fn get_standby_cache_reserve_bytes(&self) -> Option<&u64> {
        self.standby_cache_reserve_bytes.as_ref()
    }

    /// Sets the value of SystemCacheResidentBytes
    pub fn set_system_cache_resident_bytes(&mut self, value: u64) {
        self.system_cache_resident_bytes = Some(value);
    }

    /// Gets the value of SystemCacheResidentBytes
    pub fn get_system_cache_resident_bytes(&self) -> Option<&u64> {
        self.system_cache_resident_bytes.as_ref()
    }

    /// Sets the value of SystemCodeResidentBytes
    pub fn set_system_code_resident_bytes(&mut self, value: u64) {
        self.system_code_resident_bytes = Some(value);
    }

    /// Gets the value of SystemCodeResidentBytes
    pub fn get_system_code_resident_bytes(&self) -> Option<&u64> {
        self.system_code_resident_bytes.as_ref()
    }

    /// Sets the value of SystemCodeTotalBytes
    pub fn set_system_code_total_bytes(&mut self, value: u64) {
        self.system_code_total_bytes = Some(value);
    }

    /// Gets the value of SystemCodeTotalBytes
    pub fn get_system_code_total_bytes(&self) -> Option<&u64> {
        self.system_code_total_bytes.as_ref()
    }

    /// Sets the value of SystemDriverResidentBytes
    pub fn set_system_driver_resident_bytes(&mut self, value: u64) {
        self.system_driver_resident_bytes = Some(value);
    }

    /// Gets the value of SystemDriverResidentBytes
    pub fn get_system_driver_resident_bytes(&self) -> Option<&u64> {
        self.system_driver_resident_bytes.as_ref()
    }

    /// Sets the value of SystemDriverTotalBytes
    pub fn set_system_driver_total_bytes(&mut self, value: u64) {
        self.system_driver_total_bytes = Some(value);
    }

    /// Gets the value of SystemDriverTotalBytes
    pub fn get_system_driver_total_bytes(&self) -> Option<&u64> {
        self.system_driver_total_bytes.as_ref()
    }

    /// Sets the value of TransitionFaultsPersec
    pub fn set_transition_faults_persec(&mut self, value: u32) {
        self.transition_faults_persec = Some(value);
    }

    /// Gets the value of TransitionFaultsPersec
    pub fn get_transition_faults_persec(&self) -> Option<&u32> {
        self.transition_faults_persec.as_ref()
    }

    /// Sets the value of TransitionPagesRePurposedPersec
    pub fn set_transition_pages_re_purposed_persec(&mut self, value: u32) {
        self.transition_pages_re_purposed_persec = Some(value);
    }

    /// Gets the value of TransitionPagesRePurposedPersec
    pub fn get_transition_pages_re_purposed_persec(&self) -> Option<&u32> {
        self.transition_pages_re_purposed_persec.as_ref()
    }

    /// Sets the value of WriteCopiesPersec
    pub fn set_write_copies_persec(&mut self, value: u32) {
        self.write_copies_persec = Some(value);
    }

    /// Gets the value of WriteCopiesPersec
    pub fn get_write_copies_persec(&self) -> Option<&u32> {
        self.write_copies_persec.as_ref()
    }
}

