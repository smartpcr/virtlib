// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceUpdateCenter_Enrollment01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceUpdateCenter_Enrollment01 {

/// 
    #[serde(rename = "CustomPackageId")]
    pub custom_package_id: Option<String>,

/// 
    #[serde(rename = "DeviceModelId")]
    pub device_model_id: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "OemPartnerRing")]
    pub oem_partner_ring: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PublisherId")]
    pub publisher_id: Option<String>,
}

impl MDM_DeviceUpdateCenter_Enrollment01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            custom_package_id: None,
            device_model_id: None,
            instance_id: None,
            oem_partner_ring: None,
            parent_id: None,
            publisher_id: None,
        }
    }


    /// Sets the value of CustomPackageId
    pub fn set_custom_package_id(&mut self, value: String) {
        self.custom_package_id = Some(value);
    }

    /// Gets the value of CustomPackageId
    pub fn get_custom_package_id(&self) -> Option<&String> {
        self.custom_package_id.as_ref()
    }

    /// Sets the value of DeviceModelId
    pub fn set_device_model_id(&mut self, value: String) {
        self.device_model_id = Some(value);
    }

    /// Gets the value of DeviceModelId
    pub fn get_device_model_id(&self) -> Option<&String> {
        self.device_model_id.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of OemPartnerRing
    pub fn set_oem_partner_ring(&mut self, value: String) {
        self.oem_partner_ring = Some(value);
    }

    /// Gets the value of OemPartnerRing
    pub fn get_oem_partner_ring(&self) -> Option<&String> {
        self.oem_partner_ring.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PublisherId
    pub fn set_publisher_id(&mut self, value: String) {
        self.publisher_id = Some(value);
    }

    /// Gets the value of PublisherId
    pub fn get_publisher_id(&self) -> Option<&String> {
        self.publisher_id.as_ref()
    }
}

