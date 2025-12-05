// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NetworkProtocol struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NetworkProtocol {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "ConnectionlessService")]
    pub connectionless_service: Option<bool>,

/// 
    #[serde(rename = "GuaranteesDelivery")]
    pub guarantees_delivery: Option<bool>,

/// 
    #[serde(rename = "GuaranteesSequencing")]
    pub guarantees_sequencing: Option<bool>,

/// 
    #[serde(rename = "MaximumAddressSize")]
    pub maximum_address_size: Option<u32>,

/// 
    #[serde(rename = "MaximumMessageSize")]
    pub maximum_message_size: Option<u32>,

/// 
    #[serde(rename = "MessageOriented")]
    pub message_oriented: Option<bool>,

/// 
    #[serde(rename = "MinimumAddressSize")]
    pub minimum_address_size: Option<u32>,

/// 
    #[serde(rename = "PseudoStreamOriented")]
    pub pseudo_stream_oriented: Option<bool>,

/// 
    #[serde(rename = "SupportsBroadcasting")]
    pub supports_broadcasting: Option<bool>,

/// 
    #[serde(rename = "SupportsConnectData")]
    pub supports_connect_data: Option<bool>,

/// 
    #[serde(rename = "SupportsDisconnectData")]
    pub supports_disconnect_data: Option<bool>,

/// 
    #[serde(rename = "SupportsEncryption")]
    pub supports_encryption: Option<bool>,

/// 
    #[serde(rename = "SupportsExpeditedData")]
    pub supports_expedited_data: Option<bool>,

/// 
    #[serde(rename = "SupportsFragmentation")]
    pub supports_fragmentation: Option<bool>,

/// 
    #[serde(rename = "SupportsGracefulClosing")]
    pub supports_graceful_closing: Option<bool>,

/// 
    #[serde(rename = "SupportsGuaranteedBandwidth")]
    pub supports_guaranteed_bandwidth: Option<bool>,

/// 
    #[serde(rename = "SupportsMulticasting")]
    pub supports_multicasting: Option<bool>,

/// 
    #[serde(rename = "SupportsQualityofService")]
    pub supports_qualityof_service: Option<bool>,
}

