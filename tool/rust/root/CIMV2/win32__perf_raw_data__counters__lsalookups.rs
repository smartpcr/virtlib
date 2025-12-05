// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_LSALookups struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_LSALookups {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "IsolatedNamesInboundRequestsPersec")]
    pub isolated_names_inbound_requests_persec: Option<u32>,

/// 
    #[serde(rename = "IsolatedNamesOutboundRequestsPersec")]
    pub isolated_names_outbound_requests_persec: Option<u32>,

/// 
    #[serde(rename = "NamesCachePercentFull")]
    pub names_cache_percent_full: Option<u32>,

/// 
    #[serde(rename = "NamesCachePercentFull_Base")]
    pub names_cache_percent_full__base: Option<u32>,

/// 
    #[serde(rename = "NamesCachePercentHit")]
    pub names_cache_percent_hit: Option<u32>,

/// 
    #[serde(rename = "NamesCachePercentHit_Base")]
    pub names_cache_percent_hit__base: Option<u32>,

/// 
    #[serde(rename = "NamesCompletionTime")]
    pub names_completion_time: Option<u32>,

/// 
    #[serde(rename = "NamesCompletionTime_Base")]
    pub names_completion_time__base: Option<u32>,

/// 
    #[serde(rename = "NamesErrorsPersec")]
    pub names_errors_persec: Option<u32>,

/// 
    #[serde(rename = "NameSIDcacheentriesaddedPersec")]
    pub name_sidcacheentriesadded_persec: Option<u32>,

/// 
    #[serde(rename = "NameSIDcacheentriespurgedPersec")]
    pub name_sidcacheentriespurged_persec: Option<u32>,

/// 
    #[serde(rename = "NameSIDCacheSizeMaxEntries")]
    pub name_sidcache_size_max_entries: Option<u32>,

/// 
    #[serde(rename = "NamesInboundRequestsPersec")]
    pub names_inbound_requests_persec: Option<u32>,

/// 
    #[serde(rename = "NamesOutboundRequestsPersec")]
    pub names_outbound_requests_persec: Option<u32>,

/// 
    #[serde(rename = "NamesPrimaryDomainRequestsPersec")]
    pub names_primary_domain_requests_persec: Option<u32>,

/// 
    #[serde(rename = "NamesPrimaryDomainTime")]
    pub names_primary_domain_time: Option<u32>,

/// 
    #[serde(rename = "NamesPrimaryDomainTime_Base")]
    pub names_primary_domain_time__base: Option<u32>,

/// 
    #[serde(rename = "NamesRemoteRequestTime")]
    pub names_remote_request_time: Option<u32>,

/// 
    #[serde(rename = "NamesRemoteRequestTime_Base")]
    pub names_remote_request_time__base: Option<u32>,

/// 
    #[serde(rename = "NamesTrustedDomainRequestsPersec")]
    pub names_trusted_domain_requests_persec: Option<u32>,

/// 
    #[serde(rename = "NamesTrustedDomainRequestTime")]
    pub names_trusted_domain_request_time: Option<u32>,

/// 
    #[serde(rename = "NamesTrustedDomainRequestTime_Base")]
    pub names_trusted_domain_request_time__base: Option<u32>,

/// 
    #[serde(rename = "NamesUnresolvedPersec")]
    pub names_unresolved_persec: Option<u32>,

/// 
    #[serde(rename = "NamesXforestRequestsPersec")]
    pub names_xforest_requests_persec: Option<u32>,

/// 
    #[serde(rename = "NamesXforestTime")]
    pub names_xforest_time: Option<u32>,

/// 
    #[serde(rename = "NamesXforestTime_Base")]
    pub names_xforest_time__base: Option<u32>,

/// 
    #[serde(rename = "SIDsCachePercentFull")]
    pub sids_cache_percent_full: Option<u32>,

/// 
    #[serde(rename = "SIDsCachePercentFull_Base")]
    pub sids_cache_percent_full__base: Option<u32>,

