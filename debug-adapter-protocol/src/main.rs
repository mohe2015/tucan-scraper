#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::vec_box,
    clippy::module_name_repetitions
)]
use async_trait::async_trait;
use futures_util::Sink;
use futures_util::SinkExt;
use futures_util::Stream;
use futures_util::StreamExt;
use json_rpc_server::run_json_rpc_server;
use json_rpc_server::JsonRpcServer;
use serde::Deserialize;
use serde::Serialize;

use std::sync::Arc;

out_dir_include::out_dir_include!("debug-adapter-protocol.rs");

pub struct Server {}

impl Server {
    async fn handle<
        R: Stream<Item = Result<String, anyhow::Error>> + std::marker::Send + std::marker::Unpin,
        W: Sink<String, Error = anyhow::Error> + std::marker::Send + std::marker::Unpin,
    >(
        self: Arc<Self>,
        mut reader: R,
        mut sender: W,
    ) -> anyhow::Result<()> {
        let mut seq = 0;
        loop {
            let read_value = reader.next().await.unwrap()?;
            let request: Requests = serde_json::from_str(&read_value)?;

            let fake_source = Source {
                name: Some("test.tucant".to_string()),
                path: Some("/home/moritz/Documents/tucant/tucant-language/test.tucant".to_string()),
                source_reference: None,
                presentation_hint: Some(SourceStructPresentationHint::Emphasize),
                origin: Some("source code".to_string()),
                sources: Some(vec![]),
                adapter_data: None,
                checksums: Some(vec![]),
            };

            match request {
                Requests::InitializeRequest(request) => {
                    let response = Response::<InitializeResponse> {
                        inner: Some(InitializeResponse {
                            body: Some(Capabilities {
                                supports_configuration_done_request: Some(true),
                                supports_function_breakpoints: Some(true),
                                supports_conditional_breakpoints: Some(true),
                                supports_hit_conditional_breakpoints: Some(true),
                                supports_evaluate_for_hovers: Some(true),
                                exception_breakpoint_filters: Some(vec![]),
                                supports_step_back: Some(true),
                                supports_set_variable: Some(true),
                                supports_restart_frame: Some(true),
                                supports_goto_targets_request: Some(true),
                                supports_step_in_targets_request: Some(true),
                                supports_completions_request: Some(true),
                                completion_trigger_characters: Some(vec![
                                    ".".to_string(),
                                    " ".to_string(),
                                ]),
                                supports_modules_request: Some(true),
                                additional_module_columns: Some(vec![]),
                                supported_checksum_algorithms: Some(vec![
                                    ChecksumAlgorithm::Md5,
                                    ChecksumAlgorithm::Sha1,
                                    ChecksumAlgorithm::Sha256,
                                    ChecksumAlgorithm::Timestamp,
                                ]),
                                supports_restart_request: Some(true),
                                supports_exception_options: Some(true),
                                supports_value_formatting_options: Some(true),
                                supports_exception_info_request: Some(true),
                                support_terminate_debuggee: Some(true),
                                support_suspend_debuggee: Some(true),
                                supports_delayed_stack_trace_loading: Some(true),
                                supports_loaded_sources_request: Some(true),
                                supports_log_points: Some(true),
                                supports_terminate_threads_request: Some(true),
                                supports_set_expression: Some(true),
                                supports_terminate_request: Some(true),
                                supports_data_breakpoints: Some(true),
                                supports_read_memory_request: Some(true),
                                supports_write_memory_request: Some(true),
                                supports_disassemble_request: Some(true),
                                supports_cancel_request: Some(true),
                                supports_breakpoint_locations_request: Some(true),
                                supports_clipboard_context: Some(true),
                                supports_stepping_granularity: Some(true),
                                supports_instruction_breakpoints: Some(true),
                                supports_exception_filter_options: Some(true),
                                supports_single_thread_execution_requests: Some(true),
                            }),
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: InitializedEvent {
                            event: InitializedEventStructEvent::Initialized,
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::LaunchRequest(request) => {
                    // TODO FIXME make this pause at start

                    // TODO FIXME force matchup of request and response

                    // TODO FIXME abstract equal fields out
                    let response = Response::<LaunchResponse> {
                        inner: Some(LaunchResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetBreakpointsResponse {
                            body: SetBreakpointsResponseStructBody {
                                breakpoints: vec![Breakpoint {
                                    id: Some(133333),
                                    verified: true,
                                    message: None,
                                    source: Some(fake_source),
                                    line: Some(1),
                                    column: Some(1),
                                    end_line: Some(1),
                                    end_column: Some(5),
                                    instruction_reference: None,
                                    offset: None,
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::BreakpointLocationsRequest(request) => {
                    // these are shown as hints when you set a breakpoint in that line
                    let response = Response {
                        inner: Some(BreakpointLocationsResponse {
                            body: BreakpointLocationsResponseStructBody {
                                breakpoints: vec![
                                    BreakpointLocation {
                                        line: 1,
                                        column: Some(1),
                                        end_line: None,
                                        end_column: None,
                                    },
                                    BreakpointLocation {
                                        line: 1,
                                        column: Some(2),
                                        end_line: None,
                                        end_column: None,
                                    },
                                    BreakpointLocation {
                                        line: 1,
                                        column: Some(3),
                                        end_line: None,
                                        end_column: None,
                                    },
                                ],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetFunctionBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetFunctionBreakpointsResponse {
                            body: SetFunctionBreakpointsResponseStructBody {
                                breakpoints: vec![],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetDataBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetDataBreakpointsResponse {
                            body: SetDataBreakpointsResponseStructBody {
                                breakpoints: vec![],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetInstructionBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetInstructionBreakpointsResponse {
                            body: SetInstructionBreakpointsResponseStructBody {
                                breakpoints: vec![],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::LoadedSourcesRequest(request) => {
                    let response = Response {
                        inner: Some(LoadedSourcesResponse {
                            body: LoadedSourcesResponseStructBody {
                                sources: vec![fake_source],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::ConfigurationDoneRequest(request) => {
                    let response = Response {
                        inner: Some(ConfigurationDoneResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::ThreadsRequest(request) => {
                    let response = Response {
                        inner: Some(ThreadsResponse {
                            body: ThreadsResponseStructBody { threads: vec![] },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::EvaluateRequest(request) => {
                    let response = Response::<EvaluateResponse> {
                        inner: Some(EvaluateResponse {
                            body: EvaluateResponseStructBody {
                                result: "42".to_string(),
                                r#type: Some("answer-to-question-about-sense-of-life".to_string()),
                                presentation_hint: Some(VariablePresentationHint {
                                    kind: Some("property".to_string()),
                                    attributes: Some(vec!["readOnly".to_string()]),
                                    visibility: Some("public".to_string()),
                                    lazy: Some(false),
                                }),
                                variables_reference: 1337,
                                named_variables: Some(10),
                                indexed_variables: Some(10),
                                memory_reference: Some("deadbeef".to_string()),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::VariablesRequest(request) => {
                    let response = Response {
                        inner: Some(VariablesResponse {
                            body: VariablesResponseStructBody {
                                variables: vec![Variable {
                                    name: "cat".to_string(),
                                    value: "nicevalue".to_string(),
                                    r#type: Some("string".to_string()),
                                    presentation_hint: Some(VariablePresentationHint {
                                        kind: Some("property".to_string()),
                                        attributes: Some(vec!["readOnly".to_string()]),
                                        visibility: Some("public".to_string()),
                                        lazy: Some(false),
                                    }),
                                    evaluate_name: Some("evaluateName".to_string()),
                                    variables_reference: 0,
                                    named_variables: Some(0),
                                    indexed_variables: Some(0),
                                    memory_reference: None,
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::CompletionsRequest(request) => {
                    let response = Response {
                        inner: Some(CompletionsResponse {
                            body: CompletionsResponseStructBody {
                                targets: vec![CompletionItem {
                                    label: ".elephant".to_string(),
                                    text: None,
                                    sort_text: None,
                                    detail: Some("this is super nice".to_string()),
                                    r#type: Some(CompletionItemType::Function),
                                    start: Some(0),
                                    length: Some(0),
                                    selection_start: None,  //Some(1),
                                    selection_length: None, // Some(1),
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                request => unimplemented!("{:?}", request),
            }

            let _cloned_self = self.clone();
        }
    }
}

#[async_trait]
impl JsonRpcServer for Server {
    async fn run<
        R: Stream<Item = Result<String, anyhow::Error>>
            + std::marker::Unpin
            + std::marker::Send
            + 'static,
        W: Sink<String, Error = anyhow::Error> + std::marker::Unpin + std::marker::Send + 'static,
    >(
        read: R,
        write: W,
    ) -> anyhow::Result<()> {
        let arc_self = Arc::new(Self {});

        arc_self.handle(read, write).await?;

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    #[serde(flatten)]
    inner: T,
    seq: u64,
    r#type: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(flatten)]
    inner: Option<T>, // probably Result
    seq: u64,
    r#type: String,
    request_seq: u64,
    success: bool,
    message: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Event<T> {
    #[serde(flatten)]
    inner: T,
    r#type: String,
}

// cargo watch -x 'run -- --port 6009'
pub fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run_json_rpc_server::<Server>().await })
}
