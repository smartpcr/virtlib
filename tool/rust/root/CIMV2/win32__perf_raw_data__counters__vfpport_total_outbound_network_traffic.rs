// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_VFPPortTotalOutboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_VFPPortTotalOutboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TotalOutboundBytes")]
    pub total_outbound_bytes: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundForwardedMulticastPackets")]
    pub total_outbound_forwarded_multicast_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundForwardedUnicastPackets")]
    pub total_outbound_forwarded_unicast_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTBytes")]
    pub total_outbound_gftbytes: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTCopyFINPackets")]
    pub total_outbound_gftcopy_finpackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTCopyPackets")]
    pub total_outbound_gftcopy_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTCopyResetPackets")]
    pub total_outbound_gftcopy_reset_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionPackets")]
    pub total_outbound_gftexception_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionUFOffloadBlockedPackets")]
    pub total_outbound_gftexception_ufoffload_blocked_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionUFOffloadDeferredPackets")]
    pub total_outbound_gftexception_ufoffload_deferred_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionUFOffloadedTCPPackets")]
    pub total_outbound_gftexception_ufoffloaded_tcppackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionUFOffloadedUDPPackets")]
    pub total_outbound_gftexception_ufoffloaded_udppackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionUFOffloadFailedPackets")]
    pub total_outbound_gftexception_ufoffload_failed_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionUFOffloadPendingPackets")]
    pub total_outbound_gftexception_ufoffload_pending_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTExceptionUFPackets")]
    pub total_outbound_gftexception_ufpackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGFTRetryAwaitingPackets")]
    pub total_outbound_gftretry_awaiting_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundGftTotalPackets")]
    pub total_outbound_gft_total_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundHairPinnedPackets")]
    pub total_outbound_hair_pinned_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundInterceptedPackets")]
    pub total_outbound_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundMissedInterceptedPackets")]
    pub total_outbound_missed_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundNonIPPackets")]
    pub total_outbound_non_ippackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundPackets")]
    pub total_outbound_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundPendingPackets")]
    pub total_outbound_pending_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundTCPSYNACKPackets")]
    pub total_outbound_tcpsynackpackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundTCPSYNPackets")]
    pub total_outbound_tcpsynpackets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundThrottledPackets")]
    pub total_outbound_throttled_packets: Option<u64>,

/// 
    #[serde(rename = "TotalOutboundUnicastForwardedGFTExceptionPackets")]
    pub total_outbound_unicast_forwarded_gftexception_packets: Option<u64>,
}

