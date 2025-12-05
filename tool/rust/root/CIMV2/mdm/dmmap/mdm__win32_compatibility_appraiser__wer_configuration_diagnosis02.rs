// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Win32CompatibilityAppraiser_WerConfigurationDiagnosis02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Win32CompatibilityAppraiser_WerConfigurationDiagnosis02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MostRestrictiveSetting")]
    pub most_restrictive_setting: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "WerTelemetryOptIn")]
    pub wer_telemetry_opt_in: Option<i32>,
}

impl MDM_Win32CompatibilityAppraiser_WerConfigurationDiagnosis02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            most_restrictive_setting: None,
            parent_id: None,
            wer_telemetry_opt_in: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MostRestrictiveSetting
    pub fn set_most_restrictive_setting(&mut self, value: i32) {
        self.most_restrictive_setting = Some(value);
    }

    /// Gets the value of MostRestrictiveSetting
    pub fn get_most_restrictive_setting(&self) -> Option<&i32> {
        self.most_restrictive_setting.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of WerTelemetryOptIn
    pub fn set_wer_telemetry_opt_in(&mut self, value: i32) {
        self.wer_telemetry_opt_in = Some(value);
    }

    /// Gets the value of WerTelemetryOptIn
    pub fn get_wer_telemetry_opt_in(&self) -> Option<&i32> {
        self.wer_telemetry_opt_in.as_ref()
    }
}

