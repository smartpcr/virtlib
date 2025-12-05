// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SynchronousReceiveBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynchronousReceiveBehavior {
    #[serde(flatten)]
    pub base: Behavior,
}

impl SynchronousReceiveBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
        }
    }

}

