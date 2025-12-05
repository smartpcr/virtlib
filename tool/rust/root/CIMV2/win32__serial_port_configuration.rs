// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SerialPortConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SerialPortConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "AbortReadWriteOnError")]
    pub abort_read_write_on_error: Option<bool>,

/// 
    #[serde(rename = "BaudRate")]
    pub baud_rate: Option<u32>,

/// 
    #[serde(rename = "BinaryModeEnabled")]
    pub binary_mode_enabled: Option<bool>,

/// 
    #[serde(rename = "BitsPerByte")]
    pub bits_per_byte: Option<u32>,

/// 
    #[serde(rename = "ContinueXMitOnXOff")]
    pub continue_xmit_on_xoff: Option<bool>,

/// 
    #[serde(rename = "CTSOutflowControl")]
    pub ctsoutflow_control: Option<bool>,

/// 
    #[serde(rename = "DiscardNULLBytes")]
    pub discard_nullbytes: Option<bool>,

/// 
    #[serde(rename = "DSROutflowControl")]
    pub dsroutflow_control: Option<bool>,

/// 
    #[serde(rename = "DSRSensitivity")]
    pub dsrsensitivity: Option<bool>,

/// 
    #[serde(rename = "DTRFlowControlType")]
    pub dtrflow_control_type: Option<String>,

/// 
    #[serde(rename = "EOFCharacter")]
    pub eofcharacter: Option<u32>,

/// 
    #[serde(rename = "ErrorReplaceCharacter")]
    pub error_replace_character: Option<u32>,

/// 
    #[serde(rename = "ErrorReplacementEnabled")]
    pub error_replacement_enabled: Option<bool>,

/// 
    #[serde(rename = "EventCharacter")]
    pub event_character: Option<u32>,

/// 
    #[serde(rename = "IsBusy")]
    pub is_busy: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Parity")]
    pub parity: Option<String>,

/// 
    #[serde(rename = "ParityCheckEnabled")]
    pub parity_check_enabled: Option<bool>,

/// 
    #[serde(rename = "RTSFlowControlType")]
    pub rtsflow_control_type: Option<String>,

/// 
    #[serde(rename = "StopBits")]
    pub stop_bits: Option<String>,

/// 
    #[serde(rename = "XOffCharacter")]
    pub xoff_character: Option<u32>,

/// 
    #[serde(rename = "XOffXMitThreshold")]
    pub xoff_xmit_threshold: Option<u32>,

/// 
    #[serde(rename = "XOnCharacter")]
    pub xon_character: Option<u32>,

/// 
    #[serde(rename = "XOnXMitThreshold")]
    pub xon_xmit_threshold: Option<u32>,

/// 
    #[serde(rename = "XOnXOffInFlowControl")]
    pub xon_xoff_in_flow_control: Option<u32>,

/// 
    #[serde(rename = "XOnXOffOutFlowControl")]
    pub xon_xoff_out_flow_control: Option<u32>,
}

