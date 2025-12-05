// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_VFPPortTotalInboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_VFPPortTotalInboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "TotalInboundBytes")]
    pub total_inbound_bytes: Option<u64>,

/// 
    #[serde(rename = "TotalInboundForwardedMulticastPackets")]
    pub total_inbound_forwarded_multicast_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundForwardedUnicastPackets")]
    pub total_inbound_forwarded_unicast_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTBytes")]
    pub total_inbound_gftbytes: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTCopyFINPackets")]
    pub total_inbound_gftcopy_finpackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTCopyPackets")]
    pub total_inbound_gftcopy_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTCopyResetPackets")]
    pub total_inbound_gftcopy_reset_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionPackets")]
    pub total_inbound_gftexception_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFOffloadBlockedPackets")]
    pub total_inbound_gftexception_ufoffload_blocked_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFOffloadDeferredPackets")]
    pub total_inbound_gftexception_ufoffload_deferred_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFOffloadedTCPPackets")]
    pub total_inbound_gftexception_ufoffloaded_tcppackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFOffloadedUDPPackets")]
    pub total_inbound_gftexception_ufoffloaded_udppackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFOffloadFailedPackets")]
    pub total_inbound_gftexception_ufoffload_failed_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFOffloadPendingPackets")]
    pub total_inbound_gftexception_ufoffload_pending_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFPackets")]
    pub total_inbound_gftexception_ufpackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTExceptionUFRetryAwaitingPackets")]
    pub total_inbound_gftexception_ufretry_awaiting_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundGFTPackets")]
    pub total_inbound_gftpackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundHairpinnedPackets")]
    pub total_inbound_hairpinned_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundInterceptedPackets")]
    pub total_inbound_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundMissedInterceptedPackets")]
    pub total_inbound_missed_intercepted_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundNonIPPackets")]
    pub total_inbound_non_ippackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundPackets")]
    pub total_inbound_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundPendingPackets")]
    pub total_inbound_pending_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundTCPSYNACKPackets")]
    pub total_inbound_tcpsynackpackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundTCPSYNPackets")]
    pub total_inbound_tcpsynpackets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundThrottledPackets")]
    pub total_inbound_throttled_packets: Option<u64>,

