// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BaseMetricDefinition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BaseMetricDefinition {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// Defines one or more strings that can be used to refine (break down) queries against the BaseMetricValues along a certain dimension. An example is a transaction name, allowing the break down of the total value for all transactions into a set of values, one for each transaction name. Other examples might be application system or user group name. The strings are free format and should be meaningful to the end users of the metric data. The strings indicate which break down dimensions are supported for this metric definition, by the underlying instrumentation.
    #[serde(rename = "BreakdownDimensions")]
    pub breakdown_dimensions: Vec<String>,

/// An enumerated value that describes the characteristics of the metric, for purposes of performing calculations. The property can take one of the following values: 
/// 1="Non-calculable" -> a string. Arithmetic makes no sense. 
/// 2="Summable" -> It is reasonable to sum this value over many instances of e.g., UnitOfWork, such as the number of files processed in a backup job. For example, if each backup job is a UnitOfWork, and each job backs up 27,000 files on average, then it makes sense to say that 100 backup jobs processed 2,700,000 files. 
/// 3="Non-summable" -> It does not make sense to sum this value over many instances of UnitOfWork. An example would be a metric that measures the queue length when a job arrives at a server. If each job is a UnitOfWork, and the average queue length when each job arrives is 33, it does not make sense to say that the queue length for 100 jobs is 3300. It does make sense to say that the mean is 33.
    #[serde(rename = "Calculable")]
    pub calculable: Option<BaseMetricDefinition_Calculable>,

/// ChangeType indicates how the metric value changes, in the form of typical combinations of finer grain attributes such as direction change, minimum and maximum values, and wrapping semantics. 
/// 0="Unknown": The metric designer did not qualify the ChangeType. 
/// 2="N/A": If the "IsContinuous" property is "false", ChangeType does not make sense and MUST be is set to "N/A". 
/// 3="Counter": The metric is a counter metric. These have non-negative integer values which increase monotonically until reaching the maximum representable number and then wrap around and start increasing from 0. Such counters, also known as rollover counters, can be used for instance to count the number of network errors or the number of transactions processed. The only way for a client application to keep track of wrap arounds is to retrieve the value of the counter in appropriately short intervals. 
/// 4="Gauge": The metric is a gauge metric. These have integer or float values that can increase and decrease arbitrarily. A gauge MUST NOT wrap when reaching the minimum or maximum representable number, instead, the value "sticks" at that number. Minimum or maximum values inside of the representable value range at which the metric value "sticks", may or may not be defined. 
/// Vendors may extend this property in the vendor reserved range.
    #[serde(rename = "ChangeType")]
    pub change_type: Option<BaseMetricDefinition_ChangeType>,

/// The data type of the metric. For example, "boolean" (value=1) or "datetime" (=3) may be specified. These types represent the datatypes defined for CIM.
    #[serde(rename = "DataType")]
    pub data_type: Option<BaseMetricDefinition_DataType>,

/// GatheringType indicates how the metric values are gathered by the underlying instrumentation. This allows the client application to choose the right metric for the purpose. 
/// 0="Unknown": Indicates that the GatheringType is not known. 
/// 2="OnChange": Indicates that the CIM metric values get updated immediately when the values inside of the measured resource change. The values of OnChange metrics truly reflect the current situation within the resource at any time. An example is the number of logged on users that gets updated immediately as users log on and off. 
/// 3="Periodic": Indicates that the CIM metric values get updated periodically. For instance, to a client application, a metric value applying to the current time will appear constant during each gathering interval, and then jumps to the new value at the end of each gathering interval. 
/// 4="OnRequest": Indicates that the CIM metric value is determined each time a client application reads it. The values of OnRequest metrics truly return the current situation within the resource if somebody asks for it. However, they do not change "unobserved", and therefore subscribing for value changes of OnRequest metrics is NOT RECOMMENDED.
    #[serde(rename = "GatheringType")]
    pub gathering_type: Option<BaseMetricDefinition_GatheringType>,

/// A string that uniquely identifies the metric definition. The use of OSF UUID/GUIDs is recommended.
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// IsContinuous indicates whether or not the metric value is continuous or scalar. Performance metrics are an example of a linear metric. Examples of non-linear metrics include error codes or operational states. Continuous metrics can be compared using the "greater than" relation.
    #[serde(rename = "IsContinuous")]
    pub is_continuous: Option<bool>,

/// The name of the metric. This name does not have to be unique, but should be descriptive and may contain blanks.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// Identifies the specific units of a value. The value of this property shall be a legal value of the Programmatic Units qualifier as defined in Appendix C.1 of DSP0004 V2.4 or later.
    #[serde(rename = "ProgrammaticUnits")]
    pub programmatic_units: Option<String>,

