use itertools::Itertools;
use re_log_types::{LogMsg, PythonVersion};
use re_smart_channel::Receiver;

use anyhow::Context as _;
use clap::Subcommand;
#[cfg(feature = "web_viewer")]
use re_web_viewer_server::WebViewerServerPort;
#[cfg(feature = "web_viewer")]
use re_ws_comms::RerunServerPort;

#[cfg(feature = "web_viewer")]
use crate::web_viewer::host_web_viewer;

// Note the extra blank lines between the point-lists below: it is required by `clap`.

/// The Rerun Viewer and Server
///
/// Features:
///
/// * Read `.rrd` (rerun recording files).
///
/// * Connect to a Rerun Server over web-sockets.
///
/// * Host a Rerun Server that Rerun SDK:s can connect to.
///
/// Environment variables:
///
/// * `RERUN`: force enable/disable logging with rerun (only relevant for the Rerun API, not the Viewer itself). Either `on`/`1`/`true` or `off`/`0`/`false`
///
/// * `RERUN_SHADER_PATH`: change the search path for shader/shader-imports. WARNING: Shaders are embedded in some build configurations.
///
/// * `RERUN_TRACK_ALLOCATIONS`: track all allocations in order to find memory leaks in the viewer. WARNING: slows down the viewer by a lot!
///
/// * `WGPU_BACKEND`: overwrites the graphics backend used, must be one of `vulkan`, `metal`, `dx12`, `dx11`, or `gl`.
///     Naturally, support depends on your OS. Default is `vulkan` everywhere except on Mac where we use `metal`.
///
/// * `WGPU_POWER_PREF`: overwrites the power setting used for choosing a graphics adapter, must be `high` or `low`. (Default is `high`)
#[derive(Debug, clap::Parser)]
#[clap(author, about)]
struct Args {
    // Note: arguments are sorted lexicographically for nicer `--help` message:
    #[command(subcommand)]
    commands: Option<Commands>,

    /// Set a maximum input latency, e.g. "200ms" or "10s".
    ///
    /// If we go over this, we start dropping packets.
    ///
    /// The default is no limit, which means Rerun might eat more and more memory,
    /// and have longer and longer latency, if you are logging data faster
    /// than Rerun can index it.
    #[clap(long)]
    drop_at_latency: Option<String>,

    /// An upper limit on how much memory the Rerun Viewer should use.
    ///
    /// When this limit is used, Rerun will purge the oldest data.
    ///
    /// Example: `16GB`
    #[clap(long)]
    memory_limit: Option<String>,

    /// Whether the Rerun Viewer should persist the state of the viewer to disk.
    ///
    /// When persisted, the state will be stored at the following locations:
    ///
    /// - Linux: /home/UserName/.local/share/rerun
    ///
    /// - macOS: /Users/UserName/Library/Application Support/rerun
    ///
    /// - Windows: C:\Users\UserName\AppData\Roaming\rerun
    #[clap(long, default_value_t = true)]
    persist_state: bool,

    /// What TCP port do we listen to (for SDKs to connect to)?
    #[cfg(feature = "server")]
    #[clap(long, default_value_t = re_sdk_comms::DEFAULT_SERVER_PORT)]
    port: u16,

    /// Start with the puffin profiler running.
    #[clap(long)]
    profile: bool,

    /// Stream incoming log events to an .rrd file at the given path.
    #[clap(long)]
    save: Option<String>,

    /// Exit with a non-zero exit code if any warning or error is logged. Useful for tests.
    #[clap(long)]
    strict: bool,

    /// Ingest data and then quit once the goodbye message has been received.
    ///
    /// Used for testing together with the `--strict` argument.
    ///
    /// Fails if no messages are received, or if no messages are received within a dozen or so seconds.
    #[clap(long)]
    test_receive: bool,