impl Win32_NetworkProtocol {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            connectionless_service: None,
            guarantees_delivery: None,
            guarantees_sequencing: None,
            maximum_address_size: None,
            maximum_message_size: None,
            message_oriented: None,
            minimum_address_size: None,
            pseudo_stream_oriented: None,
            supports_broadcasting: None,
            supports_connect_data: None,
            supports_disconnect_data: None,
            supports_encryption: None,
            supports_expedited_data: None,
            supports_fragmentation: None,
            supports_graceful_closing: None,
            supports_guaranteed_bandwidth: None,
            supports_multicasting: None,
            supports_qualityof_service: None,
        }
    }


    /// Sets the value of ConnectionlessService
    pub fn set_connectionless_service(&mut self, value: bool) {
        self.connectionless_service = Some(value);
    }

    /// Gets the value of ConnectionlessService
    pub fn get_connectionless_service(&self) -> Option<&bool> {
        self.connectionless_service.as_ref()
    }

    /// Sets the value of GuaranteesDelivery
    pub fn set_guarantees_delivery(&mut self, value: bool) {
        self.guarantees_delivery = Some(value);
    }

    /// Gets the value of GuaranteesDelivery
    pub fn get_guarantees_delivery(&self) -> Option<&bool> {
        self.guarantees_delivery.as_ref()
    }

    /// Sets the value of GuaranteesSequencing
    pub fn set_guarantees_sequencing(&mut self, value: bool) {
        self.guarantees_sequencing = Some(value);
    }

    /// Gets the value of GuaranteesSequencing
    pub fn get_guarantees_sequencing(&self) -> Option<&bool> {
        self.guarantees_sequencing.as_ref()
    }

    /// Sets the value of MaximumAddressSize
    pub fn set_maximum_address_size(&mut self, value: u32) {
        self.maximum_address_size = Some(value);
    }

    /// Gets the value of MaximumAddressSize
    pub fn get_maximum_address_size(&self) -> Option<&u32> {
        self.maximum_address_size.as_ref()
    }

    /// Sets the value of MaximumMessageSize
    pub fn set_maximum_message_size(&mut self, value: u32) {
        self.maximum_message_size = Some(value);
    }

    /// Gets the value of MaximumMessageSize
    pub fn get_maximum_message_size(&self) -> Option<&u32> {
        self.maximum_message_size.as_ref()
    }

    /// Sets the value of MessageOriented
    pub fn set_message_oriented(&mut self, value: bool) {
        self.message_oriented = Some(value);
    }

    /// Gets the value of MessageOriented
    pub fn get_message_oriented(&self) -> Option<&bool> {
        self.message_oriented.as_ref()
    }

    /// Sets the value of MinimumAddressSize
    pub fn set_minimum_address_size(&mut self, value: u32) {
        self.minimum_address_size = Some(value);
    }

    /// Gets the value of MinimumAddressSize
    pub fn get_minimum_address_size(&self) -> Option<&u32> {
        self.minimum_address_size.as_ref()
    }

    /// Sets the value of PseudoStreamOriented
    pub fn set_pseudo_stream_oriented(&mut self, value: bool) {
        self.pseudo_stream_oriented = Some(value);
    }

    /// Gets the value of PseudoStreamOriented
    pub fn get_pseudo_stream_oriented(&self) -> Option<&bool> {
        self.pseudo_stream_oriented.as_ref()
    }

    /// Sets the value of SupportsBroadcasting
    pub fn set_supports_broadcasting(&mut self, value: bool) {
        self.supports_broadcasting = Some(value);
    }

    /// Gets the value of SupportsBroadcasting
    pub fn get_supports_broadcasting(&self) -> Option<&bool> {
        self.supports_broadcasting.as_ref()
    }

    /// Sets the value of SupportsConnectData
    pub fn set_supports_connect_data(&mut self, value: bool) {
        self.supports_connect_data = Some(value);
    }

    /// Gets the value of SupportsConnectData
    pub fn get_supports_connect_data(&self) -> Option<&bool> {
        self.supports_connect_data.as_ref()
    }

    /// Sets the value of SupportsDisconnectData
    pub fn set_supports_disconnect_data(&mut self, value: bool) {
        self.supports_disconnect_data = Some(value);
    }

    /// Gets the value of SupportsDisconnectData
    pub fn get_supports_disconnect_data(&self) -> Option<&bool> {
        self.supports_disconnect_data.as_ref()
    }

    /// Sets the value of SupportsEncryption
    pub fn set_supports_encryption(&mut self, value: bool) {
        self.supports_encryption = Some(value);
    }

    /// Gets the value of SupportsEncryption
    pub fn get_supports_encryption(&self) -> Option<&bool> {
        self.supports_encryption.as_ref()
    }

    /// Sets the value of SupportsExpeditedData
    pub fn set_supports_expedited_data(&mut self, value: bool) {
        self.supports_expedited_data = Some(value);
    }

    /// Gets the value of SupportsExpeditedData
    pub fn get_supports_expedited_data(&self) -> Option<&bool> {
        self.supports_expedited_data.as_ref()
    }

    /// Sets the value of SupportsFragmentation
    pub fn set_supports_fragmentation(&mut self, value: bool) {
        self.supports_fragmentation = Some(value);
    }

    /// Gets the value of SupportsFragmentation
    pub fn get_supports_fragmentation(&self) -> Option<&bool> {
        self.supports_fragmentation.as_ref()
    }

    /// Sets the value of SupportsGracefulClosing
    pub fn set_supports_graceful_closing(&mut self, value: bool) {
        self.supports_graceful_closing = Some(value);
    }

    /// Gets the value of SupportsGracefulClosing
    pub fn get_supports_graceful_closing(&self) -> Option<&bool> {
        self.supports_graceful_closing.as_ref()
    }

    /// Sets the value of SupportsGuaranteedBandwidth
    pub fn set_supports_guaranteed_bandwidth(&mut self, value: bool) {
        self.supports_guaranteed_bandwidth = Some(value);
    }

    /// Gets the value of SupportsGuaranteedBandwidth
    pub fn get_supports_guaranteed_bandwidth(&self) -> Option<&bool> {
        self.supports_guaranteed_bandwidth.as_ref()
    }

    /// Sets the value of SupportsMulticasting
    pub fn set_supports_multicasting(&mut self, value: bool) {
        self.supports_multicasting = Some(value);
    }

    /// Gets the value of SupportsMulticasting
    pub fn get_supports_multicasting(&self) -> Option<&bool> {
        self.supports_multicasting.as_ref()
    }

    /// Sets the value of SupportsQualityofService
    pub fn set_supports_qualityof_service(&mut self, value: bool) {
        self.supports_qualityof_service = Some(value);
    }

    /// Gets the value of SupportsQualityofService
    pub fn get_supports_qualityof_service(&self) -> Option<&bool> {
        self.supports_qualityof_service.as_ref()
    }
}

