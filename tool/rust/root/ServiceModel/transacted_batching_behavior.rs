// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TransactedBatchingBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactedBatchingBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// The maximum batch size for the transacted batching.
    #[serde(rename = "MaxBatchSize")]
    pub max_batch_size: Option<i32>,
}

impl TransactedBatchingBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            max_batch_size: None,
        }
    }


    /// Sets the value of MaxBatchSize
    pub fn set_max_batch_size(&mut self, value: i32) {
        self.max_batch_size = Some(value);
    }

    /// Gets the value of MaxBatchSize
    pub fn get_max_batch_size(&self) -> Option<&i32> {
        self.max_batch_size.as_ref()
    }
}

