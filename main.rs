use axum::{routing::{get, post}, Json, Router, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::time::{timeout, Duration};
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

static PII_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4})|(\+?351\s?9[1236]\d{7})|(\b\d{9}\b)|([A-Z]{2}\d{2}[A-Z0-9]{11,30})").unwrap()
});

#[derive(Deserialize)]
struct ChatRequest { message: String }

#[derive(Serialize)]
struct ChatResponse { reply: String, provider: String, node: String, latency_ms: u128 }

struct AppState { api_key: String, http_client: Client }

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let state = Arc::new(AppState {
        api_key: std::env::var("GROQ_API_KEY").unwrap_or_else(|_| "FALTA_CHAVE".into()),
        http_client: Client::new(),
    });

    let app = Router::new()
        .route("/api/chat", post(chat_handler))
        .route("/api/logs", get(get_logs)) 
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    println!("\n==========================================");
    println!("🚀 NOVACORE GATEWAY V2.1 | PORTUGAL");
    println!("🛡️ RGPD COMPLIANCE: ATIVO");
    println!("==========================================\n");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn chat_handler(State(state): State<Arc<AppState>>, Json(payload): Json<ChatRequest>) -> Result<Json<ChatResponse>, StatusCode> {
    let start = std::time::Instant::now();
    let contains_pii = PII_RE.is_match(&payload.message);
    let sanitized = PII_RE.replace_all(&payload.message, "[DADO PROTEGIDO]").to_string();
    
    if contains_pii {
        let mut file = OpenOptions::new().create(true).append(true).open("audit.log").unwrap();
        writeln!(file, "[{:?}] BLOQUEIO RGPD: Dados sensíveis interceptados.", SystemTime::now()).unwrap();
    }
    
    let reply = match timeout(Duration::from_secs(10), call_real_ai(&sanitized, &state)).await {
        Ok(Ok(res)) => res,
        _ => "Sistema em modo de segurança: Verifique a ligação ao nó de IA.".into(),
    };

    Ok(Json(ChatResponse {
        reply,
        provider: "Llama-3 (Groq Europe)".into(),
        node: "LX-01".into(),
        latency_ms: start.elapsed().as_millis(),
    }))
}

async fn get_logs() -> Result<String, StatusCode> {
    std::fs::read_to_string("audit.log").map_err(|_| StatusCode::NOT_FOUND)
}

async fn call_real_ai(msg: &str, state: &AppState) -> anyhow::Result<String> {
    let response = state.http_client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", state.api_key))
        .json(&serde_json::json!({
            "model": "llama3-8b-8192",
            "messages": [{"role": "user", "content": msg}]
        }))
        .send().await?.json::<serde_json::Value>().await?;
    Ok(response["choices"][0]["message"]["content"].as_str().unwrap_or("Erro").to_string())
}