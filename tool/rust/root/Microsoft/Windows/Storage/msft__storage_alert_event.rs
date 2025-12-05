// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageAlertEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageAlertEvent {
    #[serde(flatten)]
    pub base: MSFT_StorageEvent,

/// 
    #[serde(rename = "AlertType")]
    pub alert_type: Option<u16>,
}

impl MSFT_StorageAlertEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageEvent::new(),
            alert_type: None,
        }
    }


    /// Sets the value of AlertType
    pub fn set_alert_type(&mut self, value: u16) {
        self.alert_type = Some(value);
    }

    /// Gets the value of AlertType
    pub fn get_alert_type(&self) -> Option<&u16> {
        self.alert_type.as_ref()
    }
}

