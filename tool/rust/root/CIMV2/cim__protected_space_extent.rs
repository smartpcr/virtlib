// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProtectedSpaceExtent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProtectedSpaceExtent {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,

/// 
    #[serde(rename = "UserDataStripeDepth")]
    pub user_data_stripe_depth: Option<u64>,
}

impl CIM_ProtectedSpaceExtent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
            user_data_stripe_depth: None,
        }
    }


    /// Sets the value of UserDataStripeDepth
    pub fn set_user_data_stripe_depth(&mut self, value: u64) {
        self.user_data_stripe_depth = Some(value);
    }

    /// Gets the value of UserDataStripeDepth
    pub fn get_user_data_stripe_depth(&self) -> Option<&u64> {
        self.user_data_stripe_depth.as_ref()
    }
}

