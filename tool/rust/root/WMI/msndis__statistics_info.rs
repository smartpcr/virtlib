// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_StatisticsInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_StatisticsInfo {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "ifHCInBroadcastOctets")]
    pub if_hcin_broadcast_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCInBroadcastPkts")]
    pub if_hcin_broadcast_pkts: Option<u64>,

/// 
    #[serde(rename = "ifHCInMulticastOctets")]
    pub if_hcin_multicast_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCInMulticastPkts")]
    pub if_hcin_multicast_pkts: Option<u64>,

/// 
    #[serde(rename = "ifHCInOctets")]
    pub if_hcin_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCInUcastOctets")]
    pub if_hcin_ucast_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCInUcastPkts")]
    pub if_hcin_ucast_pkts: Option<u64>,

/// 
    #[serde(rename = "ifHCOutBroadcastOctets")]
    pub if_hcout_broadcast_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCOutBroadcastPkts")]
    pub if_hcout_broadcast_pkts: Option<u64>,

/// 
    #[serde(rename = "ifHCOutMulticastOctets")]
    pub if_hcout_multicast_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCOutMulticastPkts")]
    pub if_hcout_multicast_pkts: Option<u64>,

/// 
    #[serde(rename = "ifHCOutOctets")]
    pub if_hcout_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCOutUcastOctets")]
    pub if_hcout_ucast_octets: Option<u64>,

/// 
    #[serde(rename = "ifHCOutUcastPkts")]
    pub if_hcout_ucast_pkts: Option<u64>,

/// 
    #[serde(rename = "ifInDiscards")]
    pub if_in_discards: Option<u64>,

/// 
    #[serde(rename = "ifInErrors")]
    pub if_in_errors: Option<u64>,

/// 
    #[serde(rename = "ifOutDiscards")]
    pub if_out_discards: Option<u64>,

/// 
    #[serde(rename = "ifOutErrors")]
    pub if_out_errors: Option<u64>,

/// 
    #[serde(rename = "SupportedStatistics")]
    pub supported_statistics: Option<u32>,
}

