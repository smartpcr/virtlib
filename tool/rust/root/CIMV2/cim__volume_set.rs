// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VolumeSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VolumeSet {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,

/// 
    #[serde(rename = "PSExtentInterleaveDepth")]
    pub psextent_interleave_depth: Option<u64>,

/// 
    #[serde(rename = "PSExtentStripeLength")]
    pub psextent_stripe_length: Option<u64>,
}

impl CIM_VolumeSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
            psextent_interleave_depth: None,
            psextent_stripe_length: None,
        }
    }


    /// Sets the value of PSExtentInterleaveDepth
    pub fn set_psextent_interleave_depth(&mut self, value: u64) {
        self.psextent_interleave_depth = Some(value);
    }

    /// Gets the value of PSExtentInterleaveDepth
    pub fn get_psextent_interleave_depth(&self) -> Option<&u64> {
        self.psextent_interleave_depth.as_ref()
    }

    /// Sets the value of PSExtentStripeLength
    pub fn set_psextent_stripe_length(&mut self, value: u64) {
        self.psextent_stripe_length = Some(value);
    }

    /// Gets the value of PSExtentStripeLength
    pub fn get_psextent_stripe_length(&self) -> Option<&u64> {
        self.psextent_stripe_length.as_ref()
    }
}

