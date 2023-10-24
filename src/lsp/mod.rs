mod completion;
mod handlers;
mod line_index;

use ast::{AnyNodeRef, ModModule};
use dashmap::DashMap;
use ouroboros::self_referencing;
use python_ast::{self as ast, visitor::Visitor, Expr, Stmt};
use python_parser::parse_program;
use ruff_text_size::{Ranged, TextRange, TextSize};
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams,
        CompletionResponse, Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, GotoDefinitionParams,
        GotoDefinitionResponse, InitializeParams, InitializeResult, InitializedParams, Location,
        MessageType, OneOf, Position, Range, ReferenceParams, ServerCapabilities, TextDocumentItem,
        TextDocumentSyncCapability, TextDocumentSyncKind, Url, WorkspaceFoldersServerCapabilities,
        WorkspaceServerCapabilities,
    },
    Client, LanguageServer,
};

use crate::semantic::{index_ast, Indexer, SemanticModel, Symbol, SymbolKind};

pub struct Server {
    pub client: Client,
    pub document_map: DashMap<Url, Document>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities: ServerCapabilities {
                inlay_hint_provider: None,
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                // execute_command_provider: Some(ExecuteCommandOptions {
                //     commands: vec!["dummy.do_something".to_string()],
                //     work_done_progress_options: Default::default(),
                // }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            language_id: params.text_document.language_id,
            version: params.text_document.version,
            text: params.text_document.text,
        })
        .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("{:?}", params.content_changes))
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            language_id: String::new(),
            version: params.text_document.version,
            text: std::mem::take(&mut params.content_changes[0].text),
        })
        .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let document = self.document_map.get(&uri).unwrap();
        let rope = ropey::Rope::from_str(document.text());

        let char = rope.try_line_to_char(position.line as usize);
        // byte offset of where the completion requisition was made (cursor position)
        let byte_offset = position.character as usize + char.unwrap();
        // TODO: maybe add a CompletionProvider to handle the completion.
        let mut pos_ctx = PositionContext::new();
        let ctx = pos_ctx.get_ctx(byte_offset as u32, document.ast());

        let symbols = match ctx {
            PosCtxKind::Id | PosCtxKind::For => complete_global(document.semantic_model()),
            _ => vec![],
        };

        Ok(Some(CompletionResponse::Array(
            symbols
                .iter()
                .map(|symbol| match symbol.kind {
                    SymbolKind::Variable => CompletionItem {
                        label: symbol.name.to_string(),
                        kind: Some(if symbol.is_constant() {
                            CompletionItemKind::CONSTANT
                        } else {
                            CompletionItemKind::VARIABLE
                        }),
                        ..Default::default()
                    },
                    SymbolKind::Function(_) => CompletionItem {
                        label: symbol.name.to_string(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        ..Default::default()
                    },
                    SymbolKind::Class(_) => CompletionItem {
                        label: symbol.name.to_string(),
                        kind: Some(CompletionItemKind::CLASS),
                        ..Default::default()
                    },
                    SymbolKind::Enum => CompletionItem {
                        label: symbol.name.to_string(),
                        kind: Some(CompletionItemKind::ENUM),
                        ..Default::default()
                    },
                    SymbolKind::Method(_) => CompletionItem {
                        label: symbol.name.to_string(),
                        kind: Some(CompletionItemKind::METHOD),
                        ..Default::default()
                    },
                    SymbolKind::Import(_) | SymbolKind::FromImport(_) => CompletionItem {
                        label: symbol.name.to_string(),
                        kind: Some(CompletionItemKind::MODULE),
                        ..Default::default()
                    },
                    SymbolKind::Parameter => CompletionItem {
                        label: symbol.name.to_string(),
                        ..Default::default()
                    },
                    _ => todo!(),
                })
                .collect(),
        )))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        self.client
            .log_message(MessageType::INFO, format!("{position:?}"))
            .await;
        let document = self.document_map.get(&uri).unwrap();
        let rope = ropey::Rope::from_str(document.text());

        let char = rope.try_line_to_char(position.line as usize);
        let byte_offset = position.character as usize + char.unwrap();

        self.client
            .log_message(
                MessageType::INFO,
                format!("goto|byte_offset: {byte_offset:?}"),
            )
            .await;
        let symbol = get_symbol_at_offset(document.semantic_model(), byte_offset as u32);
        self.client
            .log_message(MessageType::INFO, format!("Symbol: {symbol:?}"))
            .await;
        let symbol_definition = symbol.and_then(|symbol| {
            let declaration = symbol.declaration(document.semantic_model()).unwrap();
            let range = declaration.range();
            let start_pos = offset_to_position(range.start().to_usize(), &rope)?;
            let end_pos = offset_to_position(range.end().to_usize(), &rope)?;
            let range = Range::new(start_pos, end_pos);

            Some(GotoDefinitionResponse::Scalar(Location::new(uri, range)))
        });

        Ok(symbol_definition)
    }

    // TODO: show the location where the symbol was defined
    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        self.client
            .log_message(MessageType::INFO, "references called")
            .await;
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let document = self.document_map.get(&uri).unwrap();
        let rope = ropey::Rope::from_str(document.text());

        let char = rope.try_line_to_char(position.line as usize);
        let byte_offset = position.character as usize + char.unwrap();
        let reference_list = get_symbol_at_offset(document.semantic_model(), byte_offset as u32)
            .map(|symbol| {
                symbol
                    .references
                    .iter()
                    .map(|&reference_id| document.semantic_model().reference(reference_id).range)
                    .filter_map(|range| text_range_to_range(range, &rope))
                    .map(|range| Location::new(uri.clone(), range))
                    .collect::<Vec<_>>()
            });

        Ok(reference_list)
    }
}

