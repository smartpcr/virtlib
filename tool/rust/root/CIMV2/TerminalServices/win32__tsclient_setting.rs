// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSClientSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSClientSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "AdvancedRemoteAppGraphics")]
    pub advanced_remote_app_graphics: Option<u32>,

/// 
    #[serde(rename = "AudioCaptureRedir")]
    pub audio_capture_redir: Option<u32>,

/// 
    #[serde(rename = "AudioMapping")]
    pub audio_mapping: Option<u32>,

/// 
    #[serde(rename = "AVC444ModePreferred")]
    pub avc444_mode_preferred: Option<u32>,

/// 
    #[serde(rename = "ClipboardMapping")]
    pub clipboard_mapping: Option<u32>,

/// 
    #[serde(rename = "ColorDepth")]
    pub color_depth: Option<u32>,

/// 
    #[serde(rename = "ColorDepthPolicy")]
    pub color_depth_policy: Option<u32>,

/// 
    #[serde(rename = "COMPortMapping")]
    pub comport_mapping: Option<u32>,

/// 
    #[serde(rename = "ConnectClientDrivesAtLogon")]
    pub connect_client_drives_at_logon: Option<u32>,

/// 
    #[serde(rename = "ConnectionPolicy")]
    pub connection_policy: Option<u32>,

/// 
    #[serde(rename = "ConnectPrinterAtLogon")]
    pub connect_printer_at_logon: Option<u32>,

/// 
    #[serde(rename = "DefaultToClientPrinter")]
    pub default_to_client_printer: Option<u32>,

/// 
    #[serde(rename = "DriveMapping")]
    pub drive_mapping: Option<u32>,

/// 
    #[serde(rename = "EncodeImageQuality")]
    pub encode_image_quality: Option<u32>,

/// 
    #[serde(rename = "HardwareGraphicsAdapter")]
    pub hardware_graphics_adapter: Option<u32>,

/// 
    #[serde(rename = "LPTPortMapping")]
    pub lptport_mapping: Option<u32>,

/// 
    #[serde(rename = "MaxMonitors")]
    pub max_monitors: Option<u32>,

/// 
    #[serde(rename = "MaxXResolution")]
    pub max_xresolution: Option<u32>,

/// 
    #[serde(rename = "MaxYResolution")]
    pub max_yresolution: Option<u32>,

/// 
    #[serde(rename = "PNPRedirection")]
    pub pnpredirection: Option<u32>,

/// 
    #[serde(rename = "PolicyAdvancedRemoteAppGraphics")]
    pub policy_advanced_remote_app_graphics: Option<u32>,

/// 
    #[serde(rename = "PolicySourceAudioCaptureRedir")]
    pub policy_source_audio_capture_redir: Option<u32>,

/// 
    #[serde(rename = "PolicySourceAudioMapping")]
    pub policy_source_audio_mapping: Option<u32>,

/// 
    #[serde(rename = "PolicySourceAvc444ModePreferred")]
    pub policy_source_avc444_mode_preferred: Option<u32>,

/// 
    #[serde(rename = "PolicySourceClipboardMapping")]
    pub policy_source_clipboard_mapping: Option<u32>,

/// 
    #[serde(rename = "PolicySourceColorDepth")]
    pub policy_source_color_depth: Option<u32>,

/// 
    #[serde(rename = "PolicySourceColorDepthPolicy")]
    pub policy_source_color_depth_policy: Option<u32>,

/// 
    #[serde(rename = "PolicySourceCOMPortMapping")]
    pub policy_source_comport_mapping: Option<u32>,

/// 
    #[serde(rename = "PolicySourceDefaultToClientPrinter")]
    pub policy_source_default_to_client_printer: Option<u32>,

/// 
    #[serde(rename = "PolicySourceDriveMapping")]
    pub policy_source_drive_mapping: Option<u32>,

/// 
    #[serde(rename = "PolicySourceEncodeImageQuality")]
    pub policy_source_encode_image_quality: Option<u32>,

/// 
    #[serde(rename = "PolicySourceHardwareGraphicsAdapter")]
    pub policy_source_hardware_graphics_adapter: Option<u32>,

/// 
    #[serde(rename = "PolicySourceLPTPortMapping")]
    pub policy_source_lptport_mapping: Option<u32>,

/// 
    #[serde(rename = "PolicySourceMaxMonitors")]
    pub policy_source_max_monitors: Option<u32>,

/// 
    #[serde(rename = "PolicySourceMaxResolution")]
    pub policy_source_max_resolution: Option<u32>,

