// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSerial_CommInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSerial_CommInfo {
    #[serde(flatten)]
    pub base: MSSerial,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BaudRate")]
    pub baud_rate: Option<u32>,

/// 
    #[serde(rename = "BitsPerByte")]
    pub bits_per_byte: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "IsBusy")]
    pub is_busy: Option<bool>,

/// 
    #[serde(rename = "MaximumBaudRate")]
    pub maximum_baud_rate: Option<u32>,

/// 
    #[serde(rename = "MaximumInputBufferSize")]
    pub maximum_input_buffer_size: Option<u32>,

/// 
    #[serde(rename = "MaximumOutputBufferSize")]
    pub maximum_output_buffer_size: Option<u32>,

/// 
    #[serde(rename = "Parity")]
    pub parity: Option<u32>,

/// 
    #[serde(rename = "ParityCheckEnable")]
    pub parity_check_enable: Option<bool>,

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
    #[serde(rename = "SettableStopBits")]
    pub settable_stop_bits: Option<bool>,

/// 
    #[serde(rename = "StopBits")]
    pub stop_bits: Option<u32>,

/// 
    #[serde(rename = "Support16BitMode")]
    pub support16_bit_mode: Option<bool>,

/// 
    #[serde(rename = "SupportDTRDSR")]
    pub support_dtrdsr: Option<bool>,

/// 
    #[serde(rename = "SupportIntervalTimeouts")]
    pub support_interval_timeouts: Option<bool>,

/// 
    #[serde(rename = "SupportParityCheck")]
    pub support_parity_check: Option<bool>,

/// 
    #[serde(rename = "SupportRTSCTS")]
    pub support_rtscts: Option<bool>,

/// 
    #[serde(rename = "SupportXonXoff")]
    pub support_xon_xoff: Option<bool>,

/// 
    #[serde(rename = "XoffCharacter")]
    pub xoff_character: Option<u32>,

/// 
    #[serde(rename = "XoffXmitThreshold")]
    pub xoff_xmit_threshold: Option<u32>,

/// 
    #[serde(rename = "XonCharacter")]
    pub xon_character: Option<u32>,

/// 
    #[serde(rename = "XonXmitThreshold")]
    pub xon_xmit_threshold: Option<u32>,
}

