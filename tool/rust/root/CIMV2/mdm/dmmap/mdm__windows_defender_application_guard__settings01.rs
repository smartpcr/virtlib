// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WindowsDefenderApplicationGuard_Settings01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WindowsDefenderApplicationGuard_Settings01 {

/// 
    #[serde(rename = "AllowCameraMicrophoneRedirection")]
    pub allow_camera_microphone_redirection: Option<i32>,

/// 
    #[serde(rename = "AllowPersistence")]
    pub allow_persistence: Option<i32>,

/// 
    #[serde(rename = "AllowVirtualGPU")]
    pub allow_virtual_gpu: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsDefenderApplicationGuard")]
    pub allow_windows_defender_application_guard: Option<i32>,

/// 
    #[serde(rename = "BlockNonEnterpriseContent")]
    pub block_non_enterprise_content: Option<i32>,

/// 
    #[serde(rename = "CertificateThumbprints")]
    pub certificate_thumbprints: Option<String>,

/// 
    #[serde(rename = "ClipboardFileType")]
    pub clipboard_file_type: Option<i32>,

/// 
    #[serde(rename = "ClipboardSettings")]
    pub clipboard_settings: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PrintingSettings")]
    pub printing_settings: Option<i32>,

/// 
    #[serde(rename = "SaveFilesToHost")]
    pub save_files_to_host: Option<i32>,
}

impl MDM_WindowsDefenderApplicationGuard_Settings01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_camera_microphone_redirection: None,
            allow_persistence: None,
            allow_virtual_gpu: None,
            allow_windows_defender_application_guard: None,
            block_non_enterprise_content: None,
            certificate_thumbprints: None,
            clipboard_file_type: None,
            clipboard_settings: None,
            instance_id: None,
            parent_id: None,
            printing_settings: None,
            save_files_to_host: None,
        }
    }


    /// Sets the value of AllowCameraMicrophoneRedirection
    pub fn set_allow_camera_microphone_redirection(&mut self, value: i32) {
        self.allow_camera_microphone_redirection = Some(value);
    }

    /// Gets the value of AllowCameraMicrophoneRedirection
    pub fn get_allow_camera_microphone_redirection(&self) -> Option<&i32> {
        self.allow_camera_microphone_redirection.as_ref()
    }

    /// Sets the value of AllowPersistence
    pub fn set_allow_persistence(&mut self, value: i32) {
        self.allow_persistence = Some(value);
    }

    /// Gets the value of AllowPersistence
    pub fn get_allow_persistence(&self) -> Option<&i32> {
        self.allow_persistence.as_ref()
    }

    /// Sets the value of AllowVirtualGPU
    pub fn set_allow_virtual_gpu(&mut self, value: i32) {
        self.allow_virtual_gpu = Some(value);
    }

    /// Gets the value of AllowVirtualGPU
    pub fn get_allow_virtual_gpu(&self) -> Option<&i32> {
        self.allow_virtual_gpu.as_ref()
    }

    /// Sets the value of AllowWindowsDefenderApplicationGuard
    pub fn set_allow_windows_defender_application_guard(&mut self, value: i32) {
        self.allow_windows_defender_application_guard = Some(value);
    }

    /// Gets the value of AllowWindowsDefenderApplicationGuard
    pub fn get_allow_windows_defender_application_guard(&self) -> Option<&i32> {
        self.allow_windows_defender_application_guard.as_ref()
    }

    /// Sets the value of BlockNonEnterpriseContent
    pub fn set_block_non_enterprise_content(&mut self, value: i32) {
        self.block_non_enterprise_content = Some(value);
    }

    /// Gets the value of BlockNonEnterpriseContent
    pub fn get_block_non_enterprise_content(&self) -> Option<&i32> {
        self.block_non_enterprise_content.as_ref()
    }

    /// Sets the value of CertificateThumbprints
    pub fn set_certificate_thumbprints(&mut self, value: String) {
        self.certificate_thumbprints = Some(value);
    }

    /// Gets the value of CertificateThumbprints
    pub fn get_certificate_thumbprints(&self) -> Option<&String> {
        self.certificate_thumbprints.as_ref()
    }

    /// Sets the value of ClipboardFileType
    pub fn set_clipboard_file_type(&mut self, value: i32) {
        self.clipboard_file_type = Some(value);
    }

    /// Gets the value of ClipboardFileType
    pub fn get_clipboard_file_type(&self) -> Option<&i32> {
        self.clipboard_file_type.as_ref()
    }

    /// Sets the value of ClipboardSettings
    pub fn set_clipboard_settings(&mut self, value: i32) {
        self.clipboard_settings = Some(value);
    }

    /// Gets the value of ClipboardSettings
    pub fn get_clipboard_settings(&self) -> Option<&i32> {
        self.clipboard_settings.as_ref()
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

    /// Sets the value of PrintingSettings
    pub fn set_printing_settings(&mut self, value: i32) {
        self.printing_settings = Some(value);
    }

    /// Gets the value of PrintingSettings
    pub fn get_printing_settings(&self) -> Option<&i32> {
        self.printing_settings.as_ref()
    }

    /// Sets the value of SaveFilesToHost
    pub fn set_save_files_to_host(&mut self, value: i32) {
        self.save_files_to_host = Some(value);
    }

    /// Gets the value of SaveFilesToHost
    pub fn get_save_files_to_host(&self) -> Option<&i32> {
        self.save_files_to_host.as_ref()
    }
}