/// 
    #[serde(rename = "PolicySourcePNPRedirection")]
    pub policy_source_pnpredirection: Option<u32>,

/// 
    #[serde(rename = "PolicySourceRemoteSessionProfile")]
    pub policy_source_remote_session_profile: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSelectNetworkDetect")]
    pub policy_source_select_network_detect: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSelectTransport")]
    pub policy_source_select_transport: Option<u32>,

/// 
    #[serde(rename = "PolicySourceVideoPlaybackRedir")]
    pub policy_source_video_playback_redir: Option<u32>,

/// 
    #[serde(rename = "PolicySourceWindowsPrinterMapping")]
    pub policy_source_windows_printer_mapping: Option<u32>,

/// 
    #[serde(rename = "RemoteSessionProfile")]
    pub remote_session_profile: Option<u32>,

/// 
    #[serde(rename = "SelectNetworkDetect")]
    pub select_network_detect: Option<u32>,

/// 
    #[serde(rename = "SelectTransport")]
    pub select_transport: Option<u32>,

/// 
    #[serde(rename = "VideoPlaybackRedir")]
    pub video_playback_redir: Option<u32>,

/// 
    #[serde(rename = "WindowsPrinterMapping")]
    pub windows_printer_mapping: Option<u32>,
}

impl Win32_TSClientSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            advanced_remote_app_graphics: None,
            audio_capture_redir: None,
            audio_mapping: None,
            avc444_mode_preferred: None,
            clipboard_mapping: None,
            color_depth: None,
            color_depth_policy: None,
            comport_mapping: None,
            connect_client_drives_at_logon: None,
            connection_policy: None,
            connect_printer_at_logon: None,
            default_to_client_printer: None,
            drive_mapping: None,
            encode_image_quality: None,
            hardware_graphics_adapter: None,
            lptport_mapping: None,
            max_monitors: None,
            max_xresolution: None,
            max_yresolution: None,
            pnpredirection: None,
            policy_advanced_remote_app_graphics: None,
            policy_source_audio_capture_redir: None,
            policy_source_audio_mapping: None,
            policy_source_avc444_mode_preferred: None,
            policy_source_clipboard_mapping: None,
            policy_source_color_depth: None,
            policy_source_color_depth_policy: None,
            policy_source_comport_mapping: None,
            policy_source_default_to_client_printer: None,
            policy_source_drive_mapping: None,
            policy_source_encode_image_quality: None,
            policy_source_hardware_graphics_adapter: None,
            policy_source_lptport_mapping: None,
            policy_source_max_monitors: None,
            policy_source_max_resolution: None,
            policy_source_pnpredirection: None,
            policy_source_remote_session_profile: None,
            policy_source_select_network_detect: None,
            policy_source_select_transport: None,
            policy_source_video_playback_redir: None,
            policy_source_windows_printer_mapping: None,
            remote_session_profile: None,
            select_network_detect: None,
            select_transport: None,
            video_playback_redir: None,
            windows_printer_mapping: None,
        }
    }


    /// Sets the value of AdvancedRemoteAppGraphics
    pub fn set_advanced_remote_app_graphics(&mut self, value: u32) {
        self.advanced_remote_app_graphics = Some(value);
    }

    /// Gets the value of AdvancedRemoteAppGraphics
    pub fn get_advanced_remote_app_graphics(&self) -> Option<&u32> {
        self.advanced_remote_app_graphics.as_ref()
    }

    /// Sets the value of AudioCaptureRedir
    pub fn set_audio_capture_redir(&mut self, value: u32) {
        self.audio_capture_redir = Some(value);
    }

    /// Gets the value of AudioCaptureRedir
    pub fn get_audio_capture_redir(&self) -> Option<&u32> {
        self.audio_capture_redir.as_ref()
    }

    /// Sets the value of AudioMapping
    pub fn set_audio_mapping(&mut self, value: u32) {
        self.audio_mapping = Some(value);
    }

    /// Gets the value of AudioMapping
    pub fn get_audio_mapping(&self) -> Option<&u32> {
        self.audio_mapping.as_ref()
    }

    /// Sets the value of AVC444ModePreferred
    pub fn set_avc444_mode_preferred(&mut self, value: u32) {
        self.avc444_mode_preferred = Some(value);
    }

    /// Gets the value of AVC444ModePreferred
    pub fn get_avc444_mode_preferred(&self) -> Option<&u32> {
        self.avc444_mode_preferred.as_ref()
    }

    /// Sets the value of ClipboardMapping
    pub fn set_clipboard_mapping(&mut self, value: u32) {
        self.clipboard_mapping = Some(value);
    }

    /// Gets the value of ClipboardMapping
    pub fn get_clipboard_mapping(&self) -> Option<&u32> {
        self.clipboard_mapping.as_ref()
    }

    /// Sets the value of ColorDepth
    pub fn set_color_depth(&mut self, value: u32) {
        self.color_depth = Some(value);
    }

    /// Gets the value of ColorDepth
    pub fn get_color_depth(&self) -> Option<&u32> {
        self.color_depth.as_ref()
    }

    /// Sets the value of ColorDepthPolicy
    pub fn set_color_depth_policy(&mut self, value: u32) {
        self.color_depth_policy = Some(value);
    }

    /// Gets the value of ColorDepthPolicy
    pub fn get_color_depth_policy(&self) -> Option<&u32> {
        self.color_depth_policy.as_ref()
    }

    /// Sets the value of COMPortMapping
    pub fn set_comport_mapping(&mut self, value: u32) {
        self.comport_mapping = Some(value);
    }

    /// Gets the value of COMPortMapping
    pub fn get_comport_mapping(&self) -> Option<&u32> {
        self.comport_mapping.as_ref()
    }

    /// Sets the value of ConnectClientDrivesAtLogon
    pub fn set_connect_client_drives_at_logon(&mut self, value: u32) {
        self.connect_client_drives_at_logon = Some(value);
    }

    /// Gets the value of ConnectClientDrivesAtLogon
    pub fn get_connect_client_drives_at_logon(&self) -> Option<&u32> {
        self.connect_client_drives_at_logon.as_ref()
    }

    /// Sets the value of ConnectionPolicy
    pub fn set_connection_policy(&mut self, value: u32) {
        self.connection_policy = Some(value);
    }

    /// Gets the value of ConnectionPolicy
    pub fn get_connection_policy(&self) -> Option<&u32> {
        self.connection_policy.as_ref()
    }

    /// Sets the value of ConnectPrinterAtLogon
    pub fn set_connect_printer_at_logon(&mut self, value: u32) {
        self.connect_printer_at_logon = Some(value);
    }

    /// Gets the value of ConnectPrinterAtLogon
    pub fn get_connect_printer_at_logon(&self) -> Option<&u32> {
        self.connect_printer_at_logon.as_ref()
    }

    /// Sets the value of DefaultToClientPrinter
    pub fn set_default_to_client_printer(&mut self, value: u32) {
        self.default_to_client_printer = Some(value);
    }

    /// Gets the value of DefaultToClientPrinter
    pub fn get_default_to_client_printer(&self) -> Option<&u32> {
        self.default_to_client_printer.as_ref()
    }

    /// Sets the value of DriveMapping
    pub fn set_drive_mapping(&mut self, value: u32) {
        self.drive_mapping = Some(value);
    }

    /// Gets the value of DriveMapping
    pub fn get_drive_mapping(&self) -> Option<&u32> {
        self.drive_mapping.as_ref()
    }

    /// Sets the value of EncodeImageQuality
    pub fn set_encode_image_quality(&mut self, value: u32) {
        self.encode_image_quality = Some(value);
    }

    /// Gets the value of EncodeImageQuality
    pub fn get_encode_image_quality(&self) -> Option<&u32> {
        self.encode_image_quality.as_ref()
    }

    /// Sets the value of HardwareGraphicsAdapter
    pub fn set_hardware_graphics_adapter(&mut self, value: u32) {
        self.hardware_graphics_adapter = Some(value);
    }

    /// Gets the value of HardwareGraphicsAdapter
    pub fn get_hardware_graphics_adapter(&self) -> Option<&u32> {
        self.hardware_graphics_adapter.as_ref()
    }

    /// Sets the value of LPTPortMapping
    pub fn set_lptport_mapping(&mut self, value: u32) {
        self.lptport_mapping = Some(value);
    }

    /// Gets the value of LPTPortMapping
    pub fn get_lptport_mapping(&self) -> Option<&u32> {
        self.lptport_mapping.as_ref()
    }

    /// Sets the value of MaxMonitors
    pub fn set_max_monitors(&mut self, value: u32) {
        self.max_monitors = Some(value);
    }

    /// Gets the value of MaxMonitors
    pub fn get_max_monitors(&self) -> Option<&u32> {
        self.max_monitors.as_ref()
    }

    /// Sets the value of MaxXResolution
    pub fn set_max_xresolution(&mut self, value: u32) {
        self.max_xresolution = Some(value);
    }

    /// Gets the value of MaxXResolution
    pub fn get_max_xresolution(&self) -> Option<&u32> {
        self.max_xresolution.as_ref()
    }

    /// Sets the value of MaxYResolution
    pub fn set_max_yresolution(&mut self, value: u32) {
        self.max_yresolution = Some(value);
    }

    /// Gets the value of MaxYResolution
    pub fn get_max_yresolution(&self) -> Option<&u32> {
        self.max_yresolution.as_ref()
    }

    /// Sets the value of PNPRedirection
    pub fn set_pnpredirection(&mut self, value: u32) {
        self.pnpredirection = Some(value);
    }

    /// Gets the value of PNPRedirection
    pub fn get_pnpredirection(&self) -> Option<&u32> {
        self.pnpredirection.as_ref()
    }

    /// Sets the value of PolicyAdvancedRemoteAppGraphics
    pub fn set_policy_advanced_remote_app_graphics(&mut self, value: u32) {
        self.policy_advanced_remote_app_graphics = Some(value);
    }

    /// Gets the value of PolicyAdvancedRemoteAppGraphics
    pub fn get_policy_advanced_remote_app_graphics(&self) -> Option<&u32> {
        self.policy_advanced_remote_app_graphics.as_ref()
    }

    /// Sets the value of PolicySourceAudioCaptureRedir
    pub fn set_policy_source_audio_capture_redir(&mut self, value: u32) {
        self.policy_source_audio_capture_redir = Some(value);
    }

    /// Gets the value of PolicySourceAudioCaptureRedir
    pub fn get_policy_source_audio_capture_redir(&self) -> Option<&u32> {
        self.policy_source_audio_capture_redir.as_ref()
    }

    /// Sets the value of PolicySourceAudioMapping
    pub fn set_policy_source_audio_mapping(&mut self, value: u32) {
        self.policy_source_audio_mapping = Some(value);
    }

    /// Gets the value of PolicySourceAudioMapping
    pub fn get_policy_source_audio_mapping(&self) -> Option<&u32> {
        self.policy_source_audio_mapping.as_ref()
    }

    /// Sets the value of PolicySourceAvc444ModePreferred
    pub fn set_policy_source_avc444_mode_preferred(&mut self, value: u32) {
        self.policy_source_avc444_mode_preferred = Some(value);
    }

    /// Gets the value of PolicySourceAvc444ModePreferred
    pub fn get_policy_source_avc444_mode_preferred(&self) -> Option<&u32> {
        self.policy_source_avc444_mode_preferred.as_ref()
    }

    /// Sets the value of PolicySourceClipboardMapping
    pub fn set_policy_source_clipboard_mapping(&mut self, value: u32) {
        self.policy_source_clipboard_mapping = Some(value);
    }

    /// Gets the value of PolicySourceClipboardMapping
    pub fn get_policy_source_clipboard_mapping(&self) -> Option<&u32> {
        self.policy_source_clipboard_mapping.as_ref()
    }

    /// Sets the value of PolicySourceColorDepth
    pub fn set_policy_source_color_depth(&mut self, value: u32) {
        self.policy_source_color_depth = Some(value);
    }

    /// Gets the value of PolicySourceColorDepth
    pub fn get_policy_source_color_depth(&self) -> Option<&u32> {
        self.policy_source_color_depth.as_ref()
    }

    /// Sets the value of PolicySourceColorDepthPolicy
    pub fn set_policy_source_color_depth_policy(&mut self, value: u32) {
        self.policy_source_color_depth_policy = Some(value);
    }

    /// Gets the value of PolicySourceColorDepthPolicy
    pub fn get_policy_source_color_depth_policy(&self) -> Option<&u32> {
        self.policy_source_color_depth_policy.as_ref()
    }

    /// Sets the value of PolicySourceCOMPortMapping
    pub fn set_policy_source_comport_mapping(&mut self, value: u32) {
        self.policy_source_comport_mapping = Some(value);
    }

    /// Gets the value of PolicySourceCOMPortMapping
    pub fn get_policy_source_comport_mapping(&self) -> Option<&u32> {
        self.policy_source_comport_mapping.as_ref()
    }

    /// Sets the value of PolicySourceDefaultToClientPrinter
    pub fn set_policy_source_default_to_client_printer(&mut self, value: u32) {
        self.policy_source_default_to_client_printer = Some(value);
    }

    /// Gets the value of PolicySourceDefaultToClientPrinter
    pub fn get_policy_source_default_to_client_printer(&self) -> Option<&u32> {
        self.policy_source_default_to_client_printer.as_ref()
    }

    /// Sets the value of PolicySourceDriveMapping
    pub fn set_policy_source_drive_mapping(&mut self, value: u32) {
        self.policy_source_drive_mapping = Some(value);
    }

    /// Gets the value of PolicySourceDriveMapping
    pub fn get_policy_source_drive_mapping(&self) -> Option<&u32> {
        self.policy_source_drive_mapping.as_ref()
    }

    /// Sets the value of PolicySourceEncodeImageQuality
    pub fn set_policy_source_encode_image_quality(&mut self, value: u32) {
        self.policy_source_encode_image_quality = Some(value);
    }

    /// Gets the value of PolicySourceEncodeImageQuality
    pub fn get_policy_source_encode_image_quality(&self) -> Option<&u32> {
        self.policy_source_encode_image_quality.as_ref()
    }

    /// Sets the value of PolicySourceHardwareGraphicsAdapter
    pub fn set_policy_source_hardware_graphics_adapter(&mut self, value: u32) {
        self.policy_source_hardware_graphics_adapter = Some(value);
    }

    /// Gets the value of PolicySourceHardwareGraphicsAdapter
    pub fn get_policy_source_hardware_graphics_adapter(&self) -> Option<&u32> {
        self.policy_source_hardware_graphics_adapter.as_ref()
    }

    /// Sets the value of PolicySourceLPTPortMapping
    pub fn set_policy_source_lptport_mapping(&mut self, value: u32) {
        self.policy_source_lptport_mapping = Some(value);
    }

    /// Gets the value of PolicySourceLPTPortMapping
    pub fn get_policy_source_lptport_mapping(&self) -> Option<&u32> {
        self.policy_source_lptport_mapping.as_ref()
    }

    /// Sets the value of PolicySourceMaxMonitors
    pub fn set_policy_source_max_monitors(&mut self, value: u32) {
        self.policy_source_max_monitors = Some(value);
    }

    /// Gets the value of PolicySourceMaxMonitors
    pub fn get_policy_source_max_monitors(&self) -> Option<&u32> {
        self.policy_source_max_monitors.as_ref()
    }

    /// Sets the value of PolicySourceMaxResolution
    pub fn set_policy_source_max_resolution(&mut self, value: u32) {
        self.policy_source_max_resolution = Some(value);
    }

    /// Gets the value of PolicySourceMaxResolution
    pub fn get_policy_source_max_resolution(&self) -> Option<&u32> {
        self.policy_source_max_resolution.as_ref()
    }

    /// Sets the value of PolicySourcePNPRedirection
    pub fn set_policy_source_pnpredirection(&mut self, value: u32) {
        self.policy_source_pnpredirection = Some(value);
    }

    /// Gets the value of PolicySourcePNPRedirection
    pub fn get_policy_source_pnpredirection(&self) -> Option<&u32> {
        self.policy_source_pnpredirection.as_ref()
    }

    /// Sets the value of PolicySourceRemoteSessionProfile
    pub fn set_policy_source_remote_session_profile(&mut self, value: u32) {
        self.policy_source_remote_session_profile = Some(value);
    }

    /// Gets the value of PolicySourceRemoteSessionProfile
    pub fn get_policy_source_remote_session_profile(&self) -> Option<&u32> {
        self.policy_source_remote_session_profile.as_ref()
    }

    /// Sets the value of PolicySourceSelectNetworkDetect
    pub fn set_policy_source_select_network_detect(&mut self, value: u32) {
        self.policy_source_select_network_detect = Some(value);
    }

    /// Gets the value of PolicySourceSelectNetworkDetect
    pub fn get_policy_source_select_network_detect(&self) -> Option<&u32> {
        self.policy_source_select_network_detect.as_ref()
    }

    /// Sets the value of PolicySourceSelectTransport
    pub fn set_policy_source_select_transport(&mut self, value: u32) {
        self.policy_source_select_transport = Some(value);
    }

    /// Gets the value of PolicySourceSelectTransport
    pub fn get_policy_source_select_transport(&self) -> Option<&u32> {
        self.policy_source_select_transport.as_ref()
    }

    /// Sets the value of PolicySourceVideoPlaybackRedir
    pub fn set_policy_source_video_playback_redir(&mut self, value: u32) {
        self.policy_source_video_playback_redir = Some(value);
    }

    /// Gets the value of PolicySourceVideoPlaybackRedir
    pub fn get_policy_source_video_playback_redir(&self) -> Option<&u32> {
        self.policy_source_video_playback_redir.as_ref()
    }

    /// Sets the value of PolicySourceWindowsPrinterMapping
    pub fn set_policy_source_windows_printer_mapping(&mut self, value: u32) {
        self.policy_source_windows_printer_mapping = Some(value);
    }

    /// Gets the value of PolicySourceWindowsPrinterMapping
    pub fn get_policy_source_windows_printer_mapping(&self) -> Option<&u32> {
        self.policy_source_windows_printer_mapping.as_ref()
    }

    /// Sets the value of RemoteSessionProfile
    pub fn set_remote_session_profile(&mut self, value: u32) {
        self.remote_session_profile = Some(value);
    }

    /// Gets the value of RemoteSessionProfile
    pub fn get_remote_session_profile(&self) -> Option<&u32> {
        self.remote_session_profile.as_ref()
    }

    /// Sets the value of SelectNetworkDetect
    pub fn set_select_network_detect(&mut self, value: u32) {
        self.select_network_detect = Some(value);
    }

    /// Gets the value of SelectNetworkDetect
    pub fn get_select_network_detect(&self) -> Option<&u32> {
        self.select_network_detect.as_ref()
    }

    /// Sets the value of SelectTransport
    pub fn set_select_transport(&mut self, value: u32) {
        self.select_transport = Some(value);
    }

    /// Gets the value of SelectTransport
    pub fn get_select_transport(&self) -> Option<&u32> {
        self.select_transport.as_ref()
    }

    /// Sets the value of VideoPlaybackRedir
    pub fn set_video_playback_redir(&mut self, value: u32) {
        self.video_playback_redir = Some(value);
    }

    /// Gets the value of VideoPlaybackRedir
    pub fn get_video_playback_redir(&self) -> Option<&u32> {
        self.video_playback_redir.as_ref()
    }

    /// Sets the value of WindowsPrinterMapping
    pub fn set_windows_printer_mapping(&mut self, value: u32) {
        self.windows_printer_mapping = Some(value);
    }

    /// Gets the value of WindowsPrinterMapping
    pub fn get_windows_printer_mapping(&self) -> Option<&u32> {
        self.windows_printer_mapping.as_ref()
    }

