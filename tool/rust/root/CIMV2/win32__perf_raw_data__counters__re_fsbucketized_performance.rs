// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_ReFSBucketizedPerformance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_ReFSBucketizedPerformance {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Value1TotalOperationsCount")]
    pub value1_total_operations_count: Option<u64>,

/// 
    #[serde(rename = "Value2Totaltimensforalloperations")]
    pub value2_totaltimensforalloperations: Option<u64>,

/// 
    #[serde(rename = "Value3Totalbytesforalloperations")]
    pub value3_totalbytesforalloperations: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket01128µs")]
    pub value4_countbucket01128µs: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket02256µs")]
    pub value4_countbucket02256µs: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket03512µs")]
    pub value4_countbucket03512µs: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket041ms")]
    pub value4_countbucket041ms: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket054ms")]
    pub value4_countbucket054ms: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket0616ms")]
    pub value4_countbucket0616ms: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket0764ms")]
    pub value4_countbucket0764ms: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket08128ms")]
    pub value4_countbucket08128ms: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket09256ms")]
    pub value4_countbucket09256ms: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket10512ms")]
    pub value4_countbucket10512ms: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket111s")]
    pub value4_countbucket111s: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket122s")]
    pub value4_countbucket122s: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket1310s")]
    pub value4_countbucket1310s: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket1420s")]
    pub value4_countbucket1420s: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket1530s")]
    pub value4_countbucket1530s: Option<u64>,

/// 
    #[serde(rename = "Value4Countbucket1630s")]
    pub value4_countbucket1630s: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket01128µs")]
    pub value5_timensbucket01128µs: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket02256µs")]
    pub value5_timensbucket02256µs: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket03512µs")]
    pub value5_timensbucket03512µs: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket041ms")]
    pub value5_timensbucket041ms: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket054ms")]
    pub value5_timensbucket054ms: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket0616ms")]
    pub value5_timensbucket0616ms: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket0764ms")]
    pub value5_timensbucket0764ms: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket08128ms")]
    pub value5_timensbucket08128ms: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket09256ms")]
    pub value5_timensbucket09256ms: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket10512ms")]
    pub value5_timensbucket10512ms: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket111s")]
    pub value5_timensbucket111s: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket122s")]
    pub value5_timensbucket122s: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket1310s")]
    pub value5_timensbucket1310s: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket1420s")]
    pub value5_timensbucket1420s: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket1530s")]
    pub value5_timensbucket1530s: Option<u64>,

/// 
    #[serde(rename = "Value5Timensbucket1630s")]
    pub value5_timensbucket1630s: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket01128µs")]
    pub value6_bytesbucket01128µs: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket02256µs")]
    pub value6_bytesbucket02256µs: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket03512µs")]
    pub value6_bytesbucket03512µs: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket041ms")]
    pub value6_bytesbucket041ms: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket054ms")]
    pub value6_bytesbucket054ms: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket0616ms")]
    pub value6_bytesbucket0616ms: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket0764ms")]
    pub value6_bytesbucket0764ms: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket08128ms")]
    pub value6_bytesbucket08128ms: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket09256ms")]
    pub value6_bytesbucket09256ms: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket10512ms")]
    pub value6_bytesbucket10512ms: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket111s")]
    pub value6_bytesbucket111s: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket122s")]
    pub value6_bytesbucket122s: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket1310s")]
    pub value6_bytesbucket1310s: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket1420s")]
    pub value6_bytesbucket1420s: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket1530s")]
    pub value6_bytesbucket1530s: Option<u64>,

/// 
    #[serde(rename = "Value6Bytesbucket1630s")]
    pub value6_bytesbucket1630s: Option<u64>,
}