impl MSSerial_CommInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSSerial::new(),
            active: None,
            baud_rate: None,
            bits_per_byte: None,
            instance_name: None,
            is_busy: None,
            maximum_baud_rate: None,
            maximum_input_buffer_size: None,
            maximum_output_buffer_size: None,
            parity: None,
            parity_check_enable: None,
            settable_baud_rate: None,
            settable_data_bits: None,
            settable_flow_control: None,
            settable_parity: None,
            settable_parity_check: None,
            settable_stop_bits: None,
            stop_bits: None,
            support16_bit_mode: None,
            support_dtrdsr: None,
            support_interval_timeouts: None,
            support_parity_check: None,
            support_rtscts: None,
            support_xon_xoff: None,
            xoff_character: None,
            xoff_xmit_threshold: None,
            xon_character: None,
            xon_xmit_threshold: None,
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

    /// Sets the value of BaudRate
    pub fn set_baud_rate(&mut self, value: u32) {
        self.baud_rate = Some(value);
    }

    /// Gets the value of BaudRate
    pub fn get_baud_rate(&self) -> Option<&u32> {
        self.baud_rate.as_ref()
    }

    /// Sets the value of BitsPerByte
    pub fn set_bits_per_byte(&mut self, value: u32) {
        self.bits_per_byte = Some(value);
    }

    /// Gets the value of BitsPerByte
    pub fn get_bits_per_byte(&self) -> Option<&u32> {
        self.bits_per_byte.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of IsBusy
    pub fn set_is_busy(&mut self, value: bool) {
        self.is_busy = Some(value);
    }

    /// Gets the value of IsBusy
    pub fn get_is_busy(&self) -> Option<&bool> {
        self.is_busy.as_ref()
    }

    /// Sets the value of MaximumBaudRate
    pub fn set_maximum_baud_rate(&mut self, value: u32) {
        self.maximum_baud_rate = Some(value);
    }

    /// Gets the value of MaximumBaudRate
    pub fn get_maximum_baud_rate(&self) -> Option<&u32> {
        self.maximum_baud_rate.as_ref()
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

    /// Sets the value of Parity
    pub fn set_parity(&mut self, value: u32) {
        self.parity = Some(value);
    }

    /// Gets the value of Parity
    pub fn get_parity(&self) -> Option<&u32> {
        self.parity.as_ref()
    }

    /// Sets the value of ParityCheckEnable
    pub fn set_parity_check_enable(&mut self, value: bool) {
        self.parity_check_enable = Some(value);
    }

    /// Gets the value of ParityCheckEnable
    pub fn get_parity_check_enable(&self) -> Option<&bool> {
        self.parity_check_enable.as_ref()
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

    /// Sets the value of SettableStopBits
    pub fn set_settable_stop_bits(&mut self, value: bool) {
        self.settable_stop_bits = Some(value);
    }

    /// Gets the value of SettableStopBits
    pub fn get_settable_stop_bits(&self) -> Option<&bool> {
        self.settable_stop_bits.as_ref()
    }

    /// Sets the value of StopBits
    pub fn set_stop_bits(&mut self, value: u32) {
        self.stop_bits = Some(value);
    }

    /// Gets the value of StopBits
    pub fn get_stop_bits(&self) -> Option<&u32> {
        self.stop_bits.as_ref()
    }

    /// Sets the value of Support16BitMode
    pub fn set_support16_bit_mode(&mut self, value: bool) {
        self.support16_bit_mode = Some(value);
    }

    /// Gets the value of Support16BitMode
    pub fn get_support16_bit_mode(&self) -> Option<&bool> {
        self.support16_bit_mode.as_ref()
    }

    /// Sets the value of SupportDTRDSR
    pub fn set_support_dtrdsr(&mut self, value: bool) {
        self.support_dtrdsr = Some(value);
    }

    /// Gets the value of SupportDTRDSR
    pub fn get_support_dtrdsr(&self) -> Option<&bool> {
        self.support_dtrdsr.as_ref()
    }

    /// Sets the value of SupportIntervalTimeouts
    pub fn set_support_interval_timeouts(&mut self, value: bool) {
        self.support_interval_timeouts = Some(value);
    }

    /// Gets the value of SupportIntervalTimeouts
    pub fn get_support_interval_timeouts(&self) -> Option<&bool> {
        self.support_interval_timeouts.as_ref()
    }

    /// Sets the value of SupportParityCheck
    pub fn set_support_parity_check(&mut self, value: bool) {
        self.support_parity_check = Some(value);
    }

    /// Gets the value of SupportParityCheck
    pub fn get_support_parity_check(&self) -> Option<&bool> {
        self.support_parity_check.as_ref()
    }

    /// Sets the value of SupportRTSCTS
    pub fn set_support_rtscts(&mut self, value: bool) {
        self.support_rtscts = Some(value);
    }

    /// Gets the value of SupportRTSCTS
    pub fn get_support_rtscts(&self) -> Option<&bool> {
        self.support_rtscts.as_ref()
    }

    /// Sets the value of SupportXonXoff
    pub fn set_support_xon_xoff(&mut self, value: bool) {
        self.support_xon_xoff = Some(value);
    }

    /// Gets the value of SupportXonXoff
    pub fn get_support_xon_xoff(&self) -> Option<&bool> {
        self.support_xon_xoff.as_ref()
    }

    /// Sets the value of XoffCharacter
    pub fn set_xoff_character(&mut self, value: u32) {
        self.xoff_character = Some(value);
    }

    /// Gets the value of XoffCharacter
    pub fn get_xoff_character(&self) -> Option<&u32> {
        self.xoff_character.as_ref()
    }

    /// Sets the value of XoffXmitThreshold
    pub fn set_xoff_xmit_threshold(&mut self, value: u32) {
        self.xoff_xmit_threshold = Some(value);
    }

    /// Gets the value of XoffXmitThreshold
    pub fn get_xoff_xmit_threshold(&self) -> Option<&u32> {
        self.xoff_xmit_threshold.as_ref()
    }

    /// Sets the value of XonCharacter
    pub fn set_xon_character(&mut self, value: u32) {
        self.xon_character = Some(value);
    }

    /// Gets the value of XonCharacter
    pub fn get_xon_character(&self) -> Option<&u32> {
        self.xon_character.as_ref()
    }

    /// Sets the value of XonXmitThreshold
    pub fn set_xon_xmit_threshold(&mut self, value: u32) {
        self.xon_xmit_threshold = Some(value);
    }

    /// Gets the value of XonXmitThreshold
    pub fn get_xon_xmit_threshold(&self) -> Option<&u32> {
        self.xon_xmit_threshold.as_ref()
    }
}

