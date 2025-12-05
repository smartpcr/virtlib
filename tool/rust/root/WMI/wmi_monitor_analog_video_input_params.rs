// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorAnalogVideoInputParams struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorAnalogVideoInputParams {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "CompositeSyncSupported")]
    pub composite_sync_supported: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "SeparateSyncsSupported")]
    pub separate_syncs_supported: Option<u8>,

/// 
    #[serde(rename = "SerrationOfVsyncRequired")]
    pub serration_of_vsync_required: Option<u8>,

/// 
    #[serde(rename = "SetupExpected")]
    pub setup_expected: Option<u8>,

/// 
    #[serde(rename = "SignalLevelStandard")]
    pub signal_level_standard: Option<u8>,

/// 
    #[serde(rename = "SyncOnGreenVideoSupported")]
    pub sync_on_green_video_supported: Option<u8>,
}

impl WmiMonitorAnalogVideoInputParams {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            composite_sync_supported: None,
            instance_name: None,
            separate_syncs_supported: None,
            serration_of_vsync_required: None,
            setup_expected: None,
            signal_level_standard: None,
            sync_on_green_video_supported: None,
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

    /// Sets the value of CompositeSyncSupported
    pub fn set_composite_sync_supported(&mut self, value: u8) {
        self.composite_sync_supported = Some(value);
    }

    /// Gets the value of CompositeSyncSupported
    pub fn get_composite_sync_supported(&self) -> Option<&u8> {
        self.composite_sync_supported.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of SeparateSyncsSupported
    pub fn set_separate_syncs_supported(&mut self, value: u8) {
        self.separate_syncs_supported = Some(value);
    }

    /// Gets the value of SeparateSyncsSupported
    pub fn get_separate_syncs_supported(&self) -> Option<&u8> {
        self.separate_syncs_supported.as_ref()
    }

    /// Sets the value of SerrationOfVsyncRequired
    pub fn set_serration_of_vsync_required(&mut self, value: u8) {
        self.serration_of_vsync_required = Some(value);
    }

    /// Gets the value of SerrationOfVsyncRequired
    pub fn get_serration_of_vsync_required(&self) -> Option<&u8> {
        self.serration_of_vsync_required.as_ref()
    }

    /// Sets the value of SetupExpected
    pub fn set_setup_expected(&mut self, value: u8) {
        self.setup_expected = Some(value);
    }

    /// Gets the value of SetupExpected
    pub fn get_setup_expected(&self) -> Option<&u8> {
        self.setup_expected.as_ref()
    }

    /// Sets the value of SignalLevelStandard
    pub fn set_signal_level_standard(&mut self, value: u8) {
        self.signal_level_standard = Some(value);
    }

    /// Gets the value of SignalLevelStandard
    pub fn get_signal_level_standard(&self) -> Option<&u8> {
        self.signal_level_standard.as_ref()
    }

    /// Sets the value of SyncOnGreenVideoSupported
    pub fn set_sync_on_green_video_supported(&mut self, value: u8) {
        self.sync_on_green_video_supported = Some(value);
    }

    /// Gets the value of SyncOnGreenVideoSupported
    pub fn get_sync_on_green_video_supported(&self) -> Option<&u8> {
        self.sync_on_green_video_supported.as_ref()
    }
}

