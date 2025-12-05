// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_RemoteFind_Location01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_RemoteFind_Location01 {

/// 
    #[serde(rename = "Accuracy")]
    pub accuracy: Option<i32>,

/// 
    #[serde(rename = "Age")]
    pub age: Option<String>,

/// 
    #[serde(rename = "Altitude")]
    pub altitude: Option<f32>,

/// 
    #[serde(rename = "AltitudeAccuracy")]
    pub altitude_accuracy: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Latitude")]
    pub latitude: Option<f32>,

/// 
    #[serde(rename = "Longitude")]
    pub longitude: Option<f32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_RemoteFind_Location01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            accuracy: None,
            age: None,
            altitude: None,
            altitude_accuracy: None,
            instance_id: None,
            latitude: None,
            longitude: None,
            parent_id: None,
        }
    }


    /// Sets the value of Accuracy
    pub fn set_accuracy(&mut self, value: i32) {
        self.accuracy = Some(value);
    }

    /// Gets the value of Accuracy
    pub fn get_accuracy(&self) -> Option<&i32> {
        self.accuracy.as_ref()
    }

    /// Sets the value of Age
    pub fn set_age(&mut self, value: String) {
        self.age = Some(value);
    }

    /// Gets the value of Age
    pub fn get_age(&self) -> Option<&String> {
        self.age.as_ref()
    }

    /// Sets the value of Altitude
    pub fn set_altitude(&mut self, value: f32) {
        self.altitude = Some(value);
    }

    /// Gets the value of Altitude
    pub fn get_altitude(&self) -> Option<&f32> {
        self.altitude.as_ref()
    }

    /// Sets the value of AltitudeAccuracy
    pub fn set_altitude_accuracy(&mut self, value: i32) {
        self.altitude_accuracy = Some(value);
    }

    /// Gets the value of AltitudeAccuracy
    pub fn get_altitude_accuracy(&self) -> Option<&i32> {
        self.altitude_accuracy.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of Latitude
    pub fn set_latitude(&mut self, value: f32) {
        self.latitude = Some(value);
    }

    /// Gets the value of Latitude
    pub fn get_latitude(&self) -> Option<&f32> {
        self.latitude.as_ref()
    }

    /// Sets the value of Longitude
    pub fn set_longitude(&mut self, value: f32) {
        self.longitude = Some(value);
    }

    /// Gets the value of Longitude
    pub fn get_longitude(&self) -> Option<&f32> {
        self.longitude.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

