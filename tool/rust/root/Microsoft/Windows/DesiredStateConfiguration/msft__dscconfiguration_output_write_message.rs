// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutputWriteMessage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutputWriteMessage {
    #[serde(flatten)]
    pub base: MSFT_DSCConfigurationOutput,

/// 
    #[serde(rename = "Channel")]
    pub channel: Option<u32>,

/// 
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

impl MSFT_DSCConfigurationOutputWriteMessage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DSCConfigurationOutput::new(),
            channel: None,
            message: None,
        }
    }


    /// Sets the value of Channel
    pub fn set_channel(&mut self, value: u32) {
        self.channel = Some(value);
    }

    /// Gets the value of Channel
    pub fn get_channel(&self) -> Option<&u32> {
        self.channel.as_ref()
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

