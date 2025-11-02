"""
Rust Proto GenRule
"""

load("@rules_rust//rust:defs.bzl", "rust_library")

# proto without dependency
# protoc \
#     --rust_out=common \
#     --rust-grpc_out=common \
#     --rust_opt=experimental-codegen=enabled,kernel=upb \
#     --proto_path=proto \
#     proto/common/v1/common.proto

# proto with dependency in crate_mapping
# protoc \
#     --rust_out=configuration \
#     --rust-grpc_out=configuration \
#     --rust_opt=experimental-codegen=enabled,kernel=upb \
#     --rust_opt=crate_mapping=proto/configuration/v1/crate_mapping.txt \
#     --rust-grpc_opt=crate_mapping=proto/configuration/v1/crate_mapping.txt \
#     --proto_path=proto \
#     proto/configuration/v1/configuration.proto

# grpc with dependency in crate mapping
# protoc \
#     --rust_out=configuration_service \
#     --rust-grpc_out=configuration_service \
#     --rust_opt=experimental-codegen=enabled,kernel=upb \
#     --rust_opt=crate_mapping=proto/configuration_service/v1/crate_mapping.txt \
#     --rust-grpc_opt=crate_mapping=proto/configuration_service/v1/crate_mapping.txt \
#     --proto_path=proto \
#     proto/configuration_service/v1/configuration_service.proto

def crate_mapping_dep(crate_name, proto_files):
    """
    Create crate mapping depency

    Args:
        crate_name: name of rust protobuf crate
        proto_files: list of proto files included in crate
    """
    return {
        "crate_name": crate_name,
        "proto_files": proto_files,
    }

def rust_proto_gen(
        name,
        srcs,
        proto_dir,
        mod = None,
        grpc = False,
        crate_mapping = None,
        deps = []):
    """
    Generate Rust protobuf/gRPC code with flexible crate mappings.

    Args:
        name: Name of the genrule
        srcs: List of .proto files
        proto_dir: output directory
        mod: name of mod wrapping generated files
        grpc: includes grpc files
        crate_mapping: crate mapping object, create list of
        deps: rust dependencies
    """

    # Calculate output paths
    outs = []
    for src in srcs:
        proto_name = src.replace(".proto", "").split("/")[-1]
        outs.append("%s.u.pb.rs" % (proto_name))
        if (grpc):
            outs.append("%s_grpc.pb.rs" % (proto_name))

    # Build the rust_opt and rust-grpc_opt flags
    rust_opts = ["experimental-codegen=enabled,kernel=upb,message_module_path=crate"]
    rust_grpc_opts = ["message_module_path=crate"]

    # Build the crate_mapping.txt file
    crate_mapping_path = "$(RULEDIR)/crate_mapping.txt"
    crate_mapping_str = ""
    if crate_mapping:
        for dep in crate_mapping:
            crate_mapping_str += "%s\n" % dep["crate_name"]
            crate_mapping_str += "%s\n" % len(dep["proto_files"])
            for proto_file in dep["proto_files"]:
                crate_mapping_str += "%s\n" % proto_file

        # Add crate_mapping.txt options
        rust_opts.append("crate_mapping=%s" % crate_mapping_path)
        rust_grpc_opts.append("crate_mapping=%s" % crate_mapping_path)

    # Add required dependencies
    all_deps = deps + ["@crates//:protobuf"]
    if grpc:
        all_deps += ["@crates//:tonic", "@crates//:tonic-protobuf"]

    cmd = """
PROTOC=$$(echo "$(locations //:protoc_gen_rust_grpc)" | awk '{print $$1}')
PROTOC_GEN_RUST_GRPC=$$(echo "$(locations //:protoc_gen_rust_grpc)" | awk '{print $$2}')
echo $$PROTOC
echo $$PROTOC_GEN_RUST_GRPC

cat > %s << 'EOF'
%s
EOF
# cat $(RULEDIR)/crate_mapping.txt

# echo test
# echo $(SRCS)
# echo test

# ls proto

$$PROTOC \\
--plugin=protoc-gen-rust-grpc=$$PROTOC_GEN_RUST_GRPC \\
--rust_out=$(RULEDIR) \\
--rust-grpc_out=$(RULEDIR) \\
--rust_opt=%s \\
--rust-grpc_opt=%s \\
--proto_path=%s \\
--proto_path=proto \\
$(SRCS)
# ls $(RULEDIR)
pwd $(RULEDIR)
    """ % (crate_mapping_path, crate_mapping_str, ",".join(rust_opts), ",".join(rust_grpc_opts), proto_dir)

    # Call protoc to generate source files
    native.genrule(
        name = "%s_source_files" % name,
        srcs = srcs,
        outs = outs,
        tools = [
            "//:protoc_gen_rust_grpc",
        ],
        cmd = cmd,
        message = "Generating Rust code for %s" % name,
    )

    # If we do not wrap into a rust module, build rust library
    if not mod:
        rust_library(
            name = name,
            srcs = [":%s_source_files" % name],
            visibility = ["//visibility:public"],
            deps = all_deps,
        )

    # Wrap into a rust module
    if mod:
        includes = "\n".join(['    include!("%s");' % out for out in outs])

        native.genrule(
            name = "%s_mod_file" % name,
            outs = ["lib.rs"],
            cmd = """
    cat > $@ <<'EOF'
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

include!("generated.rs");

pub mod %s {
%s
}
            """ % (mod, includes),
        )

        rust_library(
            name = name,
            srcs = [":%s_mod_file" % name],
            compile_data = [":%s_source_files" % name],
            visibility = ["//visibility:public"],
            deps = all_deps,
        )
