use cbqn::{eval, BQN};
use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod bqn;
mod diagnostics;
mod help;

use diagnostics::get_diagnostics;

#[derive(Debug)]
struct Backend {
    client: Client,
    system_values: Vec<CompletionItem>,
    documents: DashMap<Url, Vec<String>>,
}

impl Backend {
    fn new(client: Client) -> Backend {
        let system_values = BQN!("{𝕩∾(-⟜32⌾(1⊸⊑)¨𝕩)} '•'⊸∾¨ •listsys")
            .to_bqnvalue_vec()
            .into_iter()
            .map(|v| CompletionItem::new_simple(v.to_string(), "System value".into()))
            .collect();

        Backend {
            client,
            system_values,
            documents: Default::default(),
        }
    }

    async fn changed_document(&self, uri: Url, text: &str) {
        self.documents
            .insert(uri.clone(), text.lines().map(str::to_owned).collect());

        self.client
            .publish_diagnostics(uri, get_diagnostics(text), None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = &params.text_document.text;
        self.changed_document(uri, text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = &params.content_changes[0].text;
        self.changed_document(uri, text).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(self.system_values.clone())))
    }

    async fn hover(&self, p: HoverParams) -> Result<Option<Hover>> {
        let contents = match self
            .documents
            .get(&p.text_document_position_params.text_document.uri)
        {
            Some(x) => x,
            None => return Ok(None),
        };
        let pos = p.text_document_position_params.position;
        let line = match contents.get(pos.line as usize) {
            Some(x) => x,
            None => return Ok(None),
        };
        let c = match line.chars().nth(pos.character as usize) {
            Some(x) => x,
            None => return Ok(None),
        };
        let contents = match help::help_for_symbol(c) {
            Some(x) => x,
            None => return Ok(None),
        };
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::from_markdown(contents.into())),
            range: None,
        }))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
