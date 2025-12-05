// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BaseMetricValue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BaseMetricValue {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// If present, specifies one BreakdownDimension from the BreakdownDimensions array defined in the associated CIM_ BaseMetricDefinition. This is the dimension along which this set of metric values is broken down. For a description of the concept, see the class CIM_BaseMetricDefinition.
    #[serde(rename = "BreakdownDimension")]
    pub breakdown_dimension: Option<String>,

/// Defines a value of the BreakdownDimension property defined for this metric value instance. For instance, if the BreakdownDimension is "TransactionName", this property could name the actual transaction to which this particular metric value applies.
    #[serde(rename = "BreakdownValue")]
    pub breakdown_value: Option<String>,

/// Property that represents the time duration over which this metric value is valid. This property should not exist for timestamps that apply only to a point in time but should be defined for values that are considered valid for a certain time period (ex. sampling). If the "Duration" property exists and is nonNull, the TimeStamp is to be considered the end of the interval.
    #[serde(rename = "Duration")]
    pub duration: Option<String>,

/// A descriptive name for the element to which the metric value belongs (i.e., the measured element). This property is required by behavior if there is no association defined to a ManagedElement, but may be used in other cases to provide supplemental information. This allows metrics to be captured independently of any ManagedElement. An example is where a metric value belongs to a combination of elements, such as the input and output ports of the traffic in a switch. If there are multiple ManagedElements associated with the metric value, then usually there is one that naturally belongs to the metric value and that one should be used to create the supplemental information. The property is not meant to be used as a foreign key to search on the measured element. Instead, the association to the ManagedElement should be used.
    #[serde(rename = "MeasuredElementName")]
    pub measured_element_name: Option<String>,

/// The key of the BaseMetricDefinition instance for this CIM_BaseMetricValue instance value.
    #[serde(rename = "MetricDefinitionId")]
    pub metric_definition_id: Option<String>,

/// The value of the metric represented as a string. Its original data type is specified in CIM_BaseMetricDefinition.
    #[serde(rename = "MetricValue")]
    pub metric_value: Option<String>,

/// Identifies the time when the value of a metric instance is computed. Note that this is different from the time when the instance is created. For a given CIM_BaseMetricValue instance, the TimeStamp changes whenever a new measurement snapshot is taken if Volatile is true. A managmenet application may establish a time series of metric data by retrieving the instances of CIM_BaseMetricValue and sorting them according to their TimeStamp.
    #[serde(rename = "TimeStamp")]
    pub time_stamp: Option<String>,

/// If true, Volatile indicates that the value for the next point in time may use the same object and just change its properties (such as the value or timestamp). If false, the existing objects remain unchanged and a new object is created for the new point in time.
    #[serde(rename = "volatile")]
    pub volatile: Option<bool>,
}

impl CIM_BaseMetricValue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            breakdown_dimension: None,
            breakdown_value: None,
            duration: None,
            measured_element_name: None,
            metric_definition_id: None,
            metric_value: None,
            time_stamp: None,
            volatile: None,
        }
    }


    /// Sets the value of BreakdownDimension
    pub fn set_breakdown_dimension(&mut self, value: String) {
        self.breakdown_dimension = Some(value);
    }

    /// Gets the value of BreakdownDimension
    pub fn get_breakdown_dimension(&self) -> Option<&String> {
        self.breakdown_dimension.as_ref()
    }

    /// Sets the value of BreakdownValue
    pub fn set_breakdown_value(&mut self, value: String) {
        self.breakdown_value = Some(value);
    }

    /// Gets the value of BreakdownValue
    pub fn get_breakdown_value(&self) -> Option<&String> {
        self.breakdown_value.as_ref()
    }

    /// Sets the value of Duration
    pub fn set_duration(&mut self, value: String) {
        self.duration = Some(value);
    }

    /// Gets the value of Duration
    pub fn get_duration(&self) -> Option<&String> {
        self.duration.as_ref()
    }

    /// Sets the value of MeasuredElementName
    pub fn set_measured_element_name(&mut self, value: String) {
        self.measured_element_name = Some(value);
    }

    /// Gets the value of MeasuredElementName
    pub fn get_measured_element_name(&self) -> Option<&String> {
        self.measured_element_name.as_ref()
    }

    /// Sets the value of MetricDefinitionId
    pub fn set_metric_definition_id(&mut self, value: String) {
        self.metric_definition_id = Some(value);
    }

    /// Gets the value of MetricDefinitionId
    pub fn get_metric_definition_id(&self) -> Option<&String> {
        self.metric_definition_id.as_ref()
    }

    /// Sets the value of MetricValue
    pub fn set_metric_value(&mut self, value: String) {
        self.metric_value = Some(value);
    }

    /// Gets the value of MetricValue
    pub fn get_metric_value(&self) -> Option<&String> {
        self.metric_value.as_ref()
    }

    /// Sets the value of TimeStamp
    pub fn set_time_stamp(&mut self, value: String) {
        self.time_stamp = Some(value);
    }

    /// Gets the value of TimeStamp
    pub fn get_time_stamp(&self) -> Option<&String> {
        self.time_stamp.as_ref()
    }

    /// Sets the value of volatile
    pub fn set_volatile(&mut self, value: bool) {
        self.volatile = Some(value);
    }

    /// Gets the value of volatile
    pub fn get_volatile(&self) -> Option<&bool> {
        self.volatile.as_ref()
    }
}

