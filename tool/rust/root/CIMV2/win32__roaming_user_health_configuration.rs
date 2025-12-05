// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RoamingUserHealthConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RoamingUserHealthConfiguration {

/// Configure how the Win32_UserProfile::HealthStatus property should reflect the use of temporary profiles.
    #[serde(rename = "HealthStatusForTempProfiles")]
    pub health_status_for_temp_profiles: Option<RoamingUserHealthConfiguration_HealthStatusForTempProfiles>,

/// This is the time threshold, in hours, after which the profile health is reported as Caution when the profile has not been downloaded yet
    #[serde(rename = "LastProfileDownloadIntervalCautionInHours")]
    pub last_profile_download_interval_caution_in_hours: Option<u16>,

/// This is the time threshold, in hours, after which the profile health is reported as Unhealthy when the profile has not been uploaded yet
    #[serde(rename = "LastProfileDownloadIntervalUnhealthyInHours")]
    pub last_profile_download_interval_unhealthy_in_hours: Option<u16>,

/// This is the time threshold, in hours, after which the profile health is reported as Caution when the profile has not been uploaded yet
    #[serde(rename = "LastProfileUploadIntervalCautionInHours")]
    pub last_profile_upload_interval_caution_in_hours: Option<u16>,

/// This is the time threshold, in hours, after which the profile health is reported as Unhealthy when the profile has not been downloaded yet
    #[serde(rename = "LastProfileUploadIntervalUnhealthyInHours")]
    pub last_profile_upload_interval_unhealthy_in_hours: Option<u16>,
}

impl Win32_RoamingUserHealthConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            health_status_for_temp_profiles: None,
            last_profile_download_interval_caution_in_hours: None,
            last_profile_download_interval_unhealthy_in_hours: None,
            last_profile_upload_interval_caution_in_hours: None,
            last_profile_upload_interval_unhealthy_in_hours: None,
        }
    }


    /// Sets the value of HealthStatusForTempProfiles
    pub fn set_health_status_for_temp_profiles(&mut self, value: RoamingUserHealthConfiguration_HealthStatusForTempProfiles) {
        self.health_status_for_temp_profiles = Some(value);
    }

    /// Gets the value of HealthStatusForTempProfiles
    pub fn get_health_status_for_temp_profiles(&self) -> Option<&RoamingUserHealthConfiguration_HealthStatusForTempProfiles> {
        self.health_status_for_temp_profiles.as_ref()
    }

    /// Sets the value of LastProfileDownloadIntervalCautionInHours
    pub fn set_last_profile_download_interval_caution_in_hours(&mut self, value: u16) {
        self.last_profile_download_interval_caution_in_hours = Some(value);
    }

    /// Gets the value of LastProfileDownloadIntervalCautionInHours
    pub fn get_last_profile_download_interval_caution_in_hours(&self) -> Option<&u16> {
        self.last_profile_download_interval_caution_in_hours.as_ref()
    }

    /// Sets the value of LastProfileDownloadIntervalUnhealthyInHours
    pub fn set_last_profile_download_interval_unhealthy_in_hours(&mut self, value: u16) {
        self.last_profile_download_interval_unhealthy_in_hours = Some(value);
    }

    /// Gets the value of LastProfileDownloadIntervalUnhealthyInHours
    pub fn get_last_profile_download_interval_unhealthy_in_hours(&self) -> Option<&u16> {
        self.last_profile_download_interval_unhealthy_in_hours.as_ref()
    }

    /// Sets the value of LastProfileUploadIntervalCautionInHours
    pub fn set_last_profile_upload_interval_caution_in_hours(&mut self, value: u16) {
        self.last_profile_upload_interval_caution_in_hours = Some(value);
    }

    /// Gets the value of LastProfileUploadIntervalCautionInHours
    pub fn get_last_profile_upload_interval_caution_in_hours(&self) -> Option<&u16> {
        self.last_profile_upload_interval_caution_in_hours.as_ref()
    }

    /// Sets the value of LastProfileUploadIntervalUnhealthyInHours
    pub fn set_last_profile_upload_interval_unhealthy_in_hours(&mut self, value: u16) {
        self.last_profile_upload_interval_unhealthy_in_hours = Some(value);
    }

    /// Gets the value of LastProfileUploadIntervalUnhealthyInHours
    pub fn get_last_profile_upload_interval_unhealthy_in_hours(&self) -> Option<&u16> {
        self.last_profile_upload_interval_unhealthy_in_hours.as_ref()
    }
}

