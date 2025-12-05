// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Power02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Power02 {

/// 
    #[serde(rename = "AllowHibernate")]
    pub allow_hibernate: Option<i32>,

/// 
    #[serde(rename = "AllowStandbyStatesWhenSleepingOnBattery")]
    pub allow_standby_states_when_sleeping_on_battery: Option<String>,

/// 
    #[serde(rename = "AllowStandbyWhenSleepingPluggedIn")]
    pub allow_standby_when_sleeping_plugged_in: Option<String>,

/// 
    #[serde(rename = "DisplayOffTimeoutOnBattery")]
    pub display_off_timeout_on_battery: Option<String>,

/// 
    #[serde(rename = "DisplayOffTimeoutPluggedIn")]
    pub display_off_timeout_plugged_in: Option<String>,

/// 
    #[serde(rename = "EnergySaverBatteryThresholdOnBattery")]
    pub energy_saver_battery_threshold_on_battery: Option<i32>,

/// 
    #[serde(rename = "EnergySaverBatteryThresholdPluggedIn")]
    pub energy_saver_battery_threshold_plugged_in: Option<i32>,

/// 
    #[serde(rename = "HibernateTimeoutOnBattery")]
    pub hibernate_timeout_on_battery: Option<String>,

/// 
    #[serde(rename = "HibernateTimeoutPluggedIn")]
    pub hibernate_timeout_plugged_in: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequirePasswordWhenComputerWakesOnBattery")]
    pub require_password_when_computer_wakes_on_battery: Option<String>,

/// 
    #[serde(rename = "RequirePasswordWhenComputerWakesPluggedIn")]
    pub require_password_when_computer_wakes_plugged_in: Option<String>,

/// 
    #[serde(rename = "SelectLidCloseActionOnBattery")]
    pub select_lid_close_action_on_battery: Option<i32>,

/// 
    #[serde(rename = "SelectLidCloseActionPluggedIn")]
    pub select_lid_close_action_plugged_in: Option<i32>,

/// 
    #[serde(rename = "SelectPowerButtonActionOnBattery")]
    pub select_power_button_action_on_battery: Option<i32>,

/// 
    #[serde(rename = "SelectPowerButtonActionPluggedIn")]
    pub select_power_button_action_plugged_in: Option<i32>,

/// 
    #[serde(rename = "SelectSleepButtonActionOnBattery")]
    pub select_sleep_button_action_on_battery: Option<i32>,

/// 
    #[serde(rename = "SelectSleepButtonActionPluggedIn")]
    pub select_sleep_button_action_plugged_in: Option<i32>,

/// 
    #[serde(rename = "StandbyTimeoutOnBattery")]
    pub standby_timeout_on_battery: Option<String>,

/// 
    #[serde(rename = "StandbyTimeoutPluggedIn")]
    pub standby_timeout_plugged_in: Option<String>,

/// 
    #[serde(rename = "TurnOffHybridSleepOnBattery")]
    pub turn_off_hybrid_sleep_on_battery: Option<i32>,

/// 
    #[serde(rename = "TurnOffHybridSleepPluggedIn")]
    pub turn_off_hybrid_sleep_plugged_in: Option<i32>,

/// 
    #[serde(rename = "UnattendedSleepTimeoutOnBattery")]
    pub unattended_sleep_timeout_on_battery: Option<i32>,

/// 
    #[serde(rename = "UnattendedSleepTimeoutPluggedIn")]
    pub unattended_sleep_timeout_plugged_in: Option<i32>,
}

