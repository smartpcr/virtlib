// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Autoplay02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Autoplay02 {

/// 
    #[serde(rename = "DisallowAutoplayForNonVolumeDevices")]
    pub disallow_autoplay_for_non_volume_devices: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SetDefaultAutoRunBehavior")]
    pub set_default_auto_run_behavior: Option<String>,

/// 
    #[serde(rename = "TurnOffAutoPlay")]
    pub turn_off_auto_play: Option<String>,
}

impl MDM_Policy_Config01_Autoplay02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disallow_autoplay_for_non_volume_devices: None,
            instance_id: None,
            parent_id: None,
            set_default_auto_run_behavior: None,
            turn_off_auto_play: None,
        }
    }


    /// Sets the value of DisallowAutoplayForNonVolumeDevices
    pub fn set_disallow_autoplay_for_non_volume_devices(&mut self, value: String) {
        self.disallow_autoplay_for_non_volume_devices = Some(value);
    }

    /// Gets the value of DisallowAutoplayForNonVolumeDevices
    pub fn get_disallow_autoplay_for_non_volume_devices(&self) -> Option<&String> {
        self.disallow_autoplay_for_non_volume_devices.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SetDefaultAutoRunBehavior
    pub fn set_set_default_auto_run_behavior(&mut self, value: String) {
        self.set_default_auto_run_behavior = Some(value);
    }

    /// Gets the value of SetDefaultAutoRunBehavior
    pub fn get_set_default_auto_run_behavior(&self) -> Option<&String> {
        self.set_default_auto_run_behavior.as_ref()
    }

    /// Sets the value of TurnOffAutoPlay
    pub fn set_turn_off_auto_play(&mut self, value: String) {
        self.turn_off_auto_play = Some(value);
    }

    /// Gets the value of TurnOffAutoPlay
    pub fn get_turn_off_auto_play(&self) -> Option<&String> {
        self.turn_off_auto_play.as_ref()
    }
}