fn text_range_to_range(range: TextRange, rope: &ropey::Rope) -> Option<Range> {
    let start_position = offset_to_position(range.start().into(), rope)?;
    let end_position = offset_to_position(range.end().into(), rope)?;

    Some(Range::new(start_position, end_position))
}

fn offset_to_position(offset: usize, rope: &ropey::Rope) -> Option<Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}

impl Server {
    async fn on_change(&self, params: TextDocumentItem) {
        let document = Document::index(params.text);

        let diagnostics = if document.borrow_parsed_file().parse_errors.is_empty() {
            Vec::new()
        } else {
            let rope = ropey::Rope::from_str(document.borrow_text());
            document
                .borrow_parsed_file()
                .parse_errors
                .iter()
                .map(|parse_error| {
                    Diagnostic::new_simple(
                        text_range_to_range(parse_error.location, &rope).unwrap(),
                        parse_error.error.to_string(),
                    )
                })
                .collect()
        };
        self.client
            .publish_diagnostics(params.uri.clone(), diagnostics, Some(params.version))
            .await;

        self.document_map.insert(params.uri, document);

        self.client
            .log_message(MessageType::INFO, "on_change called")
            .await;
    }
}

fn complete_global<'a>(semantic: &'a SemanticModel) -> Vec<&'a Symbol<'a>> {
    semantic
        .global_scope()
        .symbols()
        .map(|(_, id)| semantic.symbol(id))
        .collect()
}

fn get_symbol_at_offset<'a>(semantic: &'a SemanticModel, offset: u32) -> Option<&'a Symbol<'a>> {
    let node = node_at_offset(semantic, offset)?;
    let id = match node {
        AnyNodeRef::NameExpr(python_ast::NameExpr { id, .. }) => id,
        AnyNodeRef::Parameter(python_ast::Parameter { name, .. }) => name.as_str(),
        AnyNodeRef::StmtFunctionDef(python_ast::FunctionDefStmt { name, .. })
            if name.range.contains(offset.into()) =>
        {
            name.as_str()
        }
        _ => return None,
    };

    semantic
        .lookup_symbol(id)
        .or_else(|| {
            semantic
                .enclosing_scope(id, offset.into())
                .map(|scope_id| semantic.lookup_symbol_in_scope(id, scope_id))?
        })
        .map(|symbol_id| semantic.symbol(symbol_id))
}

#[derive(Debug, Clone, Copy)]
enum PosCtxKind {
    AttribRef,
    Id,

    None,
    For,
}

struct PositionContext {
    current_ctx: PosCtxKind,
    offset: u32,
}

