// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_TCPIPCounters_TCPIPExtendedPerformanceDiagnostics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_TCPIPCounters_TCPIPExtendedPerformanceDiagnostics {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "InboundTCPconnectionattempts")]
    pub inbound_tcpconnectionattempts: Option<u64>,

/// 
    #[serde(rename = "IPv4NBLsindicatedwithprevalidation")]
    pub ipv4_nblsindicatedwithprevalidation: Option<u64>,

/// 
    #[serde(rename = "IPv4NBLsPersecindicatedwithprevalidation")]
    pub ipv4_nbls_persecindicatedwithprevalidation: Option<u64>,

/// 
    #[serde(rename = "IPv6NBLsindicatedwithprevalidation")]
    pub ipv6_nblsindicatedwithprevalidation: Option<u64>,

/// 
    #[serde(rename = "IPv6NBLsPersecindicatedwithprevalidation")]
    pub ipv6_nbls_persecindicatedwithprevalidation: Option<u64>,

/// 
    #[serde(rename = "OutboundTCPconnectionattempts")]
    pub outbound_tcpconnectionattempts: Option<u64>,

/// 
    #[serde(rename = "RSTsegmentssent")]
    pub rstsegmentssent: Option<u64>,

/// 
    #[serde(rename = "SYNTCBscreated")]
    pub syntcbscreated: Option<u64>,

/// 
    #[serde(rename = "SYNTCBsdestroyed")]
    pub syntcbsdestroyed: Option<u64>,

/// 
    #[serde(rename = "TCBscreated")]
    pub tcbscreated: Option<u64>,

/// 
    #[serde(rename = "TCBsdestroyed")]
    pub tcbsdestroyed: Option<u64>,

/// 
    #[serde(rename = "TCPbadsegmentPersecreceived")]
    pub tcpbadsegment_persecreceived: Option<u64>,

/// 
    #[serde(rename = "TCPbadsegmentsreceived")]
    pub tcpbadsegmentsreceived: Option<u64>,

/// 
    #[serde(rename = "TCPconnectionattemptsfailed")]
    pub tcpconnectionattemptsfailed: Option<u64>,

/// 
    #[serde(rename = "TCPdeliverybufferingfailures")]
    pub tcpdeliverybufferingfailures: Option<u64>,

/// 
    #[serde(rename = "TCPIPpendableRXNBLsaftercoalescing")]
    pub tcpippendable_rxnblsaftercoalescing: Option<u64>,

/// 
    #[serde(rename = "TCPIPpendableRXNBLsreceivedfromminiport")]
    pub tcpippendable_rxnblsreceivedfromminiport: Option<u64>,

/// 
    #[serde(rename = "TCPIPpendableRXNBLsreturnedtominiport")]
    pub tcpippendable_rxnblsreturnedtominiport: Option<u64>,

/// 
    #[serde(rename = "TcpipRscDisabledMask")]
    pub tcpip_rsc_disabled_mask: Option<u32>,

/// 
    #[serde(rename = "TcpipUsoDisabledMask")]
    pub tcpip_uso_disabled_mask: Option<u32>,

/// 
    #[serde(rename = "TCPLSObytePersecsent")]
    pub tcplsobyte_persecsent: Option<u64>,

/// 
    #[serde(rename = "TCPLSOsegmentPersecsent")]
    pub tcplsosegment_persecsent: Option<u64>,

/// 
    #[serde(rename = "TCPsegmentPersecreceived")]
    pub tcpsegment_persecreceived: Option<u64>,

/// 
    #[serde(rename = "TCPsegmentPersecretransmitted")]
    pub tcpsegment_persecretransmitted: Option<u64>,

/// 
    #[serde(rename = "TCPsegmentPersecsent")]
    pub tcpsegment_persecsent: Option<u64>,

/// 
    #[serde(rename = "TCPtuplesinserted")]
    pub tcptuplesinserted: Option<u64>,

/// 
    #[serde(rename = "TCPtuplesremoved")]
    pub tcptuplesremoved: Option<u64>,

/// 
    #[serde(rename = "TWTCBcollisions")]
    pub twtcbcollisions: Option<u64>,

