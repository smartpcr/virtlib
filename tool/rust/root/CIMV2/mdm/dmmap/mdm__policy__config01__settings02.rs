// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Settings02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Settings02 {

/// 
    #[serde(rename = "AllowAutoPlay")]
    pub allow_auto_play: Option<i32>,

/// 
    #[serde(rename = "AllowDataSense")]
    pub allow_data_sense: Option<i32>,

/// 
    #[serde(rename = "AllowDateTime")]
    pub allow_date_time: Option<i32>,

/// 
    #[serde(rename = "AllowEditDeviceName")]
    pub allow_edit_device_name: Option<i32>,

/// 
    #[serde(rename = "AllowLanguage")]
    pub allow_language: Option<i32>,

/// 
    #[serde(rename = "AllowOnlineTips")]
    pub allow_online_tips: Option<i32>,

/// 
    #[serde(rename = "AllowPowerSleep")]
    pub allow_power_sleep: Option<i32>,

/// 
    #[serde(rename = "AllowRegion")]
    pub allow_region: Option<i32>,

/// 
    #[serde(rename = "AllowSignInOptions")]
    pub allow_sign_in_options: Option<i32>,

/// 
    #[serde(rename = "AllowVPN")]
    pub allow_vpn: Option<i32>,

/// 
    #[serde(rename = "AllowWorkplace")]
    pub allow_workplace: Option<i32>,

/// 
    #[serde(rename = "AllowYourAccount")]
    pub allow_your_account: Option<i32>,

/// 
    #[serde(rename = "EnableBatteryCapacity")]
    pub enable_battery_capacity: Option<i32>,

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

impl MDM_Policy_Config01_Settings02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_auto_play: None,
            allow_data_sense: None,
            allow_date_time: None,
            allow_edit_device_name: None,
            allow_language: None,
            allow_online_tips: None,
            allow_power_sleep: None,
            allow_region: None,
            allow_sign_in_options: None,
            allow_vpn: None,
            allow_workplace: None,
            allow_your_account: None,
            enable_battery_capacity: None,
            instance_id: None,
            page_visibility_list: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowAutoPlay
    pub fn set_allow_auto_play(&mut self, value: i32) {
        self.allow_auto_play = Some(value);
    }

    /// Gets the value of AllowAutoPlay
    pub fn get_allow_auto_play(&self) -> Option<&i32> {
        self.allow_auto_play.as_ref()
    }

    /// Sets the value of AllowDataSense
    pub fn set_allow_data_sense(&mut self, value: i32) {
        self.allow_data_sense = Some(value);
    }

    /// Gets the value of AllowDataSense
    pub fn get_allow_data_sense(&self) -> Option<&i32> {
        self.allow_data_sense.as_ref()
    }

    /// Sets the value of AllowDateTime
    pub fn set_allow_date_time(&mut self, value: i32) {
        self.allow_date_time = Some(value);
    }

    /// Gets the value of AllowDateTime
    pub fn get_allow_date_time(&self) -> Option<&i32> {
        self.allow_date_time.as_ref()
    }

    /// Sets the value of AllowEditDeviceName
    pub fn set_allow_edit_device_name(&mut self, value: i32) {
        self.allow_edit_device_name = Some(value);
    }

    /// Gets the value of AllowEditDeviceName
    pub fn get_allow_edit_device_name(&self) -> Option<&i32> {
        self.allow_edit_device_name.as_ref()
    }

    /// Sets the value of AllowLanguage
    pub fn set_allow_language(&mut self, value: i32) {
        self.allow_language = Some(value);
    }

    /// Gets the value of AllowLanguage
    pub fn get_allow_language(&self) -> Option<&i32> {
        self.allow_language.as_ref()
    }

    /// Sets the value of AllowOnlineTips
    pub fn set_allow_online_tips(&mut self, value: i32) {
        self.allow_online_tips = Some(value);
    }

    /// Gets the value of AllowOnlineTips
    pub fn get_allow_online_tips(&self) -> Option<&i32> {
        self.allow_online_tips.as_ref()
    }

    /// Sets the value of AllowPowerSleep
    pub fn set_allow_power_sleep(&mut self, value: i32) {
        self.allow_power_sleep = Some(value);
    }

    /// Gets the value of AllowPowerSleep
    pub fn get_allow_power_sleep(&self) -> Option<&i32> {
        self.allow_power_sleep.as_ref()
    }

    /// Sets the value of AllowRegion
    pub fn set_allow_region(&mut self, value: i32) {
        self.allow_region = Some(value);
    }

    /// Gets the value of AllowRegion
    pub fn get_allow_region(&self) -> Option<&i32> {
        self.allow_region.as_ref()
    }

    /// Sets the value of AllowSignInOptions
    pub fn set_allow_sign_in_options(&mut self, value: i32) {
        self.allow_sign_in_options = Some(value);
    }

    /// Gets the value of AllowSignInOptions
    pub fn get_allow_sign_in_options(&self) -> Option<&i32> {
        self.allow_sign_in_options.as_ref()
    }

    /// Sets the value of AllowVPN
    pub fn set_allow_vpn(&mut self, value: i32) {
        self.allow_vpn = Some(value);
    }

    /// Gets the value of AllowVPN
    pub fn get_allow_vpn(&self) -> Option<&i32> {
        self.allow_vpn.as_ref()
    }

    /// Sets the value of AllowWorkplace
    pub fn set_allow_workplace(&mut self, value: i32) {
        self.allow_workplace = Some(value);
    }

    /// Gets the value of AllowWorkplace
    pub fn get_allow_workplace(&self) -> Option<&i32> {
        self.allow_workplace.as_ref()
    }

    /// Sets the value of AllowYourAccount
    pub fn set_allow_your_account(&mut self, value: i32) {
        self.allow_your_account = Some(value);
    }

    /// Gets the value of AllowYourAccount
    pub fn get_allow_your_account(&self) -> Option<&i32> {
        self.allow_your_account.as_ref()
    }

    /// Sets the value of EnableBatteryCapacity
    pub fn set_enable_battery_capacity(&mut self, value: i32) {
        self.enable_battery_capacity = Some(value);
    }

    /// Gets the value of EnableBatteryCapacity
    pub fn get_enable_battery_capacity(&self) -> Option<&i32> {
        self.enable_battery_capacity.as_ref()
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

