// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_PortalInfoClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_PortalInfoClass {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Number of elements in iScsiPortalInfo array
    #[serde(rename = "PortalInfoCount")]
    pub portal_info_count: Option<u32>,

/// Variable length array of iScsiPortalInfo.  PortalInfoCount specifies the number of elements in the array. NOTE: this is a variable length array.
    #[serde(rename = "PortalInformation")]
    pub portal_information: Vec<ISCSI_PortalInfo>,
}

impl MSiSCSI_PortalInfoClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            portal_info_count: None,
            portal_information: Vec::new(),
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PortalInfoCount
    pub fn set_portal_info_count(&mut self, value: u32) {
        self.portal_info_count = Some(value);
    }

    /// Gets the value of PortalInfoCount
    pub fn get_portal_info_count(&self) -> Option<&u32> {
        self.portal_info_count.as_ref()
    }

    /// Sets the value of PortalInformation
    pub fn set_portal_information(&mut self, value: Vec<ISCSI_PortalInfo>) {
        self.portal_information = value;
    }

    /// Gets the value of PortalInformation
    pub fn get_portal_information(&self) -> &Vec<ISCSI_PortalInfo> {
        &self.portal_information
    }
}

