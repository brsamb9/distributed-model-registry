use model_registry_interface::{ModelMetadataRequest, Modelregistry, ModelregistrySender, Storage};
use serde_json::json;
use std::collections::HashMap;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::info;

const BLOBSTORE_ACTOR: &str = "blobstore";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ApiActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ApiActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let segments: Vec<&str> = req.path[1..].split_terminator('/').collect();
        let query_map: HashMap<String, String> =
            form_urlencoded::parse(req.query_string.as_bytes())
                .into_owned()
                .collect();

        info!("Segments: {:?}", segments);
        info!("Query Map: {:?}", query_map);

        let body = match (req.method.as_ref(), segments.as_slice()) {
            // curl -H 'Content-Type: application/json' -X POST "localhost:8087/model/create" -d \
            //  '{"modelMetadata": { "modelName": "test", "trainingTime": 0 }, "storage": {"fileName": "file_test", "containerName": "container_test"}}'
            ("POST", ["model", "create"]) => {
                let sender = ModelregistrySender::to_actor(BLOBSTORE_ACTOR);
                let req_body: ModelMetadataRequest = serde_json::from_slice(req.body.as_slice())
                    .map_err(|e| {
                        RpcError::Other(format!(
                            "Post body can not be parsed into ModelMetadataRequest struct - {e}."
                        ))
                    })?;

                sender.create_metadata(ctx, &req_body).await?;
                json!({
                    "message":
                        format!(
                            "model created: {} in {}",
                            req_body.model_metadata.model_name, req_body.storage.file_name
                        )
                })
            }
            ("POST", ["model", "update"]) => {
                let sender = ModelregistrySender::to_actor(BLOBSTORE_ACTOR);
                let req_body: ModelMetadataRequest = serde_json::from_slice(req.body.as_slice())
                    .map_err(|e| {
                        RpcError::Other(format!(
                            "Post body can not be parsed into ModelMetadataRequest struct - {e}."
                        ))
                    })?;

                sender.update_metadata(ctx, &req_body).await?;
                json!({
                    "message":
                        format!(
                            "model created: {} in {}",
                            req_body.model_metadata.model_name, req_body.storage.file_name
                        )
                })
            }
            // curl "localhost:8087/model/get?file_name=file_test&container_name=container_test"
            ("GET", ["model", "get"]) => {
                let sender = ModelregistrySender::to_actor(BLOBSTORE_ACTOR);

                let file_name = query_map
                    .get("file_name")
                    .ok_or_else(|| {
                        RpcError::Other("'file_name' query parameter not provided.".into())
                    })?
                    .to_string();
                let container_name = query_map
                    .get("container_name")
                    .ok_or_else(|| {
                        RpcError::Other("'container_name' query parameter not provided.".into())
                    })?
                    .to_string();
                let storage: Storage = Storage {
                    container_name,
                    file_name,
                };

                info!("About to read metadata! w/ {storage:?}");
                let resp = sender.read_metadata(ctx, &storage).await?;

                // let body = bincode::serialize(&resp)
                //     .expect("bincode should be able to deserialise struct");
                // info!("Returning byte array");
                // body
                json!(resp)
            }
            ("DELETE", ["model", "delete"]) => {
                let sender = ModelregistrySender::to_actor(BLOBSTORE_ACTOR);
                let req_body: ModelMetadataRequest = serde_json::from_slice(req.body.as_slice())
                    .map_err(|e| {
                        RpcError::Other(format!(
                            "Post body can not be parsed into ModelMetadataRequest struct - {e}."
                        ))
                    })?;
                sender.delete_metadata(ctx, &req_body).await?;
                json!({"message": "Deleted"})
            }
            (_, _) => json!({"error": "unexpected method and/or path"}),
        };

        Ok(HttpResponse {
            body: body.to_string().into_bytes(),
            ..Default::default()
        })
    }
}

// impl ApiActor {
//     // Attempt at reducing DRY
//     async fn crud_parse_handler<T>(
//         &self,
//         ctx: &Context,
//         body: &[u8],
//         request_closure: fn(body: &ModelMetadataRequest) -> RpcResult<T>,
//     ) -> RpcResult<()> {
//         let sender = ModelregistrySender::to_actor(BLOBSTORE_ACTOR);
//         let req_body: ModelMetadataRequest = serde_json::from_slice(body).map_err(|e| {
//             RpcError::Other(format!(
//                 "Post body can not be parsed into ModelMetadataRequest struct - {e}."
//             ))
//         })?;
//         request_closure(&req_body);
//         Ok(())
//     }
// }
