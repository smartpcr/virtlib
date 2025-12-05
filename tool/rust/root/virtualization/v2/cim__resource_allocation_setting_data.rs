// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ResourceAllocationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ResourceAllocationSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// The address of the resource. For example, the MAC address of a Ethernet port.
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// Describes the address of this resource in the context of the Parent. The Parent/AddressOnParent properties are used to describe the controller relationship as well the ordering of devices on a controller.For example, if the parent is a PCI Controller, this property would specify the PCI slot of this child device.
    #[serde(rename = "AddressOnParent")]
    pub address_on_parent: Option<String>,

/// This property specifies the units of allocation used by the Reservation and Limit properties. For example, when ResourceType=Processor, AllocationUnits may be set to hertz*10^6 or percent. When ResourceType=Memory, AllocationUnits may be set to bytes*10^3. 
/// It is expected that profiles constrain the units that apply in context of particular resource types.
/// The value of this property shall be a legal value of the Programmatic Units qualifier as defined in Annex C.1 of DSP0004 V2.5 or later.
    #[serde(rename = "AllocationUnits")]
    pub allocation_units: Option<String>,

/// This property specifies if the resource will be automatically allocated. For example when set to true, when the consuming virtual computer system is powered on, this resource would be allocated. A value of false indicates the resource must be explicitly allocated. For example, the setting may represent removable media (cdrom, floppy, etc.) where at power on time, the media is not present. An explicit operation is required to allocate the resource.
    #[serde(rename = "AutomaticAllocation")]
    pub automatic_allocation: Option<bool>,

/// This property specifies if the resource will be automatically de-allocated. For example, when set to true, when the consuming virtual computer system is powered off, this resource would be de-allocated. When set to false, the resource will remain allocated and must be explicitly de-allocated.
    #[serde(rename = "AutomaticDeallocation")]
    pub automatic_deallocation: Option<bool>,

/// The thing to which this resource is connected. For example, a named network or switch port.
    #[serde(rename = "Connection")]
    pub connection: Vec<String>,

/// Describes the consumers visibility to the allocated resource.
/// A value of "Passed-Through" indicates the underlying or host resource is utilized and passed through to the consumer, possibly using partitioning. At least one item shall be present in the HostResource property. 
/// A value of "Virtualized" indicates the resource is virtualized and may not map directly to an underlying/host resource. Some implementations may support specific assignment for virtualized resources, in which case the host resource(s) are exposed using the HostResource property. 
/// A value of "Not represented" indicates a representation of the resource does not exist within the context of the resource consumer.
    #[serde(rename = "ConsumerVisibility")]
    pub consumer_visibility: Option<ResourceAllocationSettingData_ConsumerVisibility>,

/// This property exposes specific assignment of resources. Each non-null value of the HostResource property shall be formated as a URI per RFC3986.
/// If this resource is modeled then a value should be a WBEM URI (DSP0207). If the resource is not modeled then see the appropriate profile. 
/// Profiles may further constrain the type of URI. A NULL value or empty array requests the implementation decide the kind of host resource.
/// If the virtual resource is mapped to more than oneunderlying resource, this property may be left NULL.
/// If NULL, the DeviceAllocatedFromPool or ResourceAllocationFromPool associations may be used to determine the pool of host resources this virtual resource may use. If specific assignment is utilized, all underlying resources used by this virtual resource should be listed.The kind of dependency is specified by the ConsumerVisibility and the MappingBehavior properties. Typically the array contains one item, however multiple host resources may be specified. 
/// A client may set the value(s) to indicate that the requested virtual resource allocation be based on host resources that are identified by element values.
    #[serde(rename = "HostResource")]
    pub host_resource: Vec<String>,

/// This property specifies the upper bound, or maximum amount of resource that will be granted for this allocation. For example, a system which supports memory paging may support setting the Limit of a Memory allocation below that of the VirtualQuantity, thus forcing paging to occur for this allocation.
/// The value of the Limit property is expressed in the unit specified by the value of the AllocationUnits property.
    #[serde(rename = "Limit")]
    pub limit: Option<u64>,

/// Specifies how this resource maps to underlying resourcesIf the HostResource array contains any entries, this property reflects how the resource maps to those specific resources.
    #[serde(rename = "MappingBehavior")]
    pub mapping_behavior: Option<ResourceAllocationSettingData_MappingBehavior>,

/// A string that describes the resource type when a well defined value is not available and ResourceType has the value "Other".
    #[serde(rename = "OtherResourceType")]
    pub other_resource_type: Option<String>,

/// The Parent of the resource. For example, a controller for the current allocation
    #[serde(rename = "Parent")]
    pub parent: Option<String>,

