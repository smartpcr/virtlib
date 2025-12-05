// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_POTSModem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_POTSModem {
    #[serde(flatten)]
    pub base: CIM_PotsModem,

/// 
    #[serde(rename = "AttachedTo")]
    pub attached_to: Option<String>,

/// 
    #[serde(rename = "BlindOff")]
    pub blind_off: Option<String>,

/// 
    #[serde(rename = "BlindOn")]
    pub blind_on: Option<String>,

/// 
    #[serde(rename = "CompatibilityFlags")]
    pub compatibility_flags: Option<String>,

/// 
    #[serde(rename = "CompressionOff")]
    pub compression_off: Option<String>,

/// 
    #[serde(rename = "CompressionOn")]
    pub compression_on: Option<String>,

/// 
    #[serde(rename = "ConfigurationDialog")]
    pub configuration_dialog: Option<String>,

/// 
    #[serde(rename = "DCB")]
    pub dcb: Vec<u8>,

/// 
    #[serde(rename = "Default")]
    pub default: Vec<u8>,

/// 
    #[serde(rename = "DeviceLoader")]
    pub device_loader: Option<String>,

/// 
    #[serde(rename = "DeviceType")]
    pub device_type: Option<String>,

/// 
    #[serde(rename = "DriverDate")]
    pub driver_date: Option<String>,

/// 
    #[serde(rename = "ErrorControlForced")]
    pub error_control_forced: Option<String>,

/// 
    #[serde(rename = "ErrorControlOff")]
    pub error_control_off: Option<String>,

/// 
    #[serde(rename = "ErrorControlOn")]
    pub error_control_on: Option<String>,

/// 
    #[serde(rename = "FlowControlHard")]
    pub flow_control_hard: Option<String>,

/// 
    #[serde(rename = "FlowControlOff")]
    pub flow_control_off: Option<String>,

/// 
    #[serde(rename = "FlowControlSoft")]
    pub flow_control_soft: Option<String>,

/// 
    #[serde(rename = "InactivityScale")]
    pub inactivity_scale: Option<String>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "IndexEx")]
    pub index_ex: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "ModemInfPath")]
    pub modem_inf_path: Option<String>,

/// 
    #[serde(rename = "ModemInfSection")]
    pub modem_inf_section: Option<String>,

/// 
    #[serde(rename = "ModulationBell")]
    pub modulation_bell: Option<String>,

/// 
    #[serde(rename = "ModulationCCITT")]
    pub modulation_ccitt: Option<String>,

/// 
    #[serde(rename = "PortSubClass")]
    pub port_sub_class: Option<String>,

/// 
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

/// 
    #[serde(rename = "Properties")]
    pub properties: Vec<u8>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,

/// 
    #[serde(rename = "Pulse")]
    pub pulse: Option<String>,

/// 
    #[serde(rename = "Reset")]
    pub reset: Option<String>,

/// 
    #[serde(rename = "ResponsesKeyName")]
    pub responses_key_name: Option<String>,

/// 
    #[serde(rename = "SpeakerModeDial")]
    pub speaker_mode_dial: Option<String>,

/// 
    #[serde(rename = "SpeakerModeOff")]
    pub speaker_mode_off: Option<String>,

/// 
    #[serde(rename = "SpeakerModeOn")]
    pub speaker_mode_on: Option<String>,

/// 
    #[serde(rename = "SpeakerModeSetup")]
    pub speaker_mode_setup: Option<String>,

/// 
    #[serde(rename = "SpeakerVolumeHigh")]
    pub speaker_volume_high: Option<String>,

/// 
    #[serde(rename = "SpeakerVolumeLow")]
    pub speaker_volume_low: Option<String>,

/// 
    #[serde(rename = "SpeakerVolumeMed")]
    pub speaker_volume_med: Option<String>,

/// 
    #[serde(rename = "StringFormat")]
    pub string_format: Option<String>,

/// 
    #[serde(rename = "Terminator")]
    pub terminator: Option<String>,

/// 
    #[serde(rename = "Tone")]
    pub tone: Option<String>,

/// 
    #[serde(rename = "VoiceSwitchFeature")]
    pub voice_switch_feature: Option<String>,
}

