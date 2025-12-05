// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_Settings02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_Settings02 {

/// 
    #[serde(rename = "ConfigureTaskbarCalendar")]
    pub configure_taskbar_calendar: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "PageVisibilityList")]
    pub page_visibility_list: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_User_Config01_Settings02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configure_taskbar_calendar: None,
            instance_id: None,
            page_visibility_list: None,
            parent_id: None,
        }
    }


    /// Sets the value of ConfigureTaskbarCalendar
    pub fn set_configure_taskbar_calendar(&mut self, value: i32) {
        self.configure_taskbar_calendar = Some(value);
    }

    /// Gets the value of ConfigureTaskbarCalendar
    pub fn get_configure_taskbar_calendar(&self) -> Option<&i32> {
        self.configure_taskbar_calendar.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of PageVisibilityList
    pub fn set_page_visibility_list(&mut self, value: String) {
        self.page_visibility_list = Some(value);
    }

    /// Gets the value of PageVisibilityList
    pub fn get_page_visibility_list(&self) -> Option<&String> {
        self.page_visibility_list.as_ref()
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

