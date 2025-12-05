// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_VFPPortAverageInboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_VFPPortAverageInboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AverageInboundBytes")]
    pub average_inbound_bytes: Option<u64>,

/// 
    #[serde(rename = "AverageInboundForwardedMulticastPackets")]
    pub average_inbound_forwarded_multicast_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundForwardedUnicastPackets")]
    pub average_inbound_forwarded_unicast_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTCopyFINPackets")]
    pub average_inbound_gftcopy_finpackets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTCopyPackets")]
    pub average_inbound_gftcopy_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTCopyResetPackets")]
    pub average_inbound_gftcopy_reset_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionPackets")]
    pub average_inbound_gftexception_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFOffloadBlockedPackets")]
    pub average_inbound_gftexception_ufoffload_blocked_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFOffloadDeferredPackets")]
    pub average_inbound_gftexception_ufoffload_deferred_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFOffloadedTCPPackets")]
    pub average_inbound_gftexception_ufoffloaded_tcppackets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFOffloadedUDPPackets")]
    pub average_inbound_gftexception_ufoffloaded_udppackets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFOffloadFailedPackets")]
    pub average_inbound_gftexception_ufoffload_failed_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFOffloadPendingPackets")]
    pub average_inbound_gftexception_ufoffload_pending_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFOffloadRetryAwaitingPackets")]
    pub average_inbound_gftexception_ufoffload_retry_awaiting_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTExceptionUFPackets")]
    pub average_inbound_gftexception_ufpackets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTTotalBytes")]
    pub average_inbound_gfttotal_bytes: Option<u64>,

/// 
    #[serde(rename = "AverageInboundGFTTotalPackets")]
    pub average_inbound_gfttotal_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundHairPinnedPackets")]
    pub average_inbound_hair_pinned_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundInterceptedPackets")]
    pub average_inbound_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundMissedInterceptedPackets")]
    pub average_inbound_missed_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundNonIPPackets")]
    pub average_inbound_non_ippackets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPackets")]
    pub average_inbound_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPendingPackets")]
    pub average_inbound_pending_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundTCPSYNACKPackets")]
    pub average_inbound_tcpsynackpackets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundTCPSYNPackets")]
    pub average_inbound_tcpsynpackets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundThrottledPackets")]
    pub average_inbound_throttled_packets: Option<u64>,

/// 
    #[serde(rename = "AverageInboundUnicastForwardedGFTExceptionPackets")]
    pub average_inbound_unicast_forwarded_gftexception_packets: Option<u64>,
}

