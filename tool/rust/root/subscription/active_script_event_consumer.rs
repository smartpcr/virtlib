// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.subscription
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ActiveScriptEventConsumer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActiveScriptEventConsumer {
    #[serde(flatten)]
    pub base: __EventConsumer,

/// 
    #[serde(rename = "KillTimeout")]
    pub kill_timeout: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ScriptFilename")]
    pub script_filename: Option<String>,

/// 
    #[serde(rename = "ScriptingEngine")]
    pub scripting_engine: Option<String>,

/// 
    #[serde(rename = "ScriptText")]
    pub script_text: Option<String>,
}

impl ActiveScriptEventConsumer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventConsumer::new(),
            kill_timeout: None,
            name: None,
            script_filename: None,
            scripting_engine: None,
            script_text: None,
        }
    }


    /// Sets the value of KillTimeout
    pub fn set_kill_timeout(&mut self, value: u32) {
        self.kill_timeout = Some(value);
    }

    /// Gets the value of KillTimeout
    pub fn get_kill_timeout(&self) -> Option<&u32> {
        self.kill_timeout.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ScriptFilename
    pub fn set_script_filename(&mut self, value: String) {
        self.script_filename = Some(value);
    }

    /// Gets the value of ScriptFilename
    pub fn get_script_filename(&self) -> Option<&String> {
        self.script_filename.as_ref()
    }

    /// Sets the value of ScriptingEngine
    pub fn set_scripting_engine(&mut self, value: String) {
        self.scripting_engine = Some(value);
    }

    /// Gets the value of ScriptingEngine
    pub fn get_scripting_engine(&self) -> Option<&String> {
        self.scripting_engine.as_ref()
    }

    /// Sets the value of ScriptText
    pub fn set_script_text(&mut self, value: String) {
        self.script_text = Some(value);
    }

    /// Gets the value of ScriptText
    pub fn get_script_text(&self) -> Option<&String> {
        self.script_text.as_ref()
    }
}