/// This property specifies which ResourcePool the resource is currently allocated from, or which ResourcePool the resource will be allocated from when the allocation occurs.
    #[serde(rename = "PoolID")]
    pub pool_id: Option<String>,

/// This property specifies the amount of resource guaranteed to be available for this allocation. On system which support over-commitment of resources, this value is typically used for admission control to prevent an an allocation from being accepted thus preventing starvation.
/// The value of the Reservation property is expressed in the unit specified by the value of the AllocationUnits property.
    #[serde(rename = "Reservation")]
    pub reservation: Option<u64>,

/// A string describing an implementation specific sub-type for this resource. For example, this may be used to distinguish different models of the same resource type.The property value shall conform to this format (in ABNF): vs-type = dmtf-value / other-org-value / legacy-value dmtf-value = "DMTF:" defining-org ":" org-vs-type org-value = defining-org ":" org-vs-type
/// Where: dmtf-value: is a property value defined by DMTF and is defined in the description of this property. other-org-value: is a property value defined by a business entity other than DMTF and is not defined in the description of this property. legacy-value: is a property value defined by a business entity other than DMTF and is not defined in the description of this property. These values are permitted but recommended to be deprecated over time. defining-org:
/// is an identifier for the business entity that defines the virtual system type. It shall include a copyrighted, trademarked, or otherwise unique name that is owned by that business entity. It shall not be "DMTF" and shall not contain a colon (:). org-vs-type:
/// is an identifier for the virtual system type within the defining business entity. It shall be unique within the defining-org. It may use any character allowed for CIM strings, except for the following: U0000-U001F (Unicode C0 controls) U0020 (space), note that the reason is that OVF allows for multiple space-separated vs-type values in this property. U007F (Unicode C0 controls) U0080-U009F (Unicode C1 controls)
/// If there is a need to structure the value into segments, the segments should be separated with a single colon (:).
/// The values of this property shall be processed case sensitively. They are intended to be processed programmatically (instead of being a display name) and should be short.
/// The following DMTF values are defined: DMTF:unknown - the resource sub-type is unknown or cannot be determined
/// Developers should consult the relevant profile for defined values.
    #[serde(rename = "ResourceSubType")]
    pub resource_sub_type: Option<String>,

/// The type of resource this allocation setting represents.
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<ResourceAllocationSettingData_ResourceType>,

/// This property specifies the quantity of resources presented to the consumer. For example, when ResourceType=Processor, this property would reflect the number of discrete Processors presented to the virtual computer system. When ResourceType=Memory, this property could reflect the number of MB reported to the virtual computer system.
/// The value of the VirtualQuantity property should be expressed in units as defined by the value of the VirtualQuantityUnits property.
    #[serde(rename = "VirtualQuantity")]
    pub virtual_quantity: Option<u64>,

/// This property specifies the units used by the VirtualQuantity property. For example
/// - if ResourceType=Processor, the value of the VirtualQuantityUnits property may be set to "count", indicating that the value of the VirtualQuantity property is expressed as a count.
/// - if ResourceType=Memory, the value of the VirtualQuantityUnits property may be set to "bytes*10^3", indicating that the value of the VirtualQuantity property is expressed in kilobyte.
/// It is expected that profiles constrain the units that apply in context of particular resource types.
/// The value of this property shall be a legal value of the Programmatic Units qualifier as defined in Annex C.1 of DSP0004 V2.5 or later.
    #[serde(rename = "VirtualQuantityUnits")]
    pub virtual_quantity_units: Option<String>,

/// This property specifies a relative priority for this allocation in relation to other allocations from the same ResourcePool. This property has no unit of measure, and is only relevant when compared to other allocations vying for the same host resources.
    #[serde(rename = "Weight")]
    pub weight: Option<u32>,
}

