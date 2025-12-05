// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_RemoteFXGraphics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_RemoteFXGraphics {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AverageEncodingTime")]
    pub average_encoding_time: Option<u32>,

/// 
    #[serde(rename = "FrameQuality")]
    pub frame_quality: Option<u32>,

/// 
    #[serde(rename = "FramesSkippedPerSecondInsufficientClientResources")]
    pub frames_skipped_per_second_insufficient_client_resources: Option<u32>,

/// 
    #[serde(rename = "FramesSkippedPerSecondInsufficientNetworkResources")]
    pub frames_skipped_per_second_insufficient_network_resources: Option<u32>,

/// 
    #[serde(rename = "FramesSkippedPerSecondInsufficientServerResources")]
    pub frames_skipped_per_second_insufficient_server_resources: Option<u32>,

/// 
    #[serde(rename = "GraphicsCompressionratio")]
    pub graphics_compressionratio: Option<u32>,

/// 
    #[serde(rename = "InputFramesPerSecond")]
    pub input_frames_per_second: Option<u32>,

/// 
    #[serde(rename = "OutputFramesPerSecond")]
    pub output_frames_per_second: Option<u32>,

/// 
    #[serde(rename = "SourceFramesPerSecond")]
    pub source_frames_per_second: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_RemoteFXGraphics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            average_encoding_time: None,
            frame_quality: None,
            frames_skipped_per_second_insufficient_client_resources: None,
            frames_skipped_per_second_insufficient_network_resources: None,
            frames_skipped_per_second_insufficient_server_resources: None,
            graphics_compressionratio: None,
            input_frames_per_second: None,
            output_frames_per_second: None,
            source_frames_per_second: None,
        }
    }


    /// Sets the value of AverageEncodingTime
    pub fn set_average_encoding_time(&mut self, value: u32) {
        self.average_encoding_time = Some(value);
    }

    /// Gets the value of AverageEncodingTime
    pub fn get_average_encoding_time(&self) -> Option<&u32> {
        self.average_encoding_time.as_ref()
    }

    /// Sets the value of FrameQuality
    pub fn set_frame_quality(&mut self, value: u32) {
        self.frame_quality = Some(value);
    }

    /// Gets the value of FrameQuality
    pub fn get_frame_quality(&self) -> Option<&u32> {
        self.frame_quality.as_ref()
    }

    /// Sets the value of FramesSkippedPerSecondInsufficientClientResources
    pub fn set_frames_skipped_per_second_insufficient_client_resources(&mut self, value: u32) {
        self.frames_skipped_per_second_insufficient_client_resources = Some(value);
    }

    /// Gets the value of FramesSkippedPerSecondInsufficientClientResources
    pub fn get_frames_skipped_per_second_insufficient_client_resources(&self) -> Option<&u32> {
        self.frames_skipped_per_second_insufficient_client_resources.as_ref()
    }

    /// Sets the value of FramesSkippedPerSecondInsufficientNetworkResources
    pub fn set_frames_skipped_per_second_insufficient_network_resources(&mut self, value: u32) {
        self.frames_skipped_per_second_insufficient_network_resources = Some(value);
    }

    /// Gets the value of FramesSkippedPerSecondInsufficientNetworkResources
    pub fn get_frames_skipped_per_second_insufficient_network_resources(&self) -> Option<&u32> {
        self.frames_skipped_per_second_insufficient_network_resources.as_ref()
    }

    /// Sets the value of FramesSkippedPerSecondInsufficientServerResources
    pub fn set_frames_skipped_per_second_insufficient_server_resources(&mut self, value: u32) {
        self.frames_skipped_per_second_insufficient_server_resources = Some(value);
    }

    /// Gets the value of FramesSkippedPerSecondInsufficientServerResources
    pub fn get_frames_skipped_per_second_insufficient_server_resources(&self) -> Option<&u32> {
        self.frames_skipped_per_second_insufficient_server_resources.as_ref()
    }

    /// Sets the value of GraphicsCompressionratio
    pub fn set_graphics_compressionratio(&mut self, value: u32) {
        self.graphics_compressionratio = Some(value);
    }

    /// Gets the value of GraphicsCompressionratio
    pub fn get_graphics_compressionratio(&self) -> Option<&u32> {
        self.graphics_compressionratio.as_ref()
    }

    /// Sets the value of InputFramesPerSecond
    pub fn set_input_frames_per_second(&mut self, value: u32) {
        self.input_frames_per_second = Some(value);
    }

    /// Gets the value of InputFramesPerSecond
    pub fn get_input_frames_per_second(&self) -> Option<&u32> {
        self.input_frames_per_second.as_ref()
    }

    /// Sets the value of OutputFramesPerSecond
    pub fn set_output_frames_per_second(&mut self, value: u32) {
        self.output_frames_per_second = Some(value);
    }

    /// Gets the value of OutputFramesPerSecond
    pub fn get_output_frames_per_second(&self) -> Option<&u32> {
        self.output_frames_per_second.as_ref()
    }

    /// Sets the value of SourceFramesPerSecond
    pub fn set_source_frames_per_second(&mut self, value: u32) {
        self.source_frames_per_second = Some(value);
    }

    /// Gets the value of SourceFramesPerSecond
    pub fn get_source_frames_per_second(&self) -> Option<&u32> {
        self.source_frames_per_second.as_ref()
    }
}

