// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DebugPrint_Event struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebugPrint_Event {
    #[serde(flatten)]
    pub base: Debugger,

/// 
    #[serde(rename = "Component")]
    pub component: Option<u32>,

/// 
    #[serde(rename = "Level")]
    pub level: Option<u32>,

/// 
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

impl DebugPrint_Event {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Debugger::new(),
            component: None,
            level: None,
            message: None,
        }
    }


    /// Sets the value of Component
    pub fn set_component(&mut self, value: u32) {
        self.component = Some(value);
    }

    /// Gets the value of Component
    pub fn get_component(&self) -> Option<&u32> {
        self.component.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: u32) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&u32> {
        self.level.as_ref()
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

