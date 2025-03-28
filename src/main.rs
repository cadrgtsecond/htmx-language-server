#![warn(clippy::pedantic)]

use itertools::Itertools;
use log::{error, info, warn};
use lsp_server::{Connection, RequestId};
use lsp_server::{Message, Request};
use lsp_types::{
    CompletionOptions, CompletionParams, DiagnosticOptions, DiagnosticServerCapabilities,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverParams,
    HoverProviderCapability, MarkupKind, Position, PositionEncodingKind, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, Uri, WorkDoneProgressOptions,
};
use textstore::TextStore;
use thiserror::Error;
use tl::{Node, Parser};

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

fn print_node(node: &Node, parser: &Parser, level: usize) {
    let indent: String = std::iter::repeat_n("  ", level).join("");
    match node {
        Node::Tag(htmltag) => {
            eprintln!("{}tag: {}", indent, htmltag.name().as_utf8_str());
            for child in htmltag.children().top().iter() {
                print_node(child.get(parser).unwrap(), parser, level + 1);
            }
        }
        Node::Raw(content) => {
            eprintln!(
                "{}content: {:?} {}",
                indent,
                content,
                content.as_bytes_borrowed().is_some()
            );
        }
        Node::Comment(comment) => {
            eprintln!(
                "{}comment: {:?}, {}",
                indent,
                comment,
                comment.as_bytes_borrowed().is_some()
            );
        }
    }
}

fn handle_hover(id: RequestId, params: HoverParams, state: &State) -> Result<(), HandleMessageErr> {
    let uri = params.text_document_position_params.text_document.uri;
    let file = state
        .textstore
        .0
        .get(&uri)
        .ok_or(HandleMessageErr::BadUri(uri))?;
    let pos = params.text_document_position_params.position;
    let off = file.line_to_offset(pos.line as usize, pos.character as usize);
    let Some(obj) = file.object_under_cursor(off) else {
        // Nothing to handle
        return Ok(());
    };

    match obj {
        textstore::HTMLObject::Tag(_) => {
            info!("Not implemented yet!");
        }
        textstore::HTMLObject::Attr(a) => {
            let Some(doc) = htmx::DESCRIPTIONS.get(a) else {
                return Ok(());
            };
            state
                .conn
                .sender
                .send(Message::Response(lsp_server::Response {
                    id,
                    result: Some(serde_json::to_value(Hover {
                        contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: String::from(*doc),
                        }),
                        range: Some(Range {
                            start: pos,
                            end: Position {
                                character: pos.character + a.len() as u32,
                                ..pos
                            },
                        }),
                    })?),
                    error: None,
                })).map_err(|_|  HandleMessageErr::SendError)?;
        }
        textstore::HTMLObject::AttrValue(_) => info!("Not implemented yet!"),
    }
    warn!("Hovering over: {:?}", obj);
    Ok(())
}

fn handle_completion(
    id: RequestId,
    params: CompletionParams,
    state: &State,
) -> Result<(), HandleMessageErr> {
    let uri = params.text_document_position.text_document.uri;
    let file = state
        .textstore
        .0
        .get(&uri)
        .ok_or(HandleMessageErr::BadUri(uri))?;
    todo!("Handle completion");
}

fn handle_message(state: &mut State, msg: Message) -> Result<(), HandleMessageErr> {
    match msg {
        lsp_server::Message::Request(Request { id, method, params }) => match method.as_str() {
            "textDocument/hover" => {
                handle_hover(id, serde_json::from_value(params)?, state)?;
            }
            "textDocument/completion" => {
                handle_completion(id, serde_json::from_value(params)?, state)?;
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
    info!("Initialized htmx language server");

    loop {
        let msg = state.conn.receiver.recv().unwrap();
        if let Err(err) = handle_message(&mut state, msg) {
            error!("Error while handling message: {:?}", err);
        }
    }
}
