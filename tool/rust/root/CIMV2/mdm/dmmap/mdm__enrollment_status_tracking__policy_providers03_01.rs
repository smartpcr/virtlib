// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnrollmentStatusTracking_PolicyProviders03_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnrollmentStatusTracking_PolicyProviders03_01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TrackingPoliciesCreated")]
    pub tracking_policies_created: Option<bool>,
}

impl MDM_EnrollmentStatusTracking_PolicyProviders03_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            tracking_policies_created: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TrackingPoliciesCreated
    pub fn set_tracking_policies_created(&mut self, value: bool) {
        self.tracking_policies_created = Some(value);
    }

    /// Gets the value of TrackingPoliciesCreated
    pub fn get_tracking_policies_created(&self) -> Option<&bool> {
        self.tracking_policies_created.as_ref()
    }
}