impl Win32_POTSModem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PotsModem::new(),
            attached_to: None,
            blind_off: None,
            blind_on: None,
            compatibility_flags: None,
            compression_off: None,
            compression_on: None,
            configuration_dialog: None,
            dcb: Vec::new(),
            default: Vec::new(),
            device_loader: None,
            device_type: None,
            driver_date: None,
            error_control_forced: None,
            error_control_off: None,
            error_control_on: None,
            flow_control_hard: None,
            flow_control_off: None,
            flow_control_soft: None,
            inactivity_scale: None,
            index: None,
            index_ex: None,
            model: None,
            modem_inf_path: None,
            modem_inf_section: None,
            modulation_bell: None,
            modulation_ccitt: None,
            port_sub_class: None,
            prefix: None,
            properties: Vec::new(),
            provider_name: None,
            pulse: None,
            reset: None,
            responses_key_name: None,
            speaker_mode_dial: None,
            speaker_mode_off: None,
            speaker_mode_on: None,
            speaker_mode_setup: None,
            speaker_volume_high: None,
            speaker_volume_low: None,
            speaker_volume_med: None,
            string_format: None,
            terminator: None,
            tone: None,
            voice_switch_feature: None,
        }
    }


    /// Sets the value of AttachedTo
    pub fn set_attached_to(&mut self, value: String) {
        self.attached_to = Some(value);
    }

    /// Gets the value of AttachedTo
    pub fn get_attached_to(&self) -> Option<&String> {
        self.attached_to.as_ref()
    }

    /// Sets the value of BlindOff
    pub fn set_blind_off(&mut self, value: String) {
        self.blind_off = Some(value);
    }

    /// Gets the value of BlindOff
    pub fn get_blind_off(&self) -> Option<&String> {
        self.blind_off.as_ref()
    }

    /// Sets the value of BlindOn
    pub fn set_blind_on(&mut self, value: String) {
        self.blind_on = Some(value);
    }

    /// Gets the value of BlindOn
    pub fn get_blind_on(&self) -> Option<&String> {
        self.blind_on.as_ref()
    }

    /// Sets the value of CompatibilityFlags
    pub fn set_compatibility_flags(&mut self, value: String) {
        self.compatibility_flags = Some(value);
    }

    /// Gets the value of CompatibilityFlags
    pub fn get_compatibility_flags(&self) -> Option<&String> {
        self.compatibility_flags.as_ref()
    }

    /// Sets the value of CompressionOff
    pub fn set_compression_off(&mut self, value: String) {
        self.compression_off = Some(value);
    }

    /// Gets the value of CompressionOff
    pub fn get_compression_off(&self) -> Option<&String> {
        self.compression_off.as_ref()
    }

    /// Sets the value of CompressionOn
    pub fn set_compression_on(&mut self, value: String) {
        self.compression_on = Some(value);
    }

    /// Gets the value of CompressionOn
    pub fn get_compression_on(&self) -> Option<&String> {
        self.compression_on.as_ref()
    }

    /// Sets the value of ConfigurationDialog
    pub fn set_configuration_dialog(&mut self, value: String) {
        self.configuration_dialog = Some(value);
    }

    /// Gets the value of ConfigurationDialog
    pub fn get_configuration_dialog(&self) -> Option<&String> {
        self.configuration_dialog.as_ref()
    }

    /// Sets the value of DCB
    pub fn set_dcb(&mut self, value: Vec<u8>) {
        self.dcb = value;
    }

    /// Gets the value of DCB
    pub fn get_dcb(&self) -> &Vec<u8> {
        &self.dcb
    }

    /// Sets the value of Default
    pub fn set_default(&mut self, value: Vec<u8>) {
        self.default = value;
    }

    /// Gets the value of Default
    pub fn get_default(&self) -> &Vec<u8> {
        &self.default
    }

    /// Sets the value of DeviceLoader
    pub fn set_device_loader(&mut self, value: String) {
        self.device_loader = Some(value);
    }

    /// Gets the value of DeviceLoader
    pub fn get_device_loader(&self) -> Option<&String> {
        self.device_loader.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: String) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&String> {
        self.device_type.as_ref()
    }

    /// Sets the value of DriverDate
    pub fn set_driver_date(&mut self, value: String) {
        self.driver_date = Some(value);
    }

    /// Gets the value of DriverDate
    pub fn get_driver_date(&self) -> Option<&String> {
        self.driver_date.as_ref()
    }

    /// Sets the value of ErrorControlForced
    pub fn set_error_control_forced(&mut self, value: String) {
        self.error_control_forced = Some(value);
    }

    /// Gets the value of ErrorControlForced
    pub fn get_error_control_forced(&self) -> Option<&String> {
        self.error_control_forced.as_ref()
    }

    /// Sets the value of ErrorControlOff
    pub fn set_error_control_off(&mut self, value: String) {
        self.error_control_off = Some(value);
    }

    /// Gets the value of ErrorControlOff
    pub fn get_error_control_off(&self) -> Option<&String> {
        self.error_control_off.as_ref()
    }

    /// Sets the value of ErrorControlOn
    pub fn set_error_control_on(&mut self, value: String) {
        self.error_control_on = Some(value);
    }

    /// Gets the value of ErrorControlOn
    pub fn get_error_control_on(&self) -> Option<&String> {
        self.error_control_on.as_ref()
    }

    /// Sets the value of FlowControlHard
    pub fn set_flow_control_hard(&mut self, value: String) {
        self.flow_control_hard = Some(value);
    }

    /// Gets the value of FlowControlHard
    pub fn get_flow_control_hard(&self) -> Option<&String> {
        self.flow_control_hard.as_ref()
    }

    /// Sets the value of FlowControlOff
    pub fn set_flow_control_off(&mut self, value: String) {
        self.flow_control_off = Some(value);
    }

    /// Gets the value of FlowControlOff
    pub fn get_flow_control_off(&self) -> Option<&String> {
        self.flow_control_off.as_ref()
    }

    /// Sets the value of FlowControlSoft
    pub fn set_flow_control_soft(&mut self, value: String) {
        self.flow_control_soft = Some(value);
    }

    /// Gets the value of FlowControlSoft
    pub fn get_flow_control_soft(&self) -> Option<&String> {
        self.flow_control_soft.as_ref()
    }

    /// Sets the value of InactivityScale
    pub fn set_inactivity_scale(&mut self, value: String) {
        self.inactivity_scale = Some(value);
    }

    /// Gets the value of InactivityScale
    pub fn get_inactivity_scale(&self) -> Option<&String> {
        self.inactivity_scale.as_ref()
    }

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of IndexEx
    pub fn set_index_ex(&mut self, value: String) {
        self.index_ex = Some(value);
    }

    /// Gets the value of IndexEx
    pub fn get_index_ex(&self) -> Option<&String> {
        self.index_ex.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of ModemInfPath
    pub fn set_modem_inf_path(&mut self, value: String) {
        self.modem_inf_path = Some(value);
    }

    /// Gets the value of ModemInfPath
    pub fn get_modem_inf_path(&self) -> Option<&String> {
        self.modem_inf_path.as_ref()
    }

    /// Sets the value of ModemInfSection
    pub fn set_modem_inf_section(&mut self, value: String) {
        self.modem_inf_section = Some(value);
    }

    /// Gets the value of ModemInfSection
    pub fn get_modem_inf_section(&self) -> Option<&String> {
        self.modem_inf_section.as_ref()
    }

    /// Sets the value of ModulationBell
    pub fn set_modulation_bell(&mut self, value: String) {
        self.modulation_bell = Some(value);
    }

    /// Gets the value of ModulationBell
    pub fn get_modulation_bell(&self) -> Option<&String> {
        self.modulation_bell.as_ref()
    }

    /// Sets the value of ModulationCCITT
    pub fn set_modulation_ccitt(&mut self, value: String) {
        self.modulation_ccitt = Some(value);
    }

    /// Gets the value of ModulationCCITT
    pub fn get_modulation_ccitt(&self) -> Option<&String> {
        self.modulation_ccitt.as_ref()
    }

    /// Sets the value of PortSubClass
    pub fn set_port_sub_class(&mut self, value: String) {
        self.port_sub_class = Some(value);
    }

    /// Gets the value of PortSubClass
    pub fn get_port_sub_class(&self) -> Option<&String> {
        self.port_sub_class.as_ref()
    }

    /// Sets the value of Prefix
    pub fn set_prefix(&mut self, value: String) {
        self.prefix = Some(value);
    }

    /// Gets the value of Prefix
    pub fn get_prefix(&self) -> Option<&String> {
        self.prefix.as_ref()
    }

    /// Sets the value of Properties
    pub fn set_properties(&mut self, value: Vec<u8>) {
        self.properties = value;
    }

    /// Gets the value of Properties
    pub fn get_properties(&self) -> &Vec<u8> {
        &self.properties
    }

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }

    /// Sets the value of Pulse
    pub fn set_pulse(&mut self, value: String) {
        self.pulse = Some(value);
    }

    /// Gets the value of Pulse
    pub fn get_pulse(&self) -> Option<&String> {
        self.pulse.as_ref()
    }

    /// Sets the value of Reset
    pub fn set_reset(&mut self, value: String) {
        self.reset = Some(value);
    }

    /// Gets the value of Reset
    pub fn get_reset(&self) -> Option<&String> {
        self.reset.as_ref()
    }

    /// Sets the value of ResponsesKeyName
    pub fn set_responses_key_name(&mut self, value: String) {
        self.responses_key_name = Some(value);
    }

    /// Gets the value of ResponsesKeyName
    pub fn get_responses_key_name(&self) -> Option<&String> {
        self.responses_key_name.as_ref()
    }

    /// Sets the value of SpeakerModeDial
    pub fn set_speaker_mode_dial(&mut self, value: String) {
        self.speaker_mode_dial = Some(value);
    }

    /// Gets the value of SpeakerModeDial
    pub fn get_speaker_mode_dial(&self) -> Option<&String> {
        self.speaker_mode_dial.as_ref()
    }

    /// Sets the value of SpeakerModeOff
    pub fn set_speaker_mode_off(&mut self, value: String) {
        self.speaker_mode_off = Some(value);
    }

    /// Gets the value of SpeakerModeOff
    pub fn get_speaker_mode_off(&self) -> Option<&String> {
        self.speaker_mode_off.as_ref()
    }

    /// Sets the value of SpeakerModeOn
    pub fn set_speaker_mode_on(&mut self, value: String) {
        self.speaker_mode_on = Some(value);
    }

    /// Gets the value of SpeakerModeOn
    pub fn get_speaker_mode_on(&self) -> Option<&String> {
        self.speaker_mode_on.as_ref()
    }

    /// Sets the value of SpeakerModeSetup
    pub fn set_speaker_mode_setup(&mut self, value: String) {
        self.speaker_mode_setup = Some(value);
    }

    /// Gets the value of SpeakerModeSetup
    pub fn get_speaker_mode_setup(&self) -> Option<&String> {
        self.speaker_mode_setup.as_ref()
    }

    /// Sets the value of SpeakerVolumeHigh
    pub fn set_speaker_volume_high(&mut self, value: String) {
        self.speaker_volume_high = Some(value);
    }

    /// Gets the value of SpeakerVolumeHigh
    pub fn get_speaker_volume_high(&self) -> Option<&String> {
        self.speaker_volume_high.as_ref()
    }

    /// Sets the value of SpeakerVolumeLow
    pub fn set_speaker_volume_low(&mut self, value: String) {
        self.speaker_volume_low = Some(value);
    }

    /// Gets the value of SpeakerVolumeLow
    pub fn get_speaker_volume_low(&self) -> Option<&String> {
        self.speaker_volume_low.as_ref()
    }

    /// Sets the value of SpeakerVolumeMed
    pub fn set_speaker_volume_med(&mut self, value: String) {
        self.speaker_volume_med = Some(value);
    }

    /// Gets the value of SpeakerVolumeMed
    pub fn get_speaker_volume_med(&self) -> Option<&String> {
        self.speaker_volume_med.as_ref()
    }

    /// Sets the value of StringFormat
    pub fn set_string_format(&mut self, value: String) {
        self.string_format = Some(value);
    }

    /// Gets the value of StringFormat
    pub fn get_string_format(&self) -> Option<&String> {
        self.string_format.as_ref()
    }

    /// Sets the value of Terminator
    pub fn set_terminator(&mut self, value: String) {
        self.terminator = Some(value);
    }

    /// Gets the value of Terminator
    pub fn get_terminator(&self) -> Option<&String> {
        self.terminator.as_ref()
    }

    /// Sets the value of Tone
    pub fn set_tone(&mut self, value: String) {
        self.tone = Some(value);
    }

    /// Gets the value of Tone
    pub fn get_tone(&self) -> Option<&String> {
        self.tone.as_ref()
    }

    /// Sets the value of VoiceSwitchFeature
    pub fn set_voice_switch_feature(&mut self, value: String) {
        self.voice_switch_feature = Some(value);
    }

    /// Gets the value of VoiceSwitchFeature
    pub fn get_voice_switch_feature(&self) -> Option<&String> {
        self.voice_switch_feature.as_ref()
    }
}

