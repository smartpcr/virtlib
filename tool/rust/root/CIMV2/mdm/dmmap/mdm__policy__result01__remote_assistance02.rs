// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_RemoteAssistance02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_RemoteAssistance02 {

/// 
    #[serde(rename = "CustomizeWarningMessages")]
    pub customize_warning_messages: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SessionLogging")]
    pub session_logging: Option<String>,

/// 
    #[serde(rename = "SolicitedRemoteAssistance")]
    pub solicited_remote_assistance: Option<String>,

/// 
    #[serde(rename = "UnsolicitedRemoteAssistance")]
    pub unsolicited_remote_assistance: Option<String>,
}

impl MDM_Policy_Result01_RemoteAssistance02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            customize_warning_messages: None,
            instance_id: None,
            parent_id: None,
            session_logging: None,
            solicited_remote_assistance: None,
            unsolicited_remote_assistance: None,
        }
    }


    /// Sets the value of CustomizeWarningMessages
    pub fn set_customize_warning_messages(&mut self, value: String) {
        self.customize_warning_messages = Some(value);
    }

    /// Gets the value of CustomizeWarningMessages
    pub fn get_customize_warning_messages(&self) -> Option<&String> {
        self.customize_warning_messages.as_ref()
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

    /// Sets the value of SessionLogging
    pub fn set_session_logging(&mut self, value: String) {
        self.session_logging = Some(value);
    }

    /// Gets the value of SessionLogging
    pub fn get_session_logging(&self) -> Option<&String> {
        self.session_logging.as_ref()
    }

    /// Sets the value of SolicitedRemoteAssistance
    pub fn set_solicited_remote_assistance(&mut self, value: String) {
        self.solicited_remote_assistance = Some(value);
    }

    /// Gets the value of SolicitedRemoteAssistance
    pub fn get_solicited_remote_assistance(&self) -> Option<&String> {
        self.solicited_remote_assistance.as_ref()
    }

    /// Sets the value of UnsolicitedRemoteAssistance
    pub fn set_unsolicited_remote_assistance(&mut self, value: String) {
        self.unsolicited_remote_assistance = Some(value);
    }

    /// Gets the value of UnsolicitedRemoteAssistance
    pub fn get_unsolicited_remote_assistance(&self) -> Option<&String> {
        self.unsolicited_remote_assistance.as_ref()
    }
}

