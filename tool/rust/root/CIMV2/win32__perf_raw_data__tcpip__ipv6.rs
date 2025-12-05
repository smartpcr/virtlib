// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Tcpip_IPv6 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Tcpip_IPv6 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "DatagramsForwardedPersec")]
    pub datagrams_forwarded_persec: Option<u32>,

/// 
    #[serde(rename = "DatagramsOutboundDiscarded")]
    pub datagrams_outbound_discarded: Option<u32>,

/// 
    #[serde(rename = "DatagramsOutboundNoRoute")]
    pub datagrams_outbound_no_route: Option<u32>,

/// 
    #[serde(rename = "DatagramsPersec")]
    pub datagrams_persec: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedAddressErrors")]
    pub datagrams_received_address_errors: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedDeliveredPersec")]
    pub datagrams_received_delivered_persec: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedDiscarded")]
    pub datagrams_received_discarded: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedHeaderErrors")]
    pub datagrams_received_header_errors: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedPersec")]
    pub datagrams_received_persec: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceivedUnknownProtocol")]
    pub datagrams_received_unknown_protocol: Option<u32>,

/// 
    #[serde(rename = "DatagramsSentPersec")]
    pub datagrams_sent_persec: Option<u32>,

/// 
    #[serde(rename = "FragmentationFailures")]
    pub fragmentation_failures: Option<u32>,

/// 
    #[serde(rename = "FragmentedDatagramsPersec")]
    pub fragmented_datagrams_persec: Option<u32>,

/// 
    #[serde(rename = "FragmentReassemblyFailures")]
    pub fragment_reassembly_failures: Option<u32>,

/// 
    #[serde(rename = "FragmentsCreatedPersec")]
    pub fragments_created_persec: Option<u32>,

/// 
    #[serde(rename = "FragmentsReassembledPersec")]
    pub fragments_reassembled_persec: Option<u32>,

/// 
    #[serde(rename = "FragmentsReceivedPersec")]
    pub fragments_received_persec: Option<u32>,
}

