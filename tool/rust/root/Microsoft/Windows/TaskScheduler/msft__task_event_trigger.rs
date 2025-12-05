// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskEventTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskEventTrigger {
    #[serde(flatten)]
    pub base: MSFT_TaskTrigger,

/// 
    #[serde(rename = "Delay")]
    pub delay: Option<String>,

/// 
    #[serde(rename = "Subscription")]
    pub subscription: Option<String>,

/// 
    #[serde(rename = "ValueQueries")]
    pub value_queries: Vec<MSFT_TaskNamedValue>,
}

impl MSFT_TaskEventTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskTrigger::new(),
            delay: None,
            subscription: None,
            value_queries: Vec::new(),
        }
    }


    /// Sets the value of Delay
    pub fn set_delay(&mut self, value: String) {
        self.delay = Some(value);
    }

    /// Gets the value of Delay
    pub fn get_delay(&self) -> Option<&String> {
        self.delay.as_ref()
    }

    /// Sets the value of Subscription
    pub fn set_subscription(&mut self, value: String) {
        self.subscription = Some(value);
    }

    /// Gets the value of Subscription
    pub fn get_subscription(&self) -> Option<&String> {
        self.subscription.as_ref()
    }

    /// Sets the value of ValueQueries
    pub fn set_value_queries(&mut self, value: Vec<MSFT_TaskNamedValue>) {
        self.value_queries = value;
    }

    /// Gets the value of ValueQueries
    pub fn get_value_queries(&self) -> &Vec<MSFT_TaskNamedValue> {
        &self.value_queries
    }
}