/// TimeScope indicates the time scope to which the metric value applies. 
/// 0="Unknown" indicates the time scope was not qualified by the metric designer, or is unknown to the provider. 
/// 2="Point" indicates that the metric applies to a point in time. On the corresponding BaseMetricValue instances, TimeStamp specifies the point in time and Duration is always 0. 
/// 3="Interval" indicates that the metric applies to a time interval. On the corresponding BaseMetricValue instances, TimeStamp specifies the end of the time interval and Duration specifies its duration. 
/// 4="StartupInterval" indicates that the metric applies to a time interval that began at the startup of the measured resource (i.e. the ManagedElement associated by MetricDefForMe). On the corresponding BaseMetricValue instances, TimeStamp specifies the end of the time interval. If Duration is 0, this indicates that the startup time of the measured resource is unknown. Else, Duration specifies the duration between startup of the resource and TimeStamp.
    #[serde(rename = "TimeScope")]
    pub time_scope: Option<BaseMetricDefinition_TimeScope>,

/// Identifies the specific units of a value. Examples are Bytes, Packets, Jobs, Files, Milliseconds, and Amps.
    #[serde(rename = "Units")]
    pub units: Option<String>,
}

impl CIM_BaseMetricDefinition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            breakdown_dimensions: Vec::new(),
            calculable: None,
            change_type: None,
            data_type: None,
            gathering_type: None,
            id: None,
            is_continuous: None,
            name: None,
            programmatic_units: None,
            time_scope: None,
            units: None,
        }
    }


    /// Sets the value of BreakdownDimensions
    pub fn set_breakdown_dimensions(&mut self, value: Vec<String>) {
        self.breakdown_dimensions = value;
    }

    /// Gets the value of BreakdownDimensions
    pub fn get_breakdown_dimensions(&self) -> &Vec<String> {
        &self.breakdown_dimensions
    }

    /// Sets the value of Calculable
    pub fn set_calculable(&mut self, value: BaseMetricDefinition_Calculable) {
        self.calculable = Some(value);
    }

    /// Gets the value of Calculable
    pub fn get_calculable(&self) -> Option<&BaseMetricDefinition_Calculable> {
        self.calculable.as_ref()
    }

    /// Sets the value of ChangeType
    pub fn set_change_type(&mut self, value: BaseMetricDefinition_ChangeType) {
        self.change_type = Some(value);
    }

    /// Gets the value of ChangeType
    pub fn get_change_type(&self) -> Option<&BaseMetricDefinition_ChangeType> {
        self.change_type.as_ref()
    }

    /// Sets the value of DataType
    pub fn set_data_type(&mut self, value: BaseMetricDefinition_DataType) {
        self.data_type = Some(value);
    }

    /// Gets the value of DataType
    pub fn get_data_type(&self) -> Option<&BaseMetricDefinition_DataType> {
        self.data_type.as_ref()
    }

    /// Sets the value of GatheringType
    pub fn set_gathering_type(&mut self, value: BaseMetricDefinition_GatheringType) {
        self.gathering_type = Some(value);
    }

    /// Gets the value of GatheringType
    pub fn get_gathering_type(&self) -> Option<&BaseMetricDefinition_GatheringType> {
        self.gathering_type.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IsContinuous
    pub fn set_is_continuous(&mut self, value: bool) {
        self.is_continuous = Some(value);
    }

    /// Gets the value of IsContinuous
    pub fn get_is_continuous(&self) -> Option<&bool> {
        self.is_continuous.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProgrammaticUnits
    pub fn set_programmatic_units(&mut self, value: String) {
        self.programmatic_units = Some(value);
    }

    /// Gets the value of ProgrammaticUnits
    pub fn get_programmatic_units(&self) -> Option<&String> {
        self.programmatic_units.as_ref()
    }

    /// Sets the value of TimeScope
    pub fn set_time_scope(&mut self, value: BaseMetricDefinition_TimeScope) {
        self.time_scope = Some(value);
    }

    /// Gets the value of TimeScope
    pub fn get_time_scope(&self) -> Option<&BaseMetricDefinition_TimeScope> {
        self.time_scope.as_ref()
    }

    /// Sets the value of Units
    pub fn set_units(&mut self, value: String) {
        self.units = Some(value);
    }

    /// Gets the value of Units
    pub fn get_units(&self) -> Option<&String> {
        self.units.as_ref()
    }
}

