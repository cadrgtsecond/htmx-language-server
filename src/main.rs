#![warn(clippy::pedantic)]

use log::{error, info, warn};
use lsp_server::{Connection, RequestId, Response};
use lsp_server::{Message, Request};
use lsp_types::{
    CompletionItem, CompletionItemTag, CompletionOptions, CompletionParams, CompletionResponse,
    CompletionTextEdit, DiagnosticOptions, DiagnosticServerCapabilities,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, HoverParams, HoverProviderCapability,
    Position, PositionEncodingKind, Range, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextEdit, Uri, WorkDoneProgressOptions,
};
use self_cell::self_cell;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use textstore::TextStore;
use thiserror::Error;
use tl::{ParserOptions, VDom};

mod htmx;
mod textstore;

#[derive(Debug, Error)]
enum HandleMessageErr {
    #[error("Failed to deserialize")]
    FailedDeserialize(#[from] serde_json::Error),
    #[error("Unknown file uri: {0:?}")]
    BadUri(Uri),
    #[error("Failed to send response")]
    SendError,
    #[error("Bad Message")]
    BadMsg,
}

/// Takes in a `Position` and returns a `usize` offset, but only if the `Position`
/// is within the text
fn position_to_offset(pos: Position, text: &str) -> Option<usize> {
    text.lines()
        .nth(pos.line as usize)
        .map(|l| (l.as_ptr() as usize - text.as_ptr() as usize) + pos.character as usize)
}

fn handle_hover(params: HoverParams, state: &State) -> Result<(), HandleMessageErr> {
    let uri = params.text_document_position_params.text_document.uri;
    let file = state
        .textstore
        .0
        .get(&uri)
        .ok_or(HandleMessageErr::BadUri(uri))?;

    let vdom = file.borrow_dependent();
    let text = file.borrow_owner();
    let Some(off) = position_to_offset(params.text_document_position_params.position, text) else {
        return Ok(());
    };
    let node = vdom
        .nodes()
        .iter()
        .filter_map(|n| match n {
            tl::Node::Tag(htmltag) => {
                let (start, end) = htmltag.boundaries(vdom.parser());
                (start..=end).contains(&off).then(|| n)
            }
            _ => None,
        })
        .last();
    info!("{:?}", node);
    Ok(())
}

fn handle_completion(
    params: CompletionParams,
    state: &State,
    id: RequestId,
) -> Result<(), HandleMessageErr> {
    let uri = params.text_document_position.text_document.uri;
    let file = state
        .textstore
        .0
        .get(&uri)
        .ok_or(HandleMessageErr::BadUri(uri))?;
    /*
    let Some(node) = search::for_deepest_matching(
        &mut cursor,
        &mut |node| {
            let p = params.text_document_position.position;
            let point = Point {
                row: p.line as usize,
                column: p.character.saturating_sub(1) as usize,
            };
            node.start_position() < point && point < node.end_position()
        },
        &mut |_| true,
    ) else {
        return Ok(());
    };
    let attr = &file.contents[node.start_byte()..node.end_byte()];
    let response = CompletionResponse::Array(
        htmx::ATTRIBUTES
            .iter()
            .filter(|a| a.starts_with(attr))
            .map(|c| CompletionItem {
                label: (*c).to_string(),
                detail: htmx::DESCRIPTIONS.get(c).map(|c| (*c).to_string()),
                documentation: None,
                text_edit: Some(CompletionTextEdit::Edit(TextEdit::new(
                    Range::new(
                        Position::new(
                            node.start_position().row.try_into().unwrap(),
                            node.start_position().column.try_into().unwrap(),
                        ),
                        Position::new(
                            node.end_position().row.try_into().unwrap(),
                            node.end_position().column.try_into().unwrap(),
                        ),
                    ),
                    (*c).to_string(),
                ))),
                tags: Some(vec![CompletionItemTag::DEPRECATED]),
                ..Default::default()
            })
            .collect(),
    );
    connection
        .sender
        .send(Message::Response(Response::new_ok(r.id, response)))
        .map_err(|_| HandleMessageErr::SendError)?;
      */
    Ok(())
}

fn handle_message(state: &mut State, msg: Message) -> Result<(), HandleMessageErr> {
    match msg {
        lsp_server::Message::Request(Request { id, method, params }) => match method.as_str() {
            "textDocument/hover" => {
                handle_hover(serde_json::from_value(params)?, state)?;
            }
            "textDocument/completion" => {
                handle_completion(serde_json::from_value(params)?, state, id)?;
            }
            _ => {}
        },
        lsp_server::Message::Notification(n) => match n.method.as_str() {
            "textDocument/didOpen" => {
                let params = serde_json::from_value::<DidOpenTextDocumentParams>(n.params)?;
                state
                    .textstore
                    .insert(params.text_document.uri, &params.text_document.text);
            }
            "textDocument/didChange" => {
                let params = serde_json::from_value::<DidChangeTextDocumentParams>(n.params)?;
                state.textstore.insert(
                    params.text_document.uri,
                    &params
                        .content_changes
                        .first()
                        .ok_or(HandleMessageErr::BadMsg)?
                        .text,
                );
            }
            _ => {}
        },

        lsp_server::Message::Response(_) => {}
    }
    Ok(())
}

struct State {
    pub conn: Connection,
    pub textstore: TextStore,
}

fn main() {
    env_logger::init();

    let (conn, _threads) = lsp_server::Connection::stdio();
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
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: Some("htmx-lsp".into()),
            inter_file_dependencies: false,
            workspace_diagnostics: false,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
        })),
        ..Default::default()
    };
    let _params = conn.initialize(
        serde_json::to_value(server_capabilities).expect("Failed to serialize server capabilities"),
    );
    let mut state = State {
        conn,
        textstore: TextStore::new(),
    };

    loop {
        let msg = state.conn.receiver.recv().unwrap();
        if let Err(err) = handle_message(&mut state, msg) {
            error!("Error while handling message: {:?}", err);
        }
    }
}
