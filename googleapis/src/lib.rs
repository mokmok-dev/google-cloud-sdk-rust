pub mod google {
    pub mod bigtable {
        pub mod v2 {
            tonic::include_proto!("google.bigtable.v2");
        }
    }

    // package referenced by generated Rust code
    mod rpc {
        tonic::include_proto!("google.rpc");
    }

    // package referenced by generated Rust code
    mod r#type {
        tonic::include_proto!("google.r#type");
    }
}
