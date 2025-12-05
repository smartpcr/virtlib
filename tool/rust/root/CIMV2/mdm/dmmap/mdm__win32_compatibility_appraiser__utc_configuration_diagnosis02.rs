// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Win32CompatibilityAppraiser_UtcConfigurationDiagnosis02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Win32CompatibilityAppraiser_UtcConfigurationDiagnosis02 {

/// 
    #[serde(rename = "CommercialDataOptIn")]
    pub commercial_data_opt_in: Option<i32>,

/// 
    #[serde(rename = "DiagTrackServiceRunning")]
    pub diag_track_service_running: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "InternetExplorerTelemetryOptIn")]
    pub internet_explorer_telemetry_opt_in: Option<i32>,

/// 
    #[serde(rename = "MsaServiceEnabled")]
    pub msa_service_enabled: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TelemetryOptIn")]
    pub telemetry_opt_in: Option<i32>,
}

impl MDM_Win32CompatibilityAppraiser_UtcConfigurationDiagnosis02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            commercial_data_opt_in: None,
            diag_track_service_running: None,
            instance_id: None,
            internet_explorer_telemetry_opt_in: None,
            msa_service_enabled: None,
            parent_id: None,
            telemetry_opt_in: None,
        }
    }


    /// Sets the value of CommercialDataOptIn
    pub fn set_commercial_data_opt_in(&mut self, value: i32) {
        self.commercial_data_opt_in = Some(value);
    }

    /// Gets the value of CommercialDataOptIn
    pub fn get_commercial_data_opt_in(&self) -> Option<&i32> {
        self.commercial_data_opt_in.as_ref()
    }

    /// Sets the value of DiagTrackServiceRunning
    pub fn set_diag_track_service_running(&mut self, value: bool) {
        self.diag_track_service_running = Some(value);
    }

    /// Gets the value of DiagTrackServiceRunning
    pub fn get_diag_track_service_running(&self) -> Option<&bool> {
        self.diag_track_service_running.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of InternetExplorerTelemetryOptIn
    pub fn set_internet_explorer_telemetry_opt_in(&mut self, value: i32) {
        self.internet_explorer_telemetry_opt_in = Some(value);
    }

    /// Gets the value of InternetExplorerTelemetryOptIn
    pub fn get_internet_explorer_telemetry_opt_in(&self) -> Option<&i32> {
        self.internet_explorer_telemetry_opt_in.as_ref()
    }

    /// Sets the value of MsaServiceEnabled
    pub fn set_msa_service_enabled(&mut self, value: bool) {
        self.msa_service_enabled = Some(value);
    }

    /// Gets the value of MsaServiceEnabled
    pub fn get_msa_service_enabled(&self) -> Option<&bool> {
        self.msa_service_enabled.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TelemetryOptIn
    pub fn set_telemetry_opt_in(&mut self, value: i32) {
        self.telemetry_opt_in = Some(value);
    }

    /// Gets the value of TelemetryOptIn
    pub fn get_telemetry_opt_in(&self) -> Option<&i32> {
        self.telemetry_opt_in.as_ref()
    }
}

