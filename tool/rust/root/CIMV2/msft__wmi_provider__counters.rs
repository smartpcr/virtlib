// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_Counters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_Counters {

/// 
    #[serde(rename = "ProviderOperation_AccessCheck")]
    pub provider_operation__access_check: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_CancelQuery")]
    pub provider_operation__cancel_query: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_CreateClassEnumAsync")]
    pub provider_operation__create_class_enum_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_CreateInstanceEnumAsync")]
    pub provider_operation__create_instance_enum_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_CreateRefreshableEnum")]
    pub provider_operation__create_refreshable_enum: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_CreateRefreshableObject")]
    pub provider_operation__create_refreshable_object: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_CreateRefresher")]
    pub provider_operation__create_refresher: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_DeleteClassAsync")]
    pub provider_operation__delete_class_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_DeleteInstanceAsync")]
    pub provider_operation__delete_instance_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_ExecMethodAsync")]
    pub provider_operation__exec_method_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_ExecQueryAsync")]
    pub provider_operation__exec_query_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_FindConsumer")]
    pub provider_operation__find_consumer: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_GetObjectAsync")]
    pub provider_operation__get_object_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_GetObjects")]
    pub provider_operation__get_objects: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_GetProperty")]
    pub provider_operation__get_property: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_NewQuery")]
    pub provider_operation__new_query: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_ProvideEvents")]
    pub provider_operation__provide_events: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_PutClassAsync")]
    pub provider_operation__put_class_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_PutInstanceAsync")]
    pub provider_operation__put_instance_async: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_PutProperty")]
    pub provider_operation__put_property: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_QueryInstances")]
    pub provider_operation__query_instances: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_SetRegistrationObject")]
    pub provider_operation__set_registration_object: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_StopRefreshing")]
    pub provider_operation__stop_refreshing: Option<u64>,

/// 
    #[serde(rename = "ProviderOperation_ValidateSubscription")]
    pub provider_operation__validate_subscription: Option<u64>,
}

