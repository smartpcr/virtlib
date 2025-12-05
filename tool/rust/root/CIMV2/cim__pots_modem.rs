// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PotsModem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PotsModem {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "AnswerMode")]
    pub answer_mode: Option<u16>,

/// 
    #[serde(rename = "CompressionInfo")]
    pub compression_info: Option<u16>,

/// 
    #[serde(rename = "CountriesSupported")]
    pub countries_supported: Vec<String>,

/// 
    #[serde(rename = "CountrySelected")]
    pub country_selected: Option<String>,

/// 
    #[serde(rename = "CurrentPasswords")]
    pub current_passwords: Vec<String>,

/// 
    #[serde(rename = "DialType")]
    pub dial_type: Option<u16>,

/// 
    #[serde(rename = "ErrorControlInfo")]
    pub error_control_info: Option<u16>,

/// 
    #[serde(rename = "InactivityTimeout")]
    pub inactivity_timeout: Option<u32>,

/// 
    #[serde(rename = "MaxBaudRateToPhone")]
    pub max_baud_rate_to_phone: Option<u32>,

/// 
    #[serde(rename = "MaxBaudRateToSerialPort")]
    pub max_baud_rate_to_serial_port: Option<u32>,

/// 
    #[serde(rename = "MaxNumberOfPasswords")]
    pub max_number_of_passwords: Option<u16>,

/// 
    #[serde(rename = "ModulationScheme")]
    pub modulation_scheme: Option<u16>,

/// 
    #[serde(rename = "RingsBeforeAnswer")]
    pub rings_before_answer: Option<u8>,

/// 
    #[serde(rename = "SpeakerVolumeInfo")]
    pub speaker_volume_info: Option<u16>,

/// 
    #[serde(rename = "SupportsCallback")]
    pub supports_callback: Option<bool>,

/// 
    #[serde(rename = "SupportsSynchronousConnect")]
    pub supports_synchronous_connect: Option<bool>,

/// 
    #[serde(rename = "TimeOfLastReset")]
    pub time_of_last_reset: Option<String>,
}

