// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_RemoteFXNetwork struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_RemoteFXNetwork {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BaseTCPRTT")]
    pub base_tcprtt: Option<u32>,

/// 
    #[serde(rename = "BaseUDPRTT")]
    pub base_udprtt: Option<u32>,

/// 
    #[serde(rename = "CurrentTCPBandwidth")]
    pub current_tcpbandwidth: Option<u32>,

/// 
    #[serde(rename = "CurrentTCPRTT")]
    pub current_tcprtt: Option<u32>,

/// 
    #[serde(rename = "CurrentUDPBandwidth")]
    pub current_udpbandwidth: Option<u32>,

/// 
    #[serde(rename = "CurrentUDPRTT")]
    pub current_udprtt: Option<u32>,

/// 
    #[serde(rename = "FECRate")]
    pub fecrate: Option<u32>,

/// 
    #[serde(rename = "FECRate_Base")]
    pub fecrate__base: Option<u32>,

/// 
    #[serde(rename = "LossRate")]
    pub loss_rate: Option<u32>,

/// 
    #[serde(rename = "LossRate_Base")]
    pub loss_rate__base: Option<u32>,

/// 
    #[serde(rename = "RetransmissionRate")]
    pub retransmission_rate: Option<u32>,

/// 
    #[serde(rename = "RetransmissionRate_Base")]
    pub retransmission_rate__base: Option<u32>,

/// 
    #[serde(rename = "SentRateP0")]
    pub sent_rate_p0: Option<u32>,

/// 
    #[serde(rename = "SentRateP1")]
    pub sent_rate_p1: Option<u32>,

/// 
    #[serde(rename = "SentRateP2")]
    pub sent_rate_p2: Option<u32>,

/// 
    #[serde(rename = "SentRateP3")]
    pub sent_rate_p3: Option<u32>,

/// 
    #[serde(rename = "TCPReceivedRate")]
    pub tcpreceived_rate: Option<u32>,

/// 
    #[serde(rename = "TCPSentRate")]
    pub tcpsent_rate: Option<u32>,

/// 
    #[serde(rename = "TotalReceivedBytes")]
    pub total_received_bytes: Option<u32>,

/// 
    #[serde(rename = "TotalReceivedRate")]
    pub total_received_rate: Option<u32>,

/// 
    #[serde(rename = "TotalSentBytes")]
    pub total_sent_bytes: Option<u32>,

/// 
    #[serde(rename = "TotalSentRate")]
    pub total_sent_rate: Option<u32>,

/// 
    #[serde(rename = "UDPPacketsReceivedPersec")]
    pub udppackets_received_persec: Option<u32>,

/// 
    #[serde(rename = "UDPPacketsSentPersec")]
    pub udppackets_sent_persec: Option<u32>,

/// 
    #[serde(rename = "UDPReceivedRate")]
    pub udpreceived_rate: Option<u32>,

/// 
    #[serde(rename = "UDPSentRate")]
    pub udpsent_rate: Option<u32>,
}

