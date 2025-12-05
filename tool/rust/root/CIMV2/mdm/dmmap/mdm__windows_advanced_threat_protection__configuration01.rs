// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WindowsAdvancedThreatProtection_Configuration01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WindowsAdvancedThreatProtection_Configuration01 {

/// 
    #[serde(rename = "AadDeviceId")]
    pub aad_device_id: Option<String>,

/// 
    #[serde(rename = "GroupIds")]
    pub group_ids: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SampleSharing")]
    pub sample_sharing: Option<i32>,

/// 
    #[serde(rename = "TelemetryReportingFrequency")]
    pub telemetry_reporting_frequency: Option<i32>,
}

impl MDM_WindowsAdvancedThreatProtection_Configuration01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            aad_device_id: None,
            group_ids: None,
            instance_id: None,
            parent_id: None,
            sample_sharing: None,
            telemetry_reporting_frequency: None,
        }
    }


    /// Sets the value of AadDeviceId
    pub fn set_aad_device_id(&mut self, value: String) {
        self.aad_device_id = Some(value);
    }

    /// Gets the value of AadDeviceId
    pub fn get_aad_device_id(&self) -> Option<&String> {
        self.aad_device_id.as_ref()
    }

    /// Sets the value of GroupIds
    pub fn set_group_ids(&mut self, value: String) {
        self.group_ids = Some(value);
    }

    /// Gets the value of GroupIds
    pub fn get_group_ids(&self) -> Option<&String> {
        self.group_ids.as_ref()
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

    /// Sets the value of SampleSharing
    pub fn set_sample_sharing(&mut self, value: i32) {
        self.sample_sharing = Some(value);
    }

    /// Gets the value of SampleSharing
    pub fn get_sample_sharing(&self) -> Option<&i32> {
        self.sample_sharing.as_ref()
    }

    /// Sets the value of TelemetryReportingFrequency
    pub fn set_telemetry_reporting_frequency(&mut self, value: i32) {
        self.telemetry_reporting_frequency = Some(value);
    }

    /// Gets the value of TelemetryReportingFrequency
    pub fn get_telemetry_reporting_frequency(&self) -> Option<&i32> {
        self.telemetry_reporting_frequency.as_ref()
    }
}

