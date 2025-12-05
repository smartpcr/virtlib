// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_MiCompare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_MiCompare {
    #[serde(flatten)]
    pub base: Msft_MiStream,

/// 
    #[serde(rename = "Input")]
    pub input: Option<Msft_MiStream>,

/// 
    #[serde(rename = "OnlyUpdateSnapshot")]
    pub only_update_snapshot: Option<bool>,

/// 
    #[serde(rename = "SuppressionHint")]
    pub suppression_hint: Option<Msft_MiCompareSuppression>,
}

impl Msft_MiCompare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_MiStream::new(),
            input: None,
            only_update_snapshot: None,
            suppression_hint: None,
        }
    }


    /// Sets the value of Input
    pub fn set_input(&mut self, value: Msft_MiStream) {
        self.input = Some(value);
    }

    /// Gets the value of Input
    pub fn get_input(&self) -> Option<&Msft_MiStream> {
        self.input.as_ref()
    }

    /// Sets the value of OnlyUpdateSnapshot
    pub fn set_only_update_snapshot(&mut self, value: bool) {
        self.only_update_snapshot = Some(value);
    }

    /// Gets the value of OnlyUpdateSnapshot
    pub fn get_only_update_snapshot(&self) -> Option<&bool> {
        self.only_update_snapshot.as_ref()
    }

    /// Sets the value of SuppressionHint
    pub fn set_suppression_hint(&mut self, value: Msft_MiCompareSuppression) {
        self.suppression_hint = Some(value);
    }

    /// Gets the value of SuppressionHint
    pub fn get_suppression_hint(&self) -> Option<&Msft_MiCompareSuppression> {
        self.suppression_hint.as_ref()
    }
}

