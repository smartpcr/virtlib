// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Result01_AttachmentManager02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Result01_AttachmentManager02 {

/// 
    #[serde(rename = "DoNotPreserveZoneInformation")]
    pub do_not_preserve_zone_information: Option<String>,

/// 
    #[serde(rename = "HideZoneInfoMechanism")]
    pub hide_zone_info_mechanism: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "NotifyAntivirusPrograms")]
    pub notify_antivirus_programs: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_User_Result01_AttachmentManager02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            do_not_preserve_zone_information: None,
            hide_zone_info_mechanism: None,
            instance_id: None,
            notify_antivirus_programs: None,
            parent_id: None,
        }
    }


    /// Sets the value of DoNotPreserveZoneInformation
    pub fn set_do_not_preserve_zone_information(&mut self, value: String) {
        self.do_not_preserve_zone_information = Some(value);
    }

    /// Gets the value of DoNotPreserveZoneInformation
    pub fn get_do_not_preserve_zone_information(&self) -> Option<&String> {
        self.do_not_preserve_zone_information.as_ref()
    }

    /// Sets the value of HideZoneInfoMechanism
    pub fn set_hide_zone_info_mechanism(&mut self, value: String) {
        self.hide_zone_info_mechanism = Some(value);
    }

    /// Gets the value of HideZoneInfoMechanism
    pub fn get_hide_zone_info_mechanism(&self) -> Option<&String> {
        self.hide_zone_info_mechanism.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of NotifyAntivirusPrograms
    pub fn set_notify_antivirus_programs(&mut self, value: String) {
        self.notify_antivirus_programs = Some(value);
    }

    /// Gets the value of NotifyAntivirusPrograms
    pub fn get_notify_antivirus_programs(&self) -> Option<&String> {
        self.notify_antivirus_programs.as_ref()
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

