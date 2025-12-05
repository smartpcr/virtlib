// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_Start02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_Start02 {

/// 
    #[serde(rename = "DisableContextMenus")]
    pub disable_context_menus: Option<i32>,

/// 
    #[serde(rename = "ForceStartSize")]
    pub force_start_size: Option<i32>,

/// 
    #[serde(rename = "HideAppList")]
    pub hide_app_list: Option<i32>,

/// 
    #[serde(rename = "HideFrequentlyUsedApps")]
    pub hide_frequently_used_apps: Option<i32>,

/// 
    #[serde(rename = "HidePeopleBar")]
    pub hide_people_bar: Option<i32>,

/// 
    #[serde(rename = "HideRecentJumplists")]
    pub hide_recent_jumplists: Option<i32>,

/// 
    #[serde(rename = "HideRecentlyAddedApps")]
    pub hide_recently_added_apps: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ShowOrHideMostUsedApps")]
    pub show_or_hide_most_used_apps: Option<i32>,

/// 
    #[serde(rename = "StartLayout")]
    pub start_layout: Option<String>,
}

impl MDM_Policy_User_Config01_Start02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disable_context_menus: None,
            force_start_size: None,
            hide_app_list: None,
            hide_frequently_used_apps: None,
            hide_people_bar: None,
            hide_recent_jumplists: None,
            hide_recently_added_apps: None,
            instance_id: None,
            parent_id: None,
            show_or_hide_most_used_apps: None,
            start_layout: None,
        }
    }


    /// Sets the value of DisableContextMenus
    pub fn set_disable_context_menus(&mut self, value: i32) {
        self.disable_context_menus = Some(value);
    }

    /// Gets the value of DisableContextMenus
    pub fn get_disable_context_menus(&self) -> Option<&i32> {
        self.disable_context_menus.as_ref()
    }

    /// Sets the value of ForceStartSize
    pub fn set_force_start_size(&mut self, value: i32) {
        self.force_start_size = Some(value);
    }

    /// Gets the value of ForceStartSize
    pub fn get_force_start_size(&self) -> Option<&i32> {
        self.force_start_size.as_ref()
    }

    /// Sets the value of HideAppList
    pub fn set_hide_app_list(&mut self, value: i32) {
        self.hide_app_list = Some(value);
    }

    /// Gets the value of HideAppList
    pub fn get_hide_app_list(&self) -> Option<&i32> {
        self.hide_app_list.as_ref()
    }

    /// Sets the value of HideFrequentlyUsedApps
    pub fn set_hide_frequently_used_apps(&mut self, value: i32) {
        self.hide_frequently_used_apps = Some(value);
    }

    /// Gets the value of HideFrequentlyUsedApps
    pub fn get_hide_frequently_used_apps(&self) -> Option<&i32> {
        self.hide_frequently_used_apps.as_ref()
    }

    /// Sets the value of HidePeopleBar
    pub fn set_hide_people_bar(&mut self, value: i32) {
        self.hide_people_bar = Some(value);
    }

    /// Gets the value of HidePeopleBar
    pub fn get_hide_people_bar(&self) -> Option<&i32> {
        self.hide_people_bar.as_ref()
    }

    /// Sets the value of HideRecentJumplists
    pub fn set_hide_recent_jumplists(&mut self, value: i32) {
        self.hide_recent_jumplists = Some(value);
    }

    /// Gets the value of HideRecentJumplists
    pub fn get_hide_recent_jumplists(&self) -> Option<&i32> {
        self.hide_recent_jumplists.as_ref()
    }

    /// Sets the value of HideRecentlyAddedApps
    pub fn set_hide_recently_added_apps(&mut self, value: i32) {
        self.hide_recently_added_apps = Some(value);
    }

    /// Gets the value of HideRecentlyAddedApps
    pub fn get_hide_recently_added_apps(&self) -> Option<&i32> {
        self.hide_recently_added_apps.as_ref()
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

    /// Sets the value of ShowOrHideMostUsedApps
    pub fn set_show_or_hide_most_used_apps(&mut self, value: i32) {
        self.show_or_hide_most_used_apps = Some(value);
    }

    /// Gets the value of ShowOrHideMostUsedApps
    pub fn get_show_or_hide_most_used_apps(&self) -> Option<&i32> {
        self.show_or_hide_most_used_apps.as_ref()
    }

    /// Sets the value of StartLayout
    pub fn set_start_layout(&mut self, value: String) {
        self.start_layout = Some(value);
    }

    /// Gets the value of StartLayout
    pub fn get_start_layout(&self) -> Option<&String> {
        self.start_layout.as_ref()
    }
}

