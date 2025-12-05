// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_EventClusterCallback struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_EventClusterCallback {
    #[serde(flatten)]
    pub base: MSCluster_Event,

/// 
    #[serde(rename = "ObjectName")]
    pub object_name: Option<String>,

/// 
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<i32>,

/// 
    #[serde(rename = "PhaseSeverity")]
    pub phase_severity: Option<i32>,

/// 
    #[serde(rename = "PhaseType")]
    pub phase_type: Option<i32>,

/// 
    #[serde(rename = "SetupPhase")]
    pub setup_phase: Option<i32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,
}

impl MSCluster_EventClusterCallback {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_Event::new(),
            object_name: None,
            percent_complete: None,
            phase_severity: None,
            phase_type: None,
            setup_phase: None,
            status: None,
        }
    }


    /// Sets the value of ObjectName
    pub fn set_object_name(&mut self, value: String) {
        self.object_name = Some(value);
    }

    /// Gets the value of ObjectName
    pub fn get_object_name(&self) -> Option<&String> {
        self.object_name.as_ref()
    }

    /// Sets the value of PercentComplete
    pub fn set_percent_complete(&mut self, value: i32) {
        self.percent_complete = Some(value);
    }

    /// Gets the value of PercentComplete
    pub fn get_percent_complete(&self) -> Option<&i32> {
        self.percent_complete.as_ref()
    }

    /// Sets the value of PhaseSeverity
    pub fn set_phase_severity(&mut self, value: i32) {
        self.phase_severity = Some(value);
    }

    /// Gets the value of PhaseSeverity
    pub fn get_phase_severity(&self) -> Option<&i32> {
        self.phase_severity.as_ref()
    }

    /// Sets the value of PhaseType
    pub fn set_phase_type(&mut self, value: i32) {
        self.phase_type = Some(value);
    }

    /// Gets the value of PhaseType
    pub fn get_phase_type(&self) -> Option<&i32> {
        self.phase_type.as_ref()
    }

    /// Sets the value of SetupPhase
    pub fn set_setup_phase(&mut self, value: i32) {
        self.setup_phase = Some(value);
    }

    /// Gets the value of SetupPhase
    pub fn get_setup_phase(&self) -> Option<&i32> {
        self.setup_phase.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }
}

