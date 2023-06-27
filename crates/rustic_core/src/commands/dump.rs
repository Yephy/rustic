use std::io::Write;

use crate::{
    error::CommandErrorKind, repository::IndexedRepository, BlobType, IndexedBackend, Node,
    NodeType, RusticResult,
};

pub(crate) fn dump<P>(
    repo: &IndexedRepository<P>,
    node: &Node,
    w: &mut impl Write,
) -> RusticResult<()> {
    if node.node_type != NodeType::File {
        return Err(CommandErrorKind::DumpNotSupported(node.node_type.clone()).into());
    }

    for id in node.content.as_ref().unwrap() {
        // TODO: cache blobs which are needed later
        let data = repo.index.blob_from_backend(BlobType::Data, id)?;
        w.write_all(&data)?;
    }
    Ok(())
}
