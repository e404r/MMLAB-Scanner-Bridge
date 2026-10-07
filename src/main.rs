use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
};

use muda::{Menu, MenuItem, PredefinedMenuItem, MenuEvent};
use tray_icon::{TrayIconBuilder, TrayIconEvent, Icon as TrayIcon};
use mdns_sd::{ServiceDaemon, ServiceEvent};

const PORT: u16 = 58260;
const CONFIG_FILE: &str = "scanner_config.json";
const UI_HTML: &str = include_str!("ui.html");
const UI_CSS: &str = include_str!("style.css");
const ICON_PNG: &[u8] = include_bytes!("../icon_128.png");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppLocale {
    Ka,
    En,
}

impl AppLocale {
    pub fn is_ka(&self) -> bool {
        matches!(self, AppLocale::Ka)
    }

    pub fn tray_title(&self) -> &'static str {
        "MMLAB Scanner Bridge"
    }

    pub fn tray_open(&self) -> &'static str {
        if self.is_ka() {
            "მართვის ცენტრის გახსნა"
        } else {
            "Open Dashboard"
        }
    }

    pub fn tray_status(&self, port: u16) -> String {
        if self.is_ka() {
            format!("სტატუსი: 🟢 ონლაინ ({})", port)
        } else {
            format!("Status: 🟢 Online ({})", port)
        }
    }

    pub fn tray_quit(&self) -> &'static str {
        if self.is_ka() {
            "გამოსვლა"
        } else {
            "Quit"
        }
    }

    pub fn tray_tooltip(&self) -> &'static str {
        if self.is_ka() {
            "MMLAB Scanner Bridge — სკანერის მართვის ცენტრი"
        } else {
            "MMLAB Scanner Bridge — Local Control Center"
        }
    }

    pub fn checking_port(&self, port: u16) -> String {
        if self.is_ka() {
            format!("🔍 მოწმდება პორტი {}...", port)
        } else {
            format!("🔍 Checking port {}...", port)
        }
    }

    pub fn already_running(&self, port: u16) -> String {
        if self.is_ka() {
            format!("ℹ️  MMLAB Scanner Bridge უკვე გაშვებულია ამ კომპიუტერზე (Port: {})!", port)
        } else {
            format!("ℹ️  MMLAB Scanner Bridge is already running (Port: {})!", port)
        }
    }

    pub fn opening_browser(&self, port: u16) -> String {
        if self.is_ka() {
            format!("🌐 მართვის პანელი იხსნება ბრაუზერში: http://127.0.0.1:{}", port)
        } else {
            format!("🌐 Opening dashboard in browser: http://127.0.0.1:{}", port)
        }
    }

    pub fn port_occupied(&self, port: u16) -> String {
        if self.is_ka() {
            format!("❌ შეცდომა: პორტი {} დაკავებულია სხვა პროგრამის მიერ!", port)
        } else {
            format!("❌ Error: Port {} is occupied by another application!", port)
        }
    }

    pub fn details(&self, msg: &str) -> String {
        if self.is_ka() {
            format!("⚠️  დეტალები: {}", msg)
        } else {
            format!("⚠️  Details: {}", msg)
        }
    }

    pub fn free_port_hint(&self, port: u16) -> String {
        if self.is_ka() {
            format!("💡 გთხოვთ გაათავისუფლოთ პორტი {} ან დახუროთ კონფლიქტური პროგრამა.", port)
        } else {
            format!("💡 Please free port {} or close the conflicting application.", port)
        }
    }

    pub fn port_available(&self, port: u16) -> String {
        if self.is_ka() {
            format!("✅ პორტი {} თავისუფალია!", port)
        } else {
            format!("✅ Port {} is available!", port)
        }
    }

    pub fn headless_mode(&self) -> &'static str {
        if self.is_ka() {
            "🚀 გაშვებულია ფონურ რეჟიმში. გასაჩერებლად დააჭირეთ Ctrl+C."
        } else {
            "🚀 Running in headless daemon mode. Press Ctrl+C to stop."
        }
    }

    pub fn launching_window(&self) -> &'static str {
        if self.is_ka() {
            "🖥️ იხსნება სამუშაო ფანჯარა..."
        } else {
            "🖥️ Launching Desktop Application Window..."
        }
    }

    pub fn conflict_dialog_msg(&self, port: u16, err_msg: &str) -> String {
        if self.is_ka() {
            format!("შეცდომა: პორტი {} დაკავებულია სხვა პროგრამის მიერ!\n\nდეტალები:\n{}", port, err_msg)
        } else {
            format!("Error: Port {} is in use by another application!\n\nDetails:\n{}", port, err_msg)
        }
    }

    pub fn err_unauthorized(&self) -> &'static str {
        if self.is_ka() {
            "არასანქცირებული წვდომა: არასწორი ან გამოტოვებული api_sec_print უსაფრთხოების კოდი (401 Unauthorized)"
        } else {
            "Unauthorized: Invalid or missing api_sec_print security token (401 Unauthorized)"
        }
    }

    pub fn err_printer_status(&self, status: StatusCode) -> String {
        if self.is_ka() {
            format!("პრინტერმა დააბრუნა შეცდომა: HTTP {}", status)
        } else {
            format!("Scanner returned error: HTTP {}", status)
        }
    }

    pub fn err_device_unreachable(&self, err: &str) -> String {
        if self.is_ka() {
            format!("მოწყობილობა მიუწვდომელია ({})", err)
        } else {
            format!("Device unreachable ({})", err)
        }
    }

    pub fn err_connect_scanner(&self, err: &str) -> String {
        if self.is_ka() {
            format!("სკანერთან დაკავშირება ვერ მოხერხდა: {}", err)
        } else {
            format!("Failed to connect to scanner: {}", err)
        }
    }

    pub fn err_scanner_rejected(&self, status: StatusCode) -> String {
        if self.is_ka() {
            format!("სკანერმა უარყო მოთხოვნა (HTTP {})", status)
        } else {
            format!("Scanner rejected request (HTTP {})", status)
        }
    }

    pub fn err_no_job_url(&self) -> &'static str {
        if self.is_ka() {
            "სკანირების სამუშაოს მისამართი ვერ განისაზღვრა"
        } else {
            "Failed to resolve scan job URL"
        }
    }

    pub fn err_scan_timeout(&self) -> &'static str {
        if self.is_ka() {
            "სკანირების დრო ამოიწურა ან მოწყობილობა არ პასუხობს"
        } else {
            "Scan timed out or device not responding"
        }
    }
}

