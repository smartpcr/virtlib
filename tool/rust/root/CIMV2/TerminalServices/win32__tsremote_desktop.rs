// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSRemoteDesktop struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSRemoteDesktop {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// Alias of the Desktop
    #[serde(rename = "Alias")]
    pub alias: Option<String>,

/// Contents of the icon corresponding to the application
    #[serde(rename = "IconContents")]
    pub icon_contents: Vec<u8>,

/// Index of the icon
    #[serde(rename = "IconIndex")]
    pub icon_index: Option<i32>,

/// Path to the Desktop icon
    #[serde(rename = "IconPath")]
    pub icon_path: Option<String>,

/// Whether this Remote Desktop is meant for a virtual machine farm
    #[serde(rename = "IsVmFarm")]
    pub is_vm_farm: Option<bool>,

/// Contents of the RDP file corresponding to the desktop
    #[serde(rename = "RDPFileContents")]
    pub rdpfile_contents: Option<String>,

/// Security Descriptor controlling access to the application, in SDDL Format. Empty string implies allow all access. Does not support DENY ACEs, or ACEs referring to non-domain users or groups.
    #[serde(rename = "SecurityDescriptor")]
    pub security_descriptor: Option<String>,

/// Whether this application should be shown in the TS Web Access
    #[serde(rename = "ShowInPortal")]
    pub show_in_portal: Option<bool>,

/// Virtual machine farm settigns corresponding to the desktop
    #[serde(rename = "VmFarmSettings")]
    pub vm_farm_settings: Option<String>,
}

impl Win32_TSRemoteDesktop {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            alias: None,
            icon_contents: Vec::new(),
            icon_index: None,
            icon_path: None,
            is_vm_farm: None,
            rdpfile_contents: None,
            security_descriptor: None,
            show_in_portal: None,
            vm_farm_settings: None,
        }
    }


    /// Sets the value of Alias
    pub fn set_alias(&mut self, value: String) {
        self.alias = Some(value);
    }

    /// Gets the value of Alias
    pub fn get_alias(&self) -> Option<&String> {
        self.alias.as_ref()
    }

    /// Sets the value of IconContents
    pub fn set_icon_contents(&mut self, value: Vec<u8>) {
        self.icon_contents = value;
    }

    /// Gets the value of IconContents
    pub fn get_icon_contents(&self) -> &Vec<u8> {
        &self.icon_contents
    }

    /// Sets the value of IconIndex
    pub fn set_icon_index(&mut self, value: i32) {
        self.icon_index = Some(value);
    }

    /// Gets the value of IconIndex
    pub fn get_icon_index(&self) -> Option<&i32> {
        self.icon_index.as_ref()
    }

    /// Sets the value of IconPath
    pub fn set_icon_path(&mut self, value: String) {
        self.icon_path = Some(value);
    }

    /// Gets the value of IconPath
    pub fn get_icon_path(&self) -> Option<&String> {
        self.icon_path.as_ref()
    }

    /// Sets the value of IsVmFarm
    pub fn set_is_vm_farm(&mut self, value: bool) {
        self.is_vm_farm = Some(value);
    }

    /// Gets the value of IsVmFarm
    pub fn get_is_vm_farm(&self) -> Option<&bool> {
        self.is_vm_farm.as_ref()
    }

    /// Sets the value of RDPFileContents
    pub fn set_rdpfile_contents(&mut self, value: String) {
        self.rdpfile_contents = Some(value);
    }

    /// Gets the value of RDPFileContents
    pub fn get_rdpfile_contents(&self) -> Option<&String> {
        self.rdpfile_contents.as_ref()
    }

    /// Sets the value of SecurityDescriptor
    pub fn set_security_descriptor(&mut self, value: String) {
        self.security_descriptor = Some(value);
    }

    /// Gets the value of SecurityDescriptor
    pub fn get_security_descriptor(&self) -> Option<&String> {
        self.security_descriptor.as_ref()
    }

    /// Sets the value of ShowInPortal
    pub fn set_show_in_portal(&mut self, value: bool) {
        self.show_in_portal = Some(value);
    }

    /// Gets the value of ShowInPortal
    pub fn get_show_in_portal(&self) -> Option<&bool> {
        self.show_in_portal.as_ref()
    }

    /// Sets the value of VmFarmSettings
    pub fn set_vm_farm_settings(&mut self, value: String) {
        self.vm_farm_settings = Some(value);
    }

    /// Gets the value of VmFarmSettings
    pub fn get_vm_farm_settings(&self) -> Option<&String> {
        self.vm_farm_settings.as_ref()
    }
}

