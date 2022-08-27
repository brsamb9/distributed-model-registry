// This file is @generated by wasmcloud/weld-codegen 0.4.6.
// It is not intended for manual editing.
// namespace: org.example.interfaces.modelregistry

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnrichedModelMetadata {
    #[serde(rename = "modelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "trainingTime")]
    #[serde(default)]
    pub training_time: u64,
    #[serde(rename = "uploadTimestamp")]
    #[serde(default)]
    pub upload_timestamp: u64,
}

// Encode EnrichedModelMetadata as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_enriched_model_metadata<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &EnrichedModelMetadata,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    e.str("modelName")?;
    e.str(&val.model_name)?;
    e.str("trainingTime")?;
    e.u64(val.training_time)?;
    e.str("uploadTimestamp")?;
    e.u64(val.upload_timestamp)?;
    Ok(())
}

// Decode EnrichedModelMetadata from cbor input stream
#[doc(hidden)]
pub fn decode_enriched_model_metadata(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<EnrichedModelMetadata, RpcError> {
    let __result = {
        let mut model_name: Option<String> = None;
        let mut training_time: Option<u64> = None;
        let mut upload_timestamp: Option<u64> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct EnrichedModelMetadata, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => model_name = Some(d.str()?.to_string()),
                    1 => training_time = Some(d.u64()?),
                    2 => upload_timestamp = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "modelName" => model_name = Some(d.str()?.to_string()),
                    "trainingTime" => training_time = Some(d.u64()?),
                    "uploadTimestamp" => upload_timestamp = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        }
        EnrichedModelMetadata {
            model_name: if let Some(__x) = model_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field EnrichedModelMetadata.model_name (#0)".to_string(),
                ));
            },

            training_time: if let Some(__x) = training_time {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field EnrichedModelMetadata.training_time (#1)".to_string(),
                ));
            },

            upload_timestamp: if let Some(__x) = upload_timestamp {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field EnrichedModelMetadata.upload_timestamp (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModelMetadata {
    #[serde(rename = "modelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "trainingTime")]
    #[serde(default)]
    pub training_time: u64,
}

// Encode ModelMetadata as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_model_metadata<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ModelMetadata,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("modelName")?;
    e.str(&val.model_name)?;
    e.str("trainingTime")?;
    e.u64(val.training_time)?;
    Ok(())
}

// Decode ModelMetadata from cbor input stream
#[doc(hidden)]
pub fn decode_model_metadata(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ModelMetadata, RpcError> {
    let __result = {
        let mut model_name: Option<String> = None;
        let mut training_time: Option<u64> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ModelMetadata, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => model_name = Some(d.str()?.to_string()),
                    1 => training_time = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "modelName" => model_name = Some(d.str()?.to_string()),
                    "trainingTime" => training_time = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        }
        ModelMetadata {
            model_name: if let Some(__x) = model_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ModelMetadata.model_name (#0)".to_string(),
                ));
            },

            training_time: if let Some(__x) = training_time {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ModelMetadata.training_time (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModelMetadataRequest {
    #[serde(rename = "modelMetadata")]
    pub model_metadata: ModelMetadata,
    pub storage: Storage,
}

// Encode ModelMetadataRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_model_metadata_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ModelMetadataRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("modelMetadata")?;
    encode_model_metadata(e, &val.model_metadata)?;
    e.str("storage")?;
    encode_storage(e, &val.storage)?;
    Ok(())
}

// Decode ModelMetadataRequest from cbor input stream
#[doc(hidden)]
pub fn decode_model_metadata_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ModelMetadataRequest, RpcError> {
    let __result = {
        let mut model_metadata: Option<ModelMetadata> = None;
        let mut storage: Option<Storage> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ModelMetadataRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        model_metadata = Some(decode_model_metadata(d).map_err(|e| {
                            format!(
                                "decoding 'org.example.interfaces.modelregistry#ModelMetadata': {}",
                                e
                            )
                        })?)
                    }
                    1 => {
                        storage = Some(decode_storage(d).map_err(|e| {
                            format!(
                                "decoding 'org.example.interfaces.modelregistry#Storage': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "modelMetadata" => {
                        model_metadata = Some(decode_model_metadata(d).map_err(|e| {
                            format!(
                                "decoding 'org.example.interfaces.modelregistry#ModelMetadata': {}",
                                e
                            )
                        })?)
                    }
                    "storage" => {
                        storage = Some(decode_storage(d).map_err(|e| {
                            format!(
                                "decoding 'org.example.interfaces.modelregistry#Storage': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        ModelMetadataRequest {
            model_metadata: if let Some(__x) = model_metadata {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ModelMetadataRequest.model_metadata (#0)".to_string(),
                ));
            },

            storage: if let Some(__x) = storage {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ModelMetadataRequest.storage (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModelMetadataResponse {
    #[serde(rename = "modelMetadata")]
    pub model_metadata: EnrichedModelMetadata,
}

// Encode ModelMetadataResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_model_metadata_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ModelMetadataResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    e.str("modelMetadata")?;
    encode_enriched_model_metadata(e, &val.model_metadata)?;
    Ok(())
}

// Decode ModelMetadataResponse from cbor input stream
#[doc(hidden)]
pub fn decode_model_metadata_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ModelMetadataResponse, RpcError> {
    let __result = {
        let mut model_metadata: Option<EnrichedModelMetadata> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ModelMetadataResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
            0 => model_metadata = Some(decode_enriched_model_metadata(d).map_err(|e| format!("decoding 'org.example.interfaces.modelregistry#EnrichedModelMetadata': {}", e))?),
                    _ => d.skip()?,
                    }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                "modelMetadata" => model_metadata = Some(decode_enriched_model_metadata(d).map_err(|e| format!("decoding 'org.example.interfaces.modelregistry#EnrichedModelMetadata': {}", e))?),         _ => d.skip()?,
                    }
            }
        }
        ModelMetadataResponse {
            model_metadata: if let Some(__x) = model_metadata {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ModelMetadataResponse.model_metadata (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Storage {
    #[serde(rename = "containerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
}

// Encode Storage as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_storage<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Storage,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("containerName")?;
    e.str(&val.container_name)?;
    e.str("fileName")?;
    e.str(&val.file_name)?;
    Ok(())
}

// Decode Storage from cbor input stream
#[doc(hidden)]
pub fn decode_storage(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Storage, RpcError> {
    let __result = {
        let mut container_name: Option<String> = None;
        let mut file_name: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Storage, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => container_name = Some(d.str()?.to_string()),
                    1 => file_name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "containerName" => container_name = Some(d.str()?.to_string()),
                    "fileName" => file_name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Storage {
            container_name: if let Some(__x) = container_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Storage.container_name (#0)".to_string(),
                ));
            },

            file_name: if let Some(__x) = file_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Storage.file_name (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Description of Modelregistry service
/// wasmbus.actorReceive
#[async_trait]
pub trait Modelregistry {
    async fn create_metadata(&self, ctx: &Context, arg: &ModelMetadataRequest) -> RpcResult<()>;
    async fn read_metadata(&self, ctx: &Context, arg: &Storage)
        -> RpcResult<ModelMetadataResponse>;
    async fn update_metadata(&self, ctx: &Context, arg: &ModelMetadataRequest) -> RpcResult<()>;
    async fn delete_metadata(&self, ctx: &Context, arg: &ModelMetadataRequest) -> RpcResult<()>;
}

/// ModelregistryReceiver receives messages defined in the Modelregistry service trait
/// Description of Modelregistry service
#[doc(hidden)]
#[async_trait]
pub trait ModelregistryReceiver: MessageDispatch + Modelregistry {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "CreateMetadata" => {
                let value: ModelMetadataRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ModelMetadataRequest': {}", e)))?;

                let _resp = Modelregistry::create_metadata(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "Modelregistry.CreateMetadata",
                    arg: Cow::Owned(buf),
                })
            }
            "ReadMetadata" => {
                let value: Storage = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'Storage': {}", e)))?;

                let resp = Modelregistry::read_metadata(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(Message {
                    method: "Modelregistry.ReadMetadata",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdateMetadata" => {
                let value: ModelMetadataRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ModelMetadataRequest': {}", e)))?;

                let _resp = Modelregistry::update_metadata(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "Modelregistry.UpdateMetadata",
                    arg: Cow::Owned(buf),
                })
            }
            "DeleteMetadata" => {
                let value: ModelMetadataRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ModelMetadataRequest': {}", e)))?;

                let _resp = Modelregistry::delete_metadata(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "Modelregistry.DeleteMetadata",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Modelregistry::{}",
                message.method
            ))),
        }
    }
}

/// ModelregistrySender sends messages to a Modelregistry service
/// Description of Modelregistry service
/// client for sending Modelregistry messages
#[derive(Debug)]
pub struct ModelregistrySender<T: Transport> {
    transport: T,
}

impl<T: Transport> ModelregistrySender<T> {
    /// Constructs a ModelregistrySender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ModelregistrySender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ModelregistrySender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Modelregistry
    for ModelregistrySender<T>
{
    #[allow(unused)]
    async fn create_metadata(&self, ctx: &Context, arg: &ModelMetadataRequest) -> RpcResult<()> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Modelregistry.CreateMetadata",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    async fn read_metadata(
        &self,
        ctx: &Context,
        arg: &Storage,
    ) -> RpcResult<ModelMetadataResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Modelregistry.ReadMetadata",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ModelMetadataResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': ModelMetadataResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn update_metadata(&self, ctx: &Context, arg: &ModelMetadataRequest) -> RpcResult<()> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Modelregistry.UpdateMetadata",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    async fn delete_metadata(&self, ctx: &Context, arg: &ModelMetadataRequest) -> RpcResult<()> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Modelregistry.DeleteMetadata",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
}