impl Win32_SerialPortConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            abort_read_write_on_error: None,
            baud_rate: None,
            binary_mode_enabled: None,
            bits_per_byte: None,
            continue_xmit_on_xoff: None,
            ctsoutflow_control: None,
            discard_nullbytes: None,
            dsroutflow_control: None,
            dsrsensitivity: None,
            dtrflow_control_type: None,
            eofcharacter: None,
            error_replace_character: None,
            error_replacement_enabled: None,
            event_character: None,
            is_busy: None,
            name: None,
            parity: None,
            parity_check_enabled: None,
            rtsflow_control_type: None,
            stop_bits: None,
            xoff_character: None,
            xoff_xmit_threshold: None,
            xon_character: None,
            xon_xmit_threshold: None,
            xon_xoff_in_flow_control: None,
            xon_xoff_out_flow_control: None,
        }
    }


    /// Sets the value of AbortReadWriteOnError
    pub fn set_abort_read_write_on_error(&mut self, value: bool) {
        self.abort_read_write_on_error = Some(value);
    }

    /// Gets the value of AbortReadWriteOnError
    pub fn get_abort_read_write_on_error(&self) -> Option<&bool> {
        self.abort_read_write_on_error.as_ref()
    }

    /// Sets the value of BaudRate
    pub fn set_baud_rate(&mut self, value: u32) {
        self.baud_rate = Some(value);
    }

    /// Gets the value of BaudRate
    pub fn get_baud_rate(&self) -> Option<&u32> {
        self.baud_rate.as_ref()
    }

    /// Sets the value of BinaryModeEnabled
    pub fn set_binary_mode_enabled(&mut self, value: bool) {
        self.binary_mode_enabled = Some(value);
    }

    /// Gets the value of BinaryModeEnabled
    pub fn get_binary_mode_enabled(&self) -> Option<&bool> {
        self.binary_mode_enabled.as_ref()
    }

    /// Sets the value of BitsPerByte
    pub fn set_bits_per_byte(&mut self, value: u32) {
        self.bits_per_byte = Some(value);
    }

    /// Gets the value of BitsPerByte
    pub fn get_bits_per_byte(&self) -> Option<&u32> {
        self.bits_per_byte.as_ref()
    }

    /// Sets the value of ContinueXMitOnXOff
    pub fn set_continue_xmit_on_xoff(&mut self, value: bool) {
        self.continue_xmit_on_xoff = Some(value);
    }

    /// Gets the value of ContinueXMitOnXOff
    pub fn get_continue_xmit_on_xoff(&self) -> Option<&bool> {
        self.continue_xmit_on_xoff.as_ref()
    }

    /// Sets the value of CTSOutflowControl
    pub fn set_ctsoutflow_control(&mut self, value: bool) {
        self.ctsoutflow_control = Some(value);
    }

    /// Gets the value of CTSOutflowControl
    pub fn get_ctsoutflow_control(&self) -> Option<&bool> {
        self.ctsoutflow_control.as_ref()
    }

    /// Sets the value of DiscardNULLBytes
    pub fn set_discard_nullbytes(&mut self, value: bool) {
        self.discard_nullbytes = Some(value);
    }

    /// Gets the value of DiscardNULLBytes
    pub fn get_discard_nullbytes(&self) -> Option<&bool> {
        self.discard_nullbytes.as_ref()
    }

    /// Sets the value of DSROutflowControl
    pub fn set_dsroutflow_control(&mut self, value: bool) {
        self.dsroutflow_control = Some(value);
    }

    /// Gets the value of DSROutflowControl
    pub fn get_dsroutflow_control(&self) -> Option<&bool> {
        self.dsroutflow_control.as_ref()
    }

    /// Sets the value of DSRSensitivity
    pub fn set_dsrsensitivity(&mut self, value: bool) {
        self.dsrsensitivity = Some(value);
    }

    /// Gets the value of DSRSensitivity
    pub fn get_dsrsensitivity(&self) -> Option<&bool> {
        self.dsrsensitivity.as_ref()
    }

    /// Sets the value of DTRFlowControlType
    pub fn set_dtrflow_control_type(&mut self, value: String) {
        self.dtrflow_control_type = Some(value);
    }

    /// Gets the value of DTRFlowControlType
    pub fn get_dtrflow_control_type(&self) -> Option<&String> {
        self.dtrflow_control_type.as_ref()
    }

    /// Sets the value of EOFCharacter
    pub fn set_eofcharacter(&mut self, value: u32) {
        self.eofcharacter = Some(value);
    }

    /// Gets the value of EOFCharacter
    pub fn get_eofcharacter(&self) -> Option<&u32> {
        self.eofcharacter.as_ref()
    }

    /// Sets the value of ErrorReplaceCharacter
    pub fn set_error_replace_character(&mut self, value: u32) {
        self.error_replace_character = Some(value);
    }

    /// Gets the value of ErrorReplaceCharacter
    pub fn get_error_replace_character(&self) -> Option<&u32> {
        self.error_replace_character.as_ref()
    }

    /// Sets the value of ErrorReplacementEnabled
    pub fn set_error_replacement_enabled(&mut self, value: bool) {
        self.error_replacement_enabled = Some(value);
    }

    /// Gets the value of ErrorReplacementEnabled
    pub fn get_error_replacement_enabled(&self) -> Option<&bool> {
        self.error_replacement_enabled.as_ref()
    }

    /// Sets the value of EventCharacter
    pub fn set_event_character(&mut self, value: u32) {
        self.event_character = Some(value);
    }

    /// Gets the value of EventCharacter
    pub fn get_event_character(&self) -> Option<&u32> {
        self.event_character.as_ref()
    }

    /// Sets the value of IsBusy
    pub fn set_is_busy(&mut self, value: bool) {
        self.is_busy = Some(value);
    }

    /// Gets the value of IsBusy
    pub fn get_is_busy(&self) -> Option<&bool> {
        self.is_busy.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Parity
    pub fn set_parity(&mut self, value: String) {
        self.parity = Some(value);
    }

    /// Gets the value of Parity
    pub fn get_parity(&self) -> Option<&String> {
        self.parity.as_ref()
    }

    /// Sets the value of ParityCheckEnabled
    pub fn set_parity_check_enabled(&mut self, value: bool) {
        self.parity_check_enabled = Some(value);
    }

    /// Gets the value of ParityCheckEnabled
    pub fn get_parity_check_enabled(&self) -> Option<&bool> {
        self.parity_check_enabled.as_ref()
    }

    /// Sets the value of RTSFlowControlType
    pub fn set_rtsflow_control_type(&mut self, value: String) {
        self.rtsflow_control_type = Some(value);
    }

    /// Gets the value of RTSFlowControlType
    pub fn get_rtsflow_control_type(&self) -> Option<&String> {
        self.rtsflow_control_type.as_ref()
    }

    /// Sets the value of StopBits
    pub fn set_stop_bits(&mut self, value: String) {
        self.stop_bits = Some(value);
    }

    /// Gets the value of StopBits
    pub fn get_stop_bits(&self) -> Option<&String> {
        self.stop_bits.as_ref()
    }

    /// Sets the value of XOffCharacter
    pub fn set_xoff_character(&mut self, value: u32) {
        self.xoff_character = Some(value);
    }

    /// Gets the value of XOffCharacter
    pub fn get_xoff_character(&self) -> Option<&u32> {
        self.xoff_character.as_ref()
    }

    /// Sets the value of XOffXMitThreshold
    pub fn set_xoff_xmit_threshold(&mut self, value: u32) {
        self.xoff_xmit_threshold = Some(value);
    }

    /// Gets the value of XOffXMitThreshold
    pub fn get_xoff_xmit_threshold(&self) -> Option<&u32> {
        self.xoff_xmit_threshold.as_ref()
    }

    /// Sets the value of XOnCharacter
    pub fn set_xon_character(&mut self, value: u32) {
        self.xon_character = Some(value);
    }

    /// Gets the value of XOnCharacter
    pub fn get_xon_character(&self) -> Option<&u32> {
        self.xon_character.as_ref()
    }

    /// Sets the value of XOnXMitThreshold
    pub fn set_xon_xmit_threshold(&mut self, value: u32) {
        self.xon_xmit_threshold = Some(value);
    }

    /// Gets the value of XOnXMitThreshold
    pub fn get_xon_xmit_threshold(&self) -> Option<&u32> {
        self.xon_xmit_threshold.as_ref()
    }

    /// Sets the value of XOnXOffInFlowControl
    pub fn set_xon_xoff_in_flow_control(&mut self, value: u32) {
        self.xon_xoff_in_flow_control = Some(value);
    }

    /// Gets the value of XOnXOffInFlowControl
    pub fn get_xon_xoff_in_flow_control(&self) -> Option<&u32> {
        self.xon_xoff_in_flow_control.as_ref()
    }

    /// Sets the value of XOnXOffOutFlowControl
    pub fn set_xon_xoff_out_flow_control(&mut self, value: u32) {
        self.xon_xoff_out_flow_control = Some(value);
    }

    /// Gets the value of XOnXOffOutFlowControl
    pub fn get_xon_xoff_out_flow_control(&self) -> Option<&u32> {
        self.xon_xoff_out_flow_control.as_ref()
    }
}