impl Msft_WmiProvider_Counters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            provider_operation__access_check: None,
            provider_operation__cancel_query: None,
            provider_operation__create_class_enum_async: None,
            provider_operation__create_instance_enum_async: None,
            provider_operation__create_refreshable_enum: None,
            provider_operation__create_refreshable_object: None,
            provider_operation__create_refresher: None,
            provider_operation__delete_class_async: None,
            provider_operation__delete_instance_async: None,
            provider_operation__exec_method_async: None,
            provider_operation__exec_query_async: None,
            provider_operation__find_consumer: None,
            provider_operation__get_object_async: None,
            provider_operation__get_objects: None,
            provider_operation__get_property: None,
            provider_operation__new_query: None,
            provider_operation__provide_events: None,
            provider_operation__put_class_async: None,
            provider_operation__put_instance_async: None,
            provider_operation__put_property: None,
            provider_operation__query_instances: None,
            provider_operation__set_registration_object: None,
            provider_operation__stop_refreshing: None,
            provider_operation__validate_subscription: None,
        }
    }


    /// Sets the value of ProviderOperation_AccessCheck
    pub fn set_provider_operation__access_check(&mut self, value: u64) {
        self.provider_operation__access_check = Some(value);
    }

    /// Gets the value of ProviderOperation_AccessCheck
    pub fn get_provider_operation__access_check(&self) -> Option<&u64> {
        self.provider_operation__access_check.as_ref()
    }

    /// Sets the value of ProviderOperation_CancelQuery
    pub fn set_provider_operation__cancel_query(&mut self, value: u64) {
        self.provider_operation__cancel_query = Some(value);
    }

    /// Gets the value of ProviderOperation_CancelQuery
    pub fn get_provider_operation__cancel_query(&self) -> Option<&u64> {
        self.provider_operation__cancel_query.as_ref()
    }

    /// Sets the value of ProviderOperation_CreateClassEnumAsync
    pub fn set_provider_operation__create_class_enum_async(&mut self, value: u64) {
        self.provider_operation__create_class_enum_async = Some(value);
    }

    /// Gets the value of ProviderOperation_CreateClassEnumAsync
    pub fn get_provider_operation__create_class_enum_async(&self) -> Option<&u64> {
        self.provider_operation__create_class_enum_async.as_ref()
    }

    /// Sets the value of ProviderOperation_CreateInstanceEnumAsync
    pub fn set_provider_operation__create_instance_enum_async(&mut self, value: u64) {
        self.provider_operation__create_instance_enum_async = Some(value);
    }

    /// Gets the value of ProviderOperation_CreateInstanceEnumAsync
    pub fn get_provider_operation__create_instance_enum_async(&self) -> Option<&u64> {
        self.provider_operation__create_instance_enum_async.as_ref()
    }

    /// Sets the value of ProviderOperation_CreateRefreshableEnum
    pub fn set_provider_operation__create_refreshable_enum(&mut self, value: u64) {
        self.provider_operation__create_refreshable_enum = Some(value);
    }

    /// Gets the value of ProviderOperation_CreateRefreshableEnum
    pub fn get_provider_operation__create_refreshable_enum(&self) -> Option<&u64> {
        self.provider_operation__create_refreshable_enum.as_ref()
    }

    /// Sets the value of ProviderOperation_CreateRefreshableObject
    pub fn set_provider_operation__create_refreshable_object(&mut self, value: u64) {
        self.provider_operation__create_refreshable_object = Some(value);
    }

    /// Gets the value of ProviderOperation_CreateRefreshableObject
    pub fn get_provider_operation__create_refreshable_object(&self) -> Option<&u64> {
        self.provider_operation__create_refreshable_object.as_ref()
    }

    /// Sets the value of ProviderOperation_CreateRefresher
    pub fn set_provider_operation__create_refresher(&mut self, value: u64) {
        self.provider_operation__create_refresher = Some(value);
    }

    /// Gets the value of ProviderOperation_CreateRefresher
    pub fn get_provider_operation__create_refresher(&self) -> Option<&u64> {
        self.provider_operation__create_refresher.as_ref()
    }

    /// Sets the value of ProviderOperation_DeleteClassAsync
    pub fn set_provider_operation__delete_class_async(&mut self, value: u64) {
        self.provider_operation__delete_class_async = Some(value);
    }

    /// Gets the value of ProviderOperation_DeleteClassAsync
    pub fn get_provider_operation__delete_class_async(&self) -> Option<&u64> {
        self.provider_operation__delete_class_async.as_ref()
    }

    /// Sets the value of ProviderOperation_DeleteInstanceAsync
    pub fn set_provider_operation__delete_instance_async(&mut self, value: u64) {
        self.provider_operation__delete_instance_async = Some(value);
    }

    /// Gets the value of ProviderOperation_DeleteInstanceAsync
    pub fn get_provider_operation__delete_instance_async(&self) -> Option<&u64> {
        self.provider_operation__delete_instance_async.as_ref()
    }

    /// Sets the value of ProviderOperation_ExecMethodAsync
    pub fn set_provider_operation__exec_method_async(&mut self, value: u64) {
        self.provider_operation__exec_method_async = Some(value);
    }

    /// Gets the value of ProviderOperation_ExecMethodAsync
    pub fn get_provider_operation__exec_method_async(&self) -> Option<&u64> {
        self.provider_operation__exec_method_async.as_ref()
    }

    /// Sets the value of ProviderOperation_ExecQueryAsync
    pub fn set_provider_operation__exec_query_async(&mut self, value: u64) {
        self.provider_operation__exec_query_async = Some(value);
    }

    /// Gets the value of ProviderOperation_ExecQueryAsync
    pub fn get_provider_operation__exec_query_async(&self) -> Option<&u64> {
        self.provider_operation__exec_query_async.as_ref()
    }

    /// Sets the value of ProviderOperation_FindConsumer
    pub fn set_provider_operation__find_consumer(&mut self, value: u64) {
        self.provider_operation__find_consumer = Some(value);
    }

    /// Gets the value of ProviderOperation_FindConsumer
    pub fn get_provider_operation__find_consumer(&self) -> Option<&u64> {
        self.provider_operation__find_consumer.as_ref()
    }

    /// Sets the value of ProviderOperation_GetObjectAsync
    pub fn set_provider_operation__get_object_async(&mut self, value: u64) {
        self.provider_operation__get_object_async = Some(value);
    }

    /// Gets the value of ProviderOperation_GetObjectAsync
    pub fn get_provider_operation__get_object_async(&self) -> Option<&u64> {
        self.provider_operation__get_object_async.as_ref()
    }

    /// Sets the value of ProviderOperation_GetObjects
    pub fn set_provider_operation__get_objects(&mut self, value: u64) {
        self.provider_operation__get_objects = Some(value);
    }

    /// Gets the value of ProviderOperation_GetObjects
    pub fn get_provider_operation__get_objects(&self) -> Option<&u64> {
        self.provider_operation__get_objects.as_ref()
    }

    /// Sets the value of ProviderOperation_GetProperty
    pub fn set_provider_operation__get_property(&mut self, value: u64) {
        self.provider_operation__get_property = Some(value);
    }

    /// Gets the value of ProviderOperation_GetProperty
    pub fn get_provider_operation__get_property(&self) -> Option<&u64> {
        self.provider_operation__get_property.as_ref()
    }

    /// Sets the value of ProviderOperation_NewQuery
    pub fn set_provider_operation__new_query(&mut self, value: u64) {
        self.provider_operation__new_query = Some(value);
    }

    /// Gets the value of ProviderOperation_NewQuery
    pub fn get_provider_operation__new_query(&self) -> Option<&u64> {
        self.provider_operation__new_query.as_ref()
    }

    /// Sets the value of ProviderOperation_ProvideEvents
    pub fn set_provider_operation__provide_events(&mut self, value: u64) {
        self.provider_operation__provide_events = Some(value);
    }

    /// Gets the value of ProviderOperation_ProvideEvents
    pub fn get_provider_operation__provide_events(&self) -> Option<&u64> {
        self.provider_operation__provide_events.as_ref()
    }

    /// Sets the value of ProviderOperation_PutClassAsync
    pub fn set_provider_operation__put_class_async(&mut self, value: u64) {
        self.provider_operation__put_class_async = Some(value);
    }

    /// Gets the value of ProviderOperation_PutClassAsync
    pub fn get_provider_operation__put_class_async(&self) -> Option<&u64> {
        self.provider_operation__put_class_async.as_ref()
    }

    /// Sets the value of ProviderOperation_PutInstanceAsync
    pub fn set_provider_operation__put_instance_async(&mut self, value: u64) {
        self.provider_operation__put_instance_async = Some(value);
    }

    /// Gets the value of ProviderOperation_PutInstanceAsync
    pub fn get_provider_operation__put_instance_async(&self) -> Option<&u64> {
        self.provider_operation__put_instance_async.as_ref()
    }

    /// Sets the value of ProviderOperation_PutProperty
    pub fn set_provider_operation__put_property(&mut self, value: u64) {
        self.provider_operation__put_property = Some(value);
    }

    /// Gets the value of ProviderOperation_PutProperty
    pub fn get_provider_operation__put_property(&self) -> Option<&u64> {
        self.provider_operation__put_property.as_ref()
    }

    /// Sets the value of ProviderOperation_QueryInstances
    pub fn set_provider_operation__query_instances(&mut self, value: u64) {
        self.provider_operation__query_instances = Some(value);
    }

    /// Gets the value of ProviderOperation_QueryInstances
    pub fn get_provider_operation__query_instances(&self) -> Option<&u64> {
        self.provider_operation__query_instances.as_ref()
    }

    /// Sets the value of ProviderOperation_SetRegistrationObject
    pub fn set_provider_operation__set_registration_object(&mut self, value: u64) {
        self.provider_operation__set_registration_object = Some(value);
    }

    /// Gets the value of ProviderOperation_SetRegistrationObject
    pub fn get_provider_operation__set_registration_object(&self) -> Option<&u64> {
        self.provider_operation__set_registration_object.as_ref()
    }

    /// Sets the value of ProviderOperation_StopRefreshing
    pub fn set_provider_operation__stop_refreshing(&mut self, value: u64) {
        self.provider_operation__stop_refreshing = Some(value);
    }

    /// Gets the value of ProviderOperation_StopRefreshing
    pub fn get_provider_operation__stop_refreshing(&self) -> Option<&u64> {
        self.provider_operation__stop_refreshing.as_ref()
    }

    /// Sets the value of ProviderOperation_ValidateSubscription
    pub fn set_provider_operation__validate_subscription(&mut self, value: u64) {
        self.provider_operation__validate_subscription = Some(value);
    }

    /// Gets the value of ProviderOperation_ValidateSubscription
    pub fn get_provider_operation__validate_subscription(&self) -> Option<&u64> {
        self.provider_operation__validate_subscription.as_ref()
    }
}

