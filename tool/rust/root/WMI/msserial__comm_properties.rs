// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSerial_CommProperties struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSerial_CommProperties {
    #[serde(flatten)]
    pub base: MSSerial,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "dwCurrentRxQueue")]
    pub dw_current_rx_queue: Option<u32>,

/// 
    #[serde(rename = "dwCurrentTxQueue")]
    pub dw_current_tx_queue: Option<u32>,

/// 
    #[serde(rename = "dwMaxBaud")]
    pub dw_max_baud: Option<u32>,

/// 
    #[serde(rename = "dwMaxRxQueue")]
    pub dw_max_rx_queue: Option<u32>,

/// 
    #[serde(rename = "dwMaxTxQueue")]
    pub dw_max_tx_queue: Option<u32>,

/// 
    #[serde(rename = "dwProvCapabilities")]
    pub dw_prov_capabilities: Option<u32>,

/// 
    #[serde(rename = "dwProvCharSize")]
    pub dw_prov_char_size: Option<u32>,

/// 
    #[serde(rename = "dwProvSpec1")]
    pub dw_prov_spec1: Option<u32>,

/// 
    #[serde(rename = "dwProvSpec2")]
    pub dw_prov_spec2: Option<u32>,

/// 
    #[serde(rename = "dwProvSubType")]
    pub dw_prov_sub_type: Option<u32>,

/// 
    #[serde(rename = "dwReserved1")]
    pub dw_reserved1: Option<u32>,

/// 
    #[serde(rename = "dwServiceMask")]
    pub dw_service_mask: Option<u32>,

/// 
    #[serde(rename = "dwSettableBaud")]
    pub dw_settable_baud: Option<u32>,

/// 
    #[serde(rename = "dwSettableParams")]
    pub dw_settable_params: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "wcProvChar")]
    pub wc_prov_char: Vec<u8>,

/// 
    #[serde(rename = "wPacketLength")]
    pub w_packet_length: Option<u16>,

/// 
    #[serde(rename = "wPacketVersion")]
    pub w_packet_version: Option<u16>,

/// 
    #[serde(rename = "wSettableData")]
    pub w_settable_data: Option<u16>,

/// 
    #[serde(rename = "wSettableStopParity")]
    pub w_settable_stop_parity: Option<u16>,
}

impl MSSerial_CommProperties {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSSerial::new(),
            active: None,
            dw_current_rx_queue: None,
            dw_current_tx_queue: None,
            dw_max_baud: None,
            dw_max_rx_queue: None,
            dw_max_tx_queue: None,
            dw_prov_capabilities: None,
            dw_prov_char_size: None,
            dw_prov_spec1: None,
            dw_prov_spec2: None,
            dw_prov_sub_type: None,
            dw_reserved1: None,
            dw_service_mask: None,
            dw_settable_baud: None,
            dw_settable_params: None,
            instance_name: None,
            wc_prov_char: Vec::new(),
            w_packet_length: None,
            w_packet_version: None,
            w_settable_data: None,
            w_settable_stop_parity: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of dwCurrentRxQueue
    pub fn set_dw_current_rx_queue(&mut self, value: u32) {
        self.dw_current_rx_queue = Some(value);
    }

    /// Gets the value of dwCurrentRxQueue
    pub fn get_dw_current_rx_queue(&self) -> Option<&u32> {
        self.dw_current_rx_queue.as_ref()
    }

    /// Sets the value of dwCurrentTxQueue
    pub fn set_dw_current_tx_queue(&mut self, value: u32) {
        self.dw_current_tx_queue = Some(value);
    }

    /// Gets the value of dwCurrentTxQueue
    pub fn get_dw_current_tx_queue(&self) -> Option<&u32> {
        self.dw_current_tx_queue.as_ref()
    }

    /// Sets the value of dwMaxBaud
    pub fn set_dw_max_baud(&mut self, value: u32) {
        self.dw_max_baud = Some(value);
    }

    /// Gets the value of dwMaxBaud
    pub fn get_dw_max_baud(&self) -> Option<&u32> {
        self.dw_max_baud.as_ref()
    }

    /// Sets the value of dwMaxRxQueue
    pub fn set_dw_max_rx_queue(&mut self, value: u32) {
        self.dw_max_rx_queue = Some(value);
    }

    /// Gets the value of dwMaxRxQueue
    pub fn get_dw_max_rx_queue(&self) -> Option<&u32> {
        self.dw_max_rx_queue.as_ref()
    }

    /// Sets the value of dwMaxTxQueue
    pub fn set_dw_max_tx_queue(&mut self, value: u32) {
        self.dw_max_tx_queue = Some(value);
    }

    /// Gets the value of dwMaxTxQueue
    pub fn get_dw_max_tx_queue(&self) -> Option<&u32> {
        self.dw_max_tx_queue.as_ref()
    }

