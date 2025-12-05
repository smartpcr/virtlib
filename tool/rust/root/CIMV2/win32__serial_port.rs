// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SerialPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SerialPort {
    #[serde(flatten)]
    pub base: CIM_SerialController,

/// 
    #[serde(rename = "Binary")]
    pub binary: Option<bool>,

/// 
    #[serde(rename = "MaximumInputBufferSize")]
    pub maximum_input_buffer_size: Option<u32>,

/// 
    #[serde(rename = "MaximumOutputBufferSize")]
    pub maximum_output_buffer_size: Option<u32>,

/// 
    #[serde(rename = "OSAutoDiscovered")]
    pub osauto_discovered: Option<bool>,

/// 
    #[serde(rename = "ProviderType")]
    pub provider_type: Option<String>,

/// 
    #[serde(rename = "SettableBaudRate")]
    pub settable_baud_rate: Option<bool>,

/// 
    #[serde(rename = "SettableDataBits")]
    pub settable_data_bits: Option<bool>,

/// 
    #[serde(rename = "SettableFlowControl")]
    pub settable_flow_control: Option<bool>,

/// 
    #[serde(rename = "SettableParity")]
    pub settable_parity: Option<bool>,

/// 
    #[serde(rename = "SettableParityCheck")]
    pub settable_parity_check: Option<bool>,

/// 
    #[serde(rename = "SettableRLSD")]
    pub settable_rlsd: Option<bool>,

/// 
    #[serde(rename = "SettableStopBits")]
    pub settable_stop_bits: Option<bool>,

/// 
    #[serde(rename = "Supports16BitMode")]
    pub supports16_bit_mode: Option<bool>,

/// 
    #[serde(rename = "SupportsDTRDSR")]
    pub supports_dtrdsr: Option<bool>,

/// 
    #[serde(rename = "SupportsElapsedTimeouts")]
    pub supports_elapsed_timeouts: Option<bool>,

/// 
    #[serde(rename = "SupportsIntTimeouts")]
    pub supports_int_timeouts: Option<bool>,

/// 
    #[serde(rename = "SupportsParityCheck")]
    pub supports_parity_check: Option<bool>,

/// 
    #[serde(rename = "SupportsRLSD")]
    pub supports_rlsd: Option<bool>,

/// 
    #[serde(rename = "SupportsRTSCTS")]
    pub supports_rtscts: Option<bool>,

/// 
    #[serde(rename = "SupportsSpecialCharacters")]
    pub supports_special_characters: Option<bool>,

/// 
    #[serde(rename = "SupportsXOnXOff")]
    pub supports_xon_xoff: Option<bool>,

/// 
    #[serde(rename = "SupportsXOnXOffSet")]
    pub supports_xon_xoff_set: Option<bool>,
}

