// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_WindowsSandbox02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_WindowsSandbox02 {

/// 
    #[serde(rename = "AllowAudioInput")]
    pub allow_audio_input: Option<i32>,

/// 
    #[serde(rename = "AllowClipboardRedirection")]
    pub allow_clipboard_redirection: Option<i32>,

/// 
    #[serde(rename = "AllowNetworking")]
    pub allow_networking: Option<i32>,

/// 
    #[serde(rename = "AllowPrinterRedirection")]
    pub allow_printer_redirection: Option<i32>,

/// 
    #[serde(rename = "AllowVGPU")]
    pub allow_vgpu: Option<i32>,

/// 
    #[serde(rename = "AllowVideoInput")]
    pub allow_video_input: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Config01_WindowsSandbox02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_audio_input: None,
            allow_clipboard_redirection: None,
            allow_networking: None,
            allow_printer_redirection: None,
            allow_vgpu: None,
            allow_video_input: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowAudioInput
    pub fn set_allow_audio_input(&mut self, value: i32) {
        self.allow_audio_input = Some(value);
    }

    /// Gets the value of AllowAudioInput
    pub fn get_allow_audio_input(&self) -> Option<&i32> {
        self.allow_audio_input.as_ref()
    }

    /// Sets the value of AllowClipboardRedirection
    pub fn set_allow_clipboard_redirection(&mut self, value: i32) {
        self.allow_clipboard_redirection = Some(value);
    }

    /// Gets the value of AllowClipboardRedirection
    pub fn get_allow_clipboard_redirection(&self) -> Option<&i32> {
        self.allow_clipboard_redirection.as_ref()
    }

    /// Sets the value of AllowNetworking
    pub fn set_allow_networking(&mut self, value: i32) {
        self.allow_networking = Some(value);
    }

    /// Gets the value of AllowNetworking
    pub fn get_allow_networking(&self) -> Option<&i32> {
        self.allow_networking.as_ref()
    }

    /// Sets the value of AllowPrinterRedirection
    pub fn set_allow_printer_redirection(&mut self, value: i32) {
        self.allow_printer_redirection = Some(value);
    }

    /// Gets the value of AllowPrinterRedirection
    pub fn get_allow_printer_redirection(&self) -> Option<&i32> {
        self.allow_printer_redirection.as_ref()
    }

    /// Sets the value of AllowVGPU
    pub fn set_allow_vgpu(&mut self, value: i32) {
        self.allow_vgpu = Some(value);
    }

    /// Gets the value of AllowVGPU
    pub fn get_allow_vgpu(&self) -> Option<&i32> {
        self.allow_vgpu.as_ref()
    }

    /// Sets the value of AllowVideoInput
    pub fn set_allow_video_input(&mut self, value: i32) {
        self.allow_video_input = Some(value);
    }

    /// Gets the value of AllowVideoInput
    pub fn get_allow_video_input(&self) -> Option<&i32> {
        self.allow_video_input.as_ref()
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
}

