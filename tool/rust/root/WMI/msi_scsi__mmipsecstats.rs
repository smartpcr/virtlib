// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_MMIPSECStats struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_MMIPSECStats {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AcquireFailures")]
    pub acquire_failures: Option<u64>,

/// 
    #[serde(rename = "AcquireHeapSize")]
    pub acquire_heap_size: Option<u64>,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "ActiveAcquire")]
    pub active_acquire: Option<u64>,

/// 
    #[serde(rename = "ActiveReceive")]
    pub active_receive: Option<u64>,

/// 
    #[serde(rename = "AuthenticationFailures")]
    pub authentication_failures: Option<u64>,

/// 
    #[serde(rename = "ConnectionListSize")]
    pub connection_list_size: Option<u64>,

/// 
    #[serde(rename = "GetSPIFailures")]
    pub get_spifailures: Option<u64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "InvalidCookiesReceived")]
    pub invalid_cookies_received: Option<u64>,

/// 
    #[serde(rename = "InvalidPackets")]
    pub invalid_packets: Option<u64>,

/// 
    #[serde(rename = "KeyAdditionFailures")]
    pub key_addition_failures: Option<u64>,

/// 
    #[serde(rename = "KeyAdditions")]
    pub key_additions: Option<u64>,

/// 
    #[serde(rename = "KeyUpdateFailures")]
    pub key_update_failures: Option<u64>,

/// 
    #[serde(rename = "KeyUpdates")]
    pub key_updates: Option<u64>,

/// 
    #[serde(rename = "NegotiationFailures")]
    pub negotiation_failures: Option<u64>,

/// 
    #[serde(rename = "OakleyMainMode")]
    pub oakley_main_mode: Option<u64>,

/// 
    #[serde(rename = "OakleyQuickMode")]
    pub oakley_quick_mode: Option<u64>,

/// 
    #[serde(rename = "ReceiveFailures")]
    pub receive_failures: Option<u64>,

/// 
    #[serde(rename = "ReceiveHeapSize")]
    pub receive_heap_size: Option<u64>,

/// 
    #[serde(rename = "SendFailures")]
    pub send_failures: Option<u64>,

/// 
    #[serde(rename = "SoftAssociations")]
    pub soft_associations: Option<u64>,

/// 
    #[serde(rename = "TotalGetSPI")]
    pub total_get_spi: Option<u64>,
}

