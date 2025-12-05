// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskPrincipal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskPrincipal {

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "LogonType")]
    pub logon_type: Option<TaskPrincipal_LogonType>,

/// 
    #[serde(rename = "RunLevel")]
    pub run_level: Option<i32>,

/// 
    #[serde(rename = "UserId")]
    pub user_id: Option<String>,
}

impl MSFT_TaskPrincipal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            display_name: None,
            group_id: None,
            id: None,
            logon_type: None,
            run_level: None,
            user_id: None,
        }
    }


    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of GroupId
    pub fn set_group_id(&mut self, value: String) {
        self.group_id = Some(value);
    }

    /// Gets the value of GroupId
    pub fn get_group_id(&self) -> Option<&String> {
        self.group_id.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of LogonType
    pub fn set_logon_type(&mut self, value: TaskPrincipal_LogonType) {
        self.logon_type = Some(value);
    }

    /// Gets the value of LogonType
    pub fn get_logon_type(&self) -> Option<&TaskPrincipal_LogonType> {
        self.logon_type.as_ref()
    }

    /// Sets the value of RunLevel
    pub fn set_run_level(&mut self, value: i32) {
        self.run_level = Some(value);
    }

    /// Gets the value of RunLevel
    pub fn get_run_level(&self) -> Option<&i32> {
        self.run_level.as_ref()
    }

    /// Sets the value of UserId
    pub fn set_user_id(&mut self, value: String) {
        self.user_id = Some(value);
    }

    /// Gets the value of UserId
    pub fn get_user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }
}

