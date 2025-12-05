// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Processor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Processor {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// Processor address width in bits.
    #[serde(rename = "AddressWidth")]
    pub address_width: Option<u16>,

/// The CPUStatus property that indicates the current status of the Processor. For example, the Processor might be disabled by the user (value=2), or disabled due to a POST error (value=3). Information in this property can be obtained from SMBIOS, the Type 4 structure, and the Status attribute.
    #[serde(rename = "CPUStatus")]
    pub cpustatus: Option<Processor_CPUStatus>,

/// The current speed (in MHz) of this Processor.
    #[serde(rename = "CurrentClockSpeed")]
    pub current_clock_speed: Option<u32>,

/// Processor data width in bits.
    #[serde(rename = "DataWidth")]
    pub data_width: Option<u16>,

/// The speed (in MHz) of the external bus interface (also known as the front side bus).
    #[serde(rename = "ExternalBusClockSpeed")]
    pub external_bus_clock_speed: Option<u32>,

/// The Processor family type. For example, values include "Pentium(R) processor with MMX(TM) technology" (value=14) and "68040" (value=96).
    #[serde(rename = "Family")]
    pub family: Option<Processor_Family>,

/// Loading of this Processor, averaged over the last minute, in Percent.
    #[serde(rename = "LoadPercentage")]
    pub load_percentage: Option<u16>,

/// The maximum speed (in MHz) of this Processor.
    #[serde(rename = "MaxClockSpeed")]
    pub max_clock_speed: Option<u32>,

/// A string that describes the Processor Family type. It is used when the Family property is set to 1 ("Other"). This string should be set to NULL when the Family property is any value other than 1.
    #[serde(rename = "OtherFamilyDescription")]
    pub other_family_description: Option<String>,

/// A free-form string that describes the role of the Processor, for example, "Central Processor" or "Math Processor".
    #[serde(rename = "Role")]
    pub role: Option<String>,

/// Stepping is a free-form string that indicates the revision level of the Processor within the Processor.Family.
    #[serde(rename = "Stepping")]
    pub stepping: Option<String>,

/// A globally unique identifier for the Processor. This identifier can be unique only within a Processor Family.
    #[serde(rename = "UniqueID")]
    pub unique_id: Option<String>,

/// CPU socket information that includes data on how this Processor can be upgraded (if upgrades are supported). This property is an integer enumeration.
    #[serde(rename = "UpgradeMethod")]
    pub upgrade_method: Option<Processor_UpgradeMethod>,
}

impl CIM_Processor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            address_width: None,
            cpustatus: None,
            current_clock_speed: None,
            data_width: None,
            external_bus_clock_speed: None,
            family: None,
            load_percentage: None,
            max_clock_speed: None,
            other_family_description: None,
            role: None,
            stepping: None,
            unique_id: None,
            upgrade_method: None,
        }
    }


    /// Sets the value of AddressWidth
    pub fn set_address_width(&mut self, value: u16) {
        self.address_width = Some(value);
    }

    /// Gets the value of AddressWidth
    pub fn get_address_width(&self) -> Option<&u16> {
        self.address_width.as_ref()
    }

    /// Sets the value of CPUStatus
    pub fn set_cpustatus(&mut self, value: Processor_CPUStatus) {
        self.cpustatus = Some(value);
    }

    /// Gets the value of CPUStatus
    pub fn get_cpustatus(&self) -> Option<&Processor_CPUStatus> {
        self.cpustatus.as_ref()
    }

    /// Sets the value of CurrentClockSpeed
    pub fn set_current_clock_speed(&mut self, value: u32) {
        self.current_clock_speed = Some(value);
    }

    /// Gets the value of CurrentClockSpeed
    pub fn get_current_clock_speed(&self) -> Option<&u32> {
        self.current_clock_speed.as_ref()
    }

    /// Sets the value of DataWidth
    pub fn set_data_width(&mut self, value: u16) {
        self.data_width = Some(value);
    }

    /// Gets the value of DataWidth
    pub fn get_data_width(&self) -> Option<&u16> {
        self.data_width.as_ref()
    }

    /// Sets the value of ExternalBusClockSpeed
    pub fn set_external_bus_clock_speed(&mut self, value: u32) {
        self.external_bus_clock_speed = Some(value);
    }

    /// Gets the value of ExternalBusClockSpeed
    pub fn get_external_bus_clock_speed(&self) -> Option<&u32> {
        self.external_bus_clock_speed.as_ref()
    }

    /// Sets the value of Family
    pub fn set_family(&mut self, value: Processor_Family) {
        self.family = Some(value);
    }

    /// Gets the value of Family
    pub fn get_family(&self) -> Option<&Processor_Family> {
        self.family.as_ref()
    }

    /// Sets the value of LoadPercentage
    pub fn set_load_percentage(&mut self, value: u16) {
        self.load_percentage = Some(value);
    }

    /// Gets the value of LoadPercentage
    pub fn get_load_percentage(&self) -> Option<&u16> {
        self.load_percentage.as_ref()
    }

    /// Sets the value of MaxClockSpeed
    pub fn set_max_clock_speed(&mut self, value: u32) {
        self.max_clock_speed = Some(value);
    }

    /// Gets the value of MaxClockSpeed
    pub fn get_max_clock_speed(&self) -> Option<&u32> {
        self.max_clock_speed.as_ref()
    }

    /// Sets the value of OtherFamilyDescription
    pub fn set_other_family_description(&mut self, value: String) {
        self.other_family_description = Some(value);
    }

    /// Gets the value of OtherFamilyDescription
    pub fn get_other_family_description(&self) -> Option<&String> {
        self.other_family_description.as_ref()
    }

    /// Sets the value of Role
    pub fn set_role(&mut self, value: String) {
        self.role = Some(value);
    }

    /// Gets the value of Role
    pub fn get_role(&self) -> Option<&String> {
        self.role.as_ref()
    }

    /// Sets the value of Stepping
    pub fn set_stepping(&mut self, value: String) {
        self.stepping = Some(value);
    }

    /// Gets the value of Stepping
    pub fn get_stepping(&self) -> Option<&String> {
        self.stepping.as_ref()
    }

    /// Sets the value of UniqueID
    pub fn set_unique_id(&mut self, value: String) {
        self.unique_id = Some(value);
    }

    /// Gets the value of UniqueID
    pub fn get_unique_id(&self) -> Option<&String> {
        self.unique_id.as_ref()
    }

    /// Sets the value of UpgradeMethod
    pub fn set_upgrade_method(&mut self, value: Processor_UpgradeMethod) {
        self.upgrade_method = Some(value);
    }

    /// Gets the value of UpgradeMethod
    pub fn get_upgrade_method(&self) -> Option<&Processor_UpgradeMethod> {
        self.upgrade_method.as_ref()
    }
}