pub fn detect_system_locale(cli_lang: Option<&str>, cfg_lang: Option<&str>) -> AppLocale {
    if let Some(l) = cli_lang {
        let low = l.to_lowercase();
        if low == "ka" || low == "geo" || low == "georgian" {
            return AppLocale::Ka;
        } else if low == "en" || low == "eng" || low == "english" {
            return AppLocale::En;
        }
    }

    if let Some(l) = cfg_lang {
        let low = l.to_lowercase();
        if low == "ka" || low == "geo" || low == "georgian" {
            return AppLocale::Ka;
        } else if low == "en" || low == "eng" || low == "english" {
            return AppLocale::En;
        }
    }

    // 1. Check environment variables
    for var in &["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(val) = std::env::var(var) {
            let v = val.to_lowercase();
            if v.starts_with("ka") || v.contains("geo") {
                return AppLocale::Ka;
            }
        }
    }

    // 2. On macOS, query defaults for AppleLanguages and AppleLocale
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleLanguages"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if let Some(first_line) = s.lines().find(|line| line.contains('"')) {
                if first_line.contains("ka") || first_line.contains("geo") {
                    return AppLocale::Ka;
                }
            }
        }

        if let Ok(output) = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleLocale"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if s.starts_with("ka") || s.contains("geo") {
                return AppLocale::Ka;
            }
        }
    }

    // 3. On Windows, check UI language
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "[System.Globalization.CultureInfo]::CurrentUICulture.Name"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if s.starts_with("ka") || s.contains("geo") {
                return AppLocale::Ka;
            }
        }
    }

    AppLocale::En
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub printer_ip: String,
    pub printer_port: u16,
    pub printer_name: String,
    #[serde(default)]
    pub api_sec_print: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            printer_ip: "192.168.3.133".to_string(),
            printer_port: 80,
            printer_name: "HP LaserJet Pro MFP 4103".to_string(),
            api_sec_print: None,
            language: None,
        }
    }
}