impl MSiSCSI_MMIPSECStats {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            acquire_failures: None,
            acquire_heap_size: None,
            active: None,
            active_acquire: None,
            active_receive: None,
            authentication_failures: None,
            connection_list_size: None,
            get_spifailures: None,
            instance_name: None,
            invalid_cookies_received: None,
            invalid_packets: None,
            key_addition_failures: None,
            key_additions: None,
            key_update_failures: None,
            key_updates: None,
            negotiation_failures: None,
            oakley_main_mode: None,
            oakley_quick_mode: None,
            receive_failures: None,
            receive_heap_size: None,
            send_failures: None,
            soft_associations: None,
            total_get_spi: None,
        }
    }


    /// Sets the value of AcquireFailures
    pub fn set_acquire_failures(&mut self, value: u64) {
        self.acquire_failures = Some(value);
    }

    /// Gets the value of AcquireFailures
    pub fn get_acquire_failures(&self) -> Option<&u64> {
        self.acquire_failures.as_ref()
    }

    /// Sets the value of AcquireHeapSize
    pub fn set_acquire_heap_size(&mut self, value: u64) {
        self.acquire_heap_size = Some(value);
    }

    /// Gets the value of AcquireHeapSize
    pub fn get_acquire_heap_size(&self) -> Option<&u64> {
        self.acquire_heap_size.as_ref()
    }

    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of ActiveAcquire
    pub fn set_active_acquire(&mut self, value: u64) {
        self.active_acquire = Some(value);
    }

    /// Gets the value of ActiveAcquire
    pub fn get_active_acquire(&self) -> Option<&u64> {
        self.active_acquire.as_ref()
    }

    /// Sets the value of ActiveReceive
    pub fn set_active_receive(&mut self, value: u64) {
        self.active_receive = Some(value);
    }

    /// Gets the value of ActiveReceive
    pub fn get_active_receive(&self) -> Option<&u64> {
        self.active_receive.as_ref()
    }

    /// Sets the value of AuthenticationFailures
    pub fn set_authentication_failures(&mut self, value: u64) {
        self.authentication_failures = Some(value);
    }

    /// Gets the value of AuthenticationFailures
    pub fn get_authentication_failures(&self) -> Option<&u64> {
        self.authentication_failures.as_ref()
    }

    /// Sets the value of ConnectionListSize
    pub fn set_connection_list_size(&mut self, value: u64) {
        self.connection_list_size = Some(value);
    }

    /// Gets the value of ConnectionListSize
    pub fn get_connection_list_size(&self) -> Option<&u64> {
        self.connection_list_size.as_ref()
    }

    /// Sets the value of GetSPIFailures
    pub fn set_get_spifailures(&mut self, value: u64) {
        self.get_spifailures = Some(value);
    }

    /// Gets the value of GetSPIFailures
    pub fn get_get_spifailures(&self) -> Option<&u64> {
        self.get_spifailures.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of InvalidCookiesReceived
    pub fn set_invalid_cookies_received(&mut self, value: u64) {
        self.invalid_cookies_received = Some(value);
    }

    /// Gets the value of InvalidCookiesReceived
    pub fn get_invalid_cookies_received(&self) -> Option<&u64> {
        self.invalid_cookies_received.as_ref()
    }

    /// Sets the value of InvalidPackets
    pub fn set_invalid_packets(&mut self, value: u64) {
        self.invalid_packets = Some(value);
    }

    /// Gets the value of InvalidPackets
    pub fn get_invalid_packets(&self) -> Option<&u64> {
        self.invalid_packets.as_ref()
    }

    /// Sets the value of KeyAdditionFailures
    pub fn set_key_addition_failures(&mut self, value: u64) {
        self.key_addition_failures = Some(value);
    }

    /// Gets the value of KeyAdditionFailures
    pub fn get_key_addition_failures(&self) -> Option<&u64> {
        self.key_addition_failures.as_ref()
    }

    /// Sets the value of KeyAdditions
    pub fn set_key_additions(&mut self, value: u64) {
        self.key_additions = Some(value);
    }

    /// Gets the value of KeyAdditions
    pub fn get_key_additions(&self) -> Option<&u64> {
        self.key_additions.as_ref()
    }

    /// Sets the value of KeyUpdateFailures
    pub fn set_key_update_failures(&mut self, value: u64) {
        self.key_update_failures = Some(value);
    }

    /// Gets the value of KeyUpdateFailures
    pub fn get_key_update_failures(&self) -> Option<&u64> {
        self.key_update_failures.as_ref()
    }

    /// Sets the value of KeyUpdates
    pub fn set_key_updates(&mut self, value: u64) {
        self.key_updates = Some(value);
    }

    /// Gets the value of KeyUpdates
    pub fn get_key_updates(&self) -> Option<&u64> {
        self.key_updates.as_ref()
    }

    /// Sets the value of NegotiationFailures
    pub fn set_negotiation_failures(&mut self, value: u64) {
        self.negotiation_failures = Some(value);
    }

    /// Gets the value of NegotiationFailures
    pub fn get_negotiation_failures(&self) -> Option<&u64> {
        self.negotiation_failures.as_ref()
    }

    /// Sets the value of OakleyMainMode
    pub fn set_oakley_main_mode(&mut self, value: u64) {
        self.oakley_main_mode = Some(value);
    }

    /// Gets the value of OakleyMainMode
    pub fn get_oakley_main_mode(&self) -> Option<&u64> {
        self.oakley_main_mode.as_ref()
    }

    /// Sets the value of OakleyQuickMode
    pub fn set_oakley_quick_mode(&mut self, value: u64) {
        self.oakley_quick_mode = Some(value);
    }

    /// Gets the value of OakleyQuickMode
    pub fn get_oakley_quick_mode(&self) -> Option<&u64> {
        self.oakley_quick_mode.as_ref()
    }

    /// Sets the value of ReceiveFailures
    pub fn set_receive_failures(&mut self, value: u64) {
        self.receive_failures = Some(value);
    }

    /// Gets the value of ReceiveFailures
    pub fn get_receive_failures(&self) -> Option<&u64> {
        self.receive_failures.as_ref()
    }

    /// Sets the value of ReceiveHeapSize
    pub fn set_receive_heap_size(&mut self, value: u64) {
        self.receive_heap_size = Some(value);
    }

    /// Gets the value of ReceiveHeapSize
    pub fn get_receive_heap_size(&self) -> Option<&u64> {
        self.receive_heap_size.as_ref()
    }

    /// Sets the value of SendFailures
    pub fn set_send_failures(&mut self, value: u64) {
        self.send_failures = Some(value);
    }

    /// Gets the value of SendFailures
    pub fn get_send_failures(&self) -> Option<&u64> {
        self.send_failures.as_ref()
    }

    /// Sets the value of SoftAssociations
    pub fn set_soft_associations(&mut self, value: u64) {
        self.soft_associations = Some(value);
    }

    /// Gets the value of SoftAssociations
    pub fn get_soft_associations(&self) -> Option<&u64> {
        self.soft_associations.as_ref()
    }

    /// Sets the value of TotalGetSPI
    pub fn set_total_get_spi(&mut self, value: u64) {
        self.total_get_spi = Some(value);
    }

    /// Gets the value of TotalGetSPI
    pub fn get_total_get_spi(&self) -> Option<&u64> {
        self.total_get_spi.as_ref()
    }
}