impl Win32_PerfRawData_Counters_RemoteFXNetwork {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            base_tcprtt: None,
            base_udprtt: None,
            current_tcpbandwidth: None,
            current_tcprtt: None,
            current_udpbandwidth: None,
            current_udprtt: None,
            fecrate: None,
            fecrate__base: None,
            loss_rate: None,
            loss_rate__base: None,
            retransmission_rate: None,
            retransmission_rate__base: None,
            sent_rate_p0: None,
            sent_rate_p1: None,
            sent_rate_p2: None,
            sent_rate_p3: None,
            tcpreceived_rate: None,
            tcpsent_rate: None,
            total_received_bytes: None,
            total_received_rate: None,
            total_sent_bytes: None,
            total_sent_rate: None,
            udppackets_received_persec: None,
            udppackets_sent_persec: None,
            udpreceived_rate: None,
            udpsent_rate: None,
        }
    }


    /// Sets the value of BaseTCPRTT
    pub fn set_base_tcprtt(&mut self, value: u32) {
        self.base_tcprtt = Some(value);
    }

    /// Gets the value of BaseTCPRTT
    pub fn get_base_tcprtt(&self) -> Option<&u32> {
        self.base_tcprtt.as_ref()
    }

    /// Sets the value of BaseUDPRTT
    pub fn set_base_udprtt(&mut self, value: u32) {
        self.base_udprtt = Some(value);
    }

    /// Gets the value of BaseUDPRTT
    pub fn get_base_udprtt(&self) -> Option<&u32> {
        self.base_udprtt.as_ref()
    }

    /// Sets the value of CurrentTCPBandwidth
    pub fn set_current_tcpbandwidth(&mut self, value: u32) {
        self.current_tcpbandwidth = Some(value);
    }

    /// Gets the value of CurrentTCPBandwidth
    pub fn get_current_tcpbandwidth(&self) -> Option<&u32> {
        self.current_tcpbandwidth.as_ref()
    }

    /// Sets the value of CurrentTCPRTT
    pub fn set_current_tcprtt(&mut self, value: u32) {
        self.current_tcprtt = Some(value);
    }

    /// Gets the value of CurrentTCPRTT
    pub fn get_current_tcprtt(&self) -> Option<&u32> {
        self.current_tcprtt.as_ref()
    }

    /// Sets the value of CurrentUDPBandwidth
    pub fn set_current_udpbandwidth(&mut self, value: u32) {
        self.current_udpbandwidth = Some(value);
    }

    /// Gets the value of CurrentUDPBandwidth
    pub fn get_current_udpbandwidth(&self) -> Option<&u32> {
        self.current_udpbandwidth.as_ref()
    }

    /// Sets the value of CurrentUDPRTT
    pub fn set_current_udprtt(&mut self, value: u32) {
        self.current_udprtt = Some(value);
    }

    /// Gets the value of CurrentUDPRTT
    pub fn get_current_udprtt(&self) -> Option<&u32> {
        self.current_udprtt.as_ref()
    }

    /// Sets the value of FECRate
    pub fn set_fecrate(&mut self, value: u32) {
        self.fecrate = Some(value);
    }

    /// Gets the value of FECRate
    pub fn get_fecrate(&self) -> Option<&u32> {
        self.fecrate.as_ref()
    }

    /// Sets the value of FECRate_Base
    pub fn set_fecrate__base(&mut self, value: u32) {
        self.fecrate__base = Some(value);
    }

    /// Gets the value of FECRate_Base
    pub fn get_fecrate__base(&self) -> Option<&u32> {
        self.fecrate__base.as_ref()
    }

    /// Sets the value of LossRate
    pub fn set_loss_rate(&mut self, value: u32) {
        self.loss_rate = Some(value);
    }

    /// Gets the value of LossRate
    pub fn get_loss_rate(&self) -> Option<&u32> {
        self.loss_rate.as_ref()
    }

    /// Sets the value of LossRate_Base
    pub fn set_loss_rate__base(&mut self, value: u32) {
        self.loss_rate__base = Some(value);
    }

    /// Gets the value of LossRate_Base
    pub fn get_loss_rate__base(&self) -> Option<&u32> {
        self.loss_rate__base.as_ref()
    }

    /// Sets the value of RetransmissionRate
    pub fn set_retransmission_rate(&mut self, value: u32) {
        self.retransmission_rate = Some(value);
    }

    /// Gets the value of RetransmissionRate
    pub fn get_retransmission_rate(&self) -> Option<&u32> {
        self.retransmission_rate.as_ref()
    }

    /// Sets the value of RetransmissionRate_Base
    pub fn set_retransmission_rate__base(&mut self, value: u32) {
        self.retransmission_rate__base = Some(value);
    }

    /// Gets the value of RetransmissionRate_Base
    pub fn get_retransmission_rate__base(&self) -> Option<&u32> {
        self.retransmission_rate__base.as_ref()
    }

    /// Sets the value of SentRateP0
    pub fn set_sent_rate_p0(&mut self, value: u32) {
        self.sent_rate_p0 = Some(value);
    }

    /// Gets the value of SentRateP0
    pub fn get_sent_rate_p0(&self) -> Option<&u32> {
        self.sent_rate_p0.as_ref()
    }

    /// Sets the value of SentRateP1
    pub fn set_sent_rate_p1(&mut self, value: u32) {
        self.sent_rate_p1 = Some(value);
    }

    /// Gets the value of SentRateP1
    pub fn get_sent_rate_p1(&self) -> Option<&u32> {
        self.sent_rate_p1.as_ref()
    }

    /// Sets the value of SentRateP2
    pub fn set_sent_rate_p2(&mut self, value: u32) {
        self.sent_rate_p2 = Some(value);
    }

    /// Gets the value of SentRateP2
    pub fn get_sent_rate_p2(&self) -> Option<&u32> {
        self.sent_rate_p2.as_ref()
    }

    /// Sets the value of SentRateP3
    pub fn set_sent_rate_p3(&mut self, value: u32) {
        self.sent_rate_p3 = Some(value);
    }

    /// Gets the value of SentRateP3
    pub fn get_sent_rate_p3(&self) -> Option<&u32> {
        self.sent_rate_p3.as_ref()
    }

    /// Sets the value of TCPReceivedRate
    pub fn set_tcpreceived_rate(&mut self, value: u32) {
        self.tcpreceived_rate = Some(value);
    }

    /// Gets the value of TCPReceivedRate
    pub fn get_tcpreceived_rate(&self) -> Option<&u32> {
        self.tcpreceived_rate.as_ref()
    }

    /// Sets the value of TCPSentRate
    pub fn set_tcpsent_rate(&mut self, value: u32) {
        self.tcpsent_rate = Some(value);
    }

    /// Gets the value of TCPSentRate
    pub fn get_tcpsent_rate(&self) -> Option<&u32> {
        self.tcpsent_rate.as_ref()
    }

    /// Sets the value of TotalReceivedBytes
    pub fn set_total_received_bytes(&mut self, value: u32) {
        self.total_received_bytes = Some(value);
    }

    /// Gets the value of TotalReceivedBytes
    pub fn get_total_received_bytes(&self) -> Option<&u32> {
        self.total_received_bytes.as_ref()
    }

    /// Sets the value of TotalReceivedRate
    pub fn set_total_received_rate(&mut self, value: u32) {
        self.total_received_rate = Some(value);
    }

    /// Gets the value of TotalReceivedRate
    pub fn get_total_received_rate(&self) -> Option<&u32> {
        self.total_received_rate.as_ref()
    }

    /// Sets the value of TotalSentBytes
    pub fn set_total_sent_bytes(&mut self, value: u32) {
        self.total_sent_bytes = Some(value);
    }

    /// Gets the value of TotalSentBytes
    pub fn get_total_sent_bytes(&self) -> Option<&u32> {
        self.total_sent_bytes.as_ref()
    }

    /// Sets the value of TotalSentRate
    pub fn set_total_sent_rate(&mut self, value: u32) {
        self.total_sent_rate = Some(value);
    }

    /// Gets the value of TotalSentRate
    pub fn get_total_sent_rate(&self) -> Option<&u32> {
        self.total_sent_rate.as_ref()
    }

    /// Sets the value of UDPPacketsReceivedPersec
    pub fn set_udppackets_received_persec(&mut self, value: u32) {
        self.udppackets_received_persec = Some(value);
    }

    /// Gets the value of UDPPacketsReceivedPersec
    pub fn get_udppackets_received_persec(&self) -> Option<&u32> {
        self.udppackets_received_persec.as_ref()
    }

    /// Sets the value of UDPPacketsSentPersec
    pub fn set_udppackets_sent_persec(&mut self, value: u32) {
        self.udppackets_sent_persec = Some(value);
    }

    /// Gets the value of UDPPacketsSentPersec
    pub fn get_udppackets_sent_persec(&self) -> Option<&u32> {
        self.udppackets_sent_persec.as_ref()
    }

    /// Sets the value of UDPReceivedRate
    pub fn set_udpreceived_rate(&mut self, value: u32) {
        self.udpreceived_rate = Some(value);
    }

    /// Gets the value of UDPReceivedRate
    pub fn get_udpreceived_rate(&self) -> Option<&u32> {
        self.udpreceived_rate.as_ref()
    }

    /// Sets the value of UDPSentRate
    pub fn set_udpsent_rate(&mut self, value: u32) {
        self.udpsent_rate = Some(value);
    }

    /// Gets the value of UDPSentRate
    pub fn get_udpsent_rate(&self) -> Option<&u32> {
        self.udpsent_rate.as_ref()
    }
}

