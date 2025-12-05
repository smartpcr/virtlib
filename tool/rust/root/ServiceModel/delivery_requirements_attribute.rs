// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DeliveryRequirementsAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeliveryRequirementsAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies whether the binding for a service supports contracts.
    #[serde(rename = "QueuedDeliveryRequirements")]
    pub queued_delivery_requirements: Option<String>,

/// Specifies whether the binding supports ordered messages.
    #[serde(rename = "RequireOrderedDelivery")]
    pub require_ordered_delivery: Option<bool>,

/// The contract to which it applies.
    #[serde(rename = "TargetContract")]
    pub target_contract: Option<String>,
}

impl DeliveryRequirementsAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            queued_delivery_requirements: None,
            require_ordered_delivery: None,
            target_contract: None,
        }
    }


    /// Sets the value of QueuedDeliveryRequirements
    pub fn set_queued_delivery_requirements(&mut self, value: String) {
        self.queued_delivery_requirements = Some(value);
    }

    /// Gets the value of QueuedDeliveryRequirements
    pub fn get_queued_delivery_requirements(&self) -> Option<&String> {
        self.queued_delivery_requirements.as_ref()
    }

    /// Sets the value of RequireOrderedDelivery
    pub fn set_require_ordered_delivery(&mut self, value: bool) {
        self.require_ordered_delivery = Some(value);
    }

    /// Gets the value of RequireOrderedDelivery
    pub fn get_require_ordered_delivery(&self) -> Option<&bool> {
        self.require_ordered_delivery.as_ref()
    }

    /// Sets the value of TargetContract
    pub fn set_target_contract(&mut self, value: String) {
        self.target_contract = Some(value);
    }

    /// Gets the value of TargetContract
    pub fn get_target_contract(&self) -> Option<&String> {
        self.target_contract.as_ref()
    }
}