impl Win32_PerfRawData_Counters_VFPPortTotalOutboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            total_outbound_bytes: None,
            total_outbound_forwarded_multicast_packets: None,
            total_outbound_forwarded_unicast_packets: None,
            total_outbound_gftbytes: None,
            total_outbound_gftcopy_finpackets: None,
            total_outbound_gftcopy_packets: None,
            total_outbound_gftcopy_reset_packets: None,
            total_outbound_gftexception_packets: None,
            total_outbound_gftexception_ufoffload_blocked_packets: None,
            total_outbound_gftexception_ufoffload_deferred_packets: None,
            total_outbound_gftexception_ufoffloaded_tcppackets: None,
            total_outbound_gftexception_ufoffloaded_udppackets: None,
            total_outbound_gftexception_ufoffload_failed_packets: None,
            total_outbound_gftexception_ufoffload_pending_packets: None,
            total_outbound_gftexception_ufpackets: None,
            total_outbound_gftretry_awaiting_packets: None,
            total_outbound_gft_total_packets: None,
            total_outbound_hair_pinned_packets: None,
            total_outbound_intercepted_packets: None,
            total_outbound_missed_intercepted_packets: None,
            total_outbound_non_ippackets: None,
            total_outbound_packets: None,
            total_outbound_pending_packets: None,
            total_outbound_tcpsynackpackets: None,
            total_outbound_tcpsynpackets: None,
            total_outbound_throttled_packets: None,
            total_outbound_unicast_forwarded_gftexception_packets: None,
        }
    }


    /// Sets the value of TotalOutboundBytes
    pub fn set_total_outbound_bytes(&mut self, value: u64) {
        self.total_outbound_bytes = Some(value);
    }

    /// Gets the value of TotalOutboundBytes
    pub fn get_total_outbound_bytes(&self) -> Option<&u64> {
        self.total_outbound_bytes.as_ref()
    }

    /// Sets the value of TotalOutboundForwardedMulticastPackets
    pub fn set_total_outbound_forwarded_multicast_packets(&mut self, value: u64) {
        self.total_outbound_forwarded_multicast_packets = Some(value);
    }

    /// Gets the value of TotalOutboundForwardedMulticastPackets
    pub fn get_total_outbound_forwarded_multicast_packets(&self) -> Option<&u64> {
        self.total_outbound_forwarded_multicast_packets.as_ref()
    }

    /// Sets the value of TotalOutboundForwardedUnicastPackets
    pub fn set_total_outbound_forwarded_unicast_packets(&mut self, value: u64) {
        self.total_outbound_forwarded_unicast_packets = Some(value);
    }

    /// Gets the value of TotalOutboundForwardedUnicastPackets
    pub fn get_total_outbound_forwarded_unicast_packets(&self) -> Option<&u64> {
        self.total_outbound_forwarded_unicast_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTBytes
    pub fn set_total_outbound_gftbytes(&mut self, value: u64) {
        self.total_outbound_gftbytes = Some(value);
    }

    /// Gets the value of TotalOutboundGFTBytes
    pub fn get_total_outbound_gftbytes(&self) -> Option<&u64> {
        self.total_outbound_gftbytes.as_ref()
    }

    /// Sets the value of TotalOutboundGFTCopyFINPackets
    pub fn set_total_outbound_gftcopy_finpackets(&mut self, value: u64) {
        self.total_outbound_gftcopy_finpackets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTCopyFINPackets
    pub fn get_total_outbound_gftcopy_finpackets(&self) -> Option<&u64> {
        self.total_outbound_gftcopy_finpackets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTCopyPackets
    pub fn set_total_outbound_gftcopy_packets(&mut self, value: u64) {
        self.total_outbound_gftcopy_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTCopyPackets
    pub fn get_total_outbound_gftcopy_packets(&self) -> Option<&u64> {
        self.total_outbound_gftcopy_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTCopyResetPackets
    pub fn set_total_outbound_gftcopy_reset_packets(&mut self, value: u64) {
        self.total_outbound_gftcopy_reset_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTCopyResetPackets
    pub fn get_total_outbound_gftcopy_reset_packets(&self) -> Option<&u64> {
        self.total_outbound_gftcopy_reset_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionPackets
    pub fn set_total_outbound_gftexception_packets(&mut self, value: u64) {
        self.total_outbound_gftexception_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionPackets
    pub fn get_total_outbound_gftexception_packets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionUFOffloadBlockedPackets
    pub fn set_total_outbound_gftexception_ufoffload_blocked_packets(&mut self, value: u64) {
        self.total_outbound_gftexception_ufoffload_blocked_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionUFOffloadBlockedPackets
    pub fn get_total_outbound_gftexception_ufoffload_blocked_packets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_ufoffload_blocked_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionUFOffloadDeferredPackets
    pub fn set_total_outbound_gftexception_ufoffload_deferred_packets(&mut self, value: u64) {
        self.total_outbound_gftexception_ufoffload_deferred_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionUFOffloadDeferredPackets
    pub fn get_total_outbound_gftexception_ufoffload_deferred_packets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_ufoffload_deferred_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionUFOffloadedTCPPackets
    pub fn set_total_outbound_gftexception_ufoffloaded_tcppackets(&mut self, value: u64) {
        self.total_outbound_gftexception_ufoffloaded_tcppackets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionUFOffloadedTCPPackets
    pub fn get_total_outbound_gftexception_ufoffloaded_tcppackets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_ufoffloaded_tcppackets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionUFOffloadedUDPPackets
    pub fn set_total_outbound_gftexception_ufoffloaded_udppackets(&mut self, value: u64) {
        self.total_outbound_gftexception_ufoffloaded_udppackets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionUFOffloadedUDPPackets
    pub fn get_total_outbound_gftexception_ufoffloaded_udppackets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_ufoffloaded_udppackets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionUFOffloadFailedPackets
    pub fn set_total_outbound_gftexception_ufoffload_failed_packets(&mut self, value: u64) {
        self.total_outbound_gftexception_ufoffload_failed_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionUFOffloadFailedPackets
    pub fn get_total_outbound_gftexception_ufoffload_failed_packets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_ufoffload_failed_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionUFOffloadPendingPackets
    pub fn set_total_outbound_gftexception_ufoffload_pending_packets(&mut self, value: u64) {
        self.total_outbound_gftexception_ufoffload_pending_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionUFOffloadPendingPackets
    pub fn get_total_outbound_gftexception_ufoffload_pending_packets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_ufoffload_pending_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTExceptionUFPackets
    pub fn set_total_outbound_gftexception_ufpackets(&mut self, value: u64) {
        self.total_outbound_gftexception_ufpackets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTExceptionUFPackets
    pub fn get_total_outbound_gftexception_ufpackets(&self) -> Option<&u64> {
        self.total_outbound_gftexception_ufpackets.as_ref()
    }

    /// Sets the value of TotalOutboundGFTRetryAwaitingPackets
    pub fn set_total_outbound_gftretry_awaiting_packets(&mut self, value: u64) {
        self.total_outbound_gftretry_awaiting_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGFTRetryAwaitingPackets
    pub fn get_total_outbound_gftretry_awaiting_packets(&self) -> Option<&u64> {
        self.total_outbound_gftretry_awaiting_packets.as_ref()
    }

    /// Sets the value of TotalOutboundGftTotalPackets
    pub fn set_total_outbound_gft_total_packets(&mut self, value: u64) {
        self.total_outbound_gft_total_packets = Some(value);
    }

    /// Gets the value of TotalOutboundGftTotalPackets
    pub fn get_total_outbound_gft_total_packets(&self) -> Option<&u64> {
        self.total_outbound_gft_total_packets.as_ref()
    }

    /// Sets the value of TotalOutboundHairPinnedPackets
    pub fn set_total_outbound_hair_pinned_packets(&mut self, value: u64) {
        self.total_outbound_hair_pinned_packets = Some(value);
    }

    /// Gets the value of TotalOutboundHairPinnedPackets
    pub fn get_total_outbound_hair_pinned_packets(&self) -> Option<&u64> {
        self.total_outbound_hair_pinned_packets.as_ref()
    }

    /// Sets the value of TotalOutboundInterceptedPackets
    pub fn set_total_outbound_intercepted_packets(&mut self, value: u64) {
        self.total_outbound_intercepted_packets = Some(value);
    }

    /// Gets the value of TotalOutboundInterceptedPackets
    pub fn get_total_outbound_intercepted_packets(&self) -> Option<&u64> {
        self.total_outbound_intercepted_packets.as_ref()
    }

    /// Sets the value of TotalOutboundMissedInterceptedPackets
    pub fn set_total_outbound_missed_intercepted_packets(&mut self, value: u64) {
        self.total_outbound_missed_intercepted_packets = Some(value);
    }

    /// Gets the value of TotalOutboundMissedInterceptedPackets
    pub fn get_total_outbound_missed_intercepted_packets(&self) -> Option<&u64> {
        self.total_outbound_missed_intercepted_packets.as_ref()
    }

    /// Sets the value of TotalOutboundNonIPPackets
    pub fn set_total_outbound_non_ippackets(&mut self, value: u64) {
        self.total_outbound_non_ippackets = Some(value);
    }

    /// Gets the value of TotalOutboundNonIPPackets
    pub fn get_total_outbound_non_ippackets(&self) -> Option<&u64> {
        self.total_outbound_non_ippackets.as_ref()
    }

    /// Sets the value of TotalOutboundPackets
    pub fn set_total_outbound_packets(&mut self, value: u64) {
        self.total_outbound_packets = Some(value);
    }

    /// Gets the value of TotalOutboundPackets
    pub fn get_total_outbound_packets(&self) -> Option<&u64> {
        self.total_outbound_packets.as_ref()
    }

    /// Sets the value of TotalOutboundPendingPackets
    pub fn set_total_outbound_pending_packets(&mut self, value: u64) {
        self.total_outbound_pending_packets = Some(value);
    }

    /// Gets the value of TotalOutboundPendingPackets
    pub fn get_total_outbound_pending_packets(&self) -> Option<&u64> {
        self.total_outbound_pending_packets.as_ref()
    }

    /// Sets the value of TotalOutboundTCPSYNACKPackets
    pub fn set_total_outbound_tcpsynackpackets(&mut self, value: u64) {
        self.total_outbound_tcpsynackpackets = Some(value);
    }

    /// Gets the value of TotalOutboundTCPSYNACKPackets
    pub fn get_total_outbound_tcpsynackpackets(&self) -> Option<&u64> {
        self.total_outbound_tcpsynackpackets.as_ref()
    }

    /// Sets the value of TotalOutboundTCPSYNPackets
    pub fn set_total_outbound_tcpsynpackets(&mut self, value: u64) {
        self.total_outbound_tcpsynpackets = Some(value);
    }

    /// Gets the value of TotalOutboundTCPSYNPackets
    pub fn get_total_outbound_tcpsynpackets(&self) -> Option<&u64> {
        self.total_outbound_tcpsynpackets.as_ref()
    }

    /// Sets the value of TotalOutboundThrottledPackets
    pub fn set_total_outbound_throttled_packets(&mut self, value: u64) {
        self.total_outbound_throttled_packets = Some(value);
    }

    /// Gets the value of TotalOutboundThrottledPackets
    pub fn get_total_outbound_throttled_packets(&self) -> Option<&u64> {
        self.total_outbound_throttled_packets.as_ref()
    }

    /// Sets the value of TotalOutboundUnicastForwardedGFTExceptionPackets
    pub fn set_total_outbound_unicast_forwarded_gftexception_packets(&mut self, value: u64) {
        self.total_outbound_unicast_forwarded_gftexception_packets = Some(value);
    }

    /// Gets the value of TotalOutboundUnicastForwardedGFTExceptionPackets
    pub fn get_total_outbound_unicast_forwarded_gftexception_packets(&self) -> Option<&u64> {
        self.total_outbound_unicast_forwarded_gftexception_packets.as_ref()
    }
}

