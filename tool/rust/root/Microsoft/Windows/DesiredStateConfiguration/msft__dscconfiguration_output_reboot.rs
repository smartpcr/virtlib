// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutputReboot struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutputReboot {
    #[serde(flatten)]
    pub base: MSFT_DSCConfigurationOutput,

/// 
    #[serde(rename = "Automatic")]
    pub automatic: Option<bool>,
}

impl MSFT_DSCConfigurationOutputReboot {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DSCConfigurationOutput::new(),
            automatic: None,
        }
    }


    /// Sets the value of Automatic
    pub fn set_automatic(&mut self, value: bool) {
        self.automatic = Some(value);
    }

    /// Gets the value of Automatic
    pub fn get_automatic(&self) -> Option<&bool> {
        self.automatic.as_ref()
    }
}

