// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPsecDoSPSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPsecDoSPSetting {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "DefBlockExemptDscp")]
    pub def_block_exempt_dscp: Option<u16>,

/// 
    #[serde(rename = "DefBlockExemptRateLimitBytesPerSec")]
    pub def_block_exempt_rate_limit_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "EffectiveAddressFamily")]
    pub effective_address_family: Option<u16>,

/// 
    #[serde(rename = "EnabledKeyingModules")]
    pub enabled_keying_modules: Option<u32>,

/// 
    #[serde(rename = "FilteringFlags")]
    pub filtering_flags: Option<u32>,

/// 
    #[serde(rename = "IcmpV6Dscp")]
    pub icmp_v6_dscp: Option<u16>,

/// 
    #[serde(rename = "IcmpV6RateLimitBytesPerSec")]
    pub icmp_v6_rate_limit_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "IpV6FilterExemptDscp")]
    pub ip_v6_filter_exempt_dscp: Option<u32>,

/// 
    #[serde(rename = "IpV6FilterExemptRateLimitBytesPerSec")]
    pub ip_v6_filter_exempt_rate_limit_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "IpV6IPsecAuthDscp")]
    pub ip_v6_ipsec_auth_dscp: Option<u16>,

/// 
    #[serde(rename = "IpV6IPsecAuthRateLimitBytesPerSec")]
    pub ip_v6_ipsec_auth_rate_limit_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "IpV6IPsecUnauthDscp")]
    pub ip_v6_ipsec_unauth_dscp: Option<u32>,

/// 
    #[serde(rename = "IpV6IPsecUnauthPerIPRateLimitBytesPerSec")]
    pub ip_v6_ipsec_unauth_per_iprate_limit_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "IpV6IPsecUnauthRateLimitBytesPerSec")]
    pub ip_v6_ipsec_unauth_rate_limit_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "MaxPerIPRateLimitQueues")]
    pub max_per_iprate_limit_queues: Option<u32>,

/// 
    #[serde(rename = "MaxStateEntries")]
    pub max_state_entries: Option<u32>,

/// 
    #[serde(rename = "PerIPRateLimitQueueIdleTimeoutSeconds")]
    pub per_iprate_limit_queue_idle_timeout_seconds: Option<u32>,

/// 
    #[serde(rename = "PrivateInterfaceAliases")]
    pub private_interface_aliases: Vec<String>,

/// 
    #[serde(rename = "PrivateV6Address")]
    pub private_v6_address: Option<String>,

/// 
    #[serde(rename = "PublicInterfaceAliases")]
    pub public_interface_aliases: Vec<String>,

/// 
    #[serde(rename = "PublicV6Address")]
    pub public_v6_address: Option<String>,

/// 
    #[serde(rename = "StateIdleTimeoutSeconds")]
    pub state_idle_timeout_seconds: Option<u32>,
}