impl Win32_PerfRawData_Tcpip_IPv6 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            datagrams_forwarded_persec: None,
            datagrams_outbound_discarded: None,
            datagrams_outbound_no_route: None,
            datagrams_persec: None,
            datagrams_received_address_errors: None,
            datagrams_received_delivered_persec: None,
            datagrams_received_discarded: None,
            datagrams_received_header_errors: None,
            datagrams_received_persec: None,
            datagrams_received_unknown_protocol: None,
            datagrams_sent_persec: None,
            fragmentation_failures: None,
            fragmented_datagrams_persec: None,
            fragment_reassembly_failures: None,
            fragments_created_persec: None,
            fragments_reassembled_persec: None,
            fragments_received_persec: None,
        }
    }


    /// Sets the value of DatagramsForwardedPersec
    pub fn set_datagrams_forwarded_persec(&mut self, value: u32) {
        self.datagrams_forwarded_persec = Some(value);
    }

    /// Gets the value of DatagramsForwardedPersec
    pub fn get_datagrams_forwarded_persec(&self) -> Option<&u32> {
        self.datagrams_forwarded_persec.as_ref()
    }

    /// Sets the value of DatagramsOutboundDiscarded
    pub fn set_datagrams_outbound_discarded(&mut self, value: u32) {
        self.datagrams_outbound_discarded = Some(value);
    }

    /// Gets the value of DatagramsOutboundDiscarded
    pub fn get_datagrams_outbound_discarded(&self) -> Option<&u32> {
        self.datagrams_outbound_discarded.as_ref()
    }

    /// Sets the value of DatagramsOutboundNoRoute
    pub fn set_datagrams_outbound_no_route(&mut self, value: u32) {
        self.datagrams_outbound_no_route = Some(value);
    }

    /// Gets the value of DatagramsOutboundNoRoute
    pub fn get_datagrams_outbound_no_route(&self) -> Option<&u32> {
        self.datagrams_outbound_no_route.as_ref()
    }

    /// Sets the value of DatagramsPersec
    pub fn set_datagrams_persec(&mut self, value: u32) {
        self.datagrams_persec = Some(value);
    }

    /// Gets the value of DatagramsPersec
    pub fn get_datagrams_persec(&self) -> Option<&u32> {
        self.datagrams_persec.as_ref()
    }

    /// Sets the value of DatagramsReceivedAddressErrors
    pub fn set_datagrams_received_address_errors(&mut self, value: u32) {
        self.datagrams_received_address_errors = Some(value);
    }

    /// Gets the value of DatagramsReceivedAddressErrors
    pub fn get_datagrams_received_address_errors(&self) -> Option<&u32> {
        self.datagrams_received_address_errors.as_ref()
    }

    /// Sets the value of DatagramsReceivedDeliveredPersec
    pub fn set_datagrams_received_delivered_persec(&mut self, value: u32) {
        self.datagrams_received_delivered_persec = Some(value);
    }

    /// Gets the value of DatagramsReceivedDeliveredPersec
    pub fn get_datagrams_received_delivered_persec(&self) -> Option<&u32> {
        self.datagrams_received_delivered_persec.as_ref()
    }

    /// Sets the value of DatagramsReceivedDiscarded
    pub fn set_datagrams_received_discarded(&mut self, value: u32) {
        self.datagrams_received_discarded = Some(value);
    }

    /// Gets the value of DatagramsReceivedDiscarded
    pub fn get_datagrams_received_discarded(&self) -> Option<&u32> {
        self.datagrams_received_discarded.as_ref()
    }

    /// Sets the value of DatagramsReceivedHeaderErrors
    pub fn set_datagrams_received_header_errors(&mut self, value: u32) {
        self.datagrams_received_header_errors = Some(value);
    }

    /// Gets the value of DatagramsReceivedHeaderErrors
    pub fn get_datagrams_received_header_errors(&self) -> Option<&u32> {
        self.datagrams_received_header_errors.as_ref()
    }

    /// Sets the value of DatagramsReceivedPersec
    pub fn set_datagrams_received_persec(&mut self, value: u32) {
        self.datagrams_received_persec = Some(value);
    }

    /// Gets the value of DatagramsReceivedPersec
    pub fn get_datagrams_received_persec(&self) -> Option<&u32> {
        self.datagrams_received_persec.as_ref()
    }

    /// Sets the value of DatagramsReceivedUnknownProtocol
    pub fn set_datagrams_received_unknown_protocol(&mut self, value: u32) {
        self.datagrams_received_unknown_protocol = Some(value);
    }

    /// Gets the value of DatagramsReceivedUnknownProtocol
    pub fn get_datagrams_received_unknown_protocol(&self) -> Option<&u32> {
        self.datagrams_received_unknown_protocol.as_ref()
    }

    /// Sets the value of DatagramsSentPersec
    pub fn set_datagrams_sent_persec(&mut self, value: u32) {
        self.datagrams_sent_persec = Some(value);
    }

    /// Gets the value of DatagramsSentPersec
    pub fn get_datagrams_sent_persec(&self) -> Option<&u32> {
        self.datagrams_sent_persec.as_ref()
    }

    /// Sets the value of FragmentationFailures
    pub fn set_fragmentation_failures(&mut self, value: u32) {
        self.fragmentation_failures = Some(value);
    }

    /// Gets the value of FragmentationFailures
    pub fn get_fragmentation_failures(&self) -> Option<&u32> {
        self.fragmentation_failures.as_ref()
    }

    /// Sets the value of FragmentedDatagramsPersec
    pub fn set_fragmented_datagrams_persec(&mut self, value: u32) {
        self.fragmented_datagrams_persec = Some(value);
    }

    /// Gets the value of FragmentedDatagramsPersec
    pub fn get_fragmented_datagrams_persec(&self) -> Option<&u32> {
        self.fragmented_datagrams_persec.as_ref()
    }

    /// Sets the value of FragmentReassemblyFailures
    pub fn set_fragment_reassembly_failures(&mut self, value: u32) {
        self.fragment_reassembly_failures = Some(value);
    }

    /// Gets the value of FragmentReassemblyFailures
    pub fn get_fragment_reassembly_failures(&self) -> Option<&u32> {
        self.fragment_reassembly_failures.as_ref()
    }

    /// Sets the value of FragmentsCreatedPersec
    pub fn set_fragments_created_persec(&mut self, value: u32) {
        self.fragments_created_persec = Some(value);
    }

    /// Gets the value of FragmentsCreatedPersec
    pub fn get_fragments_created_persec(&self) -> Option<&u32> {
        self.fragments_created_persec.as_ref()
    }

    /// Sets the value of FragmentsReassembledPersec
    pub fn set_fragments_reassembled_persec(&mut self, value: u32) {
        self.fragments_reassembled_persec = Some(value);
    }

    /// Gets the value of FragmentsReassembledPersec
    pub fn get_fragments_reassembled_persec(&self) -> Option<&u32> {
        self.fragments_reassembled_persec.as_ref()
    }

    /// Sets the value of FragmentsReceivedPersec
    pub fn set_fragments_received_persec(&mut self, value: u32) {
        self.fragments_received_persec = Some(value);
    }

    /// Gets the value of FragmentsReceivedPersec
    pub fn get_fragments_received_persec(&self) -> Option<&u32> {
        self.fragments_received_persec.as_ref()
    }
}

