// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ResourcePool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ResourcePool {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// This property specifies the units of allocation used by the Reservation and Limit properties. For example, when ResourceType=Processor, AllocationUnits may be set to hertz*10^6 or percent. When ResourceType=Memory, AllocationUnits may be set to bytes*10^3. The value of this property shall be a legal value of the Programmatic Units qualifier as defined in Appendix C.1 of DSP0004 V2.4 or later.
    #[serde(rename = "AllocationUnits")]
    pub allocation_units: Option<String>,

/// This property represents the maximum amount (in units of AllocationUnits) of reservations that the ResourcePool can support.
    #[serde(rename = "Capacity")]
    pub capacity: Option<u64>,

/// This property specifies the units for the MaxConsumable and the Consumed properties.
    #[serde(rename = "ConsumedResourceUnits")]
    pub consumed_resource_units: Option<String>,

/// This property specifies the amount of resource that the resource pool currently presents to consumers.
/// This property is different from the Reserved property in that it describes the consumers view of the resource while the Reserved property describes the producers view of the resource.
    #[serde(rename = "CurrentlyConsumedResource")]
    pub currently_consumed_resource: Option<u64>,

/// This property specifies the maximum of amount of consumable resource that the resource pool can present to consumers.
/// This property is different from the Capacity property in that it describes the consumers view of the resource while the Capacity property describes the producers view of the resource.
    #[serde(rename = "MaxConsumableResource")]
    pub max_consumable_resource: Option<u64>,

/// A string that describes the resource type when a well defined value is not available and ResourceType is set to 0 for Other.
    #[serde(rename = "OtherResourceType")]
    pub other_resource_type: Option<String>,

/// An opaque identifier for the pool. This property is used to provide correlation across save and restore of configuration data to underlying persistent storage.
    #[serde(rename = "PoolID")]
    pub pool_id: Option<String>,

/// If true, "Primordial" indicates that this ResourcePool is a base from which resources are drawn and returned in the activity of resource management. Being primordial means that this ResourcePool shall not be created or deleted by consumers of this model. However, other actions, modeled or not, may affect the characteristics or size of primordial ResourcePools. If false, "Primordial" indicates that the ResourcePool, a concrete Resource Pool, is subject to resource allocation services functions. This distinction is important because higher-level ResourcePools may be assembled using the Component or ElementAllocatedFromPool associations. Although the higher-level abstractions can be created and deleted, the most basic, (i.e. primordial), hardware-based ResourcePools cannot. They are physically realized as part of the System, or are actually managed by some other System and imported as if they were physically realized.
    #[serde(rename = "Primordial")]
    pub primordial: Option<bool>,

/// This property represents the current reservations (in units of AllocationUnits) spread across all active allocations from this pool. In a hierarchical configuration, this represents the sum of all descendant ResourcePool current reservations.
    #[serde(rename = "Reserved")]
    pub reserved: Option<u64>,

/// A string describing an implementation specific sub-type for this pool. For example, this may be used to distinguish different models of the same resource type.
    #[serde(rename = "ResourceSubType")]
    pub resource_sub_type: Option<String>,

/// The type of resource this ResourcePool may allocate.
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<ResourcePool_ResourceType>,
}

impl CIM_ResourcePool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            allocation_units: None,
            capacity: None,
            consumed_resource_units: None,
            currently_consumed_resource: None,
            max_consumable_resource: None,
            other_resource_type: None,
            pool_id: None,
            primordial: None,
            reserved: None,
            resource_sub_type: None,
            resource_type: None,
        }
    }


    /// Sets the value of AllocationUnits
    pub fn set_allocation_units(&mut self, value: String) {
        self.allocation_units = Some(value);
    }

    /// Gets the value of AllocationUnits
    pub fn get_allocation_units(&self) -> Option<&String> {
        self.allocation_units.as_ref()
    }

    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: u64) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&u64> {
        self.capacity.as_ref()
    }

    /// Sets the value of ConsumedResourceUnits
    pub fn set_consumed_resource_units(&mut self, value: String) {
        self.consumed_resource_units = Some(value);
    }

    /// Gets the value of ConsumedResourceUnits
    pub fn get_consumed_resource_units(&self) -> Option<&String> {
        self.consumed_resource_units.as_ref()
    }

    /// Sets the value of CurrentlyConsumedResource
    pub fn set_currently_consumed_resource(&mut self, value: u64) {
        self.currently_consumed_resource = Some(value);
    }

    /// Gets the value of CurrentlyConsumedResource
    pub fn get_currently_consumed_resource(&self) -> Option<&u64> {
        self.currently_consumed_resource.as_ref()
    }

    /// Sets the value of MaxConsumableResource
    pub fn set_max_consumable_resource(&mut self, value: u64) {
        self.max_consumable_resource = Some(value);
    }

    /// Gets the value of MaxConsumableResource
    pub fn get_max_consumable_resource(&self) -> Option<&u64> {
        self.max_consumable_resource.as_ref()
    }

    /// Sets the value of OtherResourceType
    pub fn set_other_resource_type(&mut self, value: String) {
        self.other_resource_type = Some(value);
    }

    /// Gets the value of OtherResourceType
    pub fn get_other_resource_type(&self) -> Option<&String> {
        self.other_resource_type.as_ref()
    }

    /// Sets the value of PoolID
    pub fn set_pool_id(&mut self, value: String) {
        self.pool_id = Some(value);
    }

    /// Gets the value of PoolID
    pub fn get_pool_id(&self) -> Option<&String> {
        self.pool_id.as_ref()
    }

    /// Sets the value of Primordial
    pub fn set_primordial(&mut self, value: bool) {
        self.primordial = Some(value);
    }

    /// Gets the value of Primordial
    pub fn get_primordial(&self) -> Option<&bool> {
        self.primordial.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u64) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u64> {
        self.reserved.as_ref()
    }

    /// Sets the value of ResourceSubType
    pub fn set_resource_sub_type(&mut self, value: String) {
        self.resource_sub_type = Some(value);
    }

    /// Gets the value of ResourceSubType
    pub fn get_resource_sub_type(&self) -> Option<&String> {
        self.resource_sub_type.as_ref()
    }

    /// Sets the value of ResourceType
    pub fn set_resource_type(&mut self, value: ResourcePool_ResourceType) {
        self.resource_type = Some(value);
    }

    /// Gets the value of ResourceType
    pub fn get_resource_type(&self) -> Option<&ResourcePool_ResourceType> {
        self.resource_type.as_ref()
    }
}