impl MSFT_NetIPsecDoSPSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            def_block_exempt_dscp: None,
            def_block_exempt_rate_limit_bytes_per_sec: None,
            effective_address_family: None,
            enabled_keying_modules: None,
            filtering_flags: None,
            icmp_v6_dscp: None,
            icmp_v6_rate_limit_bytes_per_sec: None,
            ip_v6_filter_exempt_dscp: None,
            ip_v6_filter_exempt_rate_limit_bytes_per_sec: None,
            ip_v6_ipsec_auth_dscp: None,
            ip_v6_ipsec_auth_rate_limit_bytes_per_sec: None,
            ip_v6_ipsec_unauth_dscp: None,
            ip_v6_ipsec_unauth_per_iprate_limit_bytes_per_sec: None,
            ip_v6_ipsec_unauth_rate_limit_bytes_per_sec: None,
            max_per_iprate_limit_queues: None,
            max_state_entries: None,
            per_iprate_limit_queue_idle_timeout_seconds: None,
            private_interface_aliases: Vec::new(),
            private_v6_address: None,
            public_interface_aliases: Vec::new(),
            public_v6_address: None,
            state_idle_timeout_seconds: None,
        }
    }


    /// Sets the value of DefBlockExemptDscp
    pub fn set_def_block_exempt_dscp(&mut self, value: u16) {
        self.def_block_exempt_dscp = Some(value);
    }

    /// Gets the value of DefBlockExemptDscp
    pub fn get_def_block_exempt_dscp(&self) -> Option<&u16> {
        self.def_block_exempt_dscp.as_ref()
    }

    /// Sets the value of DefBlockExemptRateLimitBytesPerSec
    pub fn set_def_block_exempt_rate_limit_bytes_per_sec(&mut self, value: u32) {
        self.def_block_exempt_rate_limit_bytes_per_sec = Some(value);
    }

    /// Gets the value of DefBlockExemptRateLimitBytesPerSec
    pub fn get_def_block_exempt_rate_limit_bytes_per_sec(&self) -> Option<&u32> {
        self.def_block_exempt_rate_limit_bytes_per_sec.as_ref()
    }

    /// Sets the value of EffectiveAddressFamily
    pub fn set_effective_address_family(&mut self, value: u16) {
        self.effective_address_family = Some(value);
    }

    /// Gets the value of EffectiveAddressFamily
    pub fn get_effective_address_family(&self) -> Option<&u16> {
        self.effective_address_family.as_ref()
    }

    /// Sets the value of EnabledKeyingModules
    pub fn set_enabled_keying_modules(&mut self, value: u32) {
        self.enabled_keying_modules = Some(value);
    }

    /// Gets the value of EnabledKeyingModules
    pub fn get_enabled_keying_modules(&self) -> Option<&u32> {
        self.enabled_keying_modules.as_ref()
    }

    /// Sets the value of FilteringFlags
    pub fn set_filtering_flags(&mut self, value: u32) {
        self.filtering_flags = Some(value);
    }

    /// Gets the value of FilteringFlags
    pub fn get_filtering_flags(&self) -> Option<&u32> {
        self.filtering_flags.as_ref()
    }

    /// Sets the value of IcmpV6Dscp
    pub fn set_icmp_v6_dscp(&mut self, value: u16) {
        self.icmp_v6_dscp = Some(value);
    }

    /// Gets the value of IcmpV6Dscp
    pub fn get_icmp_v6_dscp(&self) -> Option<&u16> {
        self.icmp_v6_dscp.as_ref()
    }

    /// Sets the value of IcmpV6RateLimitBytesPerSec
    pub fn set_icmp_v6_rate_limit_bytes_per_sec(&mut self, value: u32) {
        self.icmp_v6_rate_limit_bytes_per_sec = Some(value);
    }

    /// Gets the value of IcmpV6RateLimitBytesPerSec
    pub fn get_icmp_v6_rate_limit_bytes_per_sec(&self) -> Option<&u32> {
        self.icmp_v6_rate_limit_bytes_per_sec.as_ref()
    }

    /// Sets the value of IpV6FilterExemptDscp
    pub fn set_ip_v6_filter_exempt_dscp(&mut self, value: u32) {
        self.ip_v6_filter_exempt_dscp = Some(value);
    }

    /// Gets the value of IpV6FilterExemptDscp
    pub fn get_ip_v6_filter_exempt_dscp(&self) -> Option<&u32> {
        self.ip_v6_filter_exempt_dscp.as_ref()
    }

    /// Sets the value of IpV6FilterExemptRateLimitBytesPerSec
    pub fn set_ip_v6_filter_exempt_rate_limit_bytes_per_sec(&mut self, value: u32) {
        self.ip_v6_filter_exempt_rate_limit_bytes_per_sec = Some(value);
    }

    /// Gets the value of IpV6FilterExemptRateLimitBytesPerSec
    pub fn get_ip_v6_filter_exempt_rate_limit_bytes_per_sec(&self) -> Option<&u32> {
        self.ip_v6_filter_exempt_rate_limit_bytes_per_sec.as_ref()
    }

    /// Sets the value of IpV6IPsecAuthDscp
    pub fn set_ip_v6_ipsec_auth_dscp(&mut self, value: u16) {
        self.ip_v6_ipsec_auth_dscp = Some(value);
    }

    /// Gets the value of IpV6IPsecAuthDscp
    pub fn get_ip_v6_ipsec_auth_dscp(&self) -> Option<&u16> {
        self.ip_v6_ipsec_auth_dscp.as_ref()
    }

    /// Sets the value of IpV6IPsecAuthRateLimitBytesPerSec
    pub fn set_ip_v6_ipsec_auth_rate_limit_bytes_per_sec(&mut self, value: u32) {
        self.ip_v6_ipsec_auth_rate_limit_bytes_per_sec = Some(value);
    }

    /// Gets the value of IpV6IPsecAuthRateLimitBytesPerSec
    pub fn get_ip_v6_ipsec_auth_rate_limit_bytes_per_sec(&self) -> Option<&u32> {
        self.ip_v6_ipsec_auth_rate_limit_bytes_per_sec.as_ref()
    }

    /// Sets the value of IpV6IPsecUnauthDscp
    pub fn set_ip_v6_ipsec_unauth_dscp(&mut self, value: u32) {
        self.ip_v6_ipsec_unauth_dscp = Some(value);
    }

    /// Gets the value of IpV6IPsecUnauthDscp
    pub fn get_ip_v6_ipsec_unauth_dscp(&self) -> Option<&u32> {
        self.ip_v6_ipsec_unauth_dscp.as_ref()
    }

    /// Sets the value of IpV6IPsecUnauthPerIPRateLimitBytesPerSec
    pub fn set_ip_v6_ipsec_unauth_per_iprate_limit_bytes_per_sec(&mut self, value: u32) {
        self.ip_v6_ipsec_unauth_per_iprate_limit_bytes_per_sec = Some(value);
    }

    /// Gets the value of IpV6IPsecUnauthPerIPRateLimitBytesPerSec
    pub fn get_ip_v6_ipsec_unauth_per_iprate_limit_bytes_per_sec(&self) -> Option<&u32> {
        self.ip_v6_ipsec_unauth_per_iprate_limit_bytes_per_sec.as_ref()
    }

    /// Sets the value of IpV6IPsecUnauthRateLimitBytesPerSec
    pub fn set_ip_v6_ipsec_unauth_rate_limit_bytes_per_sec(&mut self, value: u32) {
        self.ip_v6_ipsec_unauth_rate_limit_bytes_per_sec = Some(value);
    }

    /// Gets the value of IpV6IPsecUnauthRateLimitBytesPerSec
    pub fn get_ip_v6_ipsec_unauth_rate_limit_bytes_per_sec(&self) -> Option<&u32> {
        self.ip_v6_ipsec_unauth_rate_limit_bytes_per_sec.as_ref()
    }

    /// Sets the value of MaxPerIPRateLimitQueues
    pub fn set_max_per_iprate_limit_queues(&mut self, value: u32) {
        self.max_per_iprate_limit_queues = Some(value);
    }

    /// Gets the value of MaxPerIPRateLimitQueues
    pub fn get_max_per_iprate_limit_queues(&self) -> Option<&u32> {
        self.max_per_iprate_limit_queues.as_ref()
    }

    /// Sets the value of MaxStateEntries
    pub fn set_max_state_entries(&mut self, value: u32) {
        self.max_state_entries = Some(value);
    }

    /// Gets the value of MaxStateEntries
    pub fn get_max_state_entries(&self) -> Option<&u32> {
        self.max_state_entries.as_ref()
    }

    /// Sets the value of PerIPRateLimitQueueIdleTimeoutSeconds
    pub fn set_per_iprate_limit_queue_idle_timeout_seconds(&mut self, value: u32) {
        self.per_iprate_limit_queue_idle_timeout_seconds = Some(value);
    }

    /// Gets the value of PerIPRateLimitQueueIdleTimeoutSeconds
    pub fn get_per_iprate_limit_queue_idle_timeout_seconds(&self) -> Option<&u32> {
        self.per_iprate_limit_queue_idle_timeout_seconds.as_ref()
    }

    /// Sets the value of PrivateInterfaceAliases
    pub fn set_private_interface_aliases(&mut self, value: Vec<String>) {
        self.private_interface_aliases = value;
    }

    /// Gets the value of PrivateInterfaceAliases
    pub fn get_private_interface_aliases(&self) -> &Vec<String> {
        &self.private_interface_aliases
    }

    /// Sets the value of PrivateV6Address
    pub fn set_private_v6_address(&mut self, value: String) {
        self.private_v6_address = Some(value);
    }

    /// Gets the value of PrivateV6Address
    pub fn get_private_v6_address(&self) -> Option<&String> {
        self.private_v6_address.as_ref()
    }

    /// Sets the value of PublicInterfaceAliases
    pub fn set_public_interface_aliases(&mut self, value: Vec<String>) {
        self.public_interface_aliases = value;
    }

    /// Gets the value of PublicInterfaceAliases
    pub fn get_public_interface_aliases(&self) -> &Vec<String> {
        &self.public_interface_aliases
    }

    /// Sets the value of PublicV6Address
    pub fn set_public_v6_address(&mut self, value: String) {
        self.public_v6_address = Some(value);
    }

    /// Gets the value of PublicV6Address
    pub fn get_public_v6_address(&self) -> Option<&String> {
        self.public_v6_address.as_ref()
    }

    /// Sets the value of StateIdleTimeoutSeconds
    pub fn set_state_idle_timeout_seconds(&mut self, value: u32) {
        self.state_idle_timeout_seconds = Some(value);
    }

    /// Gets the value of StateIdleTimeoutSeconds
    pub fn get_state_idle_timeout_seconds(&self) -> Option<&u32> {
        self.state_idle_timeout_seconds.as_ref()
    }
}