impl CIM_PotsModem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            answer_mode: None,
            compression_info: None,
            countries_supported: Vec::new(),
            country_selected: None,
            current_passwords: Vec::new(),
            dial_type: None,
            error_control_info: None,
            inactivity_timeout: None,
            max_baud_rate_to_phone: None,
            max_baud_rate_to_serial_port: None,
            max_number_of_passwords: None,
            modulation_scheme: None,
            rings_before_answer: None,
            speaker_volume_info: None,
            supports_callback: None,
            supports_synchronous_connect: None,
            time_of_last_reset: None,
        }
    }


    /// Sets the value of AnswerMode
    pub fn set_answer_mode(&mut self, value: u16) {
        self.answer_mode = Some(value);
    }

    /// Gets the value of AnswerMode
    pub fn get_answer_mode(&self) -> Option<&u16> {
        self.answer_mode.as_ref()
    }

    /// Sets the value of CompressionInfo
    pub fn set_compression_info(&mut self, value: u16) {
        self.compression_info = Some(value);
    }

    /// Gets the value of CompressionInfo
    pub fn get_compression_info(&self) -> Option<&u16> {
        self.compression_info.as_ref()
    }

    /// Sets the value of CountriesSupported
    pub fn set_countries_supported(&mut self, value: Vec<String>) {
        self.countries_supported = value;
    }

    /// Gets the value of CountriesSupported
    pub fn get_countries_supported(&self) -> &Vec<String> {
        &self.countries_supported
    }

    /// Sets the value of CountrySelected
    pub fn set_country_selected(&mut self, value: String) {
        self.country_selected = Some(value);
    }

    /// Gets the value of CountrySelected
    pub fn get_country_selected(&self) -> Option<&String> {
        self.country_selected.as_ref()
    }

    /// Sets the value of CurrentPasswords
    pub fn set_current_passwords(&mut self, value: Vec<String>) {
        self.current_passwords = value;
    }

    /// Gets the value of CurrentPasswords
    pub fn get_current_passwords(&self) -> &Vec<String> {
        &self.current_passwords
    }

    /// Sets the value of DialType
    pub fn set_dial_type(&mut self, value: u16) {
        self.dial_type = Some(value);
    }

    /// Gets the value of DialType
    pub fn get_dial_type(&self) -> Option<&u16> {
        self.dial_type.as_ref()
    }

    /// Sets the value of ErrorControlInfo
    pub fn set_error_control_info(&mut self, value: u16) {
        self.error_control_info = Some(value);
    }

    /// Gets the value of ErrorControlInfo
    pub fn get_error_control_info(&self) -> Option<&u16> {
        self.error_control_info.as_ref()
    }

    /// Sets the value of InactivityTimeout
    pub fn set_inactivity_timeout(&mut self, value: u32) {
        self.inactivity_timeout = Some(value);
    }

    /// Gets the value of InactivityTimeout
    pub fn get_inactivity_timeout(&self) -> Option<&u32> {
        self.inactivity_timeout.as_ref()
    }

    /// Sets the value of MaxBaudRateToPhone
    pub fn set_max_baud_rate_to_phone(&mut self, value: u32) {
        self.max_baud_rate_to_phone = Some(value);
    }

    /// Gets the value of MaxBaudRateToPhone
    pub fn get_max_baud_rate_to_phone(&self) -> Option<&u32> {
        self.max_baud_rate_to_phone.as_ref()
    }

    /// Sets the value of MaxBaudRateToSerialPort
    pub fn set_max_baud_rate_to_serial_port(&mut self, value: u32) {
        self.max_baud_rate_to_serial_port = Some(value);
    }

    /// Gets the value of MaxBaudRateToSerialPort
    pub fn get_max_baud_rate_to_serial_port(&self) -> Option<&u32> {
        self.max_baud_rate_to_serial_port.as_ref()
    }

    /// Sets the value of MaxNumberOfPasswords
    pub fn set_max_number_of_passwords(&mut self, value: u16) {
        self.max_number_of_passwords = Some(value);
    }

    /// Gets the value of MaxNumberOfPasswords
    pub fn get_max_number_of_passwords(&self) -> Option<&u16> {
        self.max_number_of_passwords.as_ref()
    }

    /// Sets the value of ModulationScheme
    pub fn set_modulation_scheme(&mut self, value: u16) {
        self.modulation_scheme = Some(value);
    }

    /// Gets the value of ModulationScheme
    pub fn get_modulation_scheme(&self) -> Option<&u16> {
        self.modulation_scheme.as_ref()
    }

    /// Sets the value of RingsBeforeAnswer
    pub fn set_rings_before_answer(&mut self, value: u8) {
        self.rings_before_answer = Some(value);
    }

    /// Gets the value of RingsBeforeAnswer
    pub fn get_rings_before_answer(&self) -> Option<&u8> {
        self.rings_before_answer.as_ref()
    }

    /// Sets the value of SpeakerVolumeInfo
    pub fn set_speaker_volume_info(&mut self, value: u16) {
        self.speaker_volume_info = Some(value);
    }

    /// Gets the value of SpeakerVolumeInfo
    pub fn get_speaker_volume_info(&self) -> Option<&u16> {
        self.speaker_volume_info.as_ref()
    }

    /// Sets the value of SupportsCallback
    pub fn set_supports_callback(&mut self, value: bool) {
        self.supports_callback = Some(value);
    }

    /// Gets the value of SupportsCallback
    pub fn get_supports_callback(&self) -> Option<&bool> {
        self.supports_callback.as_ref()
    }

    /// Sets the value of SupportsSynchronousConnect
    pub fn set_supports_synchronous_connect(&mut self, value: bool) {
        self.supports_synchronous_connect = Some(value);
    }

    /// Gets the value of SupportsSynchronousConnect
    pub fn get_supports_synchronous_connect(&self) -> Option<&bool> {
        self.supports_synchronous_connect.as_ref()
    }

    /// Sets the value of TimeOfLastReset
    pub fn set_time_of_last_reset(&mut self, value: String) {
        self.time_of_last_reset = Some(value);
    }

    /// Gets the value of TimeOfLastReset
    pub fn get_time_of_last_reset(&self) -> Option<&String> {
        self.time_of_last_reset.as_ref()
    }
}

