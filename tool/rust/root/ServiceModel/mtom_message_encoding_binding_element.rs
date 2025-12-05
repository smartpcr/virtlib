// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MtomMessageEncodingBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MtomMessageEncodingBindingElement {
    #[serde(flatten)]
    pub base: MessageEncodingBindingElement,

/// The character set encoding to be used for emitting messages on the binding.
    #[serde(rename = "Encoding")]
    pub encoding: Option<String>,

/// An integer that defines how many messages can be read simultaneously without allocating new readers. 
    #[serde(rename = "MaxReadPoolSize")]
    pub max_read_pool_size: Option<i32>,

/// An integer that defines how many messages can be sent simultaneously without allocating new writers.
    #[serde(rename = "MaxWritePoolSize")]
    pub max_write_pool_size: Option<i32>,

/// The quotas of the readers.
    #[serde(rename = "ReaderQuotas")]
    pub reader_quotas: Option<XmlDictionaryReaderQuotas>,
}

impl MtomMessageEncodingBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MessageEncodingBindingElement::new(),
            encoding: None,
            max_read_pool_size: None,
            max_write_pool_size: None,
            reader_quotas: None,
        }
    }


    /// Sets the value of Encoding
    pub fn set_encoding(&mut self, value: String) {
        self.encoding = Some(value);
    }

    /// Gets the value of Encoding
    pub fn get_encoding(&self) -> Option<&String> {
        self.encoding.as_ref()
    }

    /// Sets the value of MaxReadPoolSize
    pub fn set_max_read_pool_size(&mut self, value: i32) {
        self.max_read_pool_size = Some(value);
    }

    /// Gets the value of MaxReadPoolSize
    pub fn get_max_read_pool_size(&self) -> Option<&i32> {
        self.max_read_pool_size.as_ref()
    }

    /// Sets the value of MaxWritePoolSize
    pub fn set_max_write_pool_size(&mut self, value: i32) {
        self.max_write_pool_size = Some(value);
    }

    /// Gets the value of MaxWritePoolSize
    pub fn get_max_write_pool_size(&self) -> Option<&i32> {
        self.max_write_pool_size.as_ref()
    }

    /// Sets the value of ReaderQuotas
    pub fn set_reader_quotas(&mut self, value: XmlDictionaryReaderQuotas) {
        self.reader_quotas = Some(value);
    }

    /// Gets the value of ReaderQuotas
    pub fn get_reader_quotas(&self) -> Option<&XmlDictionaryReaderQuotas> {
        self.reader_quotas.as_ref()
    }
}