pub struct AppState {
    pub config: RwLock<BridgeConfig>,
    pub http_client: reqwest::Client,
    pub config_path: PathBuf,
    pub locale: AppLocale,
}

#[derive(Deserialize)]
pub struct StatusQuery {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub online: bool,
    pub state: String,
    pub ip: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredScanner {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub model: String,
    pub protocol: String,
}

#[derive(Deserialize)]
pub struct DiscoverQuery {
    pub timeout: Option<u64>,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub source: Option<String>,
    pub duplex: Option<bool>,
    pub resolution: Option<u32>,
    pub api_sec_print: Option<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|a| a == "--headless" || a == "--server-only");
    let cli_lang = args.iter().position(|a| a == "--lang" || a == "-l")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let config_path = exe_dir.join(CONFIG_FILE);

    let initial_config = if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => BridgeConfig::default(),
        }
    } else {
        let def = BridgeConfig::default();
        if let Ok(content) = serde_json::to_string_pretty(&def) {
            let _ = std::fs::write(&config_path, content);
        }
        def
    };

    let locale = detect_system_locale(cli_lang, initial_config.language.as_deref());
    let locale_name = if locale.is_ka() { "ქართული (Georgian)" } else { "English" };

    println!("🦀 =================================================");
    println!("🦀 MMLAB Scanner Bridge v1.0.0 (Rust Native Desktop App)");
    println!("🦀 Dedicated Port: http://127.0.0.1:{}", PORT);
    println!("🦀 Language: {}", locale_name);
    println!("🦀 =================================================");

    // 1. Check if port 58260 is already in use BEFORE attempting to start
    println!("{}", locale.checking_port(PORT));
    if let Err(err_msg) = check_port_availability() {
        if check_existing_instance() {
            println!("{}", locale.already_running(PORT));
            println!("{}", locale.opening_browser(PORT));
            open_url_in_browser(&format!("http://127.0.0.1:{}", PORT));
            return;
        } else {
            eprintln!("{}", locale.port_occupied(PORT));
            eprintln!("{}", locale.details(&err_msg));
            eprintln!("{}", locale.free_port_hint(PORT));
            show_conflict_dialog(locale, PORT, &err_msg);
            std::process::exit(1);
        }
    }
    println!("{}", locale.port_available(PORT));

    // Channel to signal server is bound and ready
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();

    // Spawn server in dedicated background thread
    let server_config = initial_config.clone();
    let server_config_path = config_path.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime");

        rt.block_on(async move {
            run_server(server_config, server_config_path, locale, ready_tx).await;
        });
    });

    // Wait until port 58260 is listening
    let _ = ready_rx.recv();

    if headless {
        println!("{}", locale.headless_mode());
        loop {
            std::thread::sleep(Duration::from_secs(3600));
        }
    } else {
        println!("{}", locale.launching_window());
        let event_loop = EventLoop::new();

        let mut builder = WindowBuilder::new()
            .with_title("MMLAB Scanner Bridge")
            .with_inner_size(LogicalSize::new(980.0, 720.0))
            .with_min_inner_size(LogicalSize::new(760.0, 520.0));

        if let Some(icon) = load_app_icon() {
            builder = builder.with_window_icon(Some(icon));
        }

        let window = builder.build(&event_loop).expect("Failed to create desktop window");

        // Setup System Tray Menu
        let tray_menu = Menu::new();
        let item_title = MenuItem::new(locale.tray_title(), false, None);
        let item_open = MenuItem::new(locale.tray_open(), true, None);
        let item_sep1 = PredefinedMenuItem::separator();
        let item_status = MenuItem::new(&locale.tray_status(PORT), false, None);
        let item_sep2 = PredefinedMenuItem::separator();
        let item_quit = MenuItem::new(locale.tray_quit(), true, None);

        let open_id = item_open.id().clone();
        let quit_id = item_quit.id().clone();

        let _ = tray_menu.append(&item_title);
        let _ = tray_menu.append(&item_open);
        let _ = tray_menu.append(&item_sep1);
        let _ = tray_menu.append(&item_status);
        let _ = tray_menu.append(&item_sep2);
        let _ = tray_menu.append(&item_quit);

        let mut tray_builder = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(locale.tray_tooltip());

        if let Some(tray_icon) = load_tray_icon() {
            tray_builder = tray_builder.with_icon(tray_icon);
        }

        let _tray = tray_builder.build().ok();

        let _webview = wry::WebViewBuilder::new()
            .with_url(format!("http://127.0.0.1:{}", PORT))
            .build(&window)
            .expect("Failed to initialize WebView");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // Process menu events
            if let Ok(menu_event) = MenuEvent::receiver().try_recv() {
                if menu_event.id == open_id {
                    window.set_visible(true);
                    window.set_focus();
                } else if menu_event.id == quit_id {
                    *control_flow = ControlFlow::Exit;
                }
            }

            // Process tray icon clicks
            if let Ok(tray_event) = TrayIconEvent::receiver().try_recv() {
                if let TrayIconEvent::Click { button: tray_icon::MouseButton::Left, .. } = tray_event {
                    window.set_visible(true);
                    window.set_focus();
                }
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    // Hide window into System Tray / Menu Bar instead of quitting
                    window.set_visible(false);
                }
                _ => (),
            }
        });
    }
}

