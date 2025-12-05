// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_FlightIds struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_FlightIds {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "FlightIdList")]
    pub flight_id_list: Option<String>,

/// 
    #[serde(rename = "UpdateId")]
    pub update_id: Option<String>,
}

impl SystemConfig_V2_FlightIds {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            flight_id_list: None,
            update_id: None,
        }
    }


    /// Sets the value of FlightIdList
    pub fn set_flight_id_list(&mut self, value: String) {
        self.flight_id_list = Some(value);
    }

    /// Gets the value of FlightIdList
    pub fn get_flight_id_list(&self) -> Option<&String> {
        self.flight_id_list.as_ref()
    }

    /// Sets the value of UpdateId
    pub fn set_update_id(&mut self, value: String) {
        self.update_id = Some(value);
    }

    /// Gets the value of UpdateId
    pub fn get_update_id(&self) -> Option<&String> {
        self.update_id.as_ref()
    }
}

