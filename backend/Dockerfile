FROM docker.io/library/rust:1.77.2-bullseye AS build

WORKDIR /app

RUN apt-get update \
    && apt-get install -y \
        libopencv-dev clang libclang-dev

# Build dependencies
COPY Cargo.toml Cargo.lock ./
COPY chimp_chomp/Cargo.toml chimp_chomp/Cargo.toml
COPY chimp_controller/Cargo.toml chimp_controller/Cargo.toml
COPY chimp_protocol/Cargo.toml chimp_protocol/Cargo.toml
COPY compound_library/Cargo.toml compound_library/Cargo.toml
COPY compound_soaking/Cargo.toml compound_soaking/Cargo.toml
COPY crystal_library/Cargo.toml crystal_library/Cargo.toml
COPY graphql_endpoints/Cargo.toml graphql_endpoints/Cargo.toml
COPY graphql_event_broker/Cargo.toml graphql_event_broker/Cargo.toml
COPY opa_client/Cargo.toml opa_client/Cargo.toml
COPY pin_packing/Cargo.toml pin_packing/Cargo.toml
COPY soakdb_io/Cargo.toml soakdb_io/Cargo.toml
COPY soakdb_sync/Cargo.toml soakdb_sync/Cargo.toml
COPY targeting/Cargo.toml targeting/Cargo.toml
COPY the_paginator/Cargo.toml the_paginator/Cargo.toml
RUN mkdir chimp_chomp/src \
    && echo "fn main() {}" > chimp_chomp/src/main.rs \
    && mkdir chimp_protocol/src \
    && touch chimp_protocol/src/lib.rs \
    && mkdir chimp_controller/src \
    && echo "fn main() {}" > chimp_controller/src/main.rs \
    && mkdir compound_library/src \
    && echo "fn main() {}" > compound_library/src/main.rs \
    && mkdir compound_soaking/src \
    && echo "fn main() {}" > compound_soaking/src/main.rs \
    && mkdir crystal_library/src \
    && echo "fn main() {}" > crystal_library/src/main.rs \
    && mkdir graphql_endpoints/src \
    && touch graphql_endpoints/src/lib.rs \
    && mkdir graphql_event_broker/src \
    && touch graphql_event_broker/src/lib.rs \
    && mkdir opa_client/src \
    && touch opa_client/src/lib.rs \
    && mkdir pin_packing/src/ \
    && echo "fn main() {}" > pin_packing/src/main.rs \
    && mkdir soakdb_io/src \
    && touch soakdb_io/src/lib.rs \
    && mkdir soakdb_sync/src/ \
    && echo "fn main() {}" > soakdb_sync/src/main.rs \
    && mkdir targeting/src/ \
    && touch targeting/src/lib.rs \
    && echo "fn main() {}" > targeting/src/main.rs \
    && mkdir the_paginator/src \
    && touch the_paginator/src/lib.rs \
    && cargo build --release

# Build workspace crates
COPY . /app
RUN touch chimp_chomp/src/main.rs \
    && touch chimp_protocol/src/lib.rs \
    && touch chimp_controller/src/main.rs \
    && touch compound_library/src/main.rs \
    && touch compound_soaking/src/main.rs \
    && touch crystal_library/src/main.rs \
    && touch graphql_endpoints/src/lib.rs \
    && touch graphql_event_broker/src/lib.rs \
    && touch opa_client/src/lib.rs \
    && touch pin_packing/src/main.rs \
    && touch soakdb_io/src/lib.rs \
    && touch soakdb_sync/src/main.rs \
    && touch targeting/src/lib.rs \
    && touch targeting/src/main.rs \
    && touch the_paginator/src/lib.rs \
    && cargo build --release

# Collate dynamically linked shared objects for chimp_chomp
RUN mkdir /chimp_chomp_libraries \
    && cp \
        $(ldd /app/target/release/chimp_chomp | grep -o '/.*\.so\S*') \
        /app/target/release/libonnxruntime.so.* \
        /chimp_chomp_libraries

FROM gcr.io/distroless/cc as chimp_chomp

COPY chimp_chomp/chimp.onnx /chimp.onnx
COPY --from=build /chimp_chomp_libraries/* /lib
COPY --from=build /app/target/release/chimp_chomp /chimp_chomp

ENTRYPOINT ["/chimp_chomp"]

FROM gcr.io/distroless/cc as chimp_controller

COPY --from=build /app/target/release/chimp_controller /chimp_controller

ENTRYPOINT ["/chimp_controller"]

FROM gcr.io/distroless/cc as pin_packing

COPY --from=build /app/target/release/pin_packing /pin_packing

ENTRYPOINT ["/pin_packing"]

FROM gcr.io/distroless/cc as compound_library

COPY --from=build /app/target/release/compound_library /compound_library

ENTRYPOINT ["/compound_library"]

FROM gcr.io/distroless/cc as compound_soaking

COPY --from=build /app/target/release/compound_soaking /compound_soaking

ENTRYPOINT ["/compound_soaking"]

FROM gcr.io/distroless/cc as crystal_library

COPY --from=build /app/target/release/crystal_library /crystal_library

ENTRYPOINT ["/crystal_library"]

FROM gcr.io/distroless/cc as soakdb_sync

COPY --from=build /app/target/release/soakdb_sync /soakdb_sync

ENTRYPOINT ["/soakdb_sync"]

FROM gcr.io/distroless/cc as targeting

COPY --from=build /app/target/release/targeting /targeting

ENTRYPOINT ["/targeting"]
