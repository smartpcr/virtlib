// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DispatcherSynchronizationBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DispatcherSynchronizationBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// When enabled the reply on the channel will be send asynchronously.
    #[serde(rename = "AsynchronousSendEnabled")]
    pub asynchronous_send_enabled: Option<bool>,

/// Limits the maximum number of pending receives that may be queued on the channel.
    #[serde(rename = "MaxPendingReceives")]
    pub max_pending_receives: Option<i32>,
}

impl DispatcherSynchronizationBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            asynchronous_send_enabled: None,
            max_pending_receives: None,
        }
    }


    /// Sets the value of AsynchronousSendEnabled
    pub fn set_asynchronous_send_enabled(&mut self, value: bool) {
        self.asynchronous_send_enabled = Some(value);
    }

    /// Gets the value of AsynchronousSendEnabled
    pub fn get_asynchronous_send_enabled(&self) -> Option<&bool> {
        self.asynchronous_send_enabled.as_ref()
    }

    /// Sets the value of MaxPendingReceives
    pub fn set_max_pending_receives(&mut self, value: i32) {
        self.max_pending_receives = Some(value);
    }

    /// Gets the value of MaxPendingReceives
    pub fn get_max_pending_receives(&self) -> Option<&i32> {
        self.max_pending_receives.as_ref()
    }
}

