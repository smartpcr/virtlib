// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalFrame struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalFrame {
    #[serde(flatten)]
    pub base: CIM_PhysicalPackage,

/// 
    #[serde(rename = "AudibleAlarm")]
    pub audible_alarm: Option<bool>,

/// 
    #[serde(rename = "BreachDescription")]
    pub breach_description: Option<String>,

/// 
    #[serde(rename = "CableManagementStrategy")]
    pub cable_management_strategy: Option<String>,

/// 
    #[serde(rename = "LockPresent")]
    pub lock_present: Option<bool>,

/// 
    #[serde(rename = "SecurityBreach")]
    pub security_breach: Option<u16>,

/// 
    #[serde(rename = "ServiceDescriptions")]
    pub service_descriptions: Vec<String>,

/// 
    #[serde(rename = "ServicePhilosophy")]
    pub service_philosophy: Vec<u16>,

/// 
    #[serde(rename = "VisibleAlarm")]
    pub visible_alarm: Option<bool>,
}

impl CIM_PhysicalFrame {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalPackage::new(),
            audible_alarm: None,
            breach_description: None,
            cable_management_strategy: None,
            lock_present: None,
            security_breach: None,
            service_descriptions: Vec::new(),
            service_philosophy: Vec::new(),
            visible_alarm: None,
        }
    }


    /// Sets the value of AudibleAlarm
    pub fn set_audible_alarm(&mut self, value: bool) {
        self.audible_alarm = Some(value);
    }

    /// Gets the value of AudibleAlarm
    pub fn get_audible_alarm(&self) -> Option<&bool> {
        self.audible_alarm.as_ref()
    }

    /// Sets the value of BreachDescription
    pub fn set_breach_description(&mut self, value: String) {
        self.breach_description = Some(value);
    }

    /// Gets the value of BreachDescription
    pub fn get_breach_description(&self) -> Option<&String> {
        self.breach_description.as_ref()
    }

    /// Sets the value of CableManagementStrategy
    pub fn set_cable_management_strategy(&mut self, value: String) {
        self.cable_management_strategy = Some(value);
    }

    /// Gets the value of CableManagementStrategy
    pub fn get_cable_management_strategy(&self) -> Option<&String> {
        self.cable_management_strategy.as_ref()
    }

    /// Sets the value of LockPresent
    pub fn set_lock_present(&mut self, value: bool) {
        self.lock_present = Some(value);
    }

    /// Gets the value of LockPresent
    pub fn get_lock_present(&self) -> Option<&bool> {
        self.lock_present.as_ref()
    }

    /// Sets the value of SecurityBreach
    pub fn set_security_breach(&mut self, value: u16) {
        self.security_breach = Some(value);
    }

    /// Gets the value of SecurityBreach
    pub fn get_security_breach(&self) -> Option<&u16> {
        self.security_breach.as_ref()
    }

    /// Sets the value of ServiceDescriptions
    pub fn set_service_descriptions(&mut self, value: Vec<String>) {
        self.service_descriptions = value;
    }

    /// Gets the value of ServiceDescriptions
    pub fn get_service_descriptions(&self) -> &Vec<String> {
        &self.service_descriptions
    }

    /// Sets the value of ServicePhilosophy
    pub fn set_service_philosophy(&mut self, value: Vec<u16>) {
        self.service_philosophy = value;
    }

    /// Gets the value of ServicePhilosophy
    pub fn get_service_philosophy(&self) -> &Vec<u16> {
        &self.service_philosophy
    }

    /// Sets the value of VisibleAlarm
    pub fn set_visible_alarm(&mut self, value: bool) {
        self.visible_alarm = Some(value);
    }

    /// Gets the value of VisibleAlarm
    pub fn get_visible_alarm(&self) -> Option<&bool> {
        self.visible_alarm.as_ref()
    }
}

