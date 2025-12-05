// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutputWhatIf struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutputWhatIf {
    #[serde(flatten)]
    pub base: MSFT_DSCConfigurationOutput,

/// 
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

impl MSFT_DSCConfigurationOutputWhatIf {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DSCConfigurationOutput::new(),
            message: None,
        }
    }


    /// Sets the value of Message
    pub fn set_message(&mut self, value: String) {
        self.message = Some(value);
    }

    /// Gets the value of Message
    pub fn get_message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