impl Win32_PerfRawData_Counters_ReFSBucketizedPerformance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            value1_total_operations_count: None,
            value2_totaltimensforalloperations: None,
            value3_totalbytesforalloperations: None,
            value4_countbucket01128µs: None,
            value4_countbucket02256µs: None,
            value4_countbucket03512µs: None,
            value4_countbucket041ms: None,
            value4_countbucket054ms: None,
            value4_countbucket0616ms: None,
            value4_countbucket0764ms: None,
            value4_countbucket08128ms: None,
            value4_countbucket09256ms: None,
            value4_countbucket10512ms: None,
            value4_countbucket111s: None,
            value4_countbucket122s: None,
            value4_countbucket1310s: None,
            value4_countbucket1420s: None,
            value4_countbucket1530s: None,
            value4_countbucket1630s: None,
            value5_timensbucket01128µs: None,
            value5_timensbucket02256µs: None,
            value5_timensbucket03512µs: None,
            value5_timensbucket041ms: None,
            value5_timensbucket054ms: None,
            value5_timensbucket0616ms: None,
            value5_timensbucket0764ms: None,
            value5_timensbucket08128ms: None,
            value5_timensbucket09256ms: None,
            value5_timensbucket10512ms: None,
            value5_timensbucket111s: None,
            value5_timensbucket122s: None,
            value5_timensbucket1310s: None,
            value5_timensbucket1420s: None,
            value5_timensbucket1530s: None,
            value5_timensbucket1630s: None,
            value6_bytesbucket01128µs: None,
            value6_bytesbucket02256µs: None,
            value6_bytesbucket03512µs: None,
            value6_bytesbucket041ms: None,
            value6_bytesbucket054ms: None,
            value6_bytesbucket0616ms: None,
            value6_bytesbucket0764ms: None,
            value6_bytesbucket08128ms: None,
            value6_bytesbucket09256ms: None,
            value6_bytesbucket10512ms: None,
            value6_bytesbucket111s: None,
            value6_bytesbucket122s: None,
            value6_bytesbucket1310s: None,
            value6_bytesbucket1420s: None,
            value6_bytesbucket1530s: None,
            value6_bytesbucket1630s: None,
        }
    }


    /// Sets the value of Value1TotalOperationsCount
    pub fn set_value1_total_operations_count(&mut self, value: u64) {
        self.value1_total_operations_count = Some(value);
    }

    /// Gets the value of Value1TotalOperationsCount
    pub fn get_value1_total_operations_count(&self) -> Option<&u64> {
        self.value1_total_operations_count.as_ref()
    }

    /// Sets the value of Value2Totaltimensforalloperations
    pub fn set_value2_totaltimensforalloperations(&mut self, value: u64) {
        self.value2_totaltimensforalloperations = Some(value);
    }

    /// Gets the value of Value2Totaltimensforalloperations
    pub fn get_value2_totaltimensforalloperations(&self) -> Option<&u64> {
        self.value2_totaltimensforalloperations.as_ref()
    }

    /// Sets the value of Value3Totalbytesforalloperations
    pub fn set_value3_totalbytesforalloperations(&mut self, value: u64) {
        self.value3_totalbytesforalloperations = Some(value);
    }

    /// Gets the value of Value3Totalbytesforalloperations
    pub fn get_value3_totalbytesforalloperations(&self) -> Option<&u64> {
        self.value3_totalbytesforalloperations.as_ref()
    }

    /// Sets the value of Value4Countbucket01128µs
    pub fn set_value4_countbucket01128µs(&mut self, value: u64) {
        self.value4_countbucket01128µs = Some(value);
    }

    /// Gets the value of Value4Countbucket01128µs
    pub fn get_value4_countbucket01128µs(&self) -> Option<&u64> {
        self.value4_countbucket01128µs.as_ref()
    }

    /// Sets the value of Value4Countbucket02256µs
    pub fn set_value4_countbucket02256µs(&mut self, value: u64) {
        self.value4_countbucket02256µs = Some(value);
    }

    /// Gets the value of Value4Countbucket02256µs
    pub fn get_value4_countbucket02256µs(&self) -> Option<&u64> {
        self.value4_countbucket02256µs.as_ref()
    }

    /// Sets the value of Value4Countbucket03512µs
    pub fn set_value4_countbucket03512µs(&mut self, value: u64) {
        self.value4_countbucket03512µs = Some(value);
    }

    /// Gets the value of Value4Countbucket03512µs
    pub fn get_value4_countbucket03512µs(&self) -> Option<&u64> {
        self.value4_countbucket03512µs.as_ref()
    }

    /// Sets the value of Value4Countbucket041ms
    pub fn set_value4_countbucket041ms(&mut self, value: u64) {
        self.value4_countbucket041ms = Some(value);
    }

    /// Gets the value of Value4Countbucket041ms
    pub fn get_value4_countbucket041ms(&self) -> Option<&u64> {
        self.value4_countbucket041ms.as_ref()
    }

    /// Sets the value of Value4Countbucket054ms
    pub fn set_value4_countbucket054ms(&mut self, value: u64) {
        self.value4_countbucket054ms = Some(value);
    }

    /// Gets the value of Value4Countbucket054ms
    pub fn get_value4_countbucket054ms(&self) -> Option<&u64> {
        self.value4_countbucket054ms.as_ref()
    }

    /// Sets the value of Value4Countbucket0616ms
    pub fn set_value4_countbucket0616ms(&mut self, value: u64) {
        self.value4_countbucket0616ms = Some(value);
    }

    /// Gets the value of Value4Countbucket0616ms
    pub fn get_value4_countbucket0616ms(&self) -> Option<&u64> {
        self.value4_countbucket0616ms.as_ref()
    }

    /// Sets the value of Value4Countbucket0764ms
    pub fn set_value4_countbucket0764ms(&mut self, value: u64) {
        self.value4_countbucket0764ms = Some(value);
    }

    /// Gets the value of Value4Countbucket0764ms
    pub fn get_value4_countbucket0764ms(&self) -> Option<&u64> {
        self.value4_countbucket0764ms.as_ref()
    }

    /// Sets the value of Value4Countbucket08128ms
    pub fn set_value4_countbucket08128ms(&mut self, value: u64) {
        self.value4_countbucket08128ms = Some(value);
    }

    /// Gets the value of Value4Countbucket08128ms
    pub fn get_value4_countbucket08128ms(&self) -> Option<&u64> {
        self.value4_countbucket08128ms.as_ref()
    }

    /// Sets the value of Value4Countbucket09256ms
    pub fn set_value4_countbucket09256ms(&mut self, value: u64) {
        self.value4_countbucket09256ms = Some(value);
    }

    /// Gets the value of Value4Countbucket09256ms
    pub fn get_value4_countbucket09256ms(&self) -> Option<&u64> {
        self.value4_countbucket09256ms.as_ref()
    }

    /// Sets the value of Value4Countbucket10512ms
    pub fn set_value4_countbucket10512ms(&mut self, value: u64) {
        self.value4_countbucket10512ms = Some(value);
    }

    /// Gets the value of Value4Countbucket10512ms
    pub fn get_value4_countbucket10512ms(&self) -> Option<&u64> {
        self.value4_countbucket10512ms.as_ref()
    }

    /// Sets the value of Value4Countbucket111s
    pub fn set_value4_countbucket111s(&mut self, value: u64) {
        self.value4_countbucket111s = Some(value);
    }

    /// Gets the value of Value4Countbucket111s
    pub fn get_value4_countbucket111s(&self) -> Option<&u64> {
        self.value4_countbucket111s.as_ref()
    }

    /// Sets the value of Value4Countbucket122s
    pub fn set_value4_countbucket122s(&mut self, value: u64) {
        self.value4_countbucket122s = Some(value);
    }

    /// Gets the value of Value4Countbucket122s
    pub fn get_value4_countbucket122s(&self) -> Option<&u64> {
        self.value4_countbucket122s.as_ref()
    }

    /// Sets the value of Value4Countbucket1310s
    pub fn set_value4_countbucket1310s(&mut self, value: u64) {
        self.value4_countbucket1310s = Some(value);
    }

    /// Gets the value of Value4Countbucket1310s
    pub fn get_value4_countbucket1310s(&self) -> Option<&u64> {
        self.value4_countbucket1310s.as_ref()
    }

    /// Sets the value of Value4Countbucket1420s
    pub fn set_value4_countbucket1420s(&mut self, value: u64) {
        self.value4_countbucket1420s = Some(value);
    }

    /// Gets the value of Value4Countbucket1420s
    pub fn get_value4_countbucket1420s(&self) -> Option<&u64> {
        self.value4_countbucket1420s.as_ref()
    }

    /// Sets the value of Value4Countbucket1530s
    pub fn set_value4_countbucket1530s(&mut self, value: u64) {
        self.value4_countbucket1530s = Some(value);
    }

    /// Gets the value of Value4Countbucket1530s
    pub fn get_value4_countbucket1530s(&self) -> Option<&u64> {
        self.value4_countbucket1530s.as_ref()
    }

    /// Sets the value of Value4Countbucket1630s
    pub fn set_value4_countbucket1630s(&mut self, value: u64) {
        self.value4_countbucket1630s = Some(value);
    }

    /// Gets the value of Value4Countbucket1630s
    pub fn get_value4_countbucket1630s(&self) -> Option<&u64> {
        self.value4_countbucket1630s.as_ref()
    }

    /// Sets the value of Value5Timensbucket01128µs
    pub fn set_value5_timensbucket01128µs(&mut self, value: u64) {
        self.value5_timensbucket01128µs = Some(value);
    }

    /// Gets the value of Value5Timensbucket01128µs
    pub fn get_value5_timensbucket01128µs(&self) -> Option<&u64> {
        self.value5_timensbucket01128µs.as_ref()
    }

    /// Sets the value of Value5Timensbucket02256µs
    pub fn set_value5_timensbucket02256µs(&mut self, value: u64) {
        self.value5_timensbucket02256µs = Some(value);
    }

    /// Gets the value of Value5Timensbucket02256µs
    pub fn get_value5_timensbucket02256µs(&self) -> Option<&u64> {
        self.value5_timensbucket02256µs.as_ref()
    }

    /// Sets the value of Value5Timensbucket03512µs
    pub fn set_value5_timensbucket03512µs(&mut self, value: u64) {
        self.value5_timensbucket03512µs = Some(value);
    }

    /// Gets the value of Value5Timensbucket03512µs
    pub fn get_value5_timensbucket03512µs(&self) -> Option<&u64> {
        self.value5_timensbucket03512µs.as_ref()
    }

    /// Sets the value of Value5Timensbucket041ms
    pub fn set_value5_timensbucket041ms(&mut self, value: u64) {
        self.value5_timensbucket041ms = Some(value);
    }

    /// Gets the value of Value5Timensbucket041ms
    pub fn get_value5_timensbucket041ms(&self) -> Option<&u64> {
        self.value5_timensbucket041ms.as_ref()
    }

    /// Sets the value of Value5Timensbucket054ms
    pub fn set_value5_timensbucket054ms(&mut self, value: u64) {
        self.value5_timensbucket054ms = Some(value);
    }

    /// Gets the value of Value5Timensbucket054ms
    pub fn get_value5_timensbucket054ms(&self) -> Option<&u64> {
        self.value5_timensbucket054ms.as_ref()
    }

    /// Sets the value of Value5Timensbucket0616ms
    pub fn set_value5_timensbucket0616ms(&mut self, value: u64) {
        self.value5_timensbucket0616ms = Some(value);
    }

    /// Gets the value of Value5Timensbucket0616ms
    pub fn get_value5_timensbucket0616ms(&self) -> Option<&u64> {
        self.value5_timensbucket0616ms.as_ref()
    }

    /// Sets the value of Value5Timensbucket0764ms
    pub fn set_value5_timensbucket0764ms(&mut self, value: u64) {
        self.value5_timensbucket0764ms = Some(value);
    }

    /// Gets the value of Value5Timensbucket0764ms
    pub fn get_value5_timensbucket0764ms(&self) -> Option<&u64> {
        self.value5_timensbucket0764ms.as_ref()
    }

    /// Sets the value of Value5Timensbucket08128ms
    pub fn set_value5_timensbucket08128ms(&mut self, value: u64) {
        self.value5_timensbucket08128ms = Some(value);
    }

    /// Gets the value of Value5Timensbucket08128ms
    pub fn get_value5_timensbucket08128ms(&self) -> Option<&u64> {
        self.value5_timensbucket08128ms.as_ref()
    }

    /// Sets the value of Value5Timensbucket09256ms
    pub fn set_value5_timensbucket09256ms(&mut self, value: u64) {
        self.value5_timensbucket09256ms = Some(value);
    }

    /// Gets the value of Value5Timensbucket09256ms
    pub fn get_value5_timensbucket09256ms(&self) -> Option<&u64> {
        self.value5_timensbucket09256ms.as_ref()
    }

    /// Sets the value of Value5Timensbucket10512ms
    pub fn set_value5_timensbucket10512ms(&mut self, value: u64) {
        self.value5_timensbucket10512ms = Some(value);
    }

    /// Gets the value of Value5Timensbucket10512ms
    pub fn get_value5_timensbucket10512ms(&self) -> Option<&u64> {
        self.value5_timensbucket10512ms.as_ref()
    }

    /// Sets the value of Value5Timensbucket111s
    pub fn set_value5_timensbucket111s(&mut self, value: u64) {
        self.value5_timensbucket111s = Some(value);
    }

    /// Gets the value of Value5Timensbucket111s
    pub fn get_value5_timensbucket111s(&self) -> Option<&u64> {
        self.value5_timensbucket111s.as_ref()
    }

    /// Sets the value of Value5Timensbucket122s
    pub fn set_value5_timensbucket122s(&mut self, value: u64) {
        self.value5_timensbucket122s = Some(value);
    }

    /// Gets the value of Value5Timensbucket122s
    pub fn get_value5_timensbucket122s(&self) -> Option<&u64> {
        self.value5_timensbucket122s.as_ref()
    }

    /// Sets the value of Value5Timensbucket1310s
    pub fn set_value5_timensbucket1310s(&mut self, value: u64) {
        self.value5_timensbucket1310s = Some(value);
    }

    /// Gets the value of Value5Timensbucket1310s
    pub fn get_value5_timensbucket1310s(&self) -> Option<&u64> {
        self.value5_timensbucket1310s.as_ref()
    }

    /// Sets the value of Value5Timensbucket1420s
    pub fn set_value5_timensbucket1420s(&mut self, value: u64) {
        self.value5_timensbucket1420s = Some(value);
    }

    /// Gets the value of Value5Timensbucket1420s
    pub fn get_value5_timensbucket1420s(&self) -> Option<&u64> {
        self.value5_timensbucket1420s.as_ref()
    }

    /// Sets the value of Value5Timensbucket1530s
    pub fn set_value5_timensbucket1530s(&mut self, value: u64) {
        self.value5_timensbucket1530s = Some(value);
    }

    /// Gets the value of Value5Timensbucket1530s
    pub fn get_value5_timensbucket1530s(&self) -> Option<&u64> {
        self.value5_timensbucket1530s.as_ref()
    }

    /// Sets the value of Value5Timensbucket1630s
    pub fn set_value5_timensbucket1630s(&mut self, value: u64) {
        self.value5_timensbucket1630s = Some(value);
    }

    /// Gets the value of Value5Timensbucket1630s
    pub fn get_value5_timensbucket1630s(&self) -> Option<&u64> {
        self.value5_timensbucket1630s.as_ref()
    }

    /// Sets the value of Value6Bytesbucket01128µs
    pub fn set_value6_bytesbucket01128µs(&mut self, value: u64) {
        self.value6_bytesbucket01128µs = Some(value);
    }

    /// Gets the value of Value6Bytesbucket01128µs
    pub fn get_value6_bytesbucket01128µs(&self) -> Option<&u64> {
        self.value6_bytesbucket01128µs.as_ref()
    }

    /// Sets the value of Value6Bytesbucket02256µs
    pub fn set_value6_bytesbucket02256µs(&mut self, value: u64) {
        self.value6_bytesbucket02256µs = Some(value);
    }

    /// Gets the value of Value6Bytesbucket02256µs
    pub fn get_value6_bytesbucket02256µs(&self) -> Option<&u64> {
        self.value6_bytesbucket02256µs.as_ref()
    }

    /// Sets the value of Value6Bytesbucket03512µs
    pub fn set_value6_bytesbucket03512µs(&mut self, value: u64) {
        self.value6_bytesbucket03512µs = Some(value);
    }

    /// Gets the value of Value6Bytesbucket03512µs
    pub fn get_value6_bytesbucket03512µs(&self) -> Option<&u64> {
        self.value6_bytesbucket03512µs.as_ref()
    }

    /// Sets the value of Value6Bytesbucket041ms
    pub fn set_value6_bytesbucket041ms(&mut self, value: u64) {
        self.value6_bytesbucket041ms = Some(value);
    }

    /// Gets the value of Value6Bytesbucket041ms
    pub fn get_value6_bytesbucket041ms(&self) -> Option<&u64> {
        self.value6_bytesbucket041ms.as_ref()
    }

    /// Sets the value of Value6Bytesbucket054ms
    pub fn set_value6_bytesbucket054ms(&mut self, value: u64) {
        self.value6_bytesbucket054ms = Some(value);
    }

    /// Gets the value of Value6Bytesbucket054ms
    pub fn get_value6_bytesbucket054ms(&self) -> Option<&u64> {
        self.value6_bytesbucket054ms.as_ref()
    }

    /// Sets the value of Value6Bytesbucket0616ms
    pub fn set_value6_bytesbucket0616ms(&mut self, value: u64) {
        self.value6_bytesbucket0616ms = Some(value);
    }

    /// Gets the value of Value6Bytesbucket0616ms
    pub fn get_value6_bytesbucket0616ms(&self) -> Option<&u64> {
        self.value6_bytesbucket0616ms.as_ref()
    }

    /// Sets the value of Value6Bytesbucket0764ms
    pub fn set_value6_bytesbucket0764ms(&mut self, value: u64) {
        self.value6_bytesbucket0764ms = Some(value);
    }

    /// Gets the value of Value6Bytesbucket0764ms
    pub fn get_value6_bytesbucket0764ms(&self) -> Option<&u64> {
        self.value6_bytesbucket0764ms.as_ref()
    }

    /// Sets the value of Value6Bytesbucket08128ms
    pub fn set_value6_bytesbucket08128ms(&mut self, value: u64) {
        self.value6_bytesbucket08128ms = Some(value);
    }

    /// Gets the value of Value6Bytesbucket08128ms
    pub fn get_value6_bytesbucket08128ms(&self) -> Option<&u64> {
        self.value6_bytesbucket08128ms.as_ref()
    }

    /// Sets the value of Value6Bytesbucket09256ms
    pub fn set_value6_bytesbucket09256ms(&mut self, value: u64) {
        self.value6_bytesbucket09256ms = Some(value);
    }

    /// Gets the value of Value6Bytesbucket09256ms
    pub fn get_value6_bytesbucket09256ms(&self) -> Option<&u64> {
        self.value6_bytesbucket09256ms.as_ref()
    }

    /// Sets the value of Value6Bytesbucket10512ms
    pub fn set_value6_bytesbucket10512ms(&mut self, value: u64) {
        self.value6_bytesbucket10512ms = Some(value);
    }

    /// Gets the value of Value6Bytesbucket10512ms
    pub fn get_value6_bytesbucket10512ms(&self) -> Option<&u64> {
        self.value6_bytesbucket10512ms.as_ref()
    }

    /// Sets the value of Value6Bytesbucket111s
    pub fn set_value6_bytesbucket111s(&mut self, value: u64) {
        self.value6_bytesbucket111s = Some(value);
    }

    /// Gets the value of Value6Bytesbucket111s
    pub fn get_value6_bytesbucket111s(&self) -> Option<&u64> {
        self.value6_bytesbucket111s.as_ref()
    }

    /// Sets the value of Value6Bytesbucket122s
    pub fn set_value6_bytesbucket122s(&mut self, value: u64) {
        self.value6_bytesbucket122s = Some(value);
    }

    /// Gets the value of Value6Bytesbucket122s
    pub fn get_value6_bytesbucket122s(&self) -> Option<&u64> {
        self.value6_bytesbucket122s.as_ref()
    }

    /// Sets the value of Value6Bytesbucket1310s
    pub fn set_value6_bytesbucket1310s(&mut self, value: u64) {
        self.value6_bytesbucket1310s = Some(value);
    }

    /// Gets the value of Value6Bytesbucket1310s
    pub fn get_value6_bytesbucket1310s(&self) -> Option<&u64> {
        self.value6_bytesbucket1310s.as_ref()
    }

    /// Sets the value of Value6Bytesbucket1420s
    pub fn set_value6_bytesbucket1420s(&mut self, value: u64) {
        self.value6_bytesbucket1420s = Some(value);
    }

    /// Gets the value of Value6Bytesbucket1420s
    pub fn get_value6_bytesbucket1420s(&self) -> Option<&u64> {
        self.value6_bytesbucket1420s.as_ref()
    }

    /// Sets the value of Value6Bytesbucket1530s
    pub fn set_value6_bytesbucket1530s(&mut self, value: u64) {
        self.value6_bytesbucket1530s = Some(value);
    }

    /// Gets the value of Value6Bytesbucket1530s
    pub fn get_value6_bytesbucket1530s(&self) -> Option<&u64> {
        self.value6_bytesbucket1530s.as_ref()
    }

    /// Sets the value of Value6Bytesbucket1630s
    pub fn set_value6_bytesbucket1630s(&mut self, value: u64) {
        self.value6_bytesbucket1630s = Some(value);
    }

    /// Gets the value of Value6Bytesbucket1630s
    pub fn get_value6_bytesbucket1630s(&self) -> Option<&u64> {
        self.value6_bytesbucket1630s.as_ref()
    }
}