impl CIM_ResourceAllocationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            address: None,
            address_on_parent: None,
            allocation_units: None,
            automatic_allocation: None,
            automatic_deallocation: None,
            connection: Vec::new(),
            consumer_visibility: None,
            host_resource: Vec::new(),
            limit: None,
            mapping_behavior: None,
            other_resource_type: None,
            parent: None,
            pool_id: None,
            reservation: None,
            resource_sub_type: None,
            resource_type: None,
            virtual_quantity: None,
            virtual_quantity_units: None,
            weight: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of AddressOnParent
    pub fn set_address_on_parent(&mut self, value: String) {
        self.address_on_parent = Some(value);
    }

    /// Gets the value of AddressOnParent
    pub fn get_address_on_parent(&self) -> Option<&String> {
        self.address_on_parent.as_ref()
    }

    /// Sets the value of AllocationUnits
    pub fn set_allocation_units(&mut self, value: String) {
        self.allocation_units = Some(value);
    }

    /// Gets the value of AllocationUnits
    pub fn get_allocation_units(&self) -> Option<&String> {
        self.allocation_units.as_ref()
    }

    /// Sets the value of AutomaticAllocation
    pub fn set_automatic_allocation(&mut self, value: bool) {
        self.automatic_allocation = Some(value);
    }

    /// Gets the value of AutomaticAllocation
    pub fn get_automatic_allocation(&self) -> Option<&bool> {
        self.automatic_allocation.as_ref()
    }

    /// Sets the value of AutomaticDeallocation
    pub fn set_automatic_deallocation(&mut self, value: bool) {
        self.automatic_deallocation = Some(value);
    }

    /// Gets the value of AutomaticDeallocation
    pub fn get_automatic_deallocation(&self) -> Option<&bool> {
        self.automatic_deallocation.as_ref()
    }

    /// Sets the value of Connection
    pub fn set_connection(&mut self, value: Vec<String>) {
        self.connection = value;
    }

    /// Gets the value of Connection
    pub fn get_connection(&self) -> &Vec<String> {
        &self.connection
    }

    /// Sets the value of ConsumerVisibility
    pub fn set_consumer_visibility(&mut self, value: ResourceAllocationSettingData_ConsumerVisibility) {
        self.consumer_visibility = Some(value);
    }

    /// Gets the value of ConsumerVisibility
    pub fn get_consumer_visibility(&self) -> Option<&ResourceAllocationSettingData_ConsumerVisibility> {
        self.consumer_visibility.as_ref()
    }

    /// Sets the value of HostResource
    pub fn set_host_resource(&mut self, value: Vec<String>) {
        self.host_resource = value;
    }

    /// Gets the value of HostResource
    pub fn get_host_resource(&self) -> &Vec<String> {
        &self.host_resource
    }

    /// Sets the value of Limit
    pub fn set_limit(&mut self, value: u64) {
        self.limit = Some(value);
    }

    /// Gets the value of Limit
    pub fn get_limit(&self) -> Option<&u64> {
        self.limit.as_ref()
    }

    /// Sets the value of MappingBehavior
    pub fn set_mapping_behavior(&mut self, value: ResourceAllocationSettingData_MappingBehavior) {
        self.mapping_behavior = Some(value);
    }

    /// Gets the value of MappingBehavior
    pub fn get_mapping_behavior(&self) -> Option<&ResourceAllocationSettingData_MappingBehavior> {
        self.mapping_behavior.as_ref()
    }

    /// Sets the value of OtherResourceType
    pub fn set_other_resource_type(&mut self, value: String) {
        self.other_resource_type = Some(value);
    }

    /// Gets the value of OtherResourceType
    pub fn get_other_resource_type(&self) -> Option<&String> {
        self.other_resource_type.as_ref()
    }

    /// Sets the value of Parent
    pub fn set_parent(&mut self, value: String) {
        self.parent = Some(value);
    }

    /// Gets the value of Parent
    pub fn get_parent(&self) -> Option<&String> {
        self.parent.as_ref()
    }

    /// Sets the value of PoolID
    pub fn set_pool_id(&mut self, value: String) {
        self.pool_id = Some(value);
    }

    /// Gets the value of PoolID
    pub fn get_pool_id(&self) -> Option<&String> {
        self.pool_id.as_ref()
    }

    /// Sets the value of Reservation
    pub fn set_reservation(&mut self, value: u64) {
        self.reservation = Some(value);
    }

    /// Gets the value of Reservation
    pub fn get_reservation(&self) -> Option<&u64> {
        self.reservation.as_ref()
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
    pub fn set_resource_type(&mut self, value: ResourceAllocationSettingData_ResourceType) {
        self.resource_type = Some(value);
    }

    /// Gets the value of ResourceType
    pub fn get_resource_type(&self) -> Option<&ResourceAllocationSettingData_ResourceType> {
        self.resource_type.as_ref()
    }

    /// Sets the value of VirtualQuantity
    pub fn set_virtual_quantity(&mut self, value: u64) {
        self.virtual_quantity = Some(value);
    }

    /// Gets the value of VirtualQuantity
    pub fn get_virtual_quantity(&self) -> Option<&u64> {
        self.virtual_quantity.as_ref()
    }

    /// Sets the value of VirtualQuantityUnits
    pub fn set_virtual_quantity_units(&mut self, value: String) {
        self.virtual_quantity_units = Some(value);
    }

    /// Gets the value of VirtualQuantityUnits
    pub fn get_virtual_quantity_units(&self) -> Option<&String> {
        self.virtual_quantity_units.as_ref()
    }

    /// Sets the value of Weight
    pub fn set_weight(&mut self, value: u32) {
        self.weight = Some(value);
    }

    /// Gets the value of Weight
    pub fn get_weight(&self) -> Option<&u32> {
        self.weight.as_ref()
    }
}