/// 
    #[serde(rename = "TotalInboundUnicastForwardedGFTExceptionPackets")]
    pub total_inbound_unicast_forwarded_gftexception_packets: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_VFPPortTotalInboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            total_inbound_bytes: None,
            total_inbound_forwarded_multicast_packets: None,
            total_inbound_forwarded_unicast_packets: None,
            total_inbound_gftbytes: None,
            total_inbound_gftcopy_finpackets: None,
            total_inbound_gftcopy_packets: None,
            total_inbound_gftcopy_reset_packets: None,
            total_inbound_gftexception_packets: None,
            total_inbound_gftexception_ufoffload_blocked_packets: None,
            total_inbound_gftexception_ufoffload_deferred_packets: None,
            total_inbound_gftexception_ufoffloaded_tcppackets: None,
            total_inbound_gftexception_ufoffloaded_udppackets: None,
            total_inbound_gftexception_ufoffload_failed_packets: None,
            total_inbound_gftexception_ufoffload_pending_packets: None,
            total_inbound_gftexception_ufpackets: None,
            total_inbound_gftexception_ufretry_awaiting_packets: None,
            total_inbound_gftpackets: None,
            total_inbound_hairpinned_packets: None,
            total_inbound_intercepted_packets: None,
            total_inbound_missed_intercepted_packets: None,
            total_inbound_non_ippackets: None,
            total_inbound_packets: None,
            total_inbound_pending_packets: None,
            total_inbound_tcpsynackpackets: None,
            total_inbound_tcpsynpackets: None,
            total_inbound_throttled_packets: None,
            total_inbound_unicast_forwarded_gftexception_packets: None,
        }
    }


    /// Sets the value of TotalInboundBytes
    pub fn set_total_inbound_bytes(&mut self, value: u64) {
        self.total_inbound_bytes = Some(value);
    }

    /// Gets the value of TotalInboundBytes
    pub fn get_total_inbound_bytes(&self) -> Option<&u64> {
        self.total_inbound_bytes.as_ref()
    }

    /// Sets the value of TotalInboundForwardedMulticastPackets
    pub fn set_total_inbound_forwarded_multicast_packets(&mut self, value: u64) {
        self.total_inbound_forwarded_multicast_packets = Some(value);
    }

    /// Gets the value of TotalInboundForwardedMulticastPackets
    pub fn get_total_inbound_forwarded_multicast_packets(&self) -> Option<&u64> {
        self.total_inbound_forwarded_multicast_packets.as_ref()
    }

    /// Sets the value of TotalInboundForwardedUnicastPackets
    pub fn set_total_inbound_forwarded_unicast_packets(&mut self, value: u64) {
        self.total_inbound_forwarded_unicast_packets = Some(value);
    }

    /// Gets the value of TotalInboundForwardedUnicastPackets
    pub fn get_total_inbound_forwarded_unicast_packets(&self) -> Option<&u64> {
        self.total_inbound_forwarded_unicast_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTBytes
    pub fn set_total_inbound_gftbytes(&mut self, value: u64) {
        self.total_inbound_gftbytes = Some(value);
    }

    /// Gets the value of TotalInboundGFTBytes
    pub fn get_total_inbound_gftbytes(&self) -> Option<&u64> {
        self.total_inbound_gftbytes.as_ref()
    }

    /// Sets the value of TotalInboundGFTCopyFINPackets
    pub fn set_total_inbound_gftcopy_finpackets(&mut self, value: u64) {
        self.total_inbound_gftcopy_finpackets = Some(value);
    }

    /// Gets the value of TotalInboundGFTCopyFINPackets
    pub fn get_total_inbound_gftcopy_finpackets(&self) -> Option<&u64> {
        self.total_inbound_gftcopy_finpackets.as_ref()
    }

    /// Sets the value of TotalInboundGFTCopyPackets
    pub fn set_total_inbound_gftcopy_packets(&mut self, value: u64) {
        self.total_inbound_gftcopy_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTCopyPackets
    pub fn get_total_inbound_gftcopy_packets(&self) -> Option<&u64> {
        self.total_inbound_gftcopy_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTCopyResetPackets
    pub fn set_total_inbound_gftcopy_reset_packets(&mut self, value: u64) {
        self.total_inbound_gftcopy_reset_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTCopyResetPackets
    pub fn get_total_inbound_gftcopy_reset_packets(&self) -> Option<&u64> {
        self.total_inbound_gftcopy_reset_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionPackets
    pub fn set_total_inbound_gftexception_packets(&mut self, value: u64) {
        self.total_inbound_gftexception_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionPackets
    pub fn get_total_inbound_gftexception_packets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFOffloadBlockedPackets
    pub fn set_total_inbound_gftexception_ufoffload_blocked_packets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufoffload_blocked_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFOffloadBlockedPackets
    pub fn get_total_inbound_gftexception_ufoffload_blocked_packets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufoffload_blocked_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFOffloadDeferredPackets
    pub fn set_total_inbound_gftexception_ufoffload_deferred_packets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufoffload_deferred_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFOffloadDeferredPackets
    pub fn get_total_inbound_gftexception_ufoffload_deferred_packets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufoffload_deferred_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFOffloadedTCPPackets
    pub fn set_total_inbound_gftexception_ufoffloaded_tcppackets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufoffloaded_tcppackets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFOffloadedTCPPackets
    pub fn get_total_inbound_gftexception_ufoffloaded_tcppackets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufoffloaded_tcppackets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFOffloadedUDPPackets
    pub fn set_total_inbound_gftexception_ufoffloaded_udppackets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufoffloaded_udppackets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFOffloadedUDPPackets
    pub fn get_total_inbound_gftexception_ufoffloaded_udppackets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufoffloaded_udppackets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFOffloadFailedPackets
    pub fn set_total_inbound_gftexception_ufoffload_failed_packets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufoffload_failed_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFOffloadFailedPackets
    pub fn get_total_inbound_gftexception_ufoffload_failed_packets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufoffload_failed_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFOffloadPendingPackets
    pub fn set_total_inbound_gftexception_ufoffload_pending_packets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufoffload_pending_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFOffloadPendingPackets
    pub fn get_total_inbound_gftexception_ufoffload_pending_packets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufoffload_pending_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFPackets
    pub fn set_total_inbound_gftexception_ufpackets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufpackets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFPackets
    pub fn get_total_inbound_gftexception_ufpackets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufpackets.as_ref()
    }

    /// Sets the value of TotalInboundGFTExceptionUFRetryAwaitingPackets
    pub fn set_total_inbound_gftexception_ufretry_awaiting_packets(&mut self, value: u64) {
        self.total_inbound_gftexception_ufretry_awaiting_packets = Some(value);
    }

    /// Gets the value of TotalInboundGFTExceptionUFRetryAwaitingPackets
    pub fn get_total_inbound_gftexception_ufretry_awaiting_packets(&self) -> Option<&u64> {
        self.total_inbound_gftexception_ufretry_awaiting_packets.as_ref()
    }

    /// Sets the value of TotalInboundGFTPackets
    pub fn set_total_inbound_gftpackets(&mut self, value: u64) {
        self.total_inbound_gftpackets = Some(value);
    }

    /// Gets the value of TotalInboundGFTPackets
    pub fn get_total_inbound_gftpackets(&self) -> Option<&u64> {
        self.total_inbound_gftpackets.as_ref()
    }

    /// Sets the value of TotalInboundHairpinnedPackets
    pub fn set_total_inbound_hairpinned_packets(&mut self, value: u64) {
        self.total_inbound_hairpinned_packets = Some(value);
    }

    /// Gets the value of TotalInboundHairpinnedPackets
    pub fn get_total_inbound_hairpinned_packets(&self) -> Option<&u64> {
        self.total_inbound_hairpinned_packets.as_ref()
    }

    /// Sets the value of TotalInboundInterceptedPackets
    pub fn set_total_inbound_intercepted_packets(&mut self, value: u64) {
        self.total_inbound_intercepted_packets = Some(value);
    }

    /// Gets the value of TotalInboundInterceptedPackets
    pub fn get_total_inbound_intercepted_packets(&self) -> Option<&u64> {
        self.total_inbound_intercepted_packets.as_ref()
    }

    /// Sets the value of TotalInboundMissedInterceptedPackets
    pub fn set_total_inbound_missed_intercepted_packets(&mut self, value: u64) {
        self.total_inbound_missed_intercepted_packets = Some(value);
    }

    /// Gets the value of TotalInboundMissedInterceptedPackets
    pub fn get_total_inbound_missed_intercepted_packets(&self) -> Option<&u64> {
        self.total_inbound_missed_intercepted_packets.as_ref()
    }

    /// Sets the value of TotalInboundNonIPPackets
    pub fn set_total_inbound_non_ippackets(&mut self, value: u64) {
        self.total_inbound_non_ippackets = Some(value);
    }

    /// Gets the value of TotalInboundNonIPPackets
    pub fn get_total_inbound_non_ippackets(&self) -> Option<&u64> {
        self.total_inbound_non_ippackets.as_ref()
    }

    /// Sets the value of TotalInboundPackets
    pub fn set_total_inbound_packets(&mut self, value: u64) {
        self.total_inbound_packets = Some(value);
    }

    /// Gets the value of TotalInboundPackets
    pub fn get_total_inbound_packets(&self) -> Option<&u64> {
        self.total_inbound_packets.as_ref()
    }

    /// Sets the value of TotalInboundPendingPackets
    pub fn set_total_inbound_pending_packets(&mut self, value: u64) {
        self.total_inbound_pending_packets = Some(value);
    }

    /// Gets the value of TotalInboundPendingPackets
    pub fn get_total_inbound_pending_packets(&self) -> Option<&u64> {
        self.total_inbound_pending_packets.as_ref()
    }

    /// Sets the value of TotalInboundTCPSYNACKPackets
    pub fn set_total_inbound_tcpsynackpackets(&mut self, value: u64) {
        self.total_inbound_tcpsynackpackets = Some(value);
    }

    /// Gets the value of TotalInboundTCPSYNACKPackets
    pub fn get_total_inbound_tcpsynackpackets(&self) -> Option<&u64> {
        self.total_inbound_tcpsynackpackets.as_ref()
    }

    /// Sets the value of TotalInboundTCPSYNPackets
    pub fn set_total_inbound_tcpsynpackets(&mut self, value: u64) {
        self.total_inbound_tcpsynpackets = Some(value);
    }

    /// Gets the value of TotalInboundTCPSYNPackets
    pub fn get_total_inbound_tcpsynpackets(&self) -> Option<&u64> {
        self.total_inbound_tcpsynpackets.as_ref()
    }

    /// Sets the value of TotalInboundThrottledPackets
    pub fn set_total_inbound_throttled_packets(&mut self, value: u64) {
        self.total_inbound_throttled_packets = Some(value);
    }

    /// Gets the value of TotalInboundThrottledPackets
    pub fn get_total_inbound_throttled_packets(&self) -> Option<&u64> {
        self.total_inbound_throttled_packets.as_ref()
    }

    /// Sets the value of TotalInboundUnicastForwardedGFTExceptionPackets
    pub fn set_total_inbound_unicast_forwarded_gftexception_packets(&mut self, value: u64) {
        self.total_inbound_unicast_forwarded_gftexception_packets = Some(value);
    }

    /// Gets the value of TotalInboundUnicastForwardedGFTExceptionPackets
    pub fn get_total_inbound_unicast_forwarded_gftexception_packets(&self) -> Option<&u64> {
        self.total_inbound_unicast_forwarded_gftexception_packets.as_ref()
    }
}

