// modelregistry.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.example.interfaces.modelregistry", crate: "model_registry_interface" } ]

namespace org.example.interfaces.modelregistry

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

/// Description of Modelregistry service
@wasmbus( actorReceive: true )
service Modelregistry {
  version: "0.1",
  operations: [ CreateMetadata, ReadMetadata, UpdateMetadata, DeleteMetadata ]
}

// Shame I can't use @mixin here

structure ModelMetadata {
  @required
  modelName: String,
  @required
  trainingTime: U64,
}


structure EnrichedModelMetadata {
  // Base ModelMetadata
  @required
  modelName: String,
  @required
  trainingTime: U64,
  // Extra
  @required
  uploadTimestamp: U64
}

structure Storage {
  @required
  containerName: String,
  @required
  fileName: String,
}

structure ModelMetadataRequest {
  @required
  storage: Storage,
  @required
  modelMetadata: ModelMetadata
}

structure ModelMetadataResponse {
  @required
  modelMetadata: EnrichedModelMetadata
}


// CRUD Operations

operation CreateMetadata {
  input: ModelMetadataRequest
}

operation ReadMetadata {
  input: Storage
  output: ModelMetadataResponse
}

operation UpdateMetadata {
  input: ModelMetadataRequest
}

operation DeleteMetadata {
  input: ModelMetadataRequest
}