async fn run_server(
    initial_config: BridgeConfig,
    config_path: PathBuf,
    locale: AppLocale,
    ready_tx: std::sync::mpsc::Sender<()>,
) {
    println!("📍 Scanner IP: {}:{} ({})", initial_config.printer_ip, initial_config.printer_port, initial_config.printer_name);

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap_or_default();

    let state = Arc::new(AppState {
        config: RwLock::new(initial_config),
        http_client,
        config_path,
        locale,
    });

    // Permissive CORS for seamless browser communication
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .expose_headers(Any);

    let app = Router::new()
        .route("/", get(handle_ui))
        .route("/dashboard", get(handle_ui))
        .route("/style.css", get(handle_css))
        .route("/health", get(handle_health))
        .route("/config", get(handle_get_config).post(handle_save_config))
        .route("/status", get(handle_status))
        .route("/discover", get(handle_discover))
        .route("/scan", post(handle_scan))
        .route("/open-url", get(handle_open_url))
        .route("/icon.png", get(handle_icon))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind port 58260");
    println!("🚀 Server listening on http://127.0.0.1:{}", PORT);

    // Signal main thread that port is open
    let _ = ready_tx.send(());

    axum::serve(listener, app).await.expect("Server error");
}

async fn handle_ui() -> impl IntoResponse {
    Html(UI_HTML)
}

async fn handle_css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/css; charset=utf-8"));
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=86400"),
    );
    (StatusCode::OK, headers, UI_CSS)
}

async fn handle_icon() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/png"));
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=86400"),
    );
    (StatusCode::OK, headers, ICON_PNG)
}

#[derive(Deserialize)]
pub struct OpenUrlQuery {
    pub url: String,
}

async fn handle_open_url(Query(q): Query<OpenUrlQuery>) -> StatusCode {
    let url = q.url.trim();
    if url.starts_with("http://") || url.starts_with("https://") {
        open_url_in_browser(url);
    }
    StatusCode::OK
}

