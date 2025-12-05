// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AlarmDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AlarmDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "AudibleAlarm")]
    pub audible_alarm: Option<bool>,

/// 
    #[serde(rename = "Urgency")]
    pub urgency: Option<u16>,

/// 
    #[serde(rename = "VisibleAlarm")]
    pub visible_alarm: Option<bool>,
}

impl CIM_AlarmDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            audible_alarm: None,
            urgency: None,
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

    /// Sets the value of Urgency
    pub fn set_urgency(&mut self, value: u16) {
        self.urgency = Some(value);
    }

    /// Gets the value of Urgency
    pub fn get_urgency(&self) -> Option<&u16> {
        self.urgency.as_ref()
    }

    /// Sets the value of VisibleAlarm
    pub fn set_visible_alarm(&mut self, value: bool) {
        self.visible_alarm = Some(value);
    }

    /// Gets the value of VisibleAlarm
    pub fn get_visible_alarm(&self) -> Option<&bool> {
        self.visible_alarm.as_ref()
    }

/// 

    /// * `requested_urgency` -  (u16)

    /// * `return_value` -  (u32)
    pub fn set_urgency(&self, requested_urgency: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedUrgency".to_string(), value: requested_urgency.into() });
        self.invoke_method("SetUrgency", &args)

    }

}