impl PositionContext {
    fn new() -> Self {
        Self {
            current_ctx: PosCtxKind::None,
            offset: 0,
        }
    }

    fn get_ctx(&mut self, offset: u32, ast: &ast::ModModule) -> PosCtxKind {
        self.offset = offset;
        self.visit_body(&ast.body);

        self.current_ctx
    }
}

impl<'a> Visitor<'a> for PositionContext {
    fn visit_stmt(&mut self, stmt: &'a Stmt) {
        match stmt {
            Stmt::For(_) => self.current_ctx = PosCtxKind::For,
            Stmt::Expr(expr) => self.visit_expr(&expr.value),
            _ => (),
        }
    }

    fn visit_expr(&mut self, expr: &'a Expr) {
        match expr {
            Expr::Attribute(attrib) => {
                if attrib.range.contains_inclusive(TextSize::from(self.offset)) {
                    self.current_ctx = PosCtxKind::AttribRef;
                }
            }
            Expr::Name(ident) => {
                if ident.range.contains_inclusive(TextSize::from(self.offset)) {
                    self.current_ctx = PosCtxKind::Id;
                }
            }
            _ => {}
        }
    }
}

// TODO: How to avoid this
#[self_referencing]
pub struct Document {
    text: String,
    // line_index: ruff_source_file::LineIndex,
    parsed_file: python_parser::ParsedFile,
    #[borrows(parsed_file)]
    #[covariant]
    indexer: Indexer<'this>,
}

impl Document {
    fn index(text: String) -> Document {
        DocumentBuilder {
            // line_index: LineIndex::from_source_text(&text),
            parsed_file: parse_program(&text),
            text,
            indexer_builder: |parsed_file| {
                index_ast(parsed_file.ast.as_module().unwrap())
                // let mut indexer = Indexer::new(SemanticModel::new());
                // indexer.visit_body(&parsed_file.ast.as_module().unwrap().body);
                // indexer
            },
        }
        .build()
    }

    fn text(&self) -> &str {
        self.borrow_text()
    }

    fn semantic_model(&self) -> &SemanticModel {
        self.borrow_indexer().semantic()
    }

    fn ast(&self) -> &ModModule {
        self.borrow_parsed_file().ast.as_module().unwrap()
    }

    // fn line_index(&self) -> &LineIndex {
    //     self.borrow_line_index()
    // }
}

fn node_at_offset<'a>(semantic: &'a SemanticModel, offset: u32) -> Option<AnyNodeRef<'a>> {
    dbg!(&semantic.nodes);
    semantic
        .nodes
        .iter()
        .rev()
        .find(|node| node.range().contains(offset.into()))
        .map(|node_with_parent| node_with_parent.node)
}

#[cfg(test)]
mod tests {
    use python_ast::AnyNodeRef;
    use python_parser::parse_program;

    use crate::{lsp::node_at_offset, semantic::index_ast};

    #[test]
    fn test_node_at_offset() {
        let src = r#"
def x(a = 0, b):
    a + b

#print(a.b)
#print(x(5, 6))

#def f(y):
#    if y:
#        return True
#    return False
"#;
        let parsed_file = parse_program(src);
        let indexer = index_ast(parsed_file.ast.as_module().unwrap());
        let offset = 7;
        let node = node_at_offset(indexer.semantic(), offset).unwrap();
        dbg!(node);

        let id = match node {
            AnyNodeRef::NameExpr(python_ast::NameExpr { id, .. }) => id,
            AnyNodeRef::Parameter(python_ast::Parameter { name, .. }) => name.as_str(),
            AnyNodeRef::StmtFunctionDef(python_ast::FunctionDefStmt { name, .. })
                if name.range.contains(offset.into()) =>
            {
                name.as_str()
            }
            _ => todo!(),
        };

        let symbol = indexer
            .semantic()
            .lookup_symbol(id)
            .or_else(|| {
                indexer
                    .semantic()
                    .enclosing_scope(id, offset.into())
                    .map(|scope_id| indexer.semantic().lookup_symbol_in_scope(id, scope_id))?
            })
            .map(|symbol_id| indexer.semantic().symbol(symbol_id));
        dbg!(symbol);
    }
}
