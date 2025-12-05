// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorColorCharacteristics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorColorCharacteristics {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Blue")]
    pub blue: Option<WmiMonitorColorXYZinCIE>,

/// 
    #[serde(rename = "DefaultWhite")]
    pub default_white: Option<WmiMonitorColorXYZinCIE>,

/// 
    #[serde(rename = "Green")]
    pub green: Option<WmiMonitorColorXYZinCIE>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Red")]
    pub red: Option<WmiMonitorColorXYZinCIE>,
}

impl WmiMonitorColorCharacteristics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            blue: None,
            default_white: None,
            green: None,
            instance_name: None,
            red: None,
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

    /// Sets the value of Blue
    pub fn set_blue(&mut self, value: WmiMonitorColorXYZinCIE) {
        self.blue = Some(value);
    }

    /// Gets the value of Blue
    pub fn get_blue(&self) -> Option<&WmiMonitorColorXYZinCIE> {
        self.blue.as_ref()
    }

    /// Sets the value of DefaultWhite
    pub fn set_default_white(&mut self, value: WmiMonitorColorXYZinCIE) {
        self.default_white = Some(value);
    }

    /// Gets the value of DefaultWhite
    pub fn get_default_white(&self) -> Option<&WmiMonitorColorXYZinCIE> {
        self.default_white.as_ref()
    }

    /// Sets the value of Green
    pub fn set_green(&mut self, value: WmiMonitorColorXYZinCIE) {
        self.green = Some(value);
    }

    /// Gets the value of Green
    pub fn get_green(&self) -> Option<&WmiMonitorColorXYZinCIE> {
        self.green.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Red
    pub fn set_red(&mut self, value: WmiMonitorColorXYZinCIE) {
        self.red = Some(value);
    }

    /// Gets the value of Red
    pub fn get_red(&self) -> Option<&WmiMonitorColorXYZinCIE> {
        self.red.as_ref()
    }
}