impl Win32_SerialPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SerialController::new(),
            binary: None,
            maximum_input_buffer_size: None,
            maximum_output_buffer_size: None,
            osauto_discovered: None,
            provider_type: None,
            settable_baud_rate: None,
            settable_data_bits: None,
            settable_flow_control: None,
            settable_parity: None,
            settable_parity_check: None,
            settable_rlsd: None,
            settable_stop_bits: None,
            supports16_bit_mode: None,
            supports_dtrdsr: None,
            supports_elapsed_timeouts: None,
            supports_int_timeouts: None,
            supports_parity_check: None,
            supports_rlsd: None,
            supports_rtscts: None,
            supports_special_characters: None,
            supports_xon_xoff: None,
            supports_xon_xoff_set: None,
        }
    }


    /// Sets the value of Binary
    pub fn set_binary(&mut self, value: bool) {
        self.binary = Some(value);
    }

    /// Gets the value of Binary
    pub fn get_binary(&self) -> Option<&bool> {
        self.binary.as_ref()
    }

    /// Sets the value of MaximumInputBufferSize
    pub fn set_maximum_input_buffer_size(&mut self, value: u32) {
        self.maximum_input_buffer_size = Some(value);
    }

    /// Gets the value of MaximumInputBufferSize
    pub fn get_maximum_input_buffer_size(&self) -> Option<&u32> {
        self.maximum_input_buffer_size.as_ref()
    }

    /// Sets the value of MaximumOutputBufferSize
    pub fn set_maximum_output_buffer_size(&mut self, value: u32) {
        self.maximum_output_buffer_size = Some(value);
    }

    /// Gets the value of MaximumOutputBufferSize
    pub fn get_maximum_output_buffer_size(&self) -> Option<&u32> {
        self.maximum_output_buffer_size.as_ref()
    }

    /// Sets the value of OSAutoDiscovered
    pub fn set_osauto_discovered(&mut self, value: bool) {
        self.osauto_discovered = Some(value);
    }

    /// Gets the value of OSAutoDiscovered
    pub fn get_osauto_discovered(&self) -> Option<&bool> {
        self.osauto_discovered.as_ref()
    }

    /// Sets the value of ProviderType
    pub fn set_provider_type(&mut self, value: String) {
        self.provider_type = Some(value);
    }

    /// Gets the value of ProviderType
    pub fn get_provider_type(&self) -> Option<&String> {
        self.provider_type.as_ref()
    }

    /// Sets the value of SettableBaudRate
    pub fn set_settable_baud_rate(&mut self, value: bool) {
        self.settable_baud_rate = Some(value);
    }

    /// Gets the value of SettableBaudRate
    pub fn get_settable_baud_rate(&self) -> Option<&bool> {
        self.settable_baud_rate.as_ref()
    }

    /// Sets the value of SettableDataBits
    pub fn set_settable_data_bits(&mut self, value: bool) {
        self.settable_data_bits = Some(value);
    }

    /// Gets the value of SettableDataBits
    pub fn get_settable_data_bits(&self) -> Option<&bool> {
        self.settable_data_bits.as_ref()
    }

    /// Sets the value of SettableFlowControl
    pub fn set_settable_flow_control(&mut self, value: bool) {
        self.settable_flow_control = Some(value);
    }

    /// Gets the value of SettableFlowControl
    pub fn get_settable_flow_control(&self) -> Option<&bool> {
        self.settable_flow_control.as_ref()
    }

    /// Sets the value of SettableParity
    pub fn set_settable_parity(&mut self, value: bool) {
        self.settable_parity = Some(value);
    }

    /// Gets the value of SettableParity
    pub fn get_settable_parity(&self) -> Option<&bool> {
        self.settable_parity.as_ref()
    }

    /// Sets the value of SettableParityCheck
    pub fn set_settable_parity_check(&mut self, value: bool) {
        self.settable_parity_check = Some(value);
    }

    /// Gets the value of SettableParityCheck
    pub fn get_settable_parity_check(&self) -> Option<&bool> {
        self.settable_parity_check.as_ref()
    }

    /// Sets the value of SettableRLSD
    pub fn set_settable_rlsd(&mut self, value: bool) {
        self.settable_rlsd = Some(value);
    }

    /// Gets the value of SettableRLSD
    pub fn get_settable_rlsd(&self) -> Option<&bool> {
        self.settable_rlsd.as_ref()
    }

    /// Sets the value of SettableStopBits
    pub fn set_settable_stop_bits(&mut self, value: bool) {
        self.settable_stop_bits = Some(value);
    }

    /// Gets the value of SettableStopBits
    pub fn get_settable_stop_bits(&self) -> Option<&bool> {
        self.settable_stop_bits.as_ref()
    }

    /// Sets the value of Supports16BitMode
    pub fn set_supports16_bit_mode(&mut self, value: bool) {
        self.supports16_bit_mode = Some(value);
    }

    /// Gets the value of Supports16BitMode
    pub fn get_supports16_bit_mode(&self) -> Option<&bool> {
        self.supports16_bit_mode.as_ref()
    }

    /// Sets the value of SupportsDTRDSR
    pub fn set_supports_dtrdsr(&mut self, value: bool) {
        self.supports_dtrdsr = Some(value);
    }

    /// Gets the value of SupportsDTRDSR
    pub fn get_supports_dtrdsr(&self) -> Option<&bool> {
        self.supports_dtrdsr.as_ref()
    }

    /// Sets the value of SupportsElapsedTimeouts
    pub fn set_supports_elapsed_timeouts(&mut self, value: bool) {
        self.supports_elapsed_timeouts = Some(value);
    }

    /// Gets the value of SupportsElapsedTimeouts
    pub fn get_supports_elapsed_timeouts(&self) -> Option<&bool> {
        self.supports_elapsed_timeouts.as_ref()
    }

    /// Sets the value of SupportsIntTimeouts
    pub fn set_supports_int_timeouts(&mut self, value: bool) {
        self.supports_int_timeouts = Some(value);
    }

    /// Gets the value of SupportsIntTimeouts
    pub fn get_supports_int_timeouts(&self) -> Option<&bool> {
        self.supports_int_timeouts.as_ref()
    }

    /// Sets the value of SupportsParityCheck
    pub fn set_supports_parity_check(&mut self, value: bool) {
        self.supports_parity_check = Some(value);
    }

    /// Gets the value of SupportsParityCheck
    pub fn get_supports_parity_check(&self) -> Option<&bool> {
        self.supports_parity_check.as_ref()
    }

    /// Sets the value of SupportsRLSD
    pub fn set_supports_rlsd(&mut self, value: bool) {
        self.supports_rlsd = Some(value);
    }

    /// Gets the value of SupportsRLSD
    pub fn get_supports_rlsd(&self) -> Option<&bool> {
        self.supports_rlsd.as_ref()
    }

    /// Sets the value of SupportsRTSCTS
    pub fn set_supports_rtscts(&mut self, value: bool) {
        self.supports_rtscts = Some(value);
    }

    /// Gets the value of SupportsRTSCTS
    pub fn get_supports_rtscts(&self) -> Option<&bool> {
        self.supports_rtscts.as_ref()
    }

    /// Sets the value of SupportsSpecialCharacters
    pub fn set_supports_special_characters(&mut self, value: bool) {
        self.supports_special_characters = Some(value);
    }

    /// Gets the value of SupportsSpecialCharacters
    pub fn get_supports_special_characters(&self) -> Option<&bool> {
        self.supports_special_characters.as_ref()
    }

    /// Sets the value of SupportsXOnXOff
    pub fn set_supports_xon_xoff(&mut self, value: bool) {
        self.supports_xon_xoff = Some(value);
    }

    /// Gets the value of SupportsXOnXOff
    pub fn get_supports_xon_xoff(&self) -> Option<&bool> {
        self.supports_xon_xoff.as_ref()
    }

    /// Sets the value of SupportsXOnXOffSet
    pub fn set_supports_xon_xoff_set(&mut self, value: bool) {
        self.supports_xon_xoff_set = Some(value);
    }

    /// Gets the value of SupportsXOnXOffSet
    pub fn get_supports_xon_xoff_set(&self) -> Option<&bool> {
        self.supports_xon_xoff_set.as_ref()
    }
}

