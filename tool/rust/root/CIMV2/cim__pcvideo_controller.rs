// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PCVideoController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PCVideoController {
    #[serde(flatten)]
    pub base: CIM_VideoController,

/// 
    #[serde(rename = "NumberOfColorPlanes")]
    pub number_of_color_planes: Option<u16>,

/// 
    #[serde(rename = "VideoArchitecture")]
    pub video_architecture: Option<u16>,

/// 
    #[serde(rename = "VideoMode")]
    pub video_mode: Option<u16>,
}

impl CIM_PCVideoController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VideoController::new(),
            number_of_color_planes: None,
            video_architecture: None,
            video_mode: None,
        }
    }


    /// Sets the value of NumberOfColorPlanes
    pub fn set_number_of_color_planes(&mut self, value: u16) {
        self.number_of_color_planes = Some(value);
    }

    /// Gets the value of NumberOfColorPlanes
    pub fn get_number_of_color_planes(&self) -> Option<&u16> {
        self.number_of_color_planes.as_ref()
    }

    /// Sets the value of VideoArchitecture
    pub fn set_video_architecture(&mut self, value: u16) {
        self.video_architecture = Some(value);
    }

    /// Gets the value of VideoArchitecture
    pub fn get_video_architecture(&self) -> Option<&u16> {
        self.video_architecture.as_ref()
    }

    /// Sets the value of VideoMode
    pub fn set_video_mode(&mut self, value: u16) {
        self.video_mode = Some(value);
    }

    /// Gets the value of VideoMode
    pub fn get_video_mode(&self) -> Option<&u16> {
        self.video_mode.as_ref()
    }
}

