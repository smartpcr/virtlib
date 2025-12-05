// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_VFPPortAverageOutboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_VFPPortAverageOutboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AverageGFTOutboundBytes")]
    pub average_gftoutbound_bytes: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundBytes")]
    pub average_outbound_bytes: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundForwardedMulticastPackets")]
    pub average_outbound_forwarded_multicast_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundForwardedUnicastPackets")]
    pub average_outbound_forwarded_unicast_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTCopyFINPackets")]
    pub average_outbound_gftcopy_finpackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTCopyPackets")]
    pub average_outbound_gftcopy_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTCopyResetPackets")]
    pub average_outbound_gftcopy_reset_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionPackets")]
    pub average_outbound_gftexception_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFOffloadBlockedPackets")]
    pub average_outbound_gftexception_ufoffload_blocked_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFOffloadDeferredPackets")]
    pub average_outbound_gftexception_ufoffload_deferred_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFOffloadedTCPPackets")]
    pub average_outbound_gftexception_ufoffloaded_tcppackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFOffloadedUDPPackets")]
    pub average_outbound_gftexception_ufoffloaded_udppackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFOffloadFailedPackets")]
    pub average_outbound_gftexception_ufoffload_failed_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFOffloadPendingPackets")]
    pub average_outbound_gftexception_ufoffload_pending_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFOffloadRetryAwaitingPackets")]
    pub average_outbound_gftexception_ufoffload_retry_awaiting_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTExceptionUFPackets")]
    pub average_outbound_gftexception_ufpackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundGFTPackets")]
    pub average_outbound_gftpackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundHairpinnedPackets")]
    pub average_outbound_hairpinned_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundInterceptedPackets")]
    pub average_outbound_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundMissedInterceptedPackets")]
    pub average_outbound_missed_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundNonIPPackets")]
    pub average_outbound_non_ippackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPackets")]
    pub average_outbound_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPendingPackets")]
    pub average_outbound_pending_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundTCPSYNACKPackets")]
    pub average_outbound_tcpsynackpackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundTCPSYNPackets")]
    pub average_outbound_tcpsynpackets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundThrottledPackets")]
    pub average_outbound_throttled_packets: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundUnicastForwardedGFTExceptionPackets")]
    pub average_outbound_unicast_forwarded_gftexception_packets: Option<u64>,
}

