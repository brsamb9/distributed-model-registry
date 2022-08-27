# Distributed Model registry

A model registry is important within the ML production workflow. It decouples model training and inference process (model that generates predictions), and is effectively a centralised tracking system that stores lineage, versioning and related metadata for published machine learning models.


Only have the very basics working so far. Very incomplete - much work.
- CRUD operations to save model metadata on the filesystem.

# Prerequisities
- wasmcloud, and their dependencies

# Start
Start NATS & wasmcloud host:
```
OTEL_TRACES_EXPORTER=otlp OTEL_EXPORTER_OTLP_ENDPOINT="https://api.honeycomb.io" OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team={API_TOKEN}" OTEL_SERVICE_NAME={NAME} \
	wash up --allowed-insecure registry:5000
```

# Todo
A lot.

1. wasmcloud manifest;
2. Extend capability




Resources:
[Model regitry medium article](https://medium.com/opendoor-labs/how-to-build-an-ml-model-registry-a-step-by-step-guide-from-opendoor-engineering-cee36d965937)