fn check_port_availability() -> Result<(), String> {
    match std::net::TcpListener::bind(("127.0.0.1", PORT)) {
        Ok(temp_listener) => {
            drop(temp_listener);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn check_existing_instance() -> bool {
    use std::io::{Read, Write};
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    if let Ok(mut stream) = std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
        let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
        let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
        let req = format!("GET /health HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nConnection: close\r\n\r\n", PORT);
        if stream.write_all(req.as_bytes()).is_ok() {
            let mut buf = [0u8; 512];
            if let Ok(n) = stream.read(&mut buf) {
                let resp = String::from_utf8_lossy(&buf[..n]);
                return resp.contains("mmlab-scanner-bridge") || resp.contains("hr-scanner-bridge") || resp.contains("Scanner Bridge");
            }
        }
    }
    false
}

fn show_conflict_dialog(locale: AppLocale, port: u16, err_msg: &str) {
    let msg = locale.conflict_dialog_msg(port, err_msg);
    #[cfg(target_os = "macos")]
    {
        let script = format!(r#"display alert "MMLAB Scanner Bridge" message "{}" as critical"#, msg.replace('"', "\\\""));
        let _ = std::process::Command::new("osascript").args(["-e", &script]).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("msg").args(["*", &msg]).spawn();
    }
}

fn open_url_in_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd").args(["/C", "start", url]).spawn();
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

fn load_app_icon() -> Option<Icon> {
    if let Ok(img) = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png) {
        let rgba = img.into_rgba8();
        let (width, height) = rgba.dimensions();
        Icon::from_rgba(rgba.into_raw(), width, height).ok()
    } else {
        None
    }
}

fn load_tray_icon() -> Option<TrayIcon> {
    if let Ok(img) = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png) {
        let rgba = img.into_rgba8();
        let (width, height) = rgba.dimensions();
        TrayIcon::from_rgba(rgba.into_raw(), width, height).ok()
    } else {
        None
    }
}

async fn handle_health(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "app": "hr-scanner-bridge",
        "version": "1.0.0",
        "port": PORT,
        "language": if state.locale.is_ka() { "ka" } else { "en" }
    }))
}

async fn handle_get_config(State(state): State<Arc<AppState>>) -> Json<BridgeConfig> {
    let cfg = state.config.read().await;
    Json(cfg.clone())
}

async fn handle_save_config(
    State(state): State<Arc<AppState>>,
    Json(new_cfg): Json<BridgeConfig>,
) -> Result<Json<BridgeConfig>, (StatusCode, String)> {
    let mut cfg = state.config.write().await;
    *cfg = new_cfg.clone();

    // Persist to file
    if let Ok(json_str) = serde_json::to_string_pretty(&new_cfg) {
        let _ = tokio::fs::write(&state.config_path, json_str).await;
    }

    println!("💾 Updated config: IP={} Name={}", new_cfg.printer_ip, new_cfg.printer_name);
    Ok(Json(new_cfg))
}

async fn handle_status(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StatusQuery>,
) -> Json<StatusResponse> {
    let (ip, port) = {
        let cfg = state.config.read().await;
        (
            query.ip.unwrap_or_else(|| cfg.printer_ip.clone()),
            query.port.unwrap_or(cfg.printer_port),
        )
    };

    let url = format!("http://{}:{}/eSCL/ScannerStatus", ip, port);

    let res = state
        .http_client
        .get(&url)
        .timeout(Duration::from_secs(4))
        .header(header::CONNECTION, "close")
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let text = response.text().await.unwrap_or_default();
                let state_re = Regex::new(r"(?i)<[^:]*:?State>([^<]+)</[^:]*:?State>").unwrap();
                let printer_state = state_re
                    .captures(&text)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_else(|| "Idle".to_string());

                Json(StatusResponse {
                    online: true,
                    state: printer_state,
                    ip,
                    port,
                    error: None,
                })
            } else {
                Json(StatusResponse {
                    online: false,
                    state: "Error".to_string(),
                    ip,
                    port,
                    error: Some(state.locale.err_printer_status(response.status())),
                })
            }
        }
        Err(err) => Json(StatusResponse {
            online: false,
            state: "Offline".to_string(),
            ip,
            port,
            error: Some(state.locale.err_device_unreachable(&err.to_string())),
        }),
    }
}