impl Win32_PerfRawData_Counters_VFPPortAverageOutboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            average_gftoutbound_bytes: None,
            average_outbound_bytes: None,
            average_outbound_forwarded_multicast_packets: None,
            average_outbound_forwarded_unicast_packets: None,
            average_outbound_gftcopy_finpackets: None,
            average_outbound_gftcopy_packets: None,
            average_outbound_gftcopy_reset_packets: None,
            average_outbound_gftexception_packets: None,
            average_outbound_gftexception_ufoffload_blocked_packets: None,
            average_outbound_gftexception_ufoffload_deferred_packets: None,
            average_outbound_gftexception_ufoffloaded_tcppackets: None,
            average_outbound_gftexception_ufoffloaded_udppackets: None,
            average_outbound_gftexception_ufoffload_failed_packets: None,
            average_outbound_gftexception_ufoffload_pending_packets: None,
            average_outbound_gftexception_ufoffload_retry_awaiting_packets: None,
            average_outbound_gftexception_ufpackets: None,
            average_outbound_gftpackets: None,
            average_outbound_hairpinned_packets: None,
            average_outbound_intercepted_packets: None,
            average_outbound_missed_intercepted_packets: None,
            average_outbound_non_ippackets: None,
            average_outbound_packets: None,
            average_outbound_pending_packets: None,
            average_outbound_tcpsynackpackets: None,
            average_outbound_tcpsynpackets: None,
            average_outbound_throttled_packets: None,
            average_outbound_unicast_forwarded_gftexception_packets: None,
        }
    }


    /// Sets the value of AverageGFTOutboundBytes
    pub fn set_average_gftoutbound_bytes(&mut self, value: u64) {
        self.average_gftoutbound_bytes = Some(value);
    }

    /// Gets the value of AverageGFTOutboundBytes
    pub fn get_average_gftoutbound_bytes(&self) -> Option<&u64> {
        self.average_gftoutbound_bytes.as_ref()
    }

    /// Sets the value of AverageOutboundBytes
    pub fn set_average_outbound_bytes(&mut self, value: u64) {
        self.average_outbound_bytes = Some(value);
    }

    /// Gets the value of AverageOutboundBytes
    pub fn get_average_outbound_bytes(&self) -> Option<&u64> {
        self.average_outbound_bytes.as_ref()
    }

    /// Sets the value of AverageOutboundForwardedMulticastPackets
    pub fn set_average_outbound_forwarded_multicast_packets(&mut self, value: u64) {
        self.average_outbound_forwarded_multicast_packets = Some(value);
    }

    /// Gets the value of AverageOutboundForwardedMulticastPackets
    pub fn get_average_outbound_forwarded_multicast_packets(&self) -> Option<&u64> {
        self.average_outbound_forwarded_multicast_packets.as_ref()
    }

    /// Sets the value of AverageOutboundForwardedUnicastPackets
    pub fn set_average_outbound_forwarded_unicast_packets(&mut self, value: u64) {
        self.average_outbound_forwarded_unicast_packets = Some(value);
    }

    /// Gets the value of AverageOutboundForwardedUnicastPackets
    pub fn get_average_outbound_forwarded_unicast_packets(&self) -> Option<&u64> {
        self.average_outbound_forwarded_unicast_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTCopyFINPackets
    pub fn set_average_outbound_gftcopy_finpackets(&mut self, value: u64) {
        self.average_outbound_gftcopy_finpackets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTCopyFINPackets
    pub fn get_average_outbound_gftcopy_finpackets(&self) -> Option<&u64> {
        self.average_outbound_gftcopy_finpackets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTCopyPackets
    pub fn set_average_outbound_gftcopy_packets(&mut self, value: u64) {
        self.average_outbound_gftcopy_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTCopyPackets
    pub fn get_average_outbound_gftcopy_packets(&self) -> Option<&u64> {
        self.average_outbound_gftcopy_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTCopyResetPackets
    pub fn set_average_outbound_gftcopy_reset_packets(&mut self, value: u64) {
        self.average_outbound_gftcopy_reset_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTCopyResetPackets
    pub fn get_average_outbound_gftcopy_reset_packets(&self) -> Option<&u64> {
        self.average_outbound_gftcopy_reset_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionPackets
    pub fn set_average_outbound_gftexception_packets(&mut self, value: u64) {
        self.average_outbound_gftexception_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionPackets
    pub fn get_average_outbound_gftexception_packets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFOffloadBlockedPackets
    pub fn set_average_outbound_gftexception_ufoffload_blocked_packets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufoffload_blocked_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFOffloadBlockedPackets
    pub fn get_average_outbound_gftexception_ufoffload_blocked_packets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufoffload_blocked_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFOffloadDeferredPackets
    pub fn set_average_outbound_gftexception_ufoffload_deferred_packets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufoffload_deferred_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFOffloadDeferredPackets
    pub fn get_average_outbound_gftexception_ufoffload_deferred_packets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufoffload_deferred_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFOffloadedTCPPackets
    pub fn set_average_outbound_gftexception_ufoffloaded_tcppackets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufoffloaded_tcppackets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFOffloadedTCPPackets
    pub fn get_average_outbound_gftexception_ufoffloaded_tcppackets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufoffloaded_tcppackets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFOffloadedUDPPackets
    pub fn set_average_outbound_gftexception_ufoffloaded_udppackets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufoffloaded_udppackets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFOffloadedUDPPackets
    pub fn get_average_outbound_gftexception_ufoffloaded_udppackets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufoffloaded_udppackets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFOffloadFailedPackets
    pub fn set_average_outbound_gftexception_ufoffload_failed_packets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufoffload_failed_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFOffloadFailedPackets
    pub fn get_average_outbound_gftexception_ufoffload_failed_packets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufoffload_failed_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFOffloadPendingPackets
    pub fn set_average_outbound_gftexception_ufoffload_pending_packets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufoffload_pending_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFOffloadPendingPackets
    pub fn get_average_outbound_gftexception_ufoffload_pending_packets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufoffload_pending_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFOffloadRetryAwaitingPackets
    pub fn set_average_outbound_gftexception_ufoffload_retry_awaiting_packets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufoffload_retry_awaiting_packets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFOffloadRetryAwaitingPackets
    pub fn get_average_outbound_gftexception_ufoffload_retry_awaiting_packets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufoffload_retry_awaiting_packets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTExceptionUFPackets
    pub fn set_average_outbound_gftexception_ufpackets(&mut self, value: u64) {
        self.average_outbound_gftexception_ufpackets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTExceptionUFPackets
    pub fn get_average_outbound_gftexception_ufpackets(&self) -> Option<&u64> {
        self.average_outbound_gftexception_ufpackets.as_ref()
    }

    /// Sets the value of AverageOutboundGFTPackets
    pub fn set_average_outbound_gftpackets(&mut self, value: u64) {
        self.average_outbound_gftpackets = Some(value);
    }

    /// Gets the value of AverageOutboundGFTPackets
    pub fn get_average_outbound_gftpackets(&self) -> Option<&u64> {
        self.average_outbound_gftpackets.as_ref()
    }

    /// Sets the value of AverageOutboundHairpinnedPackets
    pub fn set_average_outbound_hairpinned_packets(&mut self, value: u64) {
        self.average_outbound_hairpinned_packets = Some(value);
    }

    /// Gets the value of AverageOutboundHairpinnedPackets
    pub fn get_average_outbound_hairpinned_packets(&self) -> Option<&u64> {
        self.average_outbound_hairpinned_packets.as_ref()
    }

    /// Sets the value of AverageOutboundInterceptedPackets
    pub fn set_average_outbound_intercepted_packets(&mut self, value: u64) {
        self.average_outbound_intercepted_packets = Some(value);
    }

    /// Gets the value of AverageOutboundInterceptedPackets
    pub fn get_average_outbound_intercepted_packets(&self) -> Option<&u64> {
        self.average_outbound_intercepted_packets.as_ref()
    }

    /// Sets the value of AverageOutboundMissedInterceptedPackets
    pub fn set_average_outbound_missed_intercepted_packets(&mut self, value: u64) {
        self.average_outbound_missed_intercepted_packets = Some(value);
    }

    /// Gets the value of AverageOutboundMissedInterceptedPackets
    pub fn get_average_outbound_missed_intercepted_packets(&self) -> Option<&u64> {
        self.average_outbound_missed_intercepted_packets.as_ref()
    }

    /// Sets the value of AverageOutboundNonIPPackets
    pub fn set_average_outbound_non_ippackets(&mut self, value: u64) {
        self.average_outbound_non_ippackets = Some(value);
    }

    /// Gets the value of AverageOutboundNonIPPackets
    pub fn get_average_outbound_non_ippackets(&self) -> Option<&u64> {
        self.average_outbound_non_ippackets.as_ref()
    }

    /// Sets the value of AverageOutboundPackets
    pub fn set_average_outbound_packets(&mut self, value: u64) {
        self.average_outbound_packets = Some(value);
    }

    /// Gets the value of AverageOutboundPackets
    pub fn get_average_outbound_packets(&self) -> Option<&u64> {
        self.average_outbound_packets.as_ref()
    }

    /// Sets the value of AverageOutboundPendingPackets
    pub fn set_average_outbound_pending_packets(&mut self, value: u64) {
        self.average_outbound_pending_packets = Some(value);
    }

    /// Gets the value of AverageOutboundPendingPackets
    pub fn get_average_outbound_pending_packets(&self) -> Option<&u64> {
        self.average_outbound_pending_packets.as_ref()
    }

    /// Sets the value of AverageOutboundTCPSYNACKPackets
    pub fn set_average_outbound_tcpsynackpackets(&mut self, value: u64) {
        self.average_outbound_tcpsynackpackets = Some(value);
    }

    /// Gets the value of AverageOutboundTCPSYNACKPackets
    pub fn get_average_outbound_tcpsynackpackets(&self) -> Option<&u64> {
        self.average_outbound_tcpsynackpackets.as_ref()
    }

    /// Sets the value of AverageOutboundTCPSYNPackets
    pub fn set_average_outbound_tcpsynpackets(&mut self, value: u64) {
        self.average_outbound_tcpsynpackets = Some(value);
    }

    /// Gets the value of AverageOutboundTCPSYNPackets
    pub fn get_average_outbound_tcpsynpackets(&self) -> Option<&u64> {
        self.average_outbound_tcpsynpackets.as_ref()
    }

    /// Sets the value of AverageOutboundThrottledPackets
    pub fn set_average_outbound_throttled_packets(&mut self, value: u64) {
        self.average_outbound_throttled_packets = Some(value);
    }

    /// Gets the value of AverageOutboundThrottledPackets
    pub fn get_average_outbound_throttled_packets(&self) -> Option<&u64> {
        self.average_outbound_throttled_packets.as_ref()
    }

    /// Sets the value of AverageOutboundUnicastForwardedGFTExceptionPackets
    pub fn set_average_outbound_unicast_forwarded_gftexception_packets(&mut self, value: u64) {
        self.average_outbound_unicast_forwarded_gftexception_packets = Some(value);
    }

    /// Gets the value of AverageOutboundUnicastForwardedGFTExceptionPackets
    pub fn get_average_outbound_unicast_forwarded_gftexception_packets(&self) -> Option<&u64> {
        self.average_outbound_unicast_forwarded_gftexception_packets.as_ref()
    }
}

