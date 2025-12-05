// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorColorXYZinCIE struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorColorXYZinCIE {

/// 
    #[serde(rename = "X")]
    pub x: Option<u16>,

/// 
    #[serde(rename = "Y")]
    pub y: Option<u16>,
}

impl WmiMonitorColorXYZinCIE {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
        }
    }


    /// Sets the value of X
    pub fn set_x(&mut self, value: u16) {
        self.x = Some(value);
    }

    /// Gets the value of X
    pub fn get_x(&self) -> Option<&u16> {
        self.x.as_ref()
    }

    /// Sets the value of Y
    pub fn set_y(&mut self, value: u16) {
        self.y = Some(value);
    }

    /// Gets the value of Y
    pub fn get_y(&self) -> Option<&u16> {
        self.y.as_ref()
    }
}