impl MDM_Policy_Config01_Power02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_hibernate: None,
            allow_standby_states_when_sleeping_on_battery: None,
            allow_standby_when_sleeping_plugged_in: None,
            display_off_timeout_on_battery: None,
            display_off_timeout_plugged_in: None,
            energy_saver_battery_threshold_on_battery: None,
            energy_saver_battery_threshold_plugged_in: None,
            hibernate_timeout_on_battery: None,
            hibernate_timeout_plugged_in: None,
            instance_id: None,
            parent_id: None,
            require_password_when_computer_wakes_on_battery: None,
            require_password_when_computer_wakes_plugged_in: None,
            select_lid_close_action_on_battery: None,
            select_lid_close_action_plugged_in: None,
            select_power_button_action_on_battery: None,
            select_power_button_action_plugged_in: None,
            select_sleep_button_action_on_battery: None,
            select_sleep_button_action_plugged_in: None,
            standby_timeout_on_battery: None,
            standby_timeout_plugged_in: None,
            turn_off_hybrid_sleep_on_battery: None,
            turn_off_hybrid_sleep_plugged_in: None,
            unattended_sleep_timeout_on_battery: None,
            unattended_sleep_timeout_plugged_in: None,
        }
    }


    /// Sets the value of AllowHibernate
    pub fn set_allow_hibernate(&mut self, value: i32) {
        self.allow_hibernate = Some(value);
    }

    /// Gets the value of AllowHibernate
    pub fn get_allow_hibernate(&self) -> Option<&i32> {
        self.allow_hibernate.as_ref()
    }

    /// Sets the value of AllowStandbyStatesWhenSleepingOnBattery
    pub fn set_allow_standby_states_when_sleeping_on_battery(&mut self, value: String) {
        self.allow_standby_states_when_sleeping_on_battery = Some(value);
    }

    /// Gets the value of AllowStandbyStatesWhenSleepingOnBattery
    pub fn get_allow_standby_states_when_sleeping_on_battery(&self) -> Option<&String> {
        self.allow_standby_states_when_sleeping_on_battery.as_ref()
    }

    /// Sets the value of AllowStandbyWhenSleepingPluggedIn
    pub fn set_allow_standby_when_sleeping_plugged_in(&mut self, value: String) {
        self.allow_standby_when_sleeping_plugged_in = Some(value);
    }

    /// Gets the value of AllowStandbyWhenSleepingPluggedIn
    pub fn get_allow_standby_when_sleeping_plugged_in(&self) -> Option<&String> {
        self.allow_standby_when_sleeping_plugged_in.as_ref()
    }

    /// Sets the value of DisplayOffTimeoutOnBattery
    pub fn set_display_off_timeout_on_battery(&mut self, value: String) {
        self.display_off_timeout_on_battery = Some(value);
    }

    /// Gets the value of DisplayOffTimeoutOnBattery
    pub fn get_display_off_timeout_on_battery(&self) -> Option<&String> {
        self.display_off_timeout_on_battery.as_ref()
    }

    /// Sets the value of DisplayOffTimeoutPluggedIn
    pub fn set_display_off_timeout_plugged_in(&mut self, value: String) {
        self.display_off_timeout_plugged_in = Some(value);
    }

    /// Gets the value of DisplayOffTimeoutPluggedIn
    pub fn get_display_off_timeout_plugged_in(&self) -> Option<&String> {
        self.display_off_timeout_plugged_in.as_ref()
    }

    /// Sets the value of EnergySaverBatteryThresholdOnBattery
    pub fn set_energy_saver_battery_threshold_on_battery(&mut self, value: i32) {
        self.energy_saver_battery_threshold_on_battery = Some(value);
    }

    /// Gets the value of EnergySaverBatteryThresholdOnBattery
    pub fn get_energy_saver_battery_threshold_on_battery(&self) -> Option<&i32> {
        self.energy_saver_battery_threshold_on_battery.as_ref()
    }

    /// Sets the value of EnergySaverBatteryThresholdPluggedIn
    pub fn set_energy_saver_battery_threshold_plugged_in(&mut self, value: i32) {
        self.energy_saver_battery_threshold_plugged_in = Some(value);
    }

    /// Gets the value of EnergySaverBatteryThresholdPluggedIn
    pub fn get_energy_saver_battery_threshold_plugged_in(&self) -> Option<&i32> {
        self.energy_saver_battery_threshold_plugged_in.as_ref()
    }

    /// Sets the value of HibernateTimeoutOnBattery
    pub fn set_hibernate_timeout_on_battery(&mut self, value: String) {
        self.hibernate_timeout_on_battery = Some(value);
    }

    /// Gets the value of HibernateTimeoutOnBattery
    pub fn get_hibernate_timeout_on_battery(&self) -> Option<&String> {
        self.hibernate_timeout_on_battery.as_ref()
    }

    /// Sets the value of HibernateTimeoutPluggedIn
    pub fn set_hibernate_timeout_plugged_in(&mut self, value: String) {
        self.hibernate_timeout_plugged_in = Some(value);
    }

    /// Gets the value of HibernateTimeoutPluggedIn
    pub fn get_hibernate_timeout_plugged_in(&self) -> Option<&String> {
        self.hibernate_timeout_plugged_in.as_ref()
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

    /// Sets the value of RequirePasswordWhenComputerWakesOnBattery
    pub fn set_require_password_when_computer_wakes_on_battery(&mut self, value: String) {
        self.require_password_when_computer_wakes_on_battery = Some(value);
    }

    /// Gets the value of RequirePasswordWhenComputerWakesOnBattery
    pub fn get_require_password_when_computer_wakes_on_battery(&self) -> Option<&String> {
        self.require_password_when_computer_wakes_on_battery.as_ref()
    }

    /// Sets the value of RequirePasswordWhenComputerWakesPluggedIn
    pub fn set_require_password_when_computer_wakes_plugged_in(&mut self, value: String) {
        self.require_password_when_computer_wakes_plugged_in = Some(value);
    }

    /// Gets the value of RequirePasswordWhenComputerWakesPluggedIn
    pub fn get_require_password_when_computer_wakes_plugged_in(&self) -> Option<&String> {
        self.require_password_when_computer_wakes_plugged_in.as_ref()
    }

    /// Sets the value of SelectLidCloseActionOnBattery
    pub fn set_select_lid_close_action_on_battery(&mut self, value: i32) {
        self.select_lid_close_action_on_battery = Some(value);
    }

    /// Gets the value of SelectLidCloseActionOnBattery
    pub fn get_select_lid_close_action_on_battery(&self) -> Option<&i32> {
        self.select_lid_close_action_on_battery.as_ref()
    }

    /// Sets the value of SelectLidCloseActionPluggedIn
    pub fn set_select_lid_close_action_plugged_in(&mut self, value: i32) {
        self.select_lid_close_action_plugged_in = Some(value);
    }

    /// Gets the value of SelectLidCloseActionPluggedIn
    pub fn get_select_lid_close_action_plugged_in(&self) -> Option<&i32> {
        self.select_lid_close_action_plugged_in.as_ref()
    }

    /// Sets the value of SelectPowerButtonActionOnBattery
    pub fn set_select_power_button_action_on_battery(&mut self, value: i32) {
        self.select_power_button_action_on_battery = Some(value);
    }

    /// Gets the value of SelectPowerButtonActionOnBattery
    pub fn get_select_power_button_action_on_battery(&self) -> Option<&i32> {
        self.select_power_button_action_on_battery.as_ref()
    }

    /// Sets the value of SelectPowerButtonActionPluggedIn
    pub fn set_select_power_button_action_plugged_in(&mut self, value: i32) {
        self.select_power_button_action_plugged_in = Some(value);
    }

    /// Gets the value of SelectPowerButtonActionPluggedIn
    pub fn get_select_power_button_action_plugged_in(&self) -> Option<&i32> {
        self.select_power_button_action_plugged_in.as_ref()
    }

    /// Sets the value of SelectSleepButtonActionOnBattery
    pub fn set_select_sleep_button_action_on_battery(&mut self, value: i32) {
        self.select_sleep_button_action_on_battery = Some(value);
    }

    /// Gets the value of SelectSleepButtonActionOnBattery
    pub fn get_select_sleep_button_action_on_battery(&self) -> Option<&i32> {
        self.select_sleep_button_action_on_battery.as_ref()
    }

    /// Sets the value of SelectSleepButtonActionPluggedIn
    pub fn set_select_sleep_button_action_plugged_in(&mut self, value: i32) {
        self.select_sleep_button_action_plugged_in = Some(value);
    }

    /// Gets the value of SelectSleepButtonActionPluggedIn
    pub fn get_select_sleep_button_action_plugged_in(&self) -> Option<&i32> {
        self.select_sleep_button_action_plugged_in.as_ref()
    }

    /// Sets the value of StandbyTimeoutOnBattery
    pub fn set_standby_timeout_on_battery(&mut self, value: String) {
        self.standby_timeout_on_battery = Some(value);
    }

    /// Gets the value of StandbyTimeoutOnBattery
    pub fn get_standby_timeout_on_battery(&self) -> Option<&String> {
        self.standby_timeout_on_battery.as_ref()
    }

    /// Sets the value of StandbyTimeoutPluggedIn
    pub fn set_standby_timeout_plugged_in(&mut self, value: String) {
        self.standby_timeout_plugged_in = Some(value);
    }

    /// Gets the value of StandbyTimeoutPluggedIn
    pub fn get_standby_timeout_plugged_in(&self) -> Option<&String> {
        self.standby_timeout_plugged_in.as_ref()
    }

    /// Sets the value of TurnOffHybridSleepOnBattery
    pub fn set_turn_off_hybrid_sleep_on_battery(&mut self, value: i32) {
        self.turn_off_hybrid_sleep_on_battery = Some(value);
    }

    /// Gets the value of TurnOffHybridSleepOnBattery
    pub fn get_turn_off_hybrid_sleep_on_battery(&self) -> Option<&i32> {
        self.turn_off_hybrid_sleep_on_battery.as_ref()
    }

    /// Sets the value of TurnOffHybridSleepPluggedIn
    pub fn set_turn_off_hybrid_sleep_plugged_in(&mut self, value: i32) {
        self.turn_off_hybrid_sleep_plugged_in = Some(value);
    }

    /// Gets the value of TurnOffHybridSleepPluggedIn
    pub fn get_turn_off_hybrid_sleep_plugged_in(&self) -> Option<&i32> {
        self.turn_off_hybrid_sleep_plugged_in.as_ref()
    }

    /// Sets the value of UnattendedSleepTimeoutOnBattery
    pub fn set_unattended_sleep_timeout_on_battery(&mut self, value: i32) {
        self.unattended_sleep_timeout_on_battery = Some(value);
    }

    /// Gets the value of UnattendedSleepTimeoutOnBattery
    pub fn get_unattended_sleep_timeout_on_battery(&self) -> Option<&i32> {
        self.unattended_sleep_timeout_on_battery.as_ref()
    }

    /// Sets the value of UnattendedSleepTimeoutPluggedIn
    pub fn set_unattended_sleep_timeout_plugged_in(&mut self, value: i32) {
        self.unattended_sleep_timeout_plugged_in = Some(value);
    }

    /// Gets the value of UnattendedSleepTimeoutPluggedIn
    pub fn get_unattended_sleep_timeout_plugged_in(&self) -> Option<&i32> {
        self.unattended_sleep_timeout_plugged_in.as_ref()
    }
}

