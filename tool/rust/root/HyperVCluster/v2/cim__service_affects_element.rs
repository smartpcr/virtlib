// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ServiceAffectsElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ServiceAffectsElement {

/// The Managed Element that is affected by the Service.
    #[serde(rename = "AffectedElement")]
    pub affected_element: Option<CIM_ManagedElement>,

/// The Service that is affecting the ManagedElement.
    #[serde(rename = "AffectingElement")]
    pub affecting_element: Option<CIM_Service>,

/// An enumeration that describes the effect on the ManagedElement. This array corresponds to the OtherElementEffectsDescriptions array, where the latter provides details that are related to the high-level effects enumerated by this property. Additional detail is required if the ElementEffects array contains the value 1 (Other). The values are defined as follows: 
/// - Exclusive Use (2): No other Service may have this association to the element. 
/// - Performance Impact (3): Deprecated in favor of "Consumes", "Enhances Performance", or "Degrades Performance". Execution of the Service may enhance or degrade the performance of the element. This may be as a side-effect of execution or as an intended consequence of methods provided by the Service. 
/// - Element Integrity (4): Deprecated in favor of "Consumes", "Enhances Integrity", or "Degrades Integrity". Execution of the Service may enhance or degrade the integrity of the element. This may be as a side-effect of execution or as an intended consequence of methods provided by the Service. 
/// - Manages (5): The Service manages the element. 
/// - Consumes (6): Execution of the Service consumes some or all of the associated element as a consequence of running the Service. For example, the Service may consume CPU cycles, which may affect performance, or Storage which may affect both performance and integrity. (For instance, the lack of free storage can degrade integrity by reducing the ability to save state. ) "Consumes" may be used alone or in conjunction with other values, in particular, "Degrades Performance" and "Degrades Integrity". 
/// "Manages" and not "Consumes" should be used to reflect allocation services that may be provided by a Service. 
/// - Enhances Integrity (7): The Service may enhance integrity of the associated element. 
/// - Degrades Integrity (8): The Service may degrade integrity of the associated element. 
/// - Enhances Performance (9): The Service may enhance performance of the associated element. 
/// - Degrades Performance (10): The Service may degrade performance of the associated element.
    #[serde(rename = "ElementEffects")]
    pub element_effects: Vec<ServiceAffectsElement_ElementEffects>,

/// Provides details for the effect at the corresponding array position in ElementEffects. This information is required if ElementEffects contains the value 1 (Other).
    #[serde(rename = "OtherElementEffectsDescriptions")]
    pub other_element_effects_descriptions: Vec<String>,
}

impl CIM_ServiceAffectsElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            affected_element: None,
            affecting_element: None,
            element_effects: Vec::new(),
            other_element_effects_descriptions: Vec::new(),
        }
    }


    /// Sets the value of AffectedElement
    pub fn set_affected_element(&mut self, value: CIM_ManagedElement) {
        self.affected_element = Some(value);
    }

    /// Gets the value of AffectedElement
    pub fn get_affected_element(&self) -> Option<&CIM_ManagedElement> {
        self.affected_element.as_ref()
    }

    /// Sets the value of AffectingElement
    pub fn set_affecting_element(&mut self, value: CIM_Service) {
        self.affecting_element = Some(value);
    }

    /// Gets the value of AffectingElement
    pub fn get_affecting_element(&self) -> Option<&CIM_Service> {
        self.affecting_element.as_ref()
    }

    /// Sets the value of ElementEffects
    pub fn set_element_effects(&mut self, value: Vec<ServiceAffectsElement_ElementEffects>) {
        self.element_effects = value;
    }

    /// Gets the value of ElementEffects
    pub fn get_element_effects(&self) -> &Vec<ServiceAffectsElement_ElementEffects> {
        &self.element_effects
    }

    /// Sets the value of OtherElementEffectsDescriptions
    pub fn set_other_element_effects_descriptions(&mut self, value: Vec<String>) {
        self.other_element_effects_descriptions = value;
    }

    /// Gets the value of OtherElementEffectsDescriptions
    pub fn get_other_element_effects_descriptions(&self) -> &Vec<String> {
        &self.other_element_effects_descriptions
    }
}

