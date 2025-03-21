//! Utilities that allow for resolving file-based subgraphs

use std::pin::Pin;
use std::sync::Arc;

use buildstructor::Builder;
use camino::Utf8PathBuf;
use futures::Future;
use rover_std::Fs;
use tower::Service;

use super::FullyResolvedSubgraph;
use crate::composition::supergraph::config::error::ResolveSubgraphError;
use crate::composition::supergraph::config::unresolved::UnresolvedSubgraph;

/// Service that resolves a file-based subgraph
#[derive(Clone, Builder)]
pub struct ResolveFileSubgraph {
    supergraph_config_root: Utf8PathBuf,
    path: Utf8PathBuf,
    unresolved_subgraph: UnresolvedSubgraph,
}

impl Service<()> for ResolveFileSubgraph {
    type Response = FullyResolvedSubgraph;
    type Error = ResolveSubgraphError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: ()) -> Self::Future {
        let unresolved_subgraph = self.unresolved_subgraph.clone();
        let supergraph_config_root = self.supergraph_config_root.clone();
        let path = self.path.clone();
        let subgraph_name = unresolved_subgraph.name().to_string();
        let schema_source = self.unresolved_subgraph.schema().clone();
        let fut = async move {
            let file = unresolved_subgraph.resolve_file_path(&supergraph_config_root, &path)?;
            let schema = Fs::read_file(&file).map_err(|err| ResolveSubgraphError::Fs {
                source: Arc::new(Box::new(err)),
            })?;

            let builder = FullyResolvedSubgraph::builder()
                .name(subgraph_name)
                .schema(schema)
                .schema_source(schema_source);

            Ok(match unresolved_subgraph.routing_url() {
                None => builder.build(),
                Some(routing_url) => builder.routing_url(routing_url).build(),
            })
        };
        Box::pin(fut)
    }
}
