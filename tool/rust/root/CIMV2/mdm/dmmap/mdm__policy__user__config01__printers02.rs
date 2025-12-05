// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_Printers02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_Printers02 {

/// 
    #[serde(rename = "ApprovedUsbPrintDevicesUser")]
    pub approved_usb_print_devices_user: Option<String>,

/// 
    #[serde(rename = "EnableDeviceControlUser")]
    pub enable_device_control_user: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PointAndPrintRestrictions_User")]
    pub point_and_print_restrictions__user: Option<String>,
}

impl MDM_Policy_User_Config01_Printers02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            approved_usb_print_devices_user: None,
            enable_device_control_user: None,
            instance_id: None,
            parent_id: None,
            point_and_print_restrictions__user: None,
        }
    }


    /// Sets the value of ApprovedUsbPrintDevicesUser
    pub fn set_approved_usb_print_devices_user(&mut self, value: String) {
        self.approved_usb_print_devices_user = Some(value);
    }

    /// Gets the value of ApprovedUsbPrintDevicesUser
    pub fn get_approved_usb_print_devices_user(&self) -> Option<&String> {
        self.approved_usb_print_devices_user.as_ref()
    }

    /// Sets the value of EnableDeviceControlUser
    pub fn set_enable_device_control_user(&mut self, value: String) {
        self.enable_device_control_user = Some(value);
    }

    /// Gets the value of EnableDeviceControlUser
    pub fn get_enable_device_control_user(&self) -> Option<&String> {
        self.enable_device_control_user.as_ref()
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

    /// Sets the value of PointAndPrintRestrictions_User
    pub fn set_point_and_print_restrictions__user(&mut self, value: String) {
        self.point_and_print_restrictions__user = Some(value);
    }

    /// Gets the value of PointAndPrintRestrictions_User
    pub fn get_point_and_print_restrictions__user(&self) -> Option<&String> {
        self.point_and_print_restrictions__user.as_ref()
    }
}

