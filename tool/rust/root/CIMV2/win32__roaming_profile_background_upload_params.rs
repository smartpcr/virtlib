// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RoamingProfileBackgroundUploadParams struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RoamingProfileBackgroundUploadParams {

/// The time interval, in hours.
    #[serde(rename = "Interval")]
    pub interval: Option<u16>,

/// Indicates when a background upload should be performed. One of the following values can be specified. SpecificTime - Perform the background upload at the time of day specified in the Time property. SetInterval  - Perform the background upload at the interval specified in the Interval property.
    #[serde(rename = "SchedulingMethod")]
    pub scheduling_method: Option<RoamingProfileBackgroundUploadParams_SchedulingMethod>,

/// An integer value that represents the hour, in 24-hour time, for the time of day when they sync should occur. This must be an integer value from 0 to 23.
    #[serde(rename = "Time")]
    pub time: Option<u16>,
}

impl Win32_RoamingProfileBackgroundUploadParams {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            interval: None,
            scheduling_method: None,
            time: None,
        }
    }


    /// Sets the value of Interval
    pub fn set_interval(&mut self, value: u16) {
        self.interval = Some(value);
    }

    /// Gets the value of Interval
    pub fn get_interval(&self) -> Option<&u16> {
        self.interval.as_ref()
    }

    /// Sets the value of SchedulingMethod
    pub fn set_scheduling_method(&mut self, value: RoamingProfileBackgroundUploadParams_SchedulingMethod) {
        self.scheduling_method = Some(value);
    }

    /// Gets the value of SchedulingMethod
    pub fn get_scheduling_method(&self) -> Option<&RoamingProfileBackgroundUploadParams_SchedulingMethod> {
        self.scheduling_method.as_ref()
    }

    /// Sets the value of Time
    pub fn set_time(&mut self, value: u16) {
        self.time = Some(value);
    }

    /// Gets the value of Time
    pub fn get_time(&self) -> Option<&u16> {
        self.time.as_ref()
    }
}