async fn handle_discover(
    State(state): State<Arc<AppState>>,
    Query(query): Query<DiscoverQuery>,
) -> Json<Vec<DiscoveredScanner>> {
    let timeout_sec = query.timeout.unwrap_or(2).clamp(1, 10);
    let is_ka = state.locale.is_ka();

    if is_ka {
        println!("🔍 მიმდინარეობს ქსელში eSCL სკანერების ძებნა (mDNS {} წმ)...", timeout_sec);
    } else {
        println!("🔍 Searching local network for eSCL scanners (mDNS {}s)...", timeout_sec);
    }

    let scanners = tokio::task::spawn_blocking(move || {
        let mdns = match ServiceDaemon::new() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to init mDNS daemon: {}", e);
                return Vec::new();
            }
        };

        let service_type = "_uscan._tcp.local.";
        let receiver = match mdns.browse(service_type) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to browse mDNS: {}", e);
                return Vec::new();
            }
        };

        let mut list: Vec<DiscoveredScanner> = Vec::new();
        let timeout = Duration::from_secs(timeout_sec);
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if let Ok(event) = receiver.recv_timeout(Duration::from_millis(200)) {
                if let ServiceEvent::ServiceResolved(info) = event {
                    let port = info.get_port();
                    let hostname = info.get_hostname().trim_end_matches('.').to_string();
                    let model = info
                        .get_properties()
                        .get("ty")
                        .or_else(|| info.get_properties().get("mdl"))
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| hostname.clone());

                    for addr in info.get_addresses() {
                        if addr.is_ipv4() {
                            let ip = addr.to_string();
                            if !list.iter().any(|s| s.ip == ip && s.port == port) {
                                if is_ka {
                                    println!("✨ ნაპოვნია სკანერი: {} ({}:{})", model, ip, port);
                                } else {
                                    println!("✨ Found scanner: {} ({}:{})", model, ip, port);
                                }
                                list.push(DiscoveredScanner {
                                    name: hostname.clone(),
                                    ip,
                                    port,
                                    model: model.clone(),
                                    protocol: "eSCL".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        let _ = mdns.stop_browse(service_type);
        list
    })
    .await
    .unwrap_or_default();

    Json(scanners)
}

async fn handle_scan(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ScanRequest>,
) -> Response {
    // 0. Verify security token api_sec_print if configured
    let expected_token = {
        let cfg = state.config.read().await;
        cfg.api_sec_print.clone()
    };

    if let Some(expected) = expected_token {
        let expected_trimmed = expected.trim();
        if !expected_trimmed.is_empty() {
            let provided = headers
                .get("api_sec_print")
                .or_else(|| headers.get("x-api-sec-print"))
                .and_then(|v| v.to_str().ok())
                .or(payload.api_sec_print.as_deref());

            if provided != Some(expected_trimmed) {
                return (
                    StatusCode::UNAUTHORIZED,
                    state.locale.err_unauthorized(),
                ).into_response();
            }
        }
    }
    let (ip, port) = {
        let cfg = state.config.read().await;
        (
            payload.ip.unwrap_or_else(|| cfg.printer_ip.clone()),
            payload.port.unwrap_or(cfg.printer_port),
        )
    };

    let source = payload.source.unwrap_or_else(|| "Platen".to_string());
    let duplex = payload.duplex.unwrap_or(false) && source == "Feeder";
    let resolution = payload.resolution.unwrap_or(300);

    let duplex_xml = if duplex {
        "\n  <scan:Duplex>true</scan:Duplex>"
    } else {
        ""
    };

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<scan:ScanSettings xmlns:scan="http://schemas.hp.com/imaging/escl/2011/05/03" xmlns:pwg="http://www.pwg.org/schemas/2010/12/sm">
  <pwg:Version>2.0</pwg:Version>
  <scan:Intent>Document</scan:Intent>
  <scan:DocumentFormat>application/pdf</scan:DocumentFormat>
  <scan:InputSource>{source}</scan:InputSource>{duplex_xml}
  <scan:XResolution>{resolution}</scan:XResolution>
  <scan:YResolution>{resolution}</scan:YResolution>
</scan:ScanSettings>"#
    );

    if state.locale.is_ka() {
        println!("🖨️ სკანირების დაწყება {}:{} (წყარო: {}, ორმხრივი: {}, DPI: {})", ip, port, source, duplex, resolution);
    } else {
        println!("🖨️ Starting scan on {}:{} (Source: {}, Duplex: {}, DPI: {})", ip, port, source, duplex, resolution);
    }

    // 1. POST to /eSCL/ScanJobs
    let post_url = format!("http://{}:{}/eSCL/ScanJobs", ip, port);
    let post_res = match state
        .http_client
        .post(&post_url)
        .header(header::CONTENT_TYPE, "text/xml")
        .body(xml)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                state.locale.err_connect_scanner(&e.to_string()),
            )
                .into_response();
        }
    };

    if !post_res.status().is_success() && post_res.status() != StatusCode::CREATED {
        return (
            StatusCode::BAD_GATEWAY,
            state.locale.err_scanner_rejected(post_res.status()),
        )
            .into_response();
    }

    // Extract Location header
    let job_location = post_res
        .headers()
        .get(header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let next_doc_url = if let Some(loc) = job_location {
        let path = if loc.starts_with("http") {
            loc
        } else {
            format!("http://{}:{}{}", ip, port, if loc.starts_with('/') { "" } else { "/" }) + &loc
        };
        if path.ends_with('/') {
            format!("{}NextDocument", path)
        } else {
            format!("{}/NextDocument", path)
        }
    } else {
        // Fallback: query ScannerStatus for active JobUri
        let status_url = format!("http://{}:{}/eSCL/ScannerStatus", ip, port);
        let status_text = if let Ok(resp) = state.http_client.get(&status_url).send().await {
            resp.text().await.unwrap_or_default()
        } else {
            String::new()
        };

        let job_re = Regex::new(r"(?i)<[^:]*:?JobUri>([^<]+)</[^:]*:?JobUri>").unwrap();
        if let Some(captures) = job_re.captures(&status_text) {
            let uri = captures.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let slash = if uri.starts_with('/') { "" } else { "/" };
            format!("http://{}:{}{}{}/NextDocument", ip, port, slash, uri)
        } else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                state.locale.err_no_job_url().to_string(),
            )
                .into_response();
        }
    };

    println!("⏳ Polling document from: {}", next_doc_url);

    // 2. Polling loop on NextDocument
    let max_attempts = 60;
    let mut pdf_bytes = None;

    for attempt in 1..=max_attempts {
        tokio::time::sleep(Duration::from_millis(500)).await;

        match state.http_client.get(&next_doc_url).send().await {
            Ok(doc_res) => {
                if doc_res.status() == StatusCode::OK {
                    if let Ok(bytes) = doc_res.bytes().await {
                        if state.locale.is_ka() {
                            println!("✅ სკანირება დასრულდა წარმატებით (მცდელობა {})! ზომა: {} ბაიტი", attempt, bytes.len());
                        } else {
                            println!("✅ Scan completed successfully (attempt {})! Size: {} bytes", attempt, bytes.len());
                        }
                        pdf_bytes = Some(bytes);
                        break;
                    }
                } else if doc_res.status() == StatusCode::SERVICE_UNAVAILABLE {
                    let retry_sec = doc_res
                        .headers()
                        .get(header::RETRY_AFTER)
                        .and_then(|h| h.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(2);

                    tokio::time::sleep(Duration::from_secs(retry_sec)).await;
                    continue;
                } else {
                    println!("⚠️ Polling returned unexpected status: {}", doc_res.status());
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
            Err(e) => {
                println!("⚠️ Polling error on attempt {}: {}", attempt, e);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    match pdf_bytes {
        Some(bytes) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
            headers.insert(
                header::CONTENT_DISPOSITION,
                HeaderValue::from_static("inline; filename=\"scanned_document.pdf\""),
            );
            (StatusCode::OK, headers, bytes).into_response()
        }
        None => (
            StatusCode::GATEWAY_TIMEOUT,
            state.locale.err_scan_timeout().to_string(),
        )
            .into_response(),
    }
}
