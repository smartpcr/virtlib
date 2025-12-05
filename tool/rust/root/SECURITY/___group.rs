// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SECURITY
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __Group struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __Group {
    #[serde(flatten)]
    pub base: __Subject,
}

impl __Group {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Subject::new(),
        }
    }

}