    /// Either: a path to `.rrd` file(s) to load,
    /// some mesh or image files to show,
    /// an http url to an `.rrd` file,
    /// or a websocket url to a Rerun Server from which to read data
    ///
    /// If none is given, a server will be hosted which the Rerun SDK can connect to.
    url_or_paths: Vec<String>,

    /// Print version and quit
    #[clap(long)]
    version: bool,

    /// Start the viewer in the browser (instead of locally).
    /// Requires Rerun to have been compiled with the 'web_viewer' feature.
    #[clap(long)]
    web_viewer: bool,

    /// What bind address IP to use.
    #[clap(long, default_value = "0.0.0.0")]
    bind: String,

    /// What port do we listen to for hosting the web viewer over HTTP.
    /// A port of 0 will pick a random port.
    #[cfg(feature = "web_viewer")]
    #[clap(long, default_value_t = Default::default())]
    web_viewer_port: WebViewerServerPort,

    /// What port do we listen to for incoming websocket connections from the viewer
    /// A port of 0 will pick a random port.
    #[cfg(feature = "web_viewer")]
    #[clap(long, default_value_t = Default::default())]
    ws_server_port: RerunServerPort,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    /// Configure the behavior of our analytics.
    #[cfg(all(feature = "analytics"))]
    #[command(subcommand)]
    Analytics(AnalyticsCommands),
}

#[derive(Debug, Clone, Subcommand)]
enum AnalyticsCommands {
    /// Prints extra information about analytics.
    Details,

    /// Deletes everything related to analytics.
    ///
    /// This will remove all pending data that hasn't yet been sent to our servers, as well as
    /// reset your analytics ID.
    Clear,

    /// Associate an email address with the current user.
    Email { email: String },

    /// Enable analytics.
    Enable,

    /// Disable analytics.
    Disable,

    /// Prints the current configuration.
    Config,
}

/// Where are we calling [`run`] from?
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallSource {
    /// Called from a command-line-input (the terminal).
    Cli,

    /// Called from the Rerun Python SDK.
    Python(PythonVersion),
}

impl CallSource {
    #[allow(dead_code)]
    fn is_python(&self) -> bool {
        matches!(self, Self::Python(_))
    }

    #[cfg(feature = "native_viewer")]
    fn app_env(&self) -> re_viewer::AppEnvironment {
        match self {
            CallSource::Cli => re_viewer::AppEnvironment::RerunCli {
                rustc_version: env!("RE_BUILD_RUSTC_VERSION").into(),
                llvm_version: env!("RE_BUILD_LLVM_VERSION").into(),
            },
            CallSource::Python(python_version) => {
                re_viewer::AppEnvironment::PythonSdk(python_version.clone())
            }
        }
    }
}

/// Run the Rerun application and return an exit code.
///
/// This is used by the `rerun` binary and the Rerun Python SDK via `python -m rerun [args...]`.
///
/// This installs crash panic and signal handlers that sends analytics on panics and signals.
/// These crash reports includes a stacktrace. We make sure the file paths in the stacktrace
/// don't include and sensitive parts of the path (like user names), but the function names
/// are all included, which means you should ONLY call `run` from a function with
/// a non-sensitive name.
///
/// In the future we plan to support installing user plugins (that act like callbacks),
/// and when we do we must make sure to give users an easy way to opt-out of the
/// crash callstacks, as those could include the file and function names of user code.
//
// It would be nice to use [`std::process::ExitCode`] here but
// then there's no good way to get back at the exit code from python
pub async fn run<I, T>(
    build_info: re_build_info::BuildInfo,
    call_source: CallSource,
    args: I,
) -> anyhow::Result<u8>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    #[cfg(feature = "native_viewer")]
    re_memory::accounting_allocator::turn_on_tracking_if_env_var(
        re_viewer::env_vars::RERUN_TRACK_ALLOCATIONS,
    );

    crate::crash_handler::install_crash_handlers(build_info);

    use clap::Parser as _;
    let args = Args::parse_from(args);

    if args.version {
        println!("{build_info}");
        return Ok(0);
    }

    if args.strict {
        re_log::add_boxed_logger(Box::new(StrictLogger {})).expect("Failed to enter --strict mode");
        re_log::info!("--strict mode: any warning or error will cause Rerun to panic.");
    }

    let res = if let Some(commands) = &args.commands {
        match commands {
            #[cfg(all(feature = "analytics"))]
            Commands::Analytics(analytics) => run_analytics(analytics).map_err(Into::into),
            #[cfg(not(all(feature = "analytics")))]
            _ => Ok(()),
        }
    } else {
        run_impl(build_info, call_source, args).await
    };

    match res {
        // Clean success
        Ok(_) => Ok(0),
        // Clean failure -- known error AddrInUse
        Err(err)
            if err
                .downcast_ref::<std::io::Error>()
                .map_or(false, |io_err| {
                    io_err.kind() == std::io::ErrorKind::AddrInUse
                }) =>
        {
            re_log::warn!("{err}");
            Ok(1)
        }
        // Unclean failure -- re-raise exception
        Err(err) => Err(err),
    }
}

