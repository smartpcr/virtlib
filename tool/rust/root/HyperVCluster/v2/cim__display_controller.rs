// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DisplayController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DisplayController {
    #[serde(flatten)]
    pub base: CIM_Controller,

/// An array of integers indicating the graphics and 3D capabilities of the DisplayController.
    #[serde(rename = "AcceleratorCapabilities")]
    pub accelerator_capabilities: Vec<DisplayController_AcceleratorCapabilities>,

/// An array of free-form strings providing more detailed explanations for any of the video Accelerator features indicated in the Capabilities array. Note, each entry of this array is related to the entry in the Capabilities array that is located at the same index.
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// Maximum amount of memory supported in bytes.
    #[serde(rename = "MaxMemorySupported")]
    pub max_memory_supported: Option<u32>,

/// Number of video pages supported given the current resolutions and available memory.
    #[serde(rename = "NumberOfVideoPages")]
    pub number_of_video_pages: Option<u32>,

/// A string describing the video architecture type when the instance's VideoArchitecture property is 1 ("Other").
    #[serde(rename = "OtherVideoArchitecture")]
    pub other_video_architecture: Option<String>,

/// A string describing the video memory type when the instance's VideoMemoryType property is 1 ("Other").
    #[serde(rename = "OtherVideoMemoryType")]
    pub other_video_memory_type: Option<String>,

/// An integer enumeration indicating the display controllers video architecture used to generate the video signal. Usually, a dedicated video processor generates the video signal in accordance with the specified architecture.It is an indicator of the maximum resolution capability of the display controller.
    #[serde(rename = "VideoArchitecture")]
    pub video_architecture: Option<DisplayController_VideoArchitecture>,

/// An integer enumeration indicating the type of video memory.
    #[serde(rename = "VideoMemoryType")]
    pub video_memory_type: Option<DisplayController_VideoMemoryType>,

/// A free-form string describing the video processor/Controller.
    #[serde(rename = "VideoProcessor")]
    pub video_processor: Option<String>,
}

impl CIM_DisplayController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Controller::new(),
            accelerator_capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            max_memory_supported: None,
            number_of_video_pages: None,
            other_video_architecture: None,
            other_video_memory_type: None,
            video_architecture: None,
            video_memory_type: None,
            video_processor: None,
        }
    }


    /// Sets the value of AcceleratorCapabilities
    pub fn set_accelerator_capabilities(&mut self, value: Vec<DisplayController_AcceleratorCapabilities>) {
        self.accelerator_capabilities = value;
    }

    /// Gets the value of AcceleratorCapabilities
    pub fn get_accelerator_capabilities(&self) -> &Vec<DisplayController_AcceleratorCapabilities> {
        &self.accelerator_capabilities
    }

    /// Sets the value of CapabilityDescriptions
    pub fn set_capability_descriptions(&mut self, value: Vec<String>) {
        self.capability_descriptions = value;
    }

    /// Gets the value of CapabilityDescriptions
    pub fn get_capability_descriptions(&self) -> &Vec<String> {
        &self.capability_descriptions
    }

    /// Sets the value of MaxMemorySupported
    pub fn set_max_memory_supported(&mut self, value: u32) {
        self.max_memory_supported = Some(value);
    }

    /// Gets the value of MaxMemorySupported
    pub fn get_max_memory_supported(&self) -> Option<&u32> {
        self.max_memory_supported.as_ref()
    }

    /// Sets the value of NumberOfVideoPages
    pub fn set_number_of_video_pages(&mut self, value: u32) {
        self.number_of_video_pages = Some(value);
    }

    /// Gets the value of NumberOfVideoPages
    pub fn get_number_of_video_pages(&self) -> Option<&u32> {
        self.number_of_video_pages.as_ref()
    }

    /// Sets the value of OtherVideoArchitecture
    pub fn set_other_video_architecture(&mut self, value: String) {
        self.other_video_architecture = Some(value);
    }

    /// Gets the value of OtherVideoArchitecture
    pub fn get_other_video_architecture(&self) -> Option<&String> {
        self.other_video_architecture.as_ref()
    }

    /// Sets the value of OtherVideoMemoryType
    pub fn set_other_video_memory_type(&mut self, value: String) {
        self.other_video_memory_type = Some(value);
    }

    /// Gets the value of OtherVideoMemoryType
    pub fn get_other_video_memory_type(&self) -> Option<&String> {
        self.other_video_memory_type.as_ref()
    }

    /// Sets the value of VideoArchitecture
    pub fn set_video_architecture(&mut self, value: DisplayController_VideoArchitecture) {
        self.video_architecture = Some(value);
    }

    /// Gets the value of VideoArchitecture
    pub fn get_video_architecture(&self) -> Option<&DisplayController_VideoArchitecture> {
        self.video_architecture.as_ref()
    }

    /// Sets the value of VideoMemoryType
    pub fn set_video_memory_type(&mut self, value: DisplayController_VideoMemoryType) {
        self.video_memory_type = Some(value);
    }

    /// Gets the value of VideoMemoryType
    pub fn get_video_memory_type(&self) -> Option<&DisplayController_VideoMemoryType> {
        self.video_memory_type.as_ref()
    }

    /// Sets the value of VideoProcessor
    pub fn set_video_processor(&mut self, value: String) {
        self.video_processor = Some(value);
    }

    /// Gets the value of VideoProcessor
    pub fn get_video_processor(&self) -> Option<&String> {
        self.video_processor.as_ref()
    }
}

