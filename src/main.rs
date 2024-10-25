#![warn(clippy::pedantic)]

use log::{debug, info};
use lsp_types::{
    CompletionOptions, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    HoverProviderCapability, PositionEncodingKind, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, Uri, WorkDoneProgressOptions,
};
use std::collections::HashMap;
use std::fmt::Debug;
use tree_sitter::{Language, Tree};

mod selectors;

/// Internal representation of a file
struct File {
    contents: Box<str>,
    tree: Tree,
}
impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File({})", self.contents)
    }
}

impl File {
    pub fn new(contents: &str, language: &Language) -> File {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .expect("Error setting tree sitter language");

        // Tree sitter never fails parsing
        let tree = parser.parse(contents.as_bytes(), None).unwrap();

        File {
            contents: Box::from(contents),
            tree,
        }
    }
}

struct TextStore(pub HashMap<Uri, File>);

fn get_tree_sitter_language(_language: &str) -> Language {
    // For now, parse everything as html
    tree_sitter_html::LANGUAGE.into()
}

impl TextStore {
    pub fn new() -> TextStore {
        TextStore(HashMap::new())
    }
    pub fn update(&mut self, uri: Uri, language: &Language, text: &str) {
        self.0.insert(uri, File::new(text, language));
    }
}

fn main() {
    env_logger::init();

    let (connection, _threads) = lsp_server::Connection::stdio();
    let server_capabilities = ServerCapabilities {
        position_encoding: Some(PositionEncodingKind::UTF8),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec!["-".to_string()]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
            completion_item: None,
        }),
        ..Default::default()
    };
    let params = connection.initialize(
        serde_json::to_value(server_capabilities).expect("Failed to serialize server capabilities"),
    );
    let mut textstore = TextStore::new();
    loop {
        let msg = connection.receiver.recv().unwrap();
        match msg {
            lsp_server::Message::Request(_r) => {}
            lsp_server::Message::Notification(n) => match n.method.as_str() {
                "textDocument/didOpen" => {
                    let Ok(params) = serde_json::from_value::<DidOpenTextDocumentParams>(n.params)
                    else {
                        debug!("Invalid message");
                        break;
                    };
                    textstore.update(
                        params.text_document.uri.clone(),
                        &get_tree_sitter_language(&params.text_document.language_id),
                        &params.text_document.text,
                    );
                    debug!("{:?}", textstore.0.get(&params.text_document.uri).unwrap());
                }
                "textDocument/didChange" => {
                    let Ok(params) =
                        serde_json::from_value::<DidChangeTextDocumentParams>(n.params)
                    else {
                        debug!("Invalid message");
                        break;
                    };
                    // This should never unwrap since didOpen was called before
                    let language = textstore
                        .0
                        .get(&params.text_document.uri)
                        .unwrap()
                        .tree
                        .language()
                        .clone();
                    // Only one change will be there because of the server capabilities
                    textstore.update(
                        params.text_document.uri.clone(),
                        &language,
                        &params
                            .content_changes
                            .first()
                            .expect("Invalid message")
                            .text,
                    );
                    info!("{:?}", textstore.0.get(&params.text_document.uri).unwrap());
                }
                _ => {}
            },

            lsp_server::Message::Response(_) => {}
        }
    }
}
