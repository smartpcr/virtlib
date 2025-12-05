// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NtlmServerAccept_Start struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NtlmServerAccept_Start {
    #[serde(flatten)]
    pub base: NtlmServerAccept,

/// In-Context
    #[serde(rename = "InContext")]
    pub in_context: Option<u32>,

/// Stage Hint
    #[serde(rename = "StageHint")]
    pub stage_hint: Option<u32>,
}

impl NtlmServerAccept_Start {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: NtlmServerAccept::new(),
            in_context: None,
            stage_hint: None,
        }
    }


    /// Sets the value of InContext
    pub fn set_in_context(&mut self, value: u32) {
        self.in_context = Some(value);
    }

    /// Gets the value of InContext
    pub fn get_in_context(&self) -> Option<&u32> {
        self.in_context.as_ref()
    }

    /// Sets the value of StageHint
    pub fn set_stage_hint(&mut self, value: u32) {
        self.stage_hint = Some(value);
    }

    /// Gets the value of StageHint
    pub fn get_stage_hint(&self) -> Option<&u32> {
        self.stage_hint.as_ref()
    }
}

