use dashmap::DashMap;
use python_lsp::lsp;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| lsp::Server {
        client,
        document_map: DashMap::default(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