/// 
    #[serde(rename = "SIDsCachePercentHit")]
    pub sids_cache_percent_hit: Option<u32>,

/// 
    #[serde(rename = "SIDsCachePercentHit_Base")]
    pub sids_cache_percent_hit__base: Option<u32>,

/// 
    #[serde(rename = "SIDsCompletionTime")]
    pub sids_completion_time: Option<u32>,

/// 
    #[serde(rename = "SIDsCompletionTime_Base")]
    pub sids_completion_time__base: Option<u32>,

/// 
    #[serde(rename = "SIDsErrorsPersec")]
    pub sids_errors_persec: Option<u32>,

/// 
    #[serde(rename = "SIDsInboundRequestsPersec")]
    pub sids_inbound_requests_persec: Option<u32>,

/// 
    #[serde(rename = "SIDsOutboundRequestsPersec")]
    pub sids_outbound_requests_persec: Option<u32>,

/// 
    #[serde(rename = "SIDsPrimaryDomainRequestsPersec")]
    pub sids_primary_domain_requests_persec: Option<u32>,

/// 
    #[serde(rename = "SIDsPrimaryDomainRequestTime")]
    pub sids_primary_domain_request_time: Option<u32>,

/// 
    #[serde(rename = "SIDsPrimaryDomainRequestTime_Base")]
    pub sids_primary_domain_request_time__base: Option<u32>,

/// 
    #[serde(rename = "SIDsRemoteRequestTime")]
    pub sids_remote_request_time: Option<u32>,

/// 
    #[serde(rename = "SIDsRemoteRequestTime_Base")]
    pub sids_remote_request_time__base: Option<u32>,

/// 
    #[serde(rename = "SIDsTrustedDomainRequestsPersec")]
    pub sids_trusted_domain_requests_persec: Option<u32>,

/// 
    #[serde(rename = "SIDsTrustedDomainRequestTime")]
    pub sids_trusted_domain_request_time: Option<u32>,

/// 
    #[serde(rename = "SIDsTrustedDomainRequestTime_Base")]
    pub sids_trusted_domain_request_time__base: Option<u32>,

/// 
    #[serde(rename = "SIDsUnresolvedPersec")]
    pub sids_unresolved_persec: Option<u32>,

/// 
    #[serde(rename = "SIDsXforestRequestsPersec")]
    pub sids_xforest_requests_persec: Option<u32>,

/// 
    #[serde(rename = "SIDsXforestRequestTime")]
    pub sids_xforest_request_time: Option<u32>,

/// 
    #[serde(rename = "SIDsXforestRequestTime_Base")]
    pub sids_xforest_request_time__base: Option<u32>,
}