    /// Sets the value of dwProvCapabilities
    pub fn set_dw_prov_capabilities(&mut self, value: u32) {
        self.dw_prov_capabilities = Some(value);
    }

    /// Gets the value of dwProvCapabilities
    pub fn get_dw_prov_capabilities(&self) -> Option<&u32> {
        self.dw_prov_capabilities.as_ref()
    }

    /// Sets the value of dwProvCharSize
    pub fn set_dw_prov_char_size(&mut self, value: u32) {
        self.dw_prov_char_size = Some(value);
    }

    /// Gets the value of dwProvCharSize
    pub fn get_dw_prov_char_size(&self) -> Option<&u32> {
        self.dw_prov_char_size.as_ref()
    }

    /// Sets the value of dwProvSpec1
    pub fn set_dw_prov_spec1(&mut self, value: u32) {
        self.dw_prov_spec1 = Some(value);
    }

    /// Gets the value of dwProvSpec1
    pub fn get_dw_prov_spec1(&self) -> Option<&u32> {
        self.dw_prov_spec1.as_ref()
    }

    /// Sets the value of dwProvSpec2
    pub fn set_dw_prov_spec2(&mut self, value: u32) {
        self.dw_prov_spec2 = Some(value);
    }

    /// Gets the value of dwProvSpec2
    pub fn get_dw_prov_spec2(&self) -> Option<&u32> {
        self.dw_prov_spec2.as_ref()
    }

    /// Sets the value of dwProvSubType
    pub fn set_dw_prov_sub_type(&mut self, value: u32) {
        self.dw_prov_sub_type = Some(value);
    }

    /// Gets the value of dwProvSubType
    pub fn get_dw_prov_sub_type(&self) -> Option<&u32> {
        self.dw_prov_sub_type.as_ref()
    }

    /// Sets the value of dwReserved1
    pub fn set_dw_reserved1(&mut self, value: u32) {
        self.dw_reserved1 = Some(value);
    }

    /// Gets the value of dwReserved1
    pub fn get_dw_reserved1(&self) -> Option<&u32> {
        self.dw_reserved1.as_ref()
    }

    /// Sets the value of dwServiceMask
    pub fn set_dw_service_mask(&mut self, value: u32) {
        self.dw_service_mask = Some(value);
    }

    /// Gets the value of dwServiceMask
    pub fn get_dw_service_mask(&self) -> Option<&u32> {
        self.dw_service_mask.as_ref()
    }

    /// Sets the value of dwSettableBaud
    pub fn set_dw_settable_baud(&mut self, value: u32) {
        self.dw_settable_baud = Some(value);
    }

    /// Gets the value of dwSettableBaud
    pub fn get_dw_settable_baud(&self) -> Option<&u32> {
        self.dw_settable_baud.as_ref()
    }

    /// Sets the value of dwSettableParams
    pub fn set_dw_settable_params(&mut self, value: u32) {
        self.dw_settable_params = Some(value);
    }

    /// Gets the value of dwSettableParams
    pub fn get_dw_settable_params(&self) -> Option<&u32> {
        self.dw_settable_params.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of wcProvChar
    pub fn set_wc_prov_char(&mut self, value: Vec<u8>) {
        self.wc_prov_char = value;
    }

    /// Gets the value of wcProvChar
    pub fn get_wc_prov_char(&self) -> &Vec<u8> {
        &self.wc_prov_char
    }

    /// Sets the value of wPacketLength
    pub fn set_w_packet_length(&mut self, value: u16) {
        self.w_packet_length = Some(value);
    }

    /// Gets the value of wPacketLength
    pub fn get_w_packet_length(&self) -> Option<&u16> {
        self.w_packet_length.as_ref()
    }

    /// Sets the value of wPacketVersion
    pub fn set_w_packet_version(&mut self, value: u16) {
        self.w_packet_version = Some(value);
    }

    /// Gets the value of wPacketVersion
    pub fn get_w_packet_version(&self) -> Option<&u16> {
        self.w_packet_version.as_ref()
    }

    /// Sets the value of wSettableData
    pub fn set_w_settable_data(&mut self, value: u16) {
        self.w_settable_data = Some(value);
    }

    /// Gets the value of wSettableData
    pub fn get_w_settable_data(&self) -> Option<&u16> {
        self.w_settable_data.as_ref()
    }

    /// Sets the value of wSettableStopParity
    pub fn set_w_settable_stop_parity(&mut self, value: u16) {
        self.w_settable_stop_parity = Some(value);
    }

    /// Gets the value of wSettableStopParity
    pub fn get_w_settable_stop_parity(&self) -> Option<&u16> {
        self.w_settable_stop_parity.as_ref()
    }
}