#[cfg(all(feature = "analytics"))]
fn run_analytics(cmd: &AnalyticsCommands) -> Result<(), re_analytics::cli::CliError> {
    match cmd {
        #[allow(clippy::unit_arg)]
        AnalyticsCommands::Details => Ok(re_analytics::cli::print_details()),
        AnalyticsCommands::Clear => re_analytics::cli::clear(),
        AnalyticsCommands::Email { email } => {
            re_analytics::cli::set([("email".to_owned(), email.clone().into())])
        }
        AnalyticsCommands::Enable => re_analytics::cli::opt(true),
        AnalyticsCommands::Disable => re_analytics::cli::opt(false),
        AnalyticsCommands::Config => re_analytics::cli::print_config(),
    }
}

#[cfg(feature = "native_viewer")]
fn profiler(args: &Args) -> re_viewer::Profiler {
    let mut profiler = re_viewer::Profiler::default();
    if args.profile {
        profiler.start();
    }
    profiler
}

async fn run_impl(
    _build_info: re_build_info::BuildInfo,
    call_source: CallSource,
    args: Args,
) -> anyhow::Result<()> {
    #[cfg(feature = "native_viewer")]
    let profiler = profiler(&args);

    #[cfg(feature = "native_viewer")]
    let startup_options = re_viewer::StartupOptions {
        memory_limit: args.memory_limit.as_ref().map_or(Default::default(), |l| {
            re_memory::MemoryLimit::parse(l)
                .unwrap_or_else(|err| panic!("Bad --memory-limit: {err}"))
        }),
        persist_state: args.persist_state,
    };

    // Where do we get the data from?
    let rx = if !args.url_or_paths.is_empty() {
        let arguments = args
            .url_or_paths
            .iter()
            .cloned()
            .map(ArgumentCategory::from_uri)
            .collect_vec();

        if arguments.len() == 1 {
            match arguments[0].clone() {
                ArgumentCategory::RrdHttpUrl(url) => {
                    re_log_encoding::stream_rrd_from_http::stream_rrd_from_http_to_channel(url)
                }
                ArgumentCategory::FilePath(path) => {
                    let (tx, rx) =
                        re_smart_channel::smart_channel(re_smart_channel::Source::Files {
                            paths: vec![path.clone()],
                        });
                    let recording_id =
                        re_log_types::RecordingId::random(re_log_types::RecordingType::Data);
                    load_file_to_channel_at(recording_id, &path, tx)
                        .with_context(|| format!("{path:?}"))?;
                    rx
                }
                ArgumentCategory::WebSocketAddr(rerun_server_ws_url) => {
                    // We are connecting to a server at a websocket address:

                    if args.web_viewer {
                        #[cfg(feature = "web_viewer")]
                        {
                            let web_viewer = host_web_viewer(
                                args.bind.clone(),
                                args.web_viewer_port,
                                true,
                                rerun_server_ws_url,
                            );
                            // We return here because the running [`WebViewerServer`] is all we need.
                            // The page we open will be pointed at a websocket url hosted by a *different* server.
                            return web_viewer.await;
                        }
                        #[cfg(not(feature = "web_viewer"))]
                        {
                            _ = rerun_server_ws_url;
                            panic!("Can't host web-viewer - rerun was not compiled with the 'web_viewer' feature");
                        }
                    } else {
                        #[cfg(feature = "native_viewer")]
                        return native_viewer_connect_to_ws_url(
                            _build_info,
                            call_source.app_env(),
                            startup_options,
                            profiler,
                            rerun_server_ws_url,
                        );

                        #[cfg(not(feature = "native_viewer"))]
                        {
                            _ = (call_source, rerun_server_ws_url);
                            anyhow::bail!("Can't start viewer - rerun was compiled without the 'native_viewer' feature");
                        }
                    }
                }
            }
        } else {
            // Load many files:
            let mut paths = vec![];
            for argument in arguments {
                if let ArgumentCategory::FilePath(path) = argument {
                    paths.push(path);
                } else {
                    // TODO(#2121): Support loading multiple `http://` and `ws://` urls
                    anyhow::bail!("Can only load a single URL, or multiple files. See https://github.com/rerun-io/rerun/issues/2121");
                }
            }

            let (tx, rx) = re_smart_channel::smart_channel(re_smart_channel::Source::Files {
                paths: paths.clone(),
            });

            let recording_id = re_log_types::RecordingId::random(re_log_types::RecordingType::Data);

            // Load the files in parallel, and log errors.
            // Failing to log one out of many files is not a big deal.
            for path in paths {
                let tx = tx.clone();
                let recording_id = recording_id.clone();
                rayon::spawn(move || {
                    if let Err(err) = load_file_to_channel_at(recording_id, &path, tx) {
                        re_log::error!("Failed to load {path:?}: {err}");
                    }
                });
            }
            rx
        }
    } else {
        #[cfg(feature = "server")]
        {
            let server_options = re_sdk_comms::ServerOptions {
                max_latency_sec: parse_max_latency(args.drop_at_latency.as_ref()),

                // `rerun.spawn()` doesn't need to log that a connection has been made
                quiet: call_source.is_python(),
            };
            re_sdk_comms::serve(&args.bind, args.port, server_options).await?
        }

        #[cfg(not(feature = "server"))]
        anyhow::bail!("No url or .rrd path given");
    };

    // Now what do we do with the data?

    if args.test_receive {
        assert_receive_into_log_db(&rx).map(|_db| ())
    } else if let Some(rrd_path) = args.save {
        Ok(stream_to_rrd(&rx, &rrd_path.into())?)
    } else if args.web_viewer {
        #[cfg(feature = "web_viewer")]
        {
            #[cfg(feature = "server")]
            if args.url_or_paths.is_empty()
                && (args.port == args.web_viewer_port.0 || args.port == args.ws_server_port.0)
            {
                anyhow::bail!(
                    "Trying to spawn a websocket server on {}, but this port is \
                already used by the server we're connecting to. Please specify a different port.",
                    args.port
                );
            }

            // This is the server which the web viewer will talk to:
            let ws_server =
                re_ws_comms::RerunServer::new(args.bind.clone(), args.ws_server_port).await?;
            let ws_server_url = ws_server.server_url();
            let ws_server_handle = tokio::spawn(ws_server.listen(rx));

            // This is the server that serves the Wasm+HTML:
            let web_server_handle = tokio::spawn(host_web_viewer(
                args.bind.clone(),
                args.web_viewer_port,
                true,
                ws_server_url,
            ));

            // Wait for both servers to shutdown.
            web_server_handle.await?.ok();
            return ws_server_handle.await?.map_err(anyhow::Error::from);
        }

        #[cfg(not(feature = "web_viewer"))]
        {
            _ = (call_source, rx);
            anyhow::bail!(
                "Can't host web-viewer - rerun was not compiled with the 'web_viewer' feature"
            );
        }
    } else {
        #[cfg(feature = "native_viewer")]
        return re_viewer::run_native_app(Box::new(move |cc, re_ui| {
            let rx = re_viewer::wake_up_ui_thread_on_each_msg(rx, cc.egui_ctx.clone());
            let mut app = re_viewer::App::from_receiver(
                _build_info,
                &call_source.app_env(),
                startup_options,
                re_ui,
                cc.storage,
                rx,
            );
            app.set_profiler(profiler);
            Box::new(app)
        }))
        .map_err(|err| err.into());

        #[cfg(not(feature = "native_viewer"))]
        {
            _ = (call_source, rx);
            anyhow::bail!(
                "Can't start viewer - rerun was compiled without the 'native_viewer' feature"
            );
        }
    }
}

