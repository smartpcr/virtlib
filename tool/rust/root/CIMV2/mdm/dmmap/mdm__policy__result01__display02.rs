// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Display02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Display02 {

/// 
    #[serde(rename = "DisablePerProcessDpiForApps")]
    pub disable_per_process_dpi_for_apps: Option<String>,

/// 
    #[serde(rename = "EnablePerProcessDpi")]
    pub enable_per_process_dpi: Option<i32>,

/// 
    #[serde(rename = "EnablePerProcessDpiForApps")]
    pub enable_per_process_dpi_for_apps: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TurnOffGdiDPIScalingForApps")]
    pub turn_off_gdi_dpiscaling_for_apps: Option<String>,

/// 
    #[serde(rename = "TurnOnGdiDPIScalingForApps")]
    pub turn_on_gdi_dpiscaling_for_apps: Option<String>,
}

impl MDM_Policy_Result01_Display02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disable_per_process_dpi_for_apps: None,
            enable_per_process_dpi: None,
            enable_per_process_dpi_for_apps: None,
            instance_id: None,
            parent_id: None,
            turn_off_gdi_dpiscaling_for_apps: None,
            turn_on_gdi_dpiscaling_for_apps: None,
        }
    }


    /// Sets the value of DisablePerProcessDpiForApps
    pub fn set_disable_per_process_dpi_for_apps(&mut self, value: String) {
        self.disable_per_process_dpi_for_apps = Some(value);
    }

    /// Gets the value of DisablePerProcessDpiForApps
    pub fn get_disable_per_process_dpi_for_apps(&self) -> Option<&String> {
        self.disable_per_process_dpi_for_apps.as_ref()
    }

    /// Sets the value of EnablePerProcessDpi
    pub fn set_enable_per_process_dpi(&mut self, value: i32) {
        self.enable_per_process_dpi = Some(value);
    }

    /// Gets the value of EnablePerProcessDpi
    pub fn get_enable_per_process_dpi(&self) -> Option<&i32> {
        self.enable_per_process_dpi.as_ref()
    }

    /// Sets the value of EnablePerProcessDpiForApps
    pub fn set_enable_per_process_dpi_for_apps(&mut self, value: String) {
        self.enable_per_process_dpi_for_apps = Some(value);
    }

    /// Gets the value of EnablePerProcessDpiForApps
    pub fn get_enable_per_process_dpi_for_apps(&self) -> Option<&String> {
        self.enable_per_process_dpi_for_apps.as_ref()
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

    /// Sets the value of TurnOffGdiDPIScalingForApps
    pub fn set_turn_off_gdi_dpiscaling_for_apps(&mut self, value: String) {
        self.turn_off_gdi_dpiscaling_for_apps = Some(value);
    }

    /// Gets the value of TurnOffGdiDPIScalingForApps
    pub fn get_turn_off_gdi_dpiscaling_for_apps(&self) -> Option<&String> {
        self.turn_off_gdi_dpiscaling_for_apps.as_ref()
    }

    /// Sets the value of TurnOnGdiDPIScalingForApps
    pub fn set_turn_on_gdi_dpiscaling_for_apps(&mut self, value: String) {
        self.turn_on_gdi_dpiscaling_for_apps = Some(value);
    }

    /// Gets the value of TurnOnGdiDPIScalingForApps
    pub fn get_turn_on_gdi_dpiscaling_for_apps(&self) -> Option<&String> {
        self.turn_on_gdi_dpiscaling_for_apps.as_ref()
    }
}

