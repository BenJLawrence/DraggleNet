// Access resources relative to the workspace root
// let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

// Handle crate/member specific logic
// let crate_name = env!("CARGO_PKG_NAME");
// match crate_name {
//     "crate_a" => { /* crate_a specific logic */ },
//     "crate_b" => { /* crate_b specific logic */ },
//     _ => { /* default logic */ },
// }

// WARNING: This builds relative to each member's root.
// This is why '../' must be specified in the proto compilation
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/draggle_net.proto")?;
    Ok(())
}