/// 
    #[serde(rename = "TWTCBscreated")]
    pub twtcbscreated: Option<u64>,

/// 
    #[serde(rename = "TWTCBsdestroyed")]
    pub twtcbsdestroyed: Option<u64>,

/// 
    #[serde(rename = "TWTCBsstateshutdown")]
    pub twtcbsstateshutdown: Option<u64>,

/// 
    #[serde(rename = "UDPUSObytePersecsent")]
    pub udpusobyte_persecsent: Option<u64>,

/// 
    #[serde(rename = "UDPUSOsegmentPersecsent")]
    pub udpusosegment_persecsent: Option<u64>,

/// 
    #[serde(rename = "UroDisabledMask")]
    pub uro_disabled_mask: Option<u32>,
}

impl Win32_PerfRawData_TCPIPCounters_TCPIPExtendedPerformanceDiagnostics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            inbound_tcpconnectionattempts: None,
            ipv4_nblsindicatedwithprevalidation: None,
            ipv4_nbls_persecindicatedwithprevalidation: None,
            ipv6_nblsindicatedwithprevalidation: None,
            ipv6_nbls_persecindicatedwithprevalidation: None,
            outbound_tcpconnectionattempts: None,
            rstsegmentssent: None,
            syntcbscreated: None,
            syntcbsdestroyed: None,
            tcbscreated: None,
            tcbsdestroyed: None,
            tcpbadsegment_persecreceived: None,
            tcpbadsegmentsreceived: None,
            tcpconnectionattemptsfailed: None,
            tcpdeliverybufferingfailures: None,
            tcpippendable_rxnblsaftercoalescing: None,
            tcpippendable_rxnblsreceivedfromminiport: None,
            tcpippendable_rxnblsreturnedtominiport: None,
            tcpip_rsc_disabled_mask: None,
            tcpip_uso_disabled_mask: None,
            tcplsobyte_persecsent: None,
            tcplsosegment_persecsent: None,
            tcpsegment_persecreceived: None,
            tcpsegment_persecretransmitted: None,
            tcpsegment_persecsent: None,
            tcptuplesinserted: None,
            tcptuplesremoved: None,
            twtcbcollisions: None,
            twtcbscreated: None,
            twtcbsdestroyed: None,
            twtcbsstateshutdown: None,
            udpusobyte_persecsent: None,
            udpusosegment_persecsent: None,
            uro_disabled_mask: None,
        }
    }


    /// Sets the value of InboundTCPconnectionattempts
    pub fn set_inbound_tcpconnectionattempts(&mut self, value: u64) {
        self.inbound_tcpconnectionattempts = Some(value);
    }

    /// Gets the value of InboundTCPconnectionattempts
    pub fn get_inbound_tcpconnectionattempts(&self) -> Option<&u64> {
        self.inbound_tcpconnectionattempts.as_ref()
    }

    /// Sets the value of IPv4NBLsindicatedwithprevalidation
    pub fn set_ipv4_nblsindicatedwithprevalidation(&mut self, value: u64) {
        self.ipv4_nblsindicatedwithprevalidation = Some(value);
    }

    /// Gets the value of IPv4NBLsindicatedwithprevalidation
    pub fn get_ipv4_nblsindicatedwithprevalidation(&self) -> Option<&u64> {
        self.ipv4_nblsindicatedwithprevalidation.as_ref()
    }

    /// Sets the value of IPv4NBLsPersecindicatedwithprevalidation
    pub fn set_ipv4_nbls_persecindicatedwithprevalidation(&mut self, value: u64) {
        self.ipv4_nbls_persecindicatedwithprevalidation = Some(value);
    }

    /// Gets the value of IPv4NBLsPersecindicatedwithprevalidation
    pub fn get_ipv4_nbls_persecindicatedwithprevalidation(&self) -> Option<&u64> {
        self.ipv4_nbls_persecindicatedwithprevalidation.as_ref()
    }

    /// Sets the value of IPv6NBLsindicatedwithprevalidation
    pub fn set_ipv6_nblsindicatedwithprevalidation(&mut self, value: u64) {
        self.ipv6_nblsindicatedwithprevalidation = Some(value);
    }

    /// Gets the value of IPv6NBLsindicatedwithprevalidation
    pub fn get_ipv6_nblsindicatedwithprevalidation(&self) -> Option<&u64> {
        self.ipv6_nblsindicatedwithprevalidation.as_ref()
    }

    /// Sets the value of IPv6NBLsPersecindicatedwithprevalidation
    pub fn set_ipv6_nbls_persecindicatedwithprevalidation(&mut self, value: u64) {
        self.ipv6_nbls_persecindicatedwithprevalidation = Some(value);
    }

    /// Gets the value of IPv6NBLsPersecindicatedwithprevalidation
    pub fn get_ipv6_nbls_persecindicatedwithprevalidation(&self) -> Option<&u64> {
        self.ipv6_nbls_persecindicatedwithprevalidation.as_ref()
    }

    /// Sets the value of OutboundTCPconnectionattempts
    pub fn set_outbound_tcpconnectionattempts(&mut self, value: u64) {
        self.outbound_tcpconnectionattempts = Some(value);
    }

    /// Gets the value of OutboundTCPconnectionattempts
    pub fn get_outbound_tcpconnectionattempts(&self) -> Option<&u64> {
        self.outbound_tcpconnectionattempts.as_ref()
    }

    /// Sets the value of RSTsegmentssent
    pub fn set_rstsegmentssent(&mut self, value: u64) {
        self.rstsegmentssent = Some(value);
    }

    /// Gets the value of RSTsegmentssent
    pub fn get_rstsegmentssent(&self) -> Option<&u64> {
        self.rstsegmentssent.as_ref()
    }

    /// Sets the value of SYNTCBscreated
    pub fn set_syntcbscreated(&mut self, value: u64) {
        self.syntcbscreated = Some(value);
    }

    /// Gets the value of SYNTCBscreated
    pub fn get_syntcbscreated(&self) -> Option<&u64> {
        self.syntcbscreated.as_ref()
    }

    /// Sets the value of SYNTCBsdestroyed
    pub fn set_syntcbsdestroyed(&mut self, value: u64) {
        self.syntcbsdestroyed = Some(value);
    }

    /// Gets the value of SYNTCBsdestroyed
    pub fn get_syntcbsdestroyed(&self) -> Option<&u64> {
        self.syntcbsdestroyed.as_ref()
    }

    /// Sets the value of TCBscreated
    pub fn set_tcbscreated(&mut self, value: u64) {
        self.tcbscreated = Some(value);
    }

    /// Gets the value of TCBscreated
    pub fn get_tcbscreated(&self) -> Option<&u64> {
        self.tcbscreated.as_ref()
    }

    /// Sets the value of TCBsdestroyed
    pub fn set_tcbsdestroyed(&mut self, value: u64) {
        self.tcbsdestroyed = Some(value);
    }

    /// Gets the value of TCBsdestroyed
    pub fn get_tcbsdestroyed(&self) -> Option<&u64> {
        self.tcbsdestroyed.as_ref()
    }

    /// Sets the value of TCPbadsegmentPersecreceived
    pub fn set_tcpbadsegment_persecreceived(&mut self, value: u64) {
        self.tcpbadsegment_persecreceived = Some(value);
    }

    /// Gets the value of TCPbadsegmentPersecreceived
    pub fn get_tcpbadsegment_persecreceived(&self) -> Option<&u64> {
        self.tcpbadsegment_persecreceived.as_ref()
    }

    /// Sets the value of TCPbadsegmentsreceived
    pub fn set_tcpbadsegmentsreceived(&mut self, value: u64) {
        self.tcpbadsegmentsreceived = Some(value);
    }

    /// Gets the value of TCPbadsegmentsreceived
    pub fn get_tcpbadsegmentsreceived(&self) -> Option<&u64> {
        self.tcpbadsegmentsreceived.as_ref()
    }

    /// Sets the value of TCPconnectionattemptsfailed
    pub fn set_tcpconnectionattemptsfailed(&mut self, value: u64) {
        self.tcpconnectionattemptsfailed = Some(value);
    }

    /// Gets the value of TCPconnectionattemptsfailed
    pub fn get_tcpconnectionattemptsfailed(&self) -> Option<&u64> {
        self.tcpconnectionattemptsfailed.as_ref()
    }

    /// Sets the value of TCPdeliverybufferingfailures
    pub fn set_tcpdeliverybufferingfailures(&mut self, value: u64) {
        self.tcpdeliverybufferingfailures = Some(value);
    }

    /// Gets the value of TCPdeliverybufferingfailures
    pub fn get_tcpdeliverybufferingfailures(&self) -> Option<&u64> {
        self.tcpdeliverybufferingfailures.as_ref()
    }

    /// Sets the value of TCPIPpendableRXNBLsaftercoalescing
    pub fn set_tcpippendable_rxnblsaftercoalescing(&mut self, value: u64) {
        self.tcpippendable_rxnblsaftercoalescing = Some(value);
    }

    /// Gets the value of TCPIPpendableRXNBLsaftercoalescing
    pub fn get_tcpippendable_rxnblsaftercoalescing(&self) -> Option<&u64> {
        self.tcpippendable_rxnblsaftercoalescing.as_ref()
    }

    /// Sets the value of TCPIPpendableRXNBLsreceivedfromminiport
    pub fn set_tcpippendable_rxnblsreceivedfromminiport(&mut self, value: u64) {
        self.tcpippendable_rxnblsreceivedfromminiport = Some(value);
    }

    /// Gets the value of TCPIPpendableRXNBLsreceivedfromminiport
    pub fn get_tcpippendable_rxnblsreceivedfromminiport(&self) -> Option<&u64> {
        self.tcpippendable_rxnblsreceivedfromminiport.as_ref()
    }

    /// Sets the value of TCPIPpendableRXNBLsreturnedtominiport
    pub fn set_tcpippendable_rxnblsreturnedtominiport(&mut self, value: u64) {
        self.tcpippendable_rxnblsreturnedtominiport = Some(value);
    }

    /// Gets the value of TCPIPpendableRXNBLsreturnedtominiport
    pub fn get_tcpippendable_rxnblsreturnedtominiport(&self) -> Option<&u64> {
        self.tcpippendable_rxnblsreturnedtominiport.as_ref()
    }

    /// Sets the value of TcpipRscDisabledMask
    pub fn set_tcpip_rsc_disabled_mask(&mut self, value: u32) {
        self.tcpip_rsc_disabled_mask = Some(value);
    }

    /// Gets the value of TcpipRscDisabledMask
    pub fn get_tcpip_rsc_disabled_mask(&self) -> Option<&u32> {
        self.tcpip_rsc_disabled_mask.as_ref()
    }

    /// Sets the value of TcpipUsoDisabledMask
    pub fn set_tcpip_uso_disabled_mask(&mut self, value: u32) {
        self.tcpip_uso_disabled_mask = Some(value);
    }

    /// Gets the value of TcpipUsoDisabledMask
    pub fn get_tcpip_uso_disabled_mask(&self) -> Option<&u32> {
        self.tcpip_uso_disabled_mask.as_ref()
    }

    /// Sets the value of TCPLSObytePersecsent
    pub fn set_tcplsobyte_persecsent(&mut self, value: u64) {
        self.tcplsobyte_persecsent = Some(value);
    }

    /// Gets the value of TCPLSObytePersecsent
    pub fn get_tcplsobyte_persecsent(&self) -> Option<&u64> {
        self.tcplsobyte_persecsent.as_ref()
    }

    /// Sets the value of TCPLSOsegmentPersecsent
    pub fn set_tcplsosegment_persecsent(&mut self, value: u64) {
        self.tcplsosegment_persecsent = Some(value);
    }

    /// Gets the value of TCPLSOsegmentPersecsent
    pub fn get_tcplsosegment_persecsent(&self) -> Option<&u64> {
        self.tcplsosegment_persecsent.as_ref()
    }

    /// Sets the value of TCPsegmentPersecreceived
    pub fn set_tcpsegment_persecreceived(&mut self, value: u64) {
        self.tcpsegment_persecreceived = Some(value);
    }

    /// Gets the value of TCPsegmentPersecreceived
    pub fn get_tcpsegment_persecreceived(&self) -> Option<&u64> {
        self.tcpsegment_persecreceived.as_ref()
    }

    /// Sets the value of TCPsegmentPersecretransmitted
    pub fn set_tcpsegment_persecretransmitted(&mut self, value: u64) {
        self.tcpsegment_persecretransmitted = Some(value);
    }

    /// Gets the value of TCPsegmentPersecretransmitted
    pub fn get_tcpsegment_persecretransmitted(&self) -> Option<&u64> {
        self.tcpsegment_persecretransmitted.as_ref()
    }

    /// Sets the value of TCPsegmentPersecsent
    pub fn set_tcpsegment_persecsent(&mut self, value: u64) {
        self.tcpsegment_persecsent = Some(value);
    }

    /// Gets the value of TCPsegmentPersecsent
    pub fn get_tcpsegment_persecsent(&self) -> Option<&u64> {
        self.tcpsegment_persecsent.as_ref()
    }

    /// Sets the value of TCPtuplesinserted
    pub fn set_tcptuplesinserted(&mut self, value: u64) {
        self.tcptuplesinserted = Some(value);
    }

    /// Gets the value of TCPtuplesinserted
    pub fn get_tcptuplesinserted(&self) -> Option<&u64> {
        self.tcptuplesinserted.as_ref()
    }

    /// Sets the value of TCPtuplesremoved
    pub fn set_tcptuplesremoved(&mut self, value: u64) {
        self.tcptuplesremoved = Some(value);
    }

    /// Gets the value of TCPtuplesremoved
    pub fn get_tcptuplesremoved(&self) -> Option<&u64> {
        self.tcptuplesremoved.as_ref()
    }

    /// Sets the value of TWTCBcollisions
    pub fn set_twtcbcollisions(&mut self, value: u64) {
        self.twtcbcollisions = Some(value);
    }

    /// Gets the value of TWTCBcollisions
    pub fn get_twtcbcollisions(&self) -> Option<&u64> {
        self.twtcbcollisions.as_ref()
    }

    /// Sets the value of TWTCBscreated
    pub fn set_twtcbscreated(&mut self, value: u64) {
        self.twtcbscreated = Some(value);
    }

    /// Gets the value of TWTCBscreated
    pub fn get_twtcbscreated(&self) -> Option<&u64> {
        self.twtcbscreated.as_ref()
    }

    /// Sets the value of TWTCBsdestroyed
    pub fn set_twtcbsdestroyed(&mut self, value: u64) {
        self.twtcbsdestroyed = Some(value);
    }

    /// Gets the value of TWTCBsdestroyed
    pub fn get_twtcbsdestroyed(&self) -> Option<&u64> {
        self.twtcbsdestroyed.as_ref()
    }

    /// Sets the value of TWTCBsstateshutdown
    pub fn set_twtcbsstateshutdown(&mut self, value: u64) {
        self.twtcbsstateshutdown = Some(value);
    }

    /// Gets the value of TWTCBsstateshutdown
    pub fn get_twtcbsstateshutdown(&self) -> Option<&u64> {
        self.twtcbsstateshutdown.as_ref()
    }

    /// Sets the value of UDPUSObytePersecsent
    pub fn set_udpusobyte_persecsent(&mut self, value: u64) {
        self.udpusobyte_persecsent = Some(value);
    }

    /// Gets the value of UDPUSObytePersecsent
    pub fn get_udpusobyte_persecsent(&self) -> Option<&u64> {
        self.udpusobyte_persecsent.as_ref()
    }

    /// Sets the value of UDPUSOsegmentPersecsent
    pub fn set_udpusosegment_persecsent(&mut self, value: u64) {
        self.udpusosegment_persecsent = Some(value);
    }

    /// Gets the value of UDPUSOsegmentPersecsent
    pub fn get_udpusosegment_persecsent(&self) -> Option<&u64> {
        self.udpusosegment_persecsent.as_ref()
    }

    /// Sets the value of UroDisabledMask
    pub fn set_uro_disabled_mask(&mut self, value: u32) {
        self.uro_disabled_mask = Some(value);
    }

    /// Gets the value of UroDisabledMask
    pub fn get_uro_disabled_mask(&self) -> Option<&u32> {
        self.uro_disabled_mask.as_ref()
    }
}

