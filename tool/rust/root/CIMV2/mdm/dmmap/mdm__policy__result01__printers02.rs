// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Printers02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Printers02 {

/// 
    #[serde(rename = "ApprovedUsbPrintDevices")]
    pub approved_usb_print_devices: Option<String>,

/// 
    #[serde(rename = "EnableDeviceControl")]
    pub enable_device_control: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PointAndPrintRestrictions")]
    pub point_and_print_restrictions: Option<String>,

/// 
    #[serde(rename = "PublishPrinters")]
    pub publish_printers: Option<String>,
}

impl MDM_Policy_Result01_Printers02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            approved_usb_print_devices: None,
            enable_device_control: None,
            instance_id: None,
            parent_id: None,
            point_and_print_restrictions: None,
            publish_printers: None,
        }
    }


    /// Sets the value of ApprovedUsbPrintDevices
    pub fn set_approved_usb_print_devices(&mut self, value: String) {
        self.approved_usb_print_devices = Some(value);
    }

    /// Gets the value of ApprovedUsbPrintDevices
    pub fn get_approved_usb_print_devices(&self) -> Option<&String> {
        self.approved_usb_print_devices.as_ref()
    }

    /// Sets the value of EnableDeviceControl
    pub fn set_enable_device_control(&mut self, value: String) {
        self.enable_device_control = Some(value);
    }

    /// Gets the value of EnableDeviceControl
    pub fn get_enable_device_control(&self) -> Option<&String> {
        self.enable_device_control.as_ref()
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

    /// Sets the value of PointAndPrintRestrictions
    pub fn set_point_and_print_restrictions(&mut self, value: String) {
        self.point_and_print_restrictions = Some(value);
    }

    /// Gets the value of PointAndPrintRestrictions
    pub fn get_point_and_print_restrictions(&self) -> Option<&String> {
        self.point_and_print_restrictions.as_ref()
    }

    /// Sets the value of PublishPrinters
    pub fn set_publish_printers(&mut self, value: String) {
        self.publish_printers = Some(value);
    }

    /// Gets the value of PublishPrinters
    pub fn get_publish_printers(&self) -> Option<&String> {
        self.publish_printers.as_ref()
    }
}

