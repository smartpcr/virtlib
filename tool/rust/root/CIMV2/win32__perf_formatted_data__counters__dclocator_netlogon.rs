// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_DCLocatorNetlogon struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_DCLocatorNetlogon {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CacheHitsPersec")]
    pub cache_hits_persec: Option<u32>,

/// 
    #[serde(rename = "CacheMissesPersec")]
    pub cache_misses_persec: Option<u32>,

/// 
    #[serde(rename = "DNSQueryFailuresPersec")]
    pub dnsquery_failures_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsDSRequiredW2KRequestsPersec")]
    pub flags_dsrequired_w2_krequests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsDSRequiredWS2008RequestsPersec")]
    pub flags_dsrequired_ws2008_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsDSRequiredWS2012R2RequestsPersec")]
    pub flags_dsrequired_ws2012_r2_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsDSRequiredWS2012RequestsPersec")]
    pub flags_dsrequired_ws2012_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsDSRequiredWS2016RequestsPersec")]
    pub flags_dsrequired_ws2016_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsForceRediscoveryRequestsPersec")]
    pub flags_force_rediscovery_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsGCRequiredRequestsPersec")]
    pub flags_gcrequired_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsKDCRequiredRequestsPersec")]
    pub flags_kdcrequired_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsKeyListSupportRequiredRequestsPersec")]
    pub flags_key_list_support_required_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsPDCRequiredRequestsPersec")]
    pub flags_pdcrequired_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsTimeServerRequiredRequestsPersec")]
    pub flags_time_server_required_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsTryNextClosestSiteRequestsPersec")]
    pub flags_try_next_closest_site_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FlagsWritableRequiredRequestsPersec")]
    pub flags_writable_required_requests_persec: Option<u32>,

/// 
    #[serde(rename = "PingsMailslotPingsSentPersec")]
    pub pings_mailslot_pings_sent_persec: Option<u32>,

/// 
    #[serde(rename = "PingsUDPPingsSentPersec")]
    pub pings_udppings_sent_persec: Option<u32>,

/// 
    #[serde(rename = "RequestsAverageFailureLatencysecs")]
    pub requests_average_failure_latencysecs: Option<u32>,

/// 
    #[serde(rename = "RequestsAverageSuccessLatencysecs")]
    pub requests_average_success_latencysecs: Option<u32>,

/// 
    #[serde(rename = "RequestsFailuresPersec")]
    pub requests_failures_persec: Option<u32>,

/// 
    #[serde(rename = "RequestsRejectedPersec")]
    pub requests_rejected_persec: Option<u32>,

/// 
    #[serde(rename = "RequestsSuccessesPersec")]
    pub requests_successes_persec: Option<u32>,

/// 
    #[serde(rename = "RequestsTotalActive")]
    pub requests_total_active: Option<u32>,

