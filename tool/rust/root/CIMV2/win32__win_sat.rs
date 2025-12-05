// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_WinSAT struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_WinSAT {

/// 
    #[serde(rename = "CPUScore")]
    pub cpuscore: Option<f32>,

/// 
    #[serde(rename = "D3DScore")]
    pub d3_dscore: Option<f32>,

/// 
    #[serde(rename = "DiskScore")]
    pub disk_score: Option<f32>,

/// 
    #[serde(rename = "GraphicsScore")]
    pub graphics_score: Option<f32>,

/// 
    #[serde(rename = "MemoryScore")]
    pub memory_score: Option<f32>,

/// 
    #[serde(rename = "TimeTaken")]
    pub time_taken: Option<String>,

/// 
    #[serde(rename = "WinSATAssessmentState")]
    pub win_satassessment_state: Option<WinSAT_WinSATAssessmentState>,

/// 
    #[serde(rename = "WinSPRLevel")]
    pub win_sprlevel: Option<f32>,
}

impl Win32_WinSAT {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cpuscore: None,
            d3_dscore: None,
            disk_score: None,
            graphics_score: None,
            memory_score: None,
            time_taken: None,
            win_satassessment_state: None,
            win_sprlevel: None,
        }
    }


    /// Sets the value of CPUScore
    pub fn set_cpuscore(&mut self, value: f32) {
        self.cpuscore = Some(value);
    }

    /// Gets the value of CPUScore
    pub fn get_cpuscore(&self) -> Option<&f32> {
        self.cpuscore.as_ref()
    }

    /// Sets the value of D3DScore
    pub fn set_d3_dscore(&mut self, value: f32) {
        self.d3_dscore = Some(value);
    }

    /// Gets the value of D3DScore
    pub fn get_d3_dscore(&self) -> Option<&f32> {
        self.d3_dscore.as_ref()
    }

    /// Sets the value of DiskScore
    pub fn set_disk_score(&mut self, value: f32) {
        self.disk_score = Some(value);
    }

    /// Gets the value of DiskScore
    pub fn get_disk_score(&self) -> Option<&f32> {
        self.disk_score.as_ref()
    }

    /// Sets the value of GraphicsScore
    pub fn set_graphics_score(&mut self, value: f32) {
        self.graphics_score = Some(value);
    }

    /// Gets the value of GraphicsScore
    pub fn get_graphics_score(&self) -> Option<&f32> {
        self.graphics_score.as_ref()
    }

    /// Sets the value of MemoryScore
    pub fn set_memory_score(&mut self, value: f32) {
        self.memory_score = Some(value);
    }

    /// Gets the value of MemoryScore
    pub fn get_memory_score(&self) -> Option<&f32> {
        self.memory_score.as_ref()
    }

    /// Sets the value of TimeTaken
    pub fn set_time_taken(&mut self, value: String) {
        self.time_taken = Some(value);
    }

    /// Gets the value of TimeTaken
    pub fn get_time_taken(&self) -> Option<&String> {
        self.time_taken.as_ref()
    }

    /// Sets the value of WinSATAssessmentState
    pub fn set_win_satassessment_state(&mut self, value: WinSAT_WinSATAssessmentState) {
        self.win_satassessment_state = Some(value);
    }

    /// Gets the value of WinSATAssessmentState
    pub fn get_win_satassessment_state(&self) -> Option<&WinSAT_WinSATAssessmentState> {
        self.win_satassessment_state.as_ref()
    }

    /// Sets the value of WinSPRLevel
    pub fn set_win_sprlevel(&mut self, value: f32) {
        self.win_sprlevel = Some(value);
    }

    /// Gets the value of WinSPRLevel
    pub fn get_win_sprlevel(&self) -> Option<&f32> {
        self.win_sprlevel.as_ref()
    }
}

