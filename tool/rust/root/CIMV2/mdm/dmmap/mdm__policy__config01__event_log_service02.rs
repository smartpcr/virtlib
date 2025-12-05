// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_EventLogService02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_EventLogService02 {

/// 
    #[serde(rename = "ControlEventLogBehavior")]
    pub control_event_log_behavior: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SpecifyMaximumFileSizeApplicationLog")]
    pub specify_maximum_file_size_application_log: Option<String>,

/// 
    #[serde(rename = "SpecifyMaximumFileSizeSecurityLog")]
    pub specify_maximum_file_size_security_log: Option<String>,

/// 
    #[serde(rename = "SpecifyMaximumFileSizeSystemLog")]
    pub specify_maximum_file_size_system_log: Option<String>,
}

impl MDM_Policy_Config01_EventLogService02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            control_event_log_behavior: None,
            instance_id: None,
            parent_id: None,
            specify_maximum_file_size_application_log: None,
            specify_maximum_file_size_security_log: None,
            specify_maximum_file_size_system_log: None,
        }
    }


    /// Sets the value of ControlEventLogBehavior
    pub fn set_control_event_log_behavior(&mut self, value: String) {
        self.control_event_log_behavior = Some(value);
    }

    /// Gets the value of ControlEventLogBehavior
    pub fn get_control_event_log_behavior(&self) -> Option<&String> {
        self.control_event_log_behavior.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SpecifyMaximumFileSizeApplicationLog
    pub fn set_specify_maximum_file_size_application_log(&mut self, value: String) {
        self.specify_maximum_file_size_application_log = Some(value);
    }

    /// Gets the value of SpecifyMaximumFileSizeApplicationLog
    pub fn get_specify_maximum_file_size_application_log(&self) -> Option<&String> {
        self.specify_maximum_file_size_application_log.as_ref()
    }

    /// Sets the value of SpecifyMaximumFileSizeSecurityLog
    pub fn set_specify_maximum_file_size_security_log(&mut self, value: String) {
        self.specify_maximum_file_size_security_log = Some(value);
    }

    /// Gets the value of SpecifyMaximumFileSizeSecurityLog
    pub fn get_specify_maximum_file_size_security_log(&self) -> Option<&String> {
        self.specify_maximum_file_size_security_log.as_ref()
    }

    /// Sets the value of SpecifyMaximumFileSizeSystemLog
    pub fn set_specify_maximum_file_size_system_log(&mut self, value: String) {
        self.specify_maximum_file_size_system_log = Some(value);
    }

    /// Gets the value of SpecifyMaximumFileSizeSystemLog
    pub fn get_specify_maximum_file_size_system_log(&self) -> Option<&String> {
        self.specify_maximum_file_size_system_log.as_ref()
    }
}