/// 
    #[serde(rename = "UDPPortsOpened")]
    pub udpports_opened: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_DCLocatorNetlogon {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            cache_hits_persec: None,
            cache_misses_persec: None,
            dnsquery_failures_persec: None,
            flags_dsrequired_w2_krequests_persec: None,
            flags_dsrequired_ws2008_requests_persec: None,
            flags_dsrequired_ws2012_r2_requests_persec: None,
            flags_dsrequired_ws2012_requests_persec: None,
            flags_dsrequired_ws2016_requests_persec: None,
            flags_force_rediscovery_requests_persec: None,
            flags_gcrequired_requests_persec: None,
            flags_kdcrequired_requests_persec: None,
            flags_key_list_support_required_requests_persec: None,
            flags_pdcrequired_requests_persec: None,
            flags_time_server_required_requests_persec: None,
            flags_try_next_closest_site_requests_persec: None,
            flags_writable_required_requests_persec: None,
            pings_mailslot_pings_sent_persec: None,
            pings_udppings_sent_persec: None,
            requests_average_failure_latencysecs: None,
            requests_average_success_latencysecs: None,
            requests_failures_persec: None,
            requests_rejected_persec: None,
            requests_successes_persec: None,
            requests_total_active: None,
            udpports_opened: None,
        }
    }


    /// Sets the value of CacheHitsPersec
    pub fn set_cache_hits_persec(&mut self, value: u32) {
        self.cache_hits_persec = Some(value);
    }

    /// Gets the value of CacheHitsPersec
    pub fn get_cache_hits_persec(&self) -> Option<&u32> {
        self.cache_hits_persec.as_ref()
    }

    /// Sets the value of CacheMissesPersec
    pub fn set_cache_misses_persec(&mut self, value: u32) {
        self.cache_misses_persec = Some(value);
    }

    /// Gets the value of CacheMissesPersec
    pub fn get_cache_misses_persec(&self) -> Option<&u32> {
        self.cache_misses_persec.as_ref()
    }

    /// Sets the value of DNSQueryFailuresPersec
    pub fn set_dnsquery_failures_persec(&mut self, value: u32) {
        self.dnsquery_failures_persec = Some(value);
    }

    /// Gets the value of DNSQueryFailuresPersec
    pub fn get_dnsquery_failures_persec(&self) -> Option<&u32> {
        self.dnsquery_failures_persec.as_ref()
    }

    /// Sets the value of FlagsDSRequiredW2KRequestsPersec
    pub fn set_flags_dsrequired_w2_krequests_persec(&mut self, value: u32) {
        self.flags_dsrequired_w2_krequests_persec = Some(value);
    }

    /// Gets the value of FlagsDSRequiredW2KRequestsPersec
    pub fn get_flags_dsrequired_w2_krequests_persec(&self) -> Option<&u32> {
        self.flags_dsrequired_w2_krequests_persec.as_ref()
    }

    /// Sets the value of FlagsDSRequiredWS2008RequestsPersec
    pub fn set_flags_dsrequired_ws2008_requests_persec(&mut self, value: u32) {
        self.flags_dsrequired_ws2008_requests_persec = Some(value);
    }

    /// Gets the value of FlagsDSRequiredWS2008RequestsPersec
    pub fn get_flags_dsrequired_ws2008_requests_persec(&self) -> Option<&u32> {
        self.flags_dsrequired_ws2008_requests_persec.as_ref()
    }

    /// Sets the value of FlagsDSRequiredWS2012R2RequestsPersec
    pub fn set_flags_dsrequired_ws2012_r2_requests_persec(&mut self, value: u32) {
        self.flags_dsrequired_ws2012_r2_requests_persec = Some(value);
    }

    /// Gets the value of FlagsDSRequiredWS2012R2RequestsPersec
    pub fn get_flags_dsrequired_ws2012_r2_requests_persec(&self) -> Option<&u32> {
        self.flags_dsrequired_ws2012_r2_requests_persec.as_ref()
    }

    /// Sets the value of FlagsDSRequiredWS2012RequestsPersec
    pub fn set_flags_dsrequired_ws2012_requests_persec(&mut self, value: u32) {
        self.flags_dsrequired_ws2012_requests_persec = Some(value);
    }

    /// Gets the value of FlagsDSRequiredWS2012RequestsPersec
    pub fn get_flags_dsrequired_ws2012_requests_persec(&self) -> Option<&u32> {
        self.flags_dsrequired_ws2012_requests_persec.as_ref()
    }

    /// Sets the value of FlagsDSRequiredWS2016RequestsPersec
    pub fn set_flags_dsrequired_ws2016_requests_persec(&mut self, value: u32) {
        self.flags_dsrequired_ws2016_requests_persec = Some(value);
    }

    /// Gets the value of FlagsDSRequiredWS2016RequestsPersec
    pub fn get_flags_dsrequired_ws2016_requests_persec(&self) -> Option<&u32> {
        self.flags_dsrequired_ws2016_requests_persec.as_ref()
    }

    /// Sets the value of FlagsForceRediscoveryRequestsPersec
    pub fn set_flags_force_rediscovery_requests_persec(&mut self, value: u32) {
        self.flags_force_rediscovery_requests_persec = Some(value);
    }

    /// Gets the value of FlagsForceRediscoveryRequestsPersec
    pub fn get_flags_force_rediscovery_requests_persec(&self) -> Option<&u32> {
        self.flags_force_rediscovery_requests_persec.as_ref()
    }

    /// Sets the value of FlagsGCRequiredRequestsPersec
    pub fn set_flags_gcrequired_requests_persec(&mut self, value: u32) {
        self.flags_gcrequired_requests_persec = Some(value);
    }

    /// Gets the value of FlagsGCRequiredRequestsPersec
    pub fn get_flags_gcrequired_requests_persec(&self) -> Option<&u32> {
        self.flags_gcrequired_requests_persec.as_ref()
    }

    /// Sets the value of FlagsKDCRequiredRequestsPersec
    pub fn set_flags_kdcrequired_requests_persec(&mut self, value: u32) {
        self.flags_kdcrequired_requests_persec = Some(value);
    }

    /// Gets the value of FlagsKDCRequiredRequestsPersec
    pub fn get_flags_kdcrequired_requests_persec(&self) -> Option<&u32> {
        self.flags_kdcrequired_requests_persec.as_ref()
    }

    /// Sets the value of FlagsKeyListSupportRequiredRequestsPersec
    pub fn set_flags_key_list_support_required_requests_persec(&mut self, value: u32) {
        self.flags_key_list_support_required_requests_persec = Some(value);
    }

    /// Gets the value of FlagsKeyListSupportRequiredRequestsPersec
    pub fn get_flags_key_list_support_required_requests_persec(&self) -> Option<&u32> {
        self.flags_key_list_support_required_requests_persec.as_ref()
    }

    /// Sets the value of FlagsPDCRequiredRequestsPersec
    pub fn set_flags_pdcrequired_requests_persec(&mut self, value: u32) {
        self.flags_pdcrequired_requests_persec = Some(value);
    }

    /// Gets the value of FlagsPDCRequiredRequestsPersec
    pub fn get_flags_pdcrequired_requests_persec(&self) -> Option<&u32> {
        self.flags_pdcrequired_requests_persec.as_ref()
    }

    /// Sets the value of FlagsTimeServerRequiredRequestsPersec
    pub fn set_flags_time_server_required_requests_persec(&mut self, value: u32) {
        self.flags_time_server_required_requests_persec = Some(value);
    }

    /// Gets the value of FlagsTimeServerRequiredRequestsPersec
    pub fn get_flags_time_server_required_requests_persec(&self) -> Option<&u32> {
        self.flags_time_server_required_requests_persec.as_ref()
    }

    /// Sets the value of FlagsTryNextClosestSiteRequestsPersec
    pub fn set_flags_try_next_closest_site_requests_persec(&mut self, value: u32) {
        self.flags_try_next_closest_site_requests_persec = Some(value);
    }

    /// Gets the value of FlagsTryNextClosestSiteRequestsPersec
    pub fn get_flags_try_next_closest_site_requests_persec(&self) -> Option<&u32> {
        self.flags_try_next_closest_site_requests_persec.as_ref()
    }

    /// Sets the value of FlagsWritableRequiredRequestsPersec
    pub fn set_flags_writable_required_requests_persec(&mut self, value: u32) {
        self.flags_writable_required_requests_persec = Some(value);
    }

    /// Gets the value of FlagsWritableRequiredRequestsPersec
    pub fn get_flags_writable_required_requests_persec(&self) -> Option<&u32> {
        self.flags_writable_required_requests_persec.as_ref()
    }

    /// Sets the value of PingsMailslotPingsSentPersec
    pub fn set_pings_mailslot_pings_sent_persec(&mut self, value: u32) {
        self.pings_mailslot_pings_sent_persec = Some(value);
    }

    /// Gets the value of PingsMailslotPingsSentPersec
    pub fn get_pings_mailslot_pings_sent_persec(&self) -> Option<&u32> {
        self.pings_mailslot_pings_sent_persec.as_ref()
    }

    /// Sets the value of PingsUDPPingsSentPersec
    pub fn set_pings_udppings_sent_persec(&mut self, value: u32) {
        self.pings_udppings_sent_persec = Some(value);
    }

    /// Gets the value of PingsUDPPingsSentPersec
    pub fn get_pings_udppings_sent_persec(&self) -> Option<&u32> {
        self.pings_udppings_sent_persec.as_ref()
    }

    /// Sets the value of RequestsAverageFailureLatencysecs
    pub fn set_requests_average_failure_latencysecs(&mut self, value: u32) {
        self.requests_average_failure_latencysecs = Some(value);
    }

    /// Gets the value of RequestsAverageFailureLatencysecs
    pub fn get_requests_average_failure_latencysecs(&self) -> Option<&u32> {
        self.requests_average_failure_latencysecs.as_ref()
    }

    /// Sets the value of RequestsAverageSuccessLatencysecs
    pub fn set_requests_average_success_latencysecs(&mut self, value: u32) {
        self.requests_average_success_latencysecs = Some(value);
    }

    /// Gets the value of RequestsAverageSuccessLatencysecs
    pub fn get_requests_average_success_latencysecs(&self) -> Option<&u32> {
        self.requests_average_success_latencysecs.as_ref()
    }

    /// Sets the value of RequestsFailuresPersec
    pub fn set_requests_failures_persec(&mut self, value: u32) {
        self.requests_failures_persec = Some(value);
    }

    /// Gets the value of RequestsFailuresPersec
    pub fn get_requests_failures_persec(&self) -> Option<&u32> {
        self.requests_failures_persec.as_ref()
    }

    /// Sets the value of RequestsRejectedPersec
    pub fn set_requests_rejected_persec(&mut self, value: u32) {
        self.requests_rejected_persec = Some(value);
    }

    /// Gets the value of RequestsRejectedPersec
    pub fn get_requests_rejected_persec(&self) -> Option<&u32> {
        self.requests_rejected_persec.as_ref()
    }

    /// Sets the value of RequestsSuccessesPersec
    pub fn set_requests_successes_persec(&mut self, value: u32) {
        self.requests_successes_persec = Some(value);
    }

    /// Gets the value of RequestsSuccessesPersec
    pub fn get_requests_successes_persec(&self) -> Option<&u32> {
        self.requests_successes_persec.as_ref()
    }

    /// Sets the value of RequestsTotalActive
    pub fn set_requests_total_active(&mut self, value: u32) {
        self.requests_total_active = Some(value);
    }

    /// Gets the value of RequestsTotalActive
    pub fn get_requests_total_active(&self) -> Option<&u32> {
        self.requests_total_active.as_ref()
    }

    /// Sets the value of UDPPortsOpened
    pub fn set_udpports_opened(&mut self, value: u32) {
        self.udpports_opened = Some(value);
    }

    /// Gets the value of UDPPortsOpened
    pub fn get_udpports_opened(&self) -> Option<&u32> {
        self.udpports_opened.as_ref()
    }
}