impl MSNdis_StatisticsInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            header: None,
            if_hcin_broadcast_octets: None,
            if_hcin_broadcast_pkts: None,
            if_hcin_multicast_octets: None,
            if_hcin_multicast_pkts: None,
            if_hcin_octets: None,
            if_hcin_ucast_octets: None,
            if_hcin_ucast_pkts: None,
            if_hcout_broadcast_octets: None,
            if_hcout_broadcast_pkts: None,
            if_hcout_multicast_octets: None,
            if_hcout_multicast_pkts: None,
            if_hcout_octets: None,
            if_hcout_ucast_octets: None,
            if_hcout_ucast_pkts: None,
            if_in_discards: None,
            if_in_errors: None,
            if_out_discards: None,
            if_out_errors: None,
            supported_statistics: None,
        }
    }


    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of ifHCInBroadcastOctets
    pub fn set_if_hcin_broadcast_octets(&mut self, value: u64) {
        self.if_hcin_broadcast_octets = Some(value);
    }

    /// Gets the value of ifHCInBroadcastOctets
    pub fn get_if_hcin_broadcast_octets(&self) -> Option<&u64> {
        self.if_hcin_broadcast_octets.as_ref()
    }

    /// Sets the value of ifHCInBroadcastPkts
    pub fn set_if_hcin_broadcast_pkts(&mut self, value: u64) {
        self.if_hcin_broadcast_pkts = Some(value);
    }

    /// Gets the value of ifHCInBroadcastPkts
    pub fn get_if_hcin_broadcast_pkts(&self) -> Option<&u64> {
        self.if_hcin_broadcast_pkts.as_ref()
    }

    /// Sets the value of ifHCInMulticastOctets
    pub fn set_if_hcin_multicast_octets(&mut self, value: u64) {
        self.if_hcin_multicast_octets = Some(value);
    }

    /// Gets the value of ifHCInMulticastOctets
    pub fn get_if_hcin_multicast_octets(&self) -> Option<&u64> {
        self.if_hcin_multicast_octets.as_ref()
    }

    /// Sets the value of ifHCInMulticastPkts
    pub fn set_if_hcin_multicast_pkts(&mut self, value: u64) {
        self.if_hcin_multicast_pkts = Some(value);
    }

    /// Gets the value of ifHCInMulticastPkts
    pub fn get_if_hcin_multicast_pkts(&self) -> Option<&u64> {
        self.if_hcin_multicast_pkts.as_ref()
    }

    /// Sets the value of ifHCInOctets
    pub fn set_if_hcin_octets(&mut self, value: u64) {
        self.if_hcin_octets = Some(value);
    }

    /// Gets the value of ifHCInOctets
    pub fn get_if_hcin_octets(&self) -> Option<&u64> {
        self.if_hcin_octets.as_ref()
    }

    /// Sets the value of ifHCInUcastOctets
    pub fn set_if_hcin_ucast_octets(&mut self, value: u64) {
        self.if_hcin_ucast_octets = Some(value);
    }

    /// Gets the value of ifHCInUcastOctets
    pub fn get_if_hcin_ucast_octets(&self) -> Option<&u64> {
        self.if_hcin_ucast_octets.as_ref()
    }

    /// Sets the value of ifHCInUcastPkts
    pub fn set_if_hcin_ucast_pkts(&mut self, value: u64) {
        self.if_hcin_ucast_pkts = Some(value);
    }

    /// Gets the value of ifHCInUcastPkts
    pub fn get_if_hcin_ucast_pkts(&self) -> Option<&u64> {
        self.if_hcin_ucast_pkts.as_ref()
    }

    /// Sets the value of ifHCOutBroadcastOctets
    pub fn set_if_hcout_broadcast_octets(&mut self, value: u64) {
        self.if_hcout_broadcast_octets = Some(value);
    }

    /// Gets the value of ifHCOutBroadcastOctets
    pub fn get_if_hcout_broadcast_octets(&self) -> Option<&u64> {
        self.if_hcout_broadcast_octets.as_ref()
    }

    /// Sets the value of ifHCOutBroadcastPkts
    pub fn set_if_hcout_broadcast_pkts(&mut self, value: u64) {
        self.if_hcout_broadcast_pkts = Some(value);
    }

    /// Gets the value of ifHCOutBroadcastPkts
    pub fn get_if_hcout_broadcast_pkts(&self) -> Option<&u64> {
        self.if_hcout_broadcast_pkts.as_ref()
    }

    /// Sets the value of ifHCOutMulticastOctets
    pub fn set_if_hcout_multicast_octets(&mut self, value: u64) {
        self.if_hcout_multicast_octets = Some(value);
    }

    /// Gets the value of ifHCOutMulticastOctets
    pub fn get_if_hcout_multicast_octets(&self) -> Option<&u64> {
        self.if_hcout_multicast_octets.as_ref()
    }

    /// Sets the value of ifHCOutMulticastPkts
    pub fn set_if_hcout_multicast_pkts(&mut self, value: u64) {
        self.if_hcout_multicast_pkts = Some(value);
    }

    /// Gets the value of ifHCOutMulticastPkts
    pub fn get_if_hcout_multicast_pkts(&self) -> Option<&u64> {
        self.if_hcout_multicast_pkts.as_ref()
    }

    /// Sets the value of ifHCOutOctets
    pub fn set_if_hcout_octets(&mut self, value: u64) {
        self.if_hcout_octets = Some(value);
    }

    /// Gets the value of ifHCOutOctets
    pub fn get_if_hcout_octets(&self) -> Option<&u64> {
        self.if_hcout_octets.as_ref()
    }

    /// Sets the value of ifHCOutUcastOctets
    pub fn set_if_hcout_ucast_octets(&mut self, value: u64) {
        self.if_hcout_ucast_octets = Some(value);
    }

    /// Gets the value of ifHCOutUcastOctets
    pub fn get_if_hcout_ucast_octets(&self) -> Option<&u64> {
        self.if_hcout_ucast_octets.as_ref()
    }

    /// Sets the value of ifHCOutUcastPkts
    pub fn set_if_hcout_ucast_pkts(&mut self, value: u64) {
        self.if_hcout_ucast_pkts = Some(value);
    }

    /// Gets the value of ifHCOutUcastPkts
    pub fn get_if_hcout_ucast_pkts(&self) -> Option<&u64> {
        self.if_hcout_ucast_pkts.as_ref()
    }

    /// Sets the value of ifInDiscards
    pub fn set_if_in_discards(&mut self, value: u64) {
        self.if_in_discards = Some(value);
    }

    /// Gets the value of ifInDiscards
    pub fn get_if_in_discards(&self) -> Option<&u64> {
        self.if_in_discards.as_ref()
    }

    /// Sets the value of ifInErrors
    pub fn set_if_in_errors(&mut self, value: u64) {
        self.if_in_errors = Some(value);
    }

    /// Gets the value of ifInErrors
    pub fn get_if_in_errors(&self) -> Option<&u64> {
        self.if_in_errors.as_ref()
    }

    /// Sets the value of ifOutDiscards
    pub fn set_if_out_discards(&mut self, value: u64) {
        self.if_out_discards = Some(value);
    }

    /// Gets the value of ifOutDiscards
    pub fn get_if_out_discards(&self) -> Option<&u64> {
        self.if_out_discards.as_ref()
    }

    /// Sets the value of ifOutErrors
    pub fn set_if_out_errors(&mut self, value: u64) {
        self.if_out_errors = Some(value);
    }

    /// Gets the value of ifOutErrors
    pub fn get_if_out_errors(&self) -> Option<&u64> {
        self.if_out_errors.as_ref()
    }

    /// Sets the value of SupportedStatistics
    pub fn set_supported_statistics(&mut self, value: u32) {
        self.supported_statistics = Some(value);
    }

    /// Gets the value of SupportedStatistics
    pub fn get_supported_statistics(&self) -> Option<&u32> {
        self.supported_statistics.as_ref()
    }
}

