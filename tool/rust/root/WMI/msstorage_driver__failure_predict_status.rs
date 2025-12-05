// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_FailurePredictStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_FailurePredictStatus {
    #[serde(flatten)]
    pub base: MSStorageDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PredictFailure")]
    pub predict_failure: Option<bool>,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<u32>,
}

impl MSStorageDriver_FailurePredictStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSStorageDriver::new(),
            active: None,
            instance_name: None,
            predict_failure: None,
            reason: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PredictFailure
    pub fn set_predict_failure(&mut self, value: bool) {
        self.predict_failure = Some(value);
    }

    /// Gets the value of PredictFailure
    pub fn get_predict_failure(&self) -> Option<&bool> {
        self.predict_failure.as_ref()
    }

    /// Sets the value of Reason
    pub fn set_reason(&mut self, value: u32) {
        self.reason = Some(value);
    }

    /// Gets the value of Reason
    pub fn get_reason(&self) -> Option<&u32> {
        self.reason.as_ref()
    }
}