impl Win32_PerfRawData_Counters_LSALookups {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            isolated_names_inbound_requests_persec: None,
            isolated_names_outbound_requests_persec: None,
            names_cache_percent_full: None,
            names_cache_percent_full__base: None,
            names_cache_percent_hit: None,
            names_cache_percent_hit__base: None,
            names_completion_time: None,
            names_completion_time__base: None,
            names_errors_persec: None,
            name_sidcacheentriesadded_persec: None,
            name_sidcacheentriespurged_persec: None,
            name_sidcache_size_max_entries: None,
            names_inbound_requests_persec: None,
            names_outbound_requests_persec: None,
            names_primary_domain_requests_persec: None,
            names_primary_domain_time: None,
            names_primary_domain_time__base: None,
            names_remote_request_time: None,
            names_remote_request_time__base: None,
            names_trusted_domain_requests_persec: None,
            names_trusted_domain_request_time: None,
            names_trusted_domain_request_time__base: None,
            names_unresolved_persec: None,
            names_xforest_requests_persec: None,
            names_xforest_time: None,
            names_xforest_time__base: None,
            sids_cache_percent_full: None,
            sids_cache_percent_full__base: None,
            sids_cache_percent_hit: None,
            sids_cache_percent_hit__base: None,
            sids_completion_time: None,
            sids_completion_time__base: None,
            sids_errors_persec: None,
            sids_inbound_requests_persec: None,
            sids_outbound_requests_persec: None,
            sids_primary_domain_requests_persec: None,
            sids_primary_domain_request_time: None,
            sids_primary_domain_request_time__base: None,
            sids_remote_request_time: None,
            sids_remote_request_time__base: None,
            sids_trusted_domain_requests_persec: None,
            sids_trusted_domain_request_time: None,
            sids_trusted_domain_request_time__base: None,
            sids_unresolved_persec: None,
            sids_xforest_requests_persec: None,
            sids_xforest_request_time: None,
            sids_xforest_request_time__base: None,
        }
    }


    /// Sets the value of IsolatedNamesInboundRequestsPersec
    pub fn set_isolated_names_inbound_requests_persec(&mut self, value: u32) {
        self.isolated_names_inbound_requests_persec = Some(value);
    }

    /// Gets the value of IsolatedNamesInboundRequestsPersec
    pub fn get_isolated_names_inbound_requests_persec(&self) -> Option<&u32> {
        self.isolated_names_inbound_requests_persec.as_ref()
    }

    /// Sets the value of IsolatedNamesOutboundRequestsPersec
    pub fn set_isolated_names_outbound_requests_persec(&mut self, value: u32) {
        self.isolated_names_outbound_requests_persec = Some(value);
    }

    /// Gets the value of IsolatedNamesOutboundRequestsPersec
    pub fn get_isolated_names_outbound_requests_persec(&self) -> Option<&u32> {
        self.isolated_names_outbound_requests_persec.as_ref()
    }

    /// Sets the value of NamesCachePercentFull
    pub fn set_names_cache_percent_full(&mut self, value: u32) {
        self.names_cache_percent_full = Some(value);
    }

    /// Gets the value of NamesCachePercentFull
    pub fn get_names_cache_percent_full(&self) -> Option<&u32> {
        self.names_cache_percent_full.as_ref()
    }

    /// Sets the value of NamesCachePercentFull_Base
    pub fn set_names_cache_percent_full__base(&mut self, value: u32) {
        self.names_cache_percent_full__base = Some(value);
    }

    /// Gets the value of NamesCachePercentFull_Base
    pub fn get_names_cache_percent_full__base(&self) -> Option<&u32> {
        self.names_cache_percent_full__base.as_ref()
    }

    /// Sets the value of NamesCachePercentHit
    pub fn set_names_cache_percent_hit(&mut self, value: u32) {
        self.names_cache_percent_hit = Some(value);
    }

    /// Gets the value of NamesCachePercentHit
    pub fn get_names_cache_percent_hit(&self) -> Option<&u32> {
        self.names_cache_percent_hit.as_ref()
    }

    /// Sets the value of NamesCachePercentHit_Base
    pub fn set_names_cache_percent_hit__base(&mut self, value: u32) {
        self.names_cache_percent_hit__base = Some(value);
    }

    /// Gets the value of NamesCachePercentHit_Base
    pub fn get_names_cache_percent_hit__base(&self) -> Option<&u32> {
        self.names_cache_percent_hit__base.as_ref()
    }

    /// Sets the value of NamesCompletionTime
    pub fn set_names_completion_time(&mut self, value: u32) {
        self.names_completion_time = Some(value);
    }

    /// Gets the value of NamesCompletionTime
    pub fn get_names_completion_time(&self) -> Option<&u32> {
        self.names_completion_time.as_ref()
    }

    /// Sets the value of NamesCompletionTime_Base
    pub fn set_names_completion_time__base(&mut self, value: u32) {
        self.names_completion_time__base = Some(value);
    }

    /// Gets the value of NamesCompletionTime_Base
    pub fn get_names_completion_time__base(&self) -> Option<&u32> {
        self.names_completion_time__base.as_ref()
    }

    /// Sets the value of NamesErrorsPersec
    pub fn set_names_errors_persec(&mut self, value: u32) {
        self.names_errors_persec = Some(value);
    }

    /// Gets the value of NamesErrorsPersec
    pub fn get_names_errors_persec(&self) -> Option<&u32> {
        self.names_errors_persec.as_ref()
    }

    /// Sets the value of NameSIDcacheentriesaddedPersec
    pub fn set_name_sidcacheentriesadded_persec(&mut self, value: u32) {
        self.name_sidcacheentriesadded_persec = Some(value);
    }

    /// Gets the value of NameSIDcacheentriesaddedPersec
    pub fn get_name_sidcacheentriesadded_persec(&self) -> Option<&u32> {
        self.name_sidcacheentriesadded_persec.as_ref()
    }

    /// Sets the value of NameSIDcacheentriespurgedPersec
    pub fn set_name_sidcacheentriespurged_persec(&mut self, value: u32) {
        self.name_sidcacheentriespurged_persec = Some(value);
    }

    /// Gets the value of NameSIDcacheentriespurgedPersec
    pub fn get_name_sidcacheentriespurged_persec(&self) -> Option<&u32> {
        self.name_sidcacheentriespurged_persec.as_ref()
    }

    /// Sets the value of NameSIDCacheSizeMaxEntries
    pub fn set_name_sidcache_size_max_entries(&mut self, value: u32) {
        self.name_sidcache_size_max_entries = Some(value);
    }

    /// Gets the value of NameSIDCacheSizeMaxEntries
    pub fn get_name_sidcache_size_max_entries(&self) -> Option<&u32> {
        self.name_sidcache_size_max_entries.as_ref()
    }

    /// Sets the value of NamesInboundRequestsPersec
    pub fn set_names_inbound_requests_persec(&mut self, value: u32) {
        self.names_inbound_requests_persec = Some(value);
    }

    /// Gets the value of NamesInboundRequestsPersec
    pub fn get_names_inbound_requests_persec(&self) -> Option<&u32> {
        self.names_inbound_requests_persec.as_ref()
    }

    /// Sets the value of NamesOutboundRequestsPersec
    pub fn set_names_outbound_requests_persec(&mut self, value: u32) {
        self.names_outbound_requests_persec = Some(value);
    }

    /// Gets the value of NamesOutboundRequestsPersec
    pub fn get_names_outbound_requests_persec(&self) -> Option<&u32> {
        self.names_outbound_requests_persec.as_ref()
    }

    /// Sets the value of NamesPrimaryDomainRequestsPersec
    pub fn set_names_primary_domain_requests_persec(&mut self, value: u32) {
        self.names_primary_domain_requests_persec = Some(value);
    }

    /// Gets the value of NamesPrimaryDomainRequestsPersec
    pub fn get_names_primary_domain_requests_persec(&self) -> Option<&u32> {
        self.names_primary_domain_requests_persec.as_ref()
    }

    /// Sets the value of NamesPrimaryDomainTime
    pub fn set_names_primary_domain_time(&mut self, value: u32) {
        self.names_primary_domain_time = Some(value);
    }

    /// Gets the value of NamesPrimaryDomainTime
    pub fn get_names_primary_domain_time(&self) -> Option<&u32> {
        self.names_primary_domain_time.as_ref()
    }

    /// Sets the value of NamesPrimaryDomainTime_Base
    pub fn set_names_primary_domain_time__base(&mut self, value: u32) {
        self.names_primary_domain_time__base = Some(value);
    }

    /// Gets the value of NamesPrimaryDomainTime_Base
    pub fn get_names_primary_domain_time__base(&self) -> Option<&u32> {
        self.names_primary_domain_time__base.as_ref()
    }

    /// Sets the value of NamesRemoteRequestTime
    pub fn set_names_remote_request_time(&mut self, value: u32) {
        self.names_remote_request_time = Some(value);
    }

    /// Gets the value of NamesRemoteRequestTime
    pub fn get_names_remote_request_time(&self) -> Option<&u32> {
        self.names_remote_request_time.as_ref()
    }

    /// Sets the value of NamesRemoteRequestTime_Base
    pub fn set_names_remote_request_time__base(&mut self, value: u32) {
        self.names_remote_request_time__base = Some(value);
    }

    /// Gets the value of NamesRemoteRequestTime_Base
    pub fn get_names_remote_request_time__base(&self) -> Option<&u32> {
        self.names_remote_request_time__base.as_ref()
    }

    /// Sets the value of NamesTrustedDomainRequestsPersec
    pub fn set_names_trusted_domain_requests_persec(&mut self, value: u32) {
        self.names_trusted_domain_requests_persec = Some(value);
    }

    /// Gets the value of NamesTrustedDomainRequestsPersec
    pub fn get_names_trusted_domain_requests_persec(&self) -> Option<&u32> {
        self.names_trusted_domain_requests_persec.as_ref()
    }

    /// Sets the value of NamesTrustedDomainRequestTime
    pub fn set_names_trusted_domain_request_time(&mut self, value: u32) {
        self.names_trusted_domain_request_time = Some(value);
    }

    /// Gets the value of NamesTrustedDomainRequestTime
    pub fn get_names_trusted_domain_request_time(&self) -> Option<&u32> {
        self.names_trusted_domain_request_time.as_ref()
    }

    /// Sets the value of NamesTrustedDomainRequestTime_Base
    pub fn set_names_trusted_domain_request_time__base(&mut self, value: u32) {
        self.names_trusted_domain_request_time__base = Some(value);
    }

    /// Gets the value of NamesTrustedDomainRequestTime_Base
    pub fn get_names_trusted_domain_request_time__base(&self) -> Option<&u32> {
        self.names_trusted_domain_request_time__base.as_ref()
    }

    /// Sets the value of NamesUnresolvedPersec
    pub fn set_names_unresolved_persec(&mut self, value: u32) {
        self.names_unresolved_persec = Some(value);
    }

    /// Gets the value of NamesUnresolvedPersec
    pub fn get_names_unresolved_persec(&self) -> Option<&u32> {
        self.names_unresolved_persec.as_ref()
    }

    /// Sets the value of NamesXforestRequestsPersec
    pub fn set_names_xforest_requests_persec(&mut self, value: u32) {
        self.names_xforest_requests_persec = Some(value);
    }

    /// Gets the value of NamesXforestRequestsPersec
    pub fn get_names_xforest_requests_persec(&self) -> Option<&u32> {
        self.names_xforest_requests_persec.as_ref()
    }

    /// Sets the value of NamesXforestTime
    pub fn set_names_xforest_time(&mut self, value: u32) {
        self.names_xforest_time = Some(value);
    }

    /// Gets the value of NamesXforestTime
    pub fn get_names_xforest_time(&self) -> Option<&u32> {
        self.names_xforest_time.as_ref()
    }

    /// Sets the value of NamesXforestTime_Base
    pub fn set_names_xforest_time__base(&mut self, value: u32) {
        self.names_xforest_time__base = Some(value);
    }

    /// Gets the value of NamesXforestTime_Base
    pub fn get_names_xforest_time__base(&self) -> Option<&u32> {
        self.names_xforest_time__base.as_ref()
    }

    /// Sets the value of SIDsCachePercentFull
    pub fn set_sids_cache_percent_full(&mut self, value: u32) {
        self.sids_cache_percent_full = Some(value);
    }

    /// Gets the value of SIDsCachePercentFull
    pub fn get_sids_cache_percent_full(&self) -> Option<&u32> {
        self.sids_cache_percent_full.as_ref()
    }

    /// Sets the value of SIDsCachePercentFull_Base
    pub fn set_sids_cache_percent_full__base(&mut self, value: u32) {
        self.sids_cache_percent_full__base = Some(value);
    }

    /// Gets the value of SIDsCachePercentFull_Base
    pub fn get_sids_cache_percent_full__base(&self) -> Option<&u32> {
        self.sids_cache_percent_full__base.as_ref()
    }

    /// Sets the value of SIDsCachePercentHit
    pub fn set_sids_cache_percent_hit(&mut self, value: u32) {
        self.sids_cache_percent_hit = Some(value);
    }

    /// Gets the value of SIDsCachePercentHit
    pub fn get_sids_cache_percent_hit(&self) -> Option<&u32> {
        self.sids_cache_percent_hit.as_ref()
    }

    /// Sets the value of SIDsCachePercentHit_Base
    pub fn set_sids_cache_percent_hit__base(&mut self, value: u32) {
        self.sids_cache_percent_hit__base = Some(value);
    }

    /// Gets the value of SIDsCachePercentHit_Base
    pub fn get_sids_cache_percent_hit__base(&self) -> Option<&u32> {
        self.sids_cache_percent_hit__base.as_ref()
    }

    /// Sets the value of SIDsCompletionTime
    pub fn set_sids_completion_time(&mut self, value: u32) {
        self.sids_completion_time = Some(value);
    }

    /// Gets the value of SIDsCompletionTime
    pub fn get_sids_completion_time(&self) -> Option<&u32> {
        self.sids_completion_time.as_ref()
    }

    /// Sets the value of SIDsCompletionTime_Base
    pub fn set_sids_completion_time__base(&mut self, value: u32) {
        self.sids_completion_time__base = Some(value);
    }

    /// Gets the value of SIDsCompletionTime_Base
    pub fn get_sids_completion_time__base(&self) -> Option<&u32> {
        self.sids_completion_time__base.as_ref()
    }

    /// Sets the value of SIDsErrorsPersec
    pub fn set_sids_errors_persec(&mut self, value: u32) {
        self.sids_errors_persec = Some(value);
    }

    /// Gets the value of SIDsErrorsPersec
    pub fn get_sids_errors_persec(&self) -> Option<&u32> {
        self.sids_errors_persec.as_ref()
    }

    /// Sets the value of SIDsInboundRequestsPersec
    pub fn set_sids_inbound_requests_persec(&mut self, value: u32) {
        self.sids_inbound_requests_persec = Some(value);
    }

    /// Gets the value of SIDsInboundRequestsPersec
    pub fn get_sids_inbound_requests_persec(&self) -> Option<&u32> {
        self.sids_inbound_requests_persec.as_ref()
    }

    /// Sets the value of SIDsOutboundRequestsPersec
    pub fn set_sids_outbound_requests_persec(&mut self, value: u32) {
        self.sids_outbound_requests_persec = Some(value);
    }

    /// Gets the value of SIDsOutboundRequestsPersec
    pub fn get_sids_outbound_requests_persec(&self) -> Option<&u32> {
        self.sids_outbound_requests_persec.as_ref()
    }

    /// Sets the value of SIDsPrimaryDomainRequestsPersec
    pub fn set_sids_primary_domain_requests_persec(&mut self, value: u32) {
        self.sids_primary_domain_requests_persec = Some(value);
    }

    /// Gets the value of SIDsPrimaryDomainRequestsPersec
    pub fn get_sids_primary_domain_requests_persec(&self) -> Option<&u32> {
        self.sids_primary_domain_requests_persec.as_ref()
    }

    /// Sets the value of SIDsPrimaryDomainRequestTime
    pub fn set_sids_primary_domain_request_time(&mut self, value: u32) {
        self.sids_primary_domain_request_time = Some(value);
    }

    /// Gets the value of SIDsPrimaryDomainRequestTime
    pub fn get_sids_primary_domain_request_time(&self) -> Option<&u32> {
        self.sids_primary_domain_request_time.as_ref()
    }

    /// Sets the value of SIDsPrimaryDomainRequestTime_Base
    pub fn set_sids_primary_domain_request_time__base(&mut self, value: u32) {
        self.sids_primary_domain_request_time__base = Some(value);
    }

    /// Gets the value of SIDsPrimaryDomainRequestTime_Base
    pub fn get_sids_primary_domain_request_time__base(&self) -> Option<&u32> {
        self.sids_primary_domain_request_time__base.as_ref()
    }

    /// Sets the value of SIDsRemoteRequestTime
    pub fn set_sids_remote_request_time(&mut self, value: u32) {
        self.sids_remote_request_time = Some(value);
    }

    /// Gets the value of SIDsRemoteRequestTime
    pub fn get_sids_remote_request_time(&self) -> Option<&u32> {
        self.sids_remote_request_time.as_ref()
    }

    /// Sets the value of SIDsRemoteRequestTime_Base
    pub fn set_sids_remote_request_time__base(&mut self, value: u32) {
        self.sids_remote_request_time__base = Some(value);
    }

    /// Gets the value of SIDsRemoteRequestTime_Base
    pub fn get_sids_remote_request_time__base(&self) -> Option<&u32> {
        self.sids_remote_request_time__base.as_ref()
    }

    /// Sets the value of SIDsTrustedDomainRequestsPersec
    pub fn set_sids_trusted_domain_requests_persec(&mut self, value: u32) {
        self.sids_trusted_domain_requests_persec = Some(value);
    }

    /// Gets the value of SIDsTrustedDomainRequestsPersec
    pub fn get_sids_trusted_domain_requests_persec(&self) -> Option<&u32> {
        self.sids_trusted_domain_requests_persec.as_ref()
    }

    /// Sets the value of SIDsTrustedDomainRequestTime
    pub fn set_sids_trusted_domain_request_time(&mut self, value: u32) {
        self.sids_trusted_domain_request_time = Some(value);
    }

    /// Gets the value of SIDsTrustedDomainRequestTime
    pub fn get_sids_trusted_domain_request_time(&self) -> Option<&u32> {
        self.sids_trusted_domain_request_time.as_ref()
    }

    /// Sets the value of SIDsTrustedDomainRequestTime_Base
    pub fn set_sids_trusted_domain_request_time__base(&mut self, value: u32) {
        self.sids_trusted_domain_request_time__base = Some(value);
    }

    /// Gets the value of SIDsTrustedDomainRequestTime_Base
    pub fn get_sids_trusted_domain_request_time__base(&self) -> Option<&u32> {
        self.sids_trusted_domain_request_time__base.as_ref()
    }

    /// Sets the value of SIDsUnresolvedPersec
    pub fn set_sids_unresolved_persec(&mut self, value: u32) {
        self.sids_unresolved_persec = Some(value);
    }

    /// Gets the value of SIDsUnresolvedPersec
    pub fn get_sids_unresolved_persec(&self) -> Option<&u32> {
        self.sids_unresolved_persec.as_ref()
    }

    /// Sets the value of SIDsXforestRequestsPersec
    pub fn set_sids_xforest_requests_persec(&mut self, value: u32) {
        self.sids_xforest_requests_persec = Some(value);
    }

    /// Gets the value of SIDsXforestRequestsPersec
    pub fn get_sids_xforest_requests_persec(&self) -> Option<&u32> {
        self.sids_xforest_requests_persec.as_ref()
    }

    /// Sets the value of SIDsXforestRequestTime
    pub fn set_sids_xforest_request_time(&mut self, value: u32) {
        self.sids_xforest_request_time = Some(value);
    }

    /// Gets the value of SIDsXforestRequestTime
    pub fn get_sids_xforest_request_time(&self) -> Option<&u32> {
        self.sids_xforest_request_time.as_ref()
    }

    /// Sets the value of SIDsXforestRequestTime_Base
    pub fn set_sids_xforest_request_time__base(&mut self, value: u32) {
        self.sids_xforest_request_time__base = Some(value);
    }

    /// Gets the value of SIDsXforestRequestTime_Base
    pub fn get_sids_xforest_request_time__base(&self) -> Option<&u32> {
        self.sids_xforest_request_time__base.as_ref()
    }
}