impl Win32_PerfRawData_Counters_VFPPortAverageInboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            average_inbound_bytes: None,
            average_inbound_forwarded_multicast_packets: None,
            average_inbound_forwarded_unicast_packets: None,
            average_inbound_gftcopy_finpackets: None,
            average_inbound_gftcopy_packets: None,
            average_inbound_gftcopy_reset_packets: None,
            average_inbound_gftexception_packets: None,
            average_inbound_gftexception_ufoffload_blocked_packets: None,
            average_inbound_gftexception_ufoffload_deferred_packets: None,
            average_inbound_gftexception_ufoffloaded_tcppackets: None,
            average_inbound_gftexception_ufoffloaded_udppackets: None,
            average_inbound_gftexception_ufoffload_failed_packets: None,
            average_inbound_gftexception_ufoffload_pending_packets: None,
            average_inbound_gftexception_ufoffload_retry_awaiting_packets: None,
            average_inbound_gftexception_ufpackets: None,
            average_inbound_gfttotal_bytes: None,
            average_inbound_gfttotal_packets: None,
            average_inbound_hair_pinned_packets: None,
            average_inbound_intercepted_packets: None,
            average_inbound_missed_intercepted_packets: None,
            average_inbound_non_ippackets: None,
            average_inbound_packets: None,
            average_inbound_pending_packets: None,
            average_inbound_tcpsynackpackets: None,
            average_inbound_tcpsynpackets: None,
            average_inbound_throttled_packets: None,
            average_inbound_unicast_forwarded_gftexception_packets: None,
        }
    }


    /// Sets the value of AverageInboundBytes
    pub fn set_average_inbound_bytes(&mut self, value: u64) {
        self.average_inbound_bytes = Some(value);
    }

    /// Gets the value of AverageInboundBytes
    pub fn get_average_inbound_bytes(&self) -> Option<&u64> {
        self.average_inbound_bytes.as_ref()
    }

    /// Sets the value of AverageInboundForwardedMulticastPackets
    pub fn set_average_inbound_forwarded_multicast_packets(&mut self, value: u64) {
        self.average_inbound_forwarded_multicast_packets = Some(value);
    }

    /// Gets the value of AverageInboundForwardedMulticastPackets
    pub fn get_average_inbound_forwarded_multicast_packets(&self) -> Option<&u64> {
        self.average_inbound_forwarded_multicast_packets.as_ref()
    }

    /// Sets the value of AverageInboundForwardedUnicastPackets
    pub fn set_average_inbound_forwarded_unicast_packets(&mut self, value: u64) {
        self.average_inbound_forwarded_unicast_packets = Some(value);
    }

    /// Gets the value of AverageInboundForwardedUnicastPackets
    pub fn get_average_inbound_forwarded_unicast_packets(&self) -> Option<&u64> {
        self.average_inbound_forwarded_unicast_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTCopyFINPackets
    pub fn set_average_inbound_gftcopy_finpackets(&mut self, value: u64) {
        self.average_inbound_gftcopy_finpackets = Some(value);
    }

    /// Gets the value of AverageInboundGFTCopyFINPackets
    pub fn get_average_inbound_gftcopy_finpackets(&self) -> Option<&u64> {
        self.average_inbound_gftcopy_finpackets.as_ref()
    }

    /// Sets the value of AverageInboundGFTCopyPackets
    pub fn set_average_inbound_gftcopy_packets(&mut self, value: u64) {
        self.average_inbound_gftcopy_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTCopyPackets
    pub fn get_average_inbound_gftcopy_packets(&self) -> Option<&u64> {
        self.average_inbound_gftcopy_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTCopyResetPackets
    pub fn set_average_inbound_gftcopy_reset_packets(&mut self, value: u64) {
        self.average_inbound_gftcopy_reset_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTCopyResetPackets
    pub fn get_average_inbound_gftcopy_reset_packets(&self) -> Option<&u64> {
        self.average_inbound_gftcopy_reset_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionPackets
    pub fn set_average_inbound_gftexception_packets(&mut self, value: u64) {
        self.average_inbound_gftexception_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionPackets
    pub fn get_average_inbound_gftexception_packets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFOffloadBlockedPackets
    pub fn set_average_inbound_gftexception_ufoffload_blocked_packets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufoffload_blocked_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFOffloadBlockedPackets
    pub fn get_average_inbound_gftexception_ufoffload_blocked_packets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufoffload_blocked_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFOffloadDeferredPackets
    pub fn set_average_inbound_gftexception_ufoffload_deferred_packets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufoffload_deferred_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFOffloadDeferredPackets
    pub fn get_average_inbound_gftexception_ufoffload_deferred_packets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufoffload_deferred_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFOffloadedTCPPackets
    pub fn set_average_inbound_gftexception_ufoffloaded_tcppackets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufoffloaded_tcppackets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFOffloadedTCPPackets
    pub fn get_average_inbound_gftexception_ufoffloaded_tcppackets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufoffloaded_tcppackets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFOffloadedUDPPackets
    pub fn set_average_inbound_gftexception_ufoffloaded_udppackets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufoffloaded_udppackets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFOffloadedUDPPackets
    pub fn get_average_inbound_gftexception_ufoffloaded_udppackets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufoffloaded_udppackets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFOffloadFailedPackets
    pub fn set_average_inbound_gftexception_ufoffload_failed_packets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufoffload_failed_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFOffloadFailedPackets
    pub fn get_average_inbound_gftexception_ufoffload_failed_packets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufoffload_failed_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFOffloadPendingPackets
    pub fn set_average_inbound_gftexception_ufoffload_pending_packets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufoffload_pending_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFOffloadPendingPackets
    pub fn get_average_inbound_gftexception_ufoffload_pending_packets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufoffload_pending_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFOffloadRetryAwaitingPackets
    pub fn set_average_inbound_gftexception_ufoffload_retry_awaiting_packets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufoffload_retry_awaiting_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFOffloadRetryAwaitingPackets
    pub fn get_average_inbound_gftexception_ufoffload_retry_awaiting_packets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufoffload_retry_awaiting_packets.as_ref()
    }

    /// Sets the value of AverageInboundGFTExceptionUFPackets
    pub fn set_average_inbound_gftexception_ufpackets(&mut self, value: u64) {
        self.average_inbound_gftexception_ufpackets = Some(value);
    }

    /// Gets the value of AverageInboundGFTExceptionUFPackets
    pub fn get_average_inbound_gftexception_ufpackets(&self) -> Option<&u64> {
        self.average_inbound_gftexception_ufpackets.as_ref()
    }

    /// Sets the value of AverageInboundGFTTotalBytes
    pub fn set_average_inbound_gfttotal_bytes(&mut self, value: u64) {
        self.average_inbound_gfttotal_bytes = Some(value);
    }

    /// Gets the value of AverageInboundGFTTotalBytes
    pub fn get_average_inbound_gfttotal_bytes(&self) -> Option<&u64> {
        self.average_inbound_gfttotal_bytes.as_ref()
    }

    /// Sets the value of AverageInboundGFTTotalPackets
    pub fn set_average_inbound_gfttotal_packets(&mut self, value: u64) {
        self.average_inbound_gfttotal_packets = Some(value);
    }

    /// Gets the value of AverageInboundGFTTotalPackets
    pub fn get_average_inbound_gfttotal_packets(&self) -> Option<&u64> {
        self.average_inbound_gfttotal_packets.as_ref()
    }

    /// Sets the value of AverageInboundHairPinnedPackets
    pub fn set_average_inbound_hair_pinned_packets(&mut self, value: u64) {
        self.average_inbound_hair_pinned_packets = Some(value);
    }

    /// Gets the value of AverageInboundHairPinnedPackets
    pub fn get_average_inbound_hair_pinned_packets(&self) -> Option<&u64> {
        self.average_inbound_hair_pinned_packets.as_ref()
    }

    /// Sets the value of AverageInboundInterceptedPackets
    pub fn set_average_inbound_intercepted_packets(&mut self, value: u64) {
        self.average_inbound_intercepted_packets = Some(value);
    }

    /// Gets the value of AverageInboundInterceptedPackets
    pub fn get_average_inbound_intercepted_packets(&self) -> Option<&u64> {
        self.average_inbound_intercepted_packets.as_ref()
    }

    /// Sets the value of AverageInboundMissedInterceptedPackets
    pub fn set_average_inbound_missed_intercepted_packets(&mut self, value: u64) {
        self.average_inbound_missed_intercepted_packets = Some(value);
    }

    /// Gets the value of AverageInboundMissedInterceptedPackets
    pub fn get_average_inbound_missed_intercepted_packets(&self) -> Option<&u64> {
        self.average_inbound_missed_intercepted_packets.as_ref()
    }

    /// Sets the value of AverageInboundNonIPPackets
    pub fn set_average_inbound_non_ippackets(&mut self, value: u64) {
        self.average_inbound_non_ippackets = Some(value);
    }

    /// Gets the value of AverageInboundNonIPPackets
    pub fn get_average_inbound_non_ippackets(&self) -> Option<&u64> {
        self.average_inbound_non_ippackets.as_ref()
    }

    /// Sets the value of AverageInboundPackets
    pub fn set_average_inbound_packets(&mut self, value: u64) {
        self.average_inbound_packets = Some(value);
    }

    /// Gets the value of AverageInboundPackets
    pub fn get_average_inbound_packets(&self) -> Option<&u64> {
        self.average_inbound_packets.as_ref()
    }

    /// Sets the value of AverageInboundPendingPackets
    pub fn set_average_inbound_pending_packets(&mut self, value: u64) {
        self.average_inbound_pending_packets = Some(value);
    }

    /// Gets the value of AverageInboundPendingPackets
    pub fn get_average_inbound_pending_packets(&self) -> Option<&u64> {
        self.average_inbound_pending_packets.as_ref()
    }

    /// Sets the value of AverageInboundTCPSYNACKPackets
    pub fn set_average_inbound_tcpsynackpackets(&mut self, value: u64) {
        self.average_inbound_tcpsynackpackets = Some(value);
    }

    /// Gets the value of AverageInboundTCPSYNACKPackets
    pub fn get_average_inbound_tcpsynackpackets(&self) -> Option<&u64> {
        self.average_inbound_tcpsynackpackets.as_ref()
    }

    /// Sets the value of AverageInboundTCPSYNPackets
    pub fn set_average_inbound_tcpsynpackets(&mut self, value: u64) {
        self.average_inbound_tcpsynpackets = Some(value);
    }

    /// Gets the value of AverageInboundTCPSYNPackets
    pub fn get_average_inbound_tcpsynpackets(&self) -> Option<&u64> {
        self.average_inbound_tcpsynpackets.as_ref()
    }

    /// Sets the value of AverageInboundThrottledPackets
    pub fn set_average_inbound_throttled_packets(&mut self, value: u64) {
        self.average_inbound_throttled_packets = Some(value);
    }

    /// Gets the value of AverageInboundThrottledPackets
    pub fn get_average_inbound_throttled_packets(&self) -> Option<&u64> {
        self.average_inbound_throttled_packets.as_ref()
    }

    /// Sets the value of AverageInboundUnicastForwardedGFTExceptionPackets
    pub fn set_average_inbound_unicast_forwarded_gftexception_packets(&mut self, value: u64) {
        self.average_inbound_unicast_forwarded_gftexception_packets = Some(value);
    }

    /// Gets the value of AverageInboundUnicastForwardedGFTExceptionPackets
    pub fn get_average_inbound_unicast_forwarded_gftexception_packets(&self) -> Option<&u64> {
        self.average_inbound_unicast_forwarded_gftexception_packets.as_ref()
    }
}

