#[test]
fn test_membarrier() {
    use rustix::thread::{membarrier, membarrier_query, MembarrierCommand, MembarrierQuery};

    let query: MembarrierQuery = membarrier_query();

    // Supported registration commands should succeed.
    for cmd in [
        MembarrierCommand::RegisterGlobalExpedited,
        MembarrierCommand::RegisterPrivateExpedited,
        MembarrierCommand::RegisterPrivateExpeditedSyncCore,
        MembarrierCommand::RegisterPrivateExpeditedRseq,
    ] {
        if query.contains_command(cmd) {
            membarrier(cmd).unwrap();
        }
    }

    // All supported commands should now succeed, and all unsupported
    // commands should fail.
    for cmd in [
        MembarrierCommand::Global,
        MembarrierCommand::GlobalExpedited,
        MembarrierCommand::PrivateExpedited,
        MembarrierCommand::PrivateExpeditedSyncCore,
        MembarrierCommand::PrivateExpeditedRseq,
    ] {
        if query.contains_command(cmd) {
            membarrier(cmd).unwrap();
        } else {
            membarrier(cmd).unwrap_err();
        }
    }
}
