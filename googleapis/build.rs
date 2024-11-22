fn main() -> anyhow::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "proto/google/bigtable/v2/bigtable.proto",
                "proto/google/bigtable/v2/data.proto",
                "proto/google/bigtable/v2/feature_flags.proto",
                "proto/google/bigtable/v2/request_stats.proto",
                "proto/google/bigtable/v2/response_params.proto",
                "proto/google/bigtable/v2/types.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
