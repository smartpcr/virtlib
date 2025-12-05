// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NtlmServerAccept_Info struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NtlmServerAccept_Info {
    #[serde(flatten)]
    pub base: NtlmServerAccept,

/// Client Domain Name
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

/// Flags
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// In-Context
    #[serde(rename = "InContext")]
    pub in_context: Option<u32>,

/// Out-Context
    #[serde(rename = "OutContext")]
    pub out_context: Option<u32>,

/// Stage Hint
    #[serde(rename = "StageHint")]
    pub stage_hint: Option<u32>,

/// Client User Name
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

/// Client Workstation
    #[serde(rename = "Workstation")]
    pub workstation: Option<String>,
}

impl NtlmServerAccept_Info {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: NtlmServerAccept::new(),
            domain_name: None,
            flags: None,
            in_context: None,
            out_context: None,
            stage_hint: None,
            user_name: None,
            workstation: None,
        }
    }


    /// Sets the value of DomainName
    pub fn set_domain_name(&mut self, value: String) {
        self.domain_name = Some(value);
    }

    /// Gets the value of DomainName
    pub fn get_domain_name(&self) -> Option<&String> {
        self.domain_name.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of InContext
    pub fn set_in_context(&mut self, value: u32) {
        self.in_context = Some(value);
    }

    /// Gets the value of InContext
    pub fn get_in_context(&self) -> Option<&u32> {
        self.in_context.as_ref()
    }

    /// Sets the value of OutContext
    pub fn set_out_context(&mut self, value: u32) {
        self.out_context = Some(value);
    }

    /// Gets the value of OutContext
    pub fn get_out_context(&self) -> Option<&u32> {
        self.out_context.as_ref()
    }

    /// Sets the value of StageHint
    pub fn set_stage_hint(&mut self, value: u32) {
        self.stage_hint = Some(value);
    }

    /// Gets the value of StageHint
    pub fn get_stage_hint(&self) -> Option<&u32> {
        self.stage_hint.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of Workstation
    pub fn set_workstation(&mut self, value: String) {
        self.workstation = Some(value);
    }

    /// Gets the value of Workstation
    pub fn get_workstation(&self) -> Option<&String> {
        self.workstation.as_ref()
    }
}