// NOTE: This is only used as part of end-to-end tests.
fn assert_receive_into_log_db(rx: &Receiver<LogMsg>) -> anyhow::Result<re_data_store::LogDb> {
    use re_smart_channel::RecvTimeoutError;

    re_log::info!("Receiving messages into a LogDb…");

    let mut db: Option<re_data_store::LogDb> = None;

    let mut num_messages = 0;

    let timeout = std::time::Duration::from_secs(12);

    loop {
        match rx.recv_timeout(timeout) {
            Ok(msg) => {
                re_log::info_once!("Received first message.");
                let is_goodbye = matches!(msg, re_log_types::LogMsg::Goodbye(_, _));

                let mut_db =
                    db.get_or_insert_with(|| re_data_store::LogDb::new(msg.recording_id().clone()));

                mut_db.add(&msg)?;
                num_messages += 1;
                if is_goodbye {
                    mut_db.entity_db.data_store.sanity_check()?;
                    anyhow::ensure!(0 < num_messages, "No messages received");
                    re_log::info!("Successfully ingested {num_messages} messages.");
                    if let Some(db) = db {
                        return Ok(db);
                    } else {
                        anyhow::bail!("logdb never initialized");
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                anyhow::bail!(
                    "Didn't receive any messages within {} seconds. Giving up.",
                    timeout.as_secs()
                );
            }
            Err(RecvTimeoutError::Disconnected) => {
                anyhow::bail!("Channel disconnected without a Goodbye message.");
            }
        }
    }
}

#[derive(Clone)]
enum ArgumentCategory {
    /// A remote RRD file, served over http.
    RrdHttpUrl(String),

    /// A path to a local file.
    FilePath(std::path::PathBuf),

    /// A remote Rerun server.
    WebSocketAddr(String),
}

impl ArgumentCategory {
    pub fn from_uri(mut uri: String) -> ArgumentCategory {
        fn looks_like_windows_abs_path(path: &str) -> bool {
            let path = path.as_bytes();
            // "C:/" etc
            path.get(1).copied() == Some(b':') && path.get(2).copied() == Some(b'/')
        }

        fn looks_like_a_file_path(uri: &str) -> bool {
            // How do we distinguish a file path from a web url? "example.zip" could be either.

            if uri.starts_with('/') {
                return true; // Unix absolute path
            }
            if looks_like_windows_abs_path(uri) {
                return true;
            }

            // We use a simple heuristic here: if there are multiple dots, it is likely an url,
            // like "example.com/foo.zip".
            // If there is only one dot, we treat it as an extension and look it up in a list of common
            // file extensions:

            let parts = uri.split('.').collect_vec();
            if parts.len() <= 1 {
                true // Only one part. Weird. Let's assume it is a file path.
            } else if parts.len() == 2 {
                let extension = parts[1];
                matches!(
                    extension,
                    // Our own:
                    "rrd"

                    // Misc:
                    | "txt"
                    | "zip"

                    // Meshes:
                    | "glb"
                    | "gltf"
                    | "obj"
                    | "ply"
                    | "stl"

                    // Images:
                    | "avif"
                    | "bmp"
                    | "dds"
                    | "exr"
                    | "farbfeld"
                    | "ff"
                    | "gif"
                    | "hdr"
                    | "ico"
                    | "jpeg"
                    | "jpg"
                    | "pam"
                    | "pbm"
                    | "pgm"
                    | "png"
                    | "ppm"
                    | "tga"
                    | "tif"
                    | "tiff"
                    | "webp"
                )
            } else {
                false // Too many dots; assume an url
            }
        }

        let path = std::path::Path::new(&uri).to_path_buf();

        if uri.starts_with("file://") || path.exists() {
            ArgumentCategory::FilePath(path)
        } else if uri.starts_with("http://")
            || uri.starts_with("https://")
            || (uri.starts_with("www.") && uri.ends_with(".rrd"))
        {
            ArgumentCategory::RrdHttpUrl(uri)
        } else if uri.starts_with("ws://") || uri.starts_with("wss://") {
            ArgumentCategory::WebSocketAddr(uri)

        // Now we are into heuristics territory:
        } else if looks_like_a_file_path(&uri) {
            ArgumentCategory::FilePath(path)
        } else if uri.ends_with(".rrd") {
            ArgumentCategory::RrdHttpUrl(uri)
        } else {
            // If this is sometyhing like `foo.com` we can't know what it is until we connect to it.
            // We could/should connect and see what it is, but for now we just take a wild guess instead:
            re_log::debug!("Assuming WebSocket endpoint");
            if !uri.contains("://") {
                uri = format!("{}://{uri}", re_ws_comms::PROTOCOL);
            }
            ArgumentCategory::WebSocketAddr(uri)
        }
    }
}

#[test]
fn test_argument_categorization() {
    let file = [
        "file://foo",
        "foo.rrd",
        "foo.zip",
        "/foo/bar/baz",
        "D:/file",
    ];
    let http = [
        "http://foo.zip",
        "https://foo.zip",
        "example.zip/foo.rrd",
        "www.foo.zip/foo.rrd",
    ];
    let ws = ["ws://foo.zip", "wss://foo.zip", "127.0.0.1"];

    for uri in file {
        assert!(
            matches!(
                ArgumentCategory::from_uri(uri.to_owned()),
                ArgumentCategory::FilePath(_)
            ),
            "Expected {uri:?} to be categorized as FilePath"
        );
    }

    for uri in http {
        assert!(
            matches!(
                ArgumentCategory::from_uri(uri.to_owned()),
                ArgumentCategory::RrdHttpUrl(_)
            ),
            "Expected {uri:?} to be categorized as RrdHttpUrl"
        );
    }

    for uri in ws {
        assert!(
            matches!(
                ArgumentCategory::from_uri(uri.to_owned()),
                ArgumentCategory::WebSocketAddr(_)
            ),
            "Expected {uri:?} to be categorized as WebSocketAddr"
        );
    }
}

#[cfg(feature = "native_viewer")]
fn native_viewer_connect_to_ws_url(
    build_info: re_build_info::BuildInfo,
    app_env: re_viewer::AppEnvironment,
    startup_options: re_viewer::StartupOptions,
    profiler: re_viewer::Profiler,
    rerun_server_ws_url: String,
) -> anyhow::Result<()> {
    // By using RemoteViewerApp we let the user change the server they are connected to.
    re_viewer::run_native_app(Box::new(move |cc, re_ui| {
        let mut app = re_viewer::RemoteViewerApp::new(
            build_info,
            app_env,
            startup_options,
            re_ui,
            cc.storage,
            rerun_server_ws_url,
        );
        app.set_profiler(profiler);
        Box::new(app)
    }))?;
    Ok(())
}

#[allow(clippy::needless_pass_by_value)] // false positive on some feature flags
fn load_file_to_channel_at(
    recording_id: re_log_types::RecordingId,
    path: &std::path::Path,
    tx: re_smart_channel::Sender<LogMsg>,
) -> Result<(), anyhow::Error> {
    puffin::profile_function!(path.to_string_lossy());
    re_log::info!("Loading {path:?}…");

    let extension = path
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .to_string_lossy()
        .to_string();

    if extension == "rrd" {
        load_rrd_file_to_channel(path.to_owned(), tx)
    } else {
        #[cfg(feature = "sdk")]
        {
            let log_msg = re_sdk::MsgSender::from_file_path(path)?.into_log_msg(recording_id)?;
            tx.send(log_msg).ok(); // .ok(): we may be running in a background thread, so who knows if the receiver is still open
            Ok(())
        }

        #[cfg(not(feature = "sdk"))]
        {
            _ = recording_id;
            anyhow::bail!("Unsupported file extension: '{extension}' for path {path:?}. Try enabling the 'sdk' feature of 'rerun'.");
        }
    }
}

fn load_rrd_file_to_channel(
    path: std::path::PathBuf,
    tx: re_smart_channel::Sender<LogMsg>,
) -> anyhow::Result<()> {
    use anyhow::Context as _;
    let file = std::fs::File::open(&path).context("Failed to open file")?;
    let decoder = re_log_encoding::decoder::Decoder::new(file)?;

    rayon::spawn(move || {
        for msg in decoder {
            match msg {
                Ok(msg) => {
                    tx.send(msg).ok(); // .ok(): we're running in a background thread, so who knows if the receiver is still open
                }
                Err(err) => {
                    re_log::warn_once!("Failed to decode message in {path:?}: {err}");
                }
            }
        }
    });

    Ok(())
}

fn stream_to_rrd(
    rx: &re_smart_channel::Receiver<LogMsg>,
    path: &std::path::PathBuf,
) -> Result<(), re_log_encoding::FileSinkError> {
    use re_log_encoding::FileSinkError;
    use re_smart_channel::RecvError;

    if path.exists() {
        re_log::warn!("Overwriting existing file at {path:?}");
    }

    re_log::info!("Saving incoming log stream to {path:?}. Abort with Ctrl-C.");

    let file =
        std::fs::File::create(path).map_err(|err| FileSinkError::CreateFile(path.clone(), err))?;
    let mut encoder = re_log_encoding::encoder::Encoder::new(file)?;

    loop {
        match rx.recv() {
            Ok(log_msg) => {
                encoder.append(&log_msg)?;
            }
            Err(RecvError) => {
                re_log::info!("Log stream disconnected, stopping.");
                break;
            }
        }
    }

    encoder.finish()?;

    re_log::info!("File saved to {path:?}");

    Ok(())
}

#[cfg(feature = "server")]
fn parse_max_latency(max_latency: Option<&String>) -> f32 {
    max_latency.as_ref().map_or(f32::INFINITY, |time| {
        re_format::parse_duration(time)
            .unwrap_or_else(|err| panic!("Failed to parse max_latency ({max_latency:?}): {err}"))
    })
}

// ----------------------------------------------------------------------------

use re_log::external::log;

struct StrictLogger {}

impl log::Log for StrictLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        match metadata.level() {
            log::Level::Error | log::Level::Warn => true,
            log::Level::Info | log::Level::Debug | log::Level::Trace => false,
        }
    }

    fn log(&self, record: &log::Record<'_>) {
        let level = match record.level() {
            log::Level::Error => "error",
            log::Level::Warn => "warning",
            log::Level::Info | log::Level::Debug | log::Level::Trace => return,
        };

        eprintln!("{level} logged in --strict mode: {}", record.args());
        eprintln!(
            "{}",
            crate::crash_handler::callstack_from(&["log::__private_api_log"])
        );
        std::process::exit(1);
    }

    fn flush(&self) {}
}