/// 

    /// * `color_depth_policy` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_color_depth_policy(&self, color_depth_policy: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ColorDepthPolicy".to_string(), value: color_depth_policy.into() });
        self.invoke_method("SetColorDepthPolicy", &args)

    }


/// 

    /// * `color_depth` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_color_depth(&self, color_depth: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ColorDepth".to_string(), value: color_depth.into() });
        self.invoke_method("SetColorDepth", &args)

    }


/// 

    /// * `max_monitors` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_max_monitors(&self, max_monitors: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MaxMonitors".to_string(), value: max_monitors.into() });
        self.invoke_method("SetMaxMonitors", &args)

    }


/// 

    /// * `max_xresolution` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_max_xresolution(&self, max_xresolution: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MaxXResolution".to_string(), value: max_xresolution.into() });
        self.invoke_method("SetMaxXResolution", &args)

    }


/// 

    /// * `max_yresolution` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_max_yresolution(&self, max_yresolution: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MaxYResolution".to_string(), value: max_yresolution.into() });
        self.invoke_method("SetMaxYResolution", &args)

    }


/// 

    /// * `connect_client_drives_at_logon` -  (u32)
    /// * `connect_printer_at_logon` -  (u32)
    /// * `default_to_client_printer` -  (u32)

    /// * `return_value` -  (u32)
    pub fn connection_settings(&self, connect_client_drives_at_logon: u32, connect_printer_at_logon: u32, default_to_client_printer: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectClientDrivesAtLogon".to_string(), value: connect_client_drives_at_logon.into() });
        args.push(MethodParameter { name: "ConnectPrinterAtLogon".to_string(), value: connect_printer_at_logon.into() });
        args.push(MethodParameter { name: "DefaultToClientPrinter".to_string(), value: default_to_client_printer.into() });
        self.invoke_method("ConnectionSettings", &args)

    }


/// 

    /// * `property_name` -  (String)
    /// * `value` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_client_property(&self, property_name: &String, value: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PropertyName".to_string(), value: property_name.into() });
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });
        self.invoke_method("SetClientProperty", &args)

    }

}

