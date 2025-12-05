// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ScheduledTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ScheduledTask {

/// 
    #[serde(rename = "Actions")]
    pub actions: Vec<MSFT_TaskAction>,

/// 
    #[serde(rename = "Author")]
    pub author: Option<String>,

/// 
    #[serde(rename = "Date")]
    pub date: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Documentation")]
    pub documentation: Option<String>,

/// 
    #[serde(rename = "Principal")]
    pub principal: Option<MSFT_TaskPrincipal>,

/// 
    #[serde(rename = "SecurityDescriptor")]
    pub security_descriptor: Option<String>,

/// 
    #[serde(rename = "Settings")]
    pub settings: Option<MSFT_TaskSettings>,

/// 
    #[serde(rename = "Source")]
    pub source: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<ScheduledTask_State>,

/// 
    #[serde(rename = "TaskName")]
    pub task_name: Option<String>,

/// 
    #[serde(rename = "TaskPath")]
    pub task_path: Option<String>,

/// 
    #[serde(rename = "Triggers")]
    pub triggers: Vec<MSFT_TaskTrigger>,

/// 
    #[serde(rename = "URI")]
    pub uri: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MSFT_ScheduledTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            author: None,
            date: None,
            description: None,
            documentation: None,
            principal: None,
            security_descriptor: None,
            settings: None,
            source: None,
            state: None,
            task_name: None,
            task_path: None,
            triggers: Vec::new(),
            uri: None,
            version: None,
        }
    }


    /// Sets the value of Actions
    pub fn set_actions(&mut self, value: Vec<MSFT_TaskAction>) {
        self.actions = value;
    }

    /// Gets the value of Actions
    pub fn get_actions(&self) -> &Vec<MSFT_TaskAction> {
        &self.actions
    }

    /// Sets the value of Author
    pub fn set_author(&mut self, value: String) {
        self.author = Some(value);
    }

    /// Gets the value of Author
    pub fn get_author(&self) -> Option<&String> {
        self.author.as_ref()
    }

    /// Sets the value of Date
    pub fn set_date(&mut self, value: String) {
        self.date = Some(value);
    }

    /// Gets the value of Date
    pub fn get_date(&self) -> Option<&String> {
        self.date.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Documentation
    pub fn set_documentation(&mut self, value: String) {
        self.documentation = Some(value);
    }

    /// Gets the value of Documentation
    pub fn get_documentation(&self) -> Option<&String> {
        self.documentation.as_ref()
    }

    /// Sets the value of Principal
    pub fn set_principal(&mut self, value: MSFT_TaskPrincipal) {
        self.principal = Some(value);
    }

    /// Gets the value of Principal
    pub fn get_principal(&self) -> Option<&MSFT_TaskPrincipal> {
        self.principal.as_ref()
    }

    /// Sets the value of SecurityDescriptor
    pub fn set_security_descriptor(&mut self, value: String) {
        self.security_descriptor = Some(value);
    }

    /// Gets the value of SecurityDescriptor
    pub fn get_security_descriptor(&self) -> Option<&String> {
        self.security_descriptor.as_ref()
    }

    /// Sets the value of Settings
    pub fn set_settings(&mut self, value: MSFT_TaskSettings) {
        self.settings = Some(value);
    }

    /// Gets the value of Settings
    pub fn get_settings(&self) -> Option<&MSFT_TaskSettings> {
        self.settings.as_ref()
    }

    /// Sets the value of Source
    pub fn set_source(&mut self, value: String) {
        self.source = Some(value);
    }

    /// Gets the value of Source
    pub fn get_source(&self) -> Option<&String> {
        self.source.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: ScheduledTask_State) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&ScheduledTask_State> {
        self.state.as_ref()
    }

    /// Sets the value of TaskName
    pub fn set_task_name(&mut self, value: String) {
        self.task_name = Some(value);
    }

    /// Gets the value of TaskName
    pub fn get_task_name(&self) -> Option<&String> {
        self.task_name.as_ref()
    }

    /// Sets the value of TaskPath
    pub fn set_task_path(&mut self, value: String) {
        self.task_path = Some(value);
    }

    /// Gets the value of TaskPath
    pub fn get_task_path(&self) -> Option<&String> {
        self.task_path.as_ref()
    }

    /// Sets the value of Triggers
    pub fn set_triggers(&mut self, value: Vec<MSFT_TaskTrigger>) {
        self.triggers = value;
    }

    /// Gets the value of Triggers
    pub fn get_triggers(&self) -> &Vec<MSFT_TaskTrigger> {
        &self.triggers
    }

    /// Sets the value of URI
    pub fn set_uri(&mut self, value: String) {
        self.uri = Some(value);
    }

    /// Gets the value of URI
    pub fn get_uri(&self) -> Option<&String> {
        self.uri.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

