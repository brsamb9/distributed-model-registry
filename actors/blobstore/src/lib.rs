// use std::time::SystemTime;

use model_registry_interface::{
    EnrichedModelMetadata, ModelMetadata, ModelMetadataRequest, ModelMetadataResponse,
    Modelregistry, ModelregistryReceiver, Storage,
};

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_blobstore::{
    Blobstore, BlobstoreSender, Chunk, ChunkReceiver, ChunkReceiverReceiver, ChunkResponse,
    ContainerObject, GetObjectRequest, ListObjectsRequest, PutObjectRequest, RemoveObjectsRequest,
};
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, ChunkReceiver, Modelregistry)]
struct BlobstoreActor {}

#[async_trait]
impl ChunkReceiver for BlobstoreActor {
    /// Receives a file chunk from a blobstore. This must be called AFTER
    /// the StartUpload operation.
    /// It is recommended to keep chunks under 1MB to not exceed wasm memory allocation
    async fn receive_chunk(&self, _ctx: &Context, chunk: &Chunk) -> RpcResult<ChunkResponse> {
        info!(
            "receive_chunk called: container = {:?}, object = {:?}",
            chunk.container_id, chunk.object_id
        );
        info!("Length: {:?}", chunk.bytes.len());

        Ok(ChunkResponse::default())
    }
}

#[async_trait]
impl Modelregistry for BlobstoreActor {
    async fn create_metadata(
        &self,
        ctx: &Context,
        metadata_request: &ModelMetadataRequest,
    ) -> RpcResult<()> {
        let ModelMetadataRequest {
            model_metadata,
            storage,
        } = metadata_request;
        let db = BlobstoreSender::new();
        let Storage {
            container_name,
            file_name,
        } = storage;

        info!("{ctx:?}");

        let object = ContainerObject {
            container_id: container_name.clone(),
            object_id: file_name.clone(),
        };
        if db.container_exists(ctx, container_name).await? && db.object_exists(ctx, &object).await?
        {
            return Err(RpcError::Other(format!(
                "Object {file_name} in {container_name} already exist- aborting"
            )));
        }

        info!("Creating container {container_name}");
        db.create_container(ctx, container_name).await?;

        info!("Enriching object {file_name}");
        let enriched_model_metadata = enrich_metadata(model_metadata);

        info!("Creating object {file_name}");
        let body = bincode::serialize::<EnrichedModelMetadata>(&enriched_model_metadata)
            .expect("bincode should be able to serialise struct with 'Serialize' trait");
        let chunk = Chunk {
            object_id: file_name.clone(),
            container_id: container_name.clone(),
            bytes: body,
            offset: 0,
            is_last: true,
        };
        let put_object = PutObjectRequest {
            chunk,
            ..Default::default()
        };
        db.put_object(ctx, &put_object).await?;

        Ok(())
    }
    async fn read_metadata(
        &self,
        ctx: &Context,
        storage: &Storage,
    ) -> RpcResult<ModelMetadataResponse> {
        let db = BlobstoreSender::new();
        let Storage {
            container_name,
            file_name,
        } = storage;

        info!("{ctx:?}");

        ensure_container_and_object_exists(ctx, &db, container_name, file_name).await?;

        info!("Retrieving object {file_name}");
        let get_object = GetObjectRequest {
            object_id: file_name.clone(),
            container_id: container_name.clone(),
            range_start: Some(0),
            range_end: None,
        };
        let resp = db.get_object(ctx, &get_object).await?;

        info!("{resp:?}");
        let chunk = resp
            .initial_chunk
            .expect("Object is expected to exist at this point");
        let model_metadata = bincode::deserialize::<EnrichedModelMetadata>(&chunk.bytes)
            .expect("Structure should be in the form of EnrichedModelMetadata");

        Ok(ModelMetadataResponse { model_metadata })
    }

    async fn update_metadata(
        &self,
        ctx: &Context,
        metadata_request: &ModelMetadataRequest,
    ) -> RpcResult<()> {
        let db = BlobstoreSender::new();
        let ModelMetadataRequest {
            model_metadata,
            storage,
        } = metadata_request;

        let Storage {
            container_name,
            file_name,
        } = storage;

        ensure_container_and_object_exists(ctx, &db, container_name, file_name).await?;

        info!("Enriching object {file_name}");
        let enriched_model_metadata = enrich_metadata(model_metadata);

        info!("Creating object {file_name}");
        let body = bincode::serialize::<EnrichedModelMetadata>(&enriched_model_metadata)
            .expect("bincode should be able to serialise struct with 'Serialize' trait");
        let chunk = Chunk {
            object_id: file_name.clone(),
            container_id: container_name.clone(),
            bytes: body,
            offset: 0,
            is_last: true,
        };
        let put_object = PutObjectRequest {
            chunk,
            ..Default::default()
        };
        db.put_object(ctx, &put_object).await?;

        Ok(())
    }
    async fn delete_metadata(
        &self,
        ctx: &Context,
        metadata_request: &ModelMetadataRequest,
    ) -> RpcResult<()> {
        let db = BlobstoreSender::new();
        let ModelMetadataRequest {
            model_metadata: _,
            storage,
        } = metadata_request;

        let Storage {
            container_name,
            file_name,
        } = storage;

        ensure_container_and_object_exists(ctx, &db, container_name, file_name).await?;

        info!("Removing object {file_name}");
        db.remove_objects(
            ctx,
            &RemoveObjectsRequest {
                container_id: container_name.clone(),
                objects: vec![file_name.clone()],
            },
        )
        .await?;

        // Just return one object to prove not empty.
        let remaining_objects = db
            .list_objects(
                ctx,
                &ListObjectsRequest {
                    container_id: container_name.clone(),
                    max_items: Some(1),
                    ..Default::default()
                },
            )
            .await?;
        if remaining_objects.objects.is_empty() {
            info!("No objects left, removing container {container_name}");
            db.remove_containers(ctx, &vec![container_name.clone()])
                .await?;
        }
        Ok(())
    }
}

async fn ensure_container_and_object_exists(
    ctx: &Context,
    db: &BlobstoreSender<WasmHost>,
    container_name: &String,
    file_name: &String,
) -> RpcResult<()> {
    info!("Checking object {file_name} exist in {container_name}");
    if !db.container_exists(ctx, container_name).await? {
        return Err(RpcError::Other(format!(
            "Container {container_name} doesn't exist"
        )));
    }
    if !db
        .object_exists(
            ctx,
            &ContainerObject {
                container_id: container_name.clone(),
                object_id: file_name.clone(),
            },
        )
        .await?
    {
        return Err(RpcError::Other(format!("Object {file_name} doesn't exist")));
    }
    Ok(())
}

// I'd rather do a From<T> conversion
fn enrich_metadata(model_metadata: &ModelMetadata) -> EnrichedModelMetadata {
    EnrichedModelMetadata {
        // Not sure time is supported yet
        // upload_timestamp: SystemTime::now()
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .expect("Invalid time comparison")
        //     .as_secs(),
        upload_timestamp: 0,
        training_time: model_metadata.training_time,
        model_name: model_metadata.model_name.clone(),
    }
}
