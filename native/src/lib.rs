use cc_switch_lib as core;
use once_cell::sync::{Lazy, OnceCell};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().unwrap_or_else(|error| panic!("failed to create tokio runtime: {error}"))
});
static APP_STATE: OnceCell<Arc<core::AppState>> = OnceCell::new();
static SKILL_SERVICE: Lazy<core::SkillService> = Lazy::new(core::SkillService::new);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppPayload {
    app: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppIdPayload {
    app: String,
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPayload {
    app: String,
    provider: core::Provider,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppTypePayload {
    app_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppTypeEnabledPayload {
    app_type: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppTypeProviderPayload {
    app_type: String,
    provider_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderHealthPayload {
    provider_id: String,
    app_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPricingPayload {
    provider_id: String,
    app_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddCustomEndpointPayload {
    app: String,
    provider_id: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSortOrderPayload {
    app: String,
    updates: Vec<core::ProviderSortUpdate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertPromptPayload {
    app: String,
    id: String,
    prompt: core::Prompt,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppPromptPayload {
    app: String,
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PromptFilePayload {
    app: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertClaudeMcpPayload {
    id: String,
    spec: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValidateMcpPayload {
    cmd: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetMcpConfigPayload {
    app: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertMcpServerInConfigPayload {
    app: String,
    id: String,
    server: core::McpServer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteMcpServerInConfigPayload {
    app: String,
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToggleMcpAppPayload {
    server_id: String,
    app: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertMcpServerPayload {
    server: core::McpServer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IdPayload {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UrlsPayload {
    urls: Vec<String>,
    timeout_secs: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UniversalProviderPayload {
    provider: core::UniversalProvider,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SkillRepoOwnerPayload {
    owner: String,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SkillRepoPayload {
    repo: core::SkillRepo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstallSkillUnifiedPayload {
    skill: core::DiscoverableSkill,
    current_app: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RestoreSkillBackupPayload {
    backup_id: String,
    current_app: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToggleSkillAppPayload {
    id: String,
    app: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportSkillsPayload {
    imports: Vec<core::ImportSkillSelection>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstallSkillFromZipPayload {
    file_path: String,
    current_app: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenClawLiveProviderPayload {
    provider_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenClawDefaultModelPayload {
    model: core::OpenClawDefaultModel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenClawModelCatalogPayload {
    catalog: std::collections::HashMap<String, core::OpenClawModelCatalogEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenClawAgentsDefaultsPayload {
    defaults: core::OpenClawAgentsDefaults,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenClawEnvPayload {
    env: core::OpenClawEnvConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenClawToolsPayload {
    tools: core::OpenClawToolsConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageQueryPayload {
    provider_id: String,
    app: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestUsageScriptPayload {
    provider_id: String,
    app: String,
    script_code: String,
    timeout: Option<u64>,
    api_key: Option<String>,
    base_url: Option<String>,
    access_token: Option<String>,
    user_id: Option<String>,
    template_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageRangePayload {
    start_date: Option<i64>,
    end_date: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestLogsPayload {
    filters: core::LogFilters,
    page: u32,
    page_size: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestDetailPayload {
    request_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceFilePayload {
    filename: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceWritePayload {
    filename: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceSearchPayload {
    query: String,
}

fn app_state() -> Result<&'static Arc<core::AppState>, String> {
    APP_STATE.get_or_try_init(|| {
        let db = Arc::new(core::Database::init().map_err(|error| error.to_string())?);
        Ok(Arc::new(core::AppState::new(db)))
    })
}

fn parse_payload<T: DeserializeOwned>(payload: Value) -> Result<T, String> {
    let normalized = if payload.is_null() {
        Value::Object(Default::default())
    } else {
        payload
    };
    serde_json::from_value(normalized).map_err(|error| format!("invalid payload: {error}"))
}

fn to_value<T: serde::Serialize>(value: T) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|error| format!("serialize response failed: {error}"))
}

fn parse_app_type(app: &str) -> Result<core::AppType, String> {
    core::AppType::from_str(app).map_err(|error| error.to_string())
}

fn parse_skill_app_type(app: &str) -> Result<core::AppType, String> {
    match app.to_ascii_lowercase().as_str() {
        "claude" => Ok(core::AppType::Claude),
        "codex" => Ok(core::AppType::Codex),
        "gemini" => Ok(core::AppType::Gemini),
        "opencode" => Ok(core::AppType::OpenCode),
        "openclaw" => Ok(core::AppType::OpenClaw),
        _ => Err(format!("unsupported app type: {app}")),
    }
}

fn with_runtime<F, T>(future: F) -> Result<T, String>
where
    F: std::future::Future<Output = Result<T, String>>,
{
    RUNTIME.block_on(future)
}

fn handle_request(method: &str, payload: Value) -> Result<Value, String> {
    match method {
        "get_init_error" => with_runtime(async { core::get_init_error().await.and_then(to_value) }),
        "is_portable_mode" => with_runtime(async { core::is_portable_mode().await.and_then(to_value) }),
        "get_settings" => with_runtime(async { core::get_settings().await.and_then(to_value) }),
        "save_settings" => {
            let settings: core::AppSettings = parse_payload(payload)?;
            with_runtime(async move { core::save_settings(settings).await.and_then(to_value) })
        }
        "get_config_dir" => {
            let payload: AppPayload = parse_payload(payload)?;
            with_runtime(async move { core::get_config_dir(payload.app).await.and_then(to_value) })
        }
        "get_claude_code_config_path" => {
            with_runtime(async { core::get_claude_code_config_path().await.and_then(to_value) })
        }
        "get_app_config_path" => {
            with_runtime(async { core::get_app_config_path().await.and_then(to_value) })
        }
        "get_app_config_dir_override" => to_value(Option::<String>::None),
        "set_app_config_dir_override" => to_value(false),
        "get_auto_launch_status" => {
            with_runtime(async { core::get_auto_launch_status().await.and_then(to_value) })
        }
        "set_auto_launch" => {
            #[derive(Deserialize)]
            struct Payload {
                enabled: bool,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move { core::set_auto_launch(payload.enabled).await.and_then(to_value) })
        }
        "get_rectifier_config" => to_value(app_state()?.db.get_rectifier_config().map_err(|e| e.to_string())?),
        "set_rectifier_config" => {
            #[derive(Deserialize)]
            struct Payload {
                config: core::RectifierConfig,
            }
            let payload: Payload = parse_payload(payload)?;
            app_state()?
                .db
                .set_rectifier_config(&payload.config)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "get_optimizer_config" => to_value(app_state()?.db.get_optimizer_config().map_err(|e| e.to_string())?),
        "set_optimizer_config" => {
            #[derive(Deserialize)]
            struct Payload {
                config: core::OptimizerConfig,
            }
            let payload: Payload = parse_payload(payload)?;
            app_state()?
                .db
                .set_optimizer_config(&payload.config)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "get_log_config" => to_value(app_state()?.db.get_log_config().map_err(|e| e.to_string())?),
        "set_log_config" => {
            #[derive(Deserialize)]
            struct Payload {
                config: core::LogConfig,
            }
            let payload: Payload = parse_payload(payload)?;
            app_state()?
                .db
                .set_log_config(&payload.config)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }

        "get_providers" => {
            let payload: AppPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::list(app_state()?, app_type).map_err(|e| e.to_string())?)
        }
        "get_current_provider" => {
            let payload: AppPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::current(app_state()?, app_type).map_err(|e| e.to_string())?)
        }
        "add_provider" => {
            let payload: ProviderPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::add(app_state()?, app_type, payload.provider).map_err(|e| e.to_string())?)
        }
        "update_provider" => {
            let payload: ProviderPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::update(app_state()?, app_type, payload.provider).map_err(|e| e.to_string())?)
        }
        "delete_provider" => {
            let payload: AppIdPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::delete(app_state()?, app_type, &payload.id).map(|_| true).map_err(|e| e.to_string())?)
        }
        "remove_provider_from_live_config" => {
            let payload: AppIdPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(
                core::ProviderService::remove_from_live_config(app_state()?, app_type, &payload.id)
                    .map(|_| true)
                    .map_err(|e| e.to_string())?,
            )
        }
        "switch_provider" => {
            let payload: AppIdPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::switch(app_state()?, app_type, &payload.id).map_err(|e| e.to_string())?)
        }
        "import_default_config" => {
            let payload: AppPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::import_default_config(app_state()?, app_type).map_err(|e| e.to_string())?)
        }
        "read_live_provider_settings" => {
            let payload: AppPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::ProviderService::read_live_settings(app_type).map_err(|e| e.to_string())?)
        }
        "test_api_endpoints" => {
            let payload: UrlsPayload = parse_payload(payload)?;
            with_runtime(async move {
                core::SpeedtestService::test_endpoints(payload.urls, payload.timeout_secs)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(to_value)
            })
        }
        "get_custom_endpoints" => {
            let payload: AddCustomEndpointPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(
                core::ProviderService::get_custom_endpoints(app_state()?, app_type, &payload.provider_id)
                    .map_err(|e| e.to_string())?,
            )
        }
        "add_custom_endpoint" => {
            let payload: AddCustomEndpointPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            core::ProviderService::add_custom_endpoint(app_state()?, app_type, &payload.provider_id, payload.url)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "remove_custom_endpoint" => {
            let payload: AddCustomEndpointPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            core::ProviderService::remove_custom_endpoint(app_state()?, app_type, &payload.provider_id, payload.url)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "update_endpoint_last_used" => {
            let payload: AddCustomEndpointPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            core::ProviderService::update_endpoint_last_used(
                app_state()?,
                app_type,
                &payload.provider_id,
                payload.url,
            )
            .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "update_providers_sort_order" => {
            let payload: UpdateSortOrderPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(
                core::ProviderService::update_sort_order(app_state()?, app_type, payload.updates)
                    .map_err(|e| e.to_string())?,
            )
        }
        "get_universal_providers" => {
            to_value(core::ProviderService::list_universal(app_state()?).map_err(|e| e.to_string())?)
        }
        "get_universal_provider" => {
            let payload: IdPayload = parse_payload(payload)?;
            to_value(core::ProviderService::get_universal(app_state()?, &payload.id).map_err(|e| e.to_string())?)
        }
        "upsert_universal_provider" => {
            let payload: UniversalProviderPayload = parse_payload(payload)?;
            to_value(core::ProviderService::upsert_universal(app_state()?, payload.provider).map_err(|e| e.to_string())?)
        }
        "delete_universal_provider" => {
            let payload: IdPayload = parse_payload(payload)?;
            to_value(core::ProviderService::delete_universal(app_state()?, &payload.id).map_err(|e| e.to_string())?)
        }
        "sync_universal_provider" => {
            let payload: IdPayload = parse_payload(payload)?;
            to_value(core::ProviderService::sync_universal_to_apps(app_state()?, &payload.id).map_err(|e| e.to_string())?)
        }

        "start_proxy_server" => with_runtime(async { app_state()?.proxy_service.start().await.and_then(to_value) }),
        "stop_proxy_with_restore" => with_runtime(async {
            app_state()?
                .proxy_service
                .stop_with_restore()
                .await
                .map(|_| Value::Null)
        }),
        "get_proxy_takeover_status" => {
            with_runtime(async { app_state()?.proxy_service.get_takeover_status().await.and_then(to_value) })
        }
        "set_proxy_takeover_for_app" => {
            let payload: AppTypeEnabledPayload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .proxy_service
                    .set_takeover_for_app(&payload.app_type, payload.enabled)
                    .await
                    .map(|_| Value::Null)
            })
        }
        "get_proxy_status" => with_runtime(async { app_state()?.proxy_service.get_status().await.and_then(to_value) }),
        "get_proxy_config" => with_runtime(async { app_state()?.proxy_service.get_config().await.and_then(to_value) }),
        "update_proxy_config" => {
            #[derive(Deserialize)]
            struct Payload {
                config: core::ProxyConfig,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .proxy_service
                    .update_config(&payload.config)
                    .await
                    .map(|_| Value::Null)
            })
        }
        "get_global_proxy_config" => with_runtime(async {
            let config = app_state()?
                .db
                .get_global_proxy_config()
                .await
                .map_err(|e| e.to_string())?;
            to_value(config)
        }),
        "update_global_proxy_config" => {
            #[derive(Deserialize)]
            struct Payload {
                config: core::GlobalProxyConfig,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .db
                    .update_global_proxy_config(payload.config)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Value::Null)
            })
        }
        "get_proxy_config_for_app" => {
            let payload: AppTypePayload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .db
                    .get_proxy_config_for_app(&payload.app_type)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(to_value)
            })
        }
        "update_proxy_config_for_app" => {
            #[derive(Deserialize)]
            struct Payload {
                config: core::AppProxyConfig,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .db
                    .update_proxy_config_for_app(payload.config)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Value::Null)
            })
        }
        "get_default_cost_multiplier" => {
            let payload: AppTypePayload = parse_payload(payload)?;
            with_runtime(async move {
                core::get_default_cost_multiplier_test_hook(app_state()?, &payload.app_type)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(to_value)
            })
        }
        "set_default_cost_multiplier" => {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Payload {
                app_type: String,
                value: String,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                core::set_default_cost_multiplier_test_hook(app_state()?, &payload.app_type, &payload.value)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Value::Null)
            })
        }
        "get_pricing_model_source" => {
            let payload: AppTypePayload = parse_payload(payload)?;
            with_runtime(async move {
                core::get_pricing_model_source_test_hook(app_state()?, &payload.app_type)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(to_value)
            })
        }
        "set_pricing_model_source" => {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Payload {
                app_type: String,
                value: String,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                core::set_pricing_model_source_test_hook(app_state()?, &payload.app_type, &payload.value)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Value::Null)
            })
        }
        "is_proxy_running" => with_runtime(async { Ok(to_value(app_state()?.proxy_service.is_running().await)?) }),
        "is_live_takeover_active" => {
            with_runtime(async { app_state()?.proxy_service.is_takeover_active().await.and_then(to_value) })
        }
        "switch_proxy_provider" => {
            let payload: AppTypeProviderPayload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .proxy_service
                    .switch_proxy_target(&payload.app_type, &payload.provider_id)
                    .await
                    .map(|_| Value::Null)
            })
        }
        "get_provider_health" => {
            let payload: ProviderHealthPayload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .db
                    .get_provider_health(&payload.provider_id, &payload.app_type)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(to_value)
            })
        }
        "reset_circuit_breaker" => {
            let payload: ProviderHealthPayload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .db
                    .update_provider_health(&payload.provider_id, &payload.app_type, true, None)
                    .await
                    .map_err(|e| e.to_string())?;
                app_state()?
                    .proxy_service
                    .reset_provider_circuit_breaker(&payload.provider_id, &payload.app_type)
                    .await?;
                Ok(Value::Null)
            })
        }
        "get_circuit_breaker_config" => {
            with_runtime(async { app_state()?.db.get_circuit_breaker_config().await.map_err(|e| e.to_string()).and_then(to_value) })
        }
        "update_circuit_breaker_config" => {
            #[derive(Deserialize)]
            struct Payload {
                config: core::CircuitBreakerConfig,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                app_state()?
                    .db
                    .update_circuit_breaker_config(&payload.config)
                    .await
                    .map_err(|e| e.to_string())?;
                app_state()?
                    .proxy_service
                    .update_circuit_breaker_configs(payload.config)
                    .await?;
                Ok(Value::Null)
            })
        }
        "get_circuit_breaker_stats" => to_value(Option::<core::CircuitBreakerStats>::None),
        "get_failover_queue" => {
            let payload: AppTypePayload = parse_payload(payload)?;
            to_value(app_state()?.db.get_failover_queue(&payload.app_type).map_err(|e| e.to_string())?)
        }
        "get_available_providers_for_failover" => {
            let payload: AppTypePayload = parse_payload(payload)?;
            to_value(app_state()?.db.get_available_providers_for_failover(&payload.app_type).map_err(|e| e.to_string())?)
        }
        "add_to_failover_queue" => {
            let payload: AppTypeProviderPayload = parse_payload(payload)?;
            app_state()?
                .db
                .add_to_failover_queue(&payload.app_type, &payload.provider_id)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "remove_from_failover_queue" => {
            let payload: AppTypeProviderPayload = parse_payload(payload)?;
            app_state()?
                .db
                .remove_from_failover_queue(&payload.app_type, &payload.provider_id)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "get_auto_failover_enabled" => {
            let payload: AppTypePayload = parse_payload(payload)?;
            with_runtime(async move {
                let enabled = app_state()?
                    .db
                    .get_proxy_config_for_app(&payload.app_type)
                    .await
                    .map_err(|e| e.to_string())?
                    .auto_failover_enabled;
                to_value(enabled)
            })
        }
        "set_auto_failover_enabled" => {
            let payload: AppTypeEnabledPayload = parse_payload(payload)?;
            with_runtime(async move {
                let mut config = app_state()?
                    .db
                    .get_proxy_config_for_app(&payload.app_type)
                    .await
                    .map_err(|e| e.to_string())?;
                config.auto_failover_enabled = payload.enabled;
                app_state()?
                    .db
                    .update_proxy_config_for_app(config)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Value::Null)
            })
        }

        "get_prompts" => {
            let payload: AppPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::PromptService::get_prompts(app_state()?, app_type).map_err(|e| e.to_string())?)
        }
        "upsert_prompt" => {
            let payload: UpsertPromptPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            core::PromptService::upsert_prompt(app_state()?, app_type, &payload.id, payload.prompt)
                .map_err(|e| e.to_string())?;
            to_value(Value::Null)
        }
        "delete_prompt" => {
            let payload: AppPromptPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            core::PromptService::delete_prompt(app_state()?, app_type, &payload.id).map_err(|e| e.to_string())?;
            to_value(Value::Null)
        }
        "enable_prompt" => {
            let payload: AppPromptPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            core::PromptService::enable_prompt(app_state()?, app_type, &payload.id).map_err(|e| e.to_string())?;
            to_value(Value::Null)
        }
        "get_current_prompt_file_content" => {
            let payload: PromptFilePayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::PromptService::get_current_file_content(app_type).map_err(|e| e.to_string())?)
        }

        "get_claude_mcp_status" => with_runtime(async { core::get_claude_mcp_status().await.and_then(to_value) }),
        "read_claude_mcp_config" => with_runtime(async { core::read_claude_mcp_config().await.and_then(to_value) }),
        "upsert_claude_mcp_server" => {
            let payload: UpsertClaudeMcpPayload = parse_payload(payload)?;
            with_runtime(async move { core::upsert_claude_mcp_server(payload.id, payload.spec).await.and_then(to_value) })
        }
        "delete_claude_mcp_server" => {
            let payload: IdPayload = parse_payload(payload)?;
            with_runtime(async move { core::delete_claude_mcp_server(payload.id).await.and_then(to_value) })
        }
        "validate_mcp_command" => {
            let payload: ValidateMcpPayload = parse_payload(payload)?;
            with_runtime(async move { core::validate_mcp_command(payload.cmd).await.and_then(to_value) })
        }
        "get_mcp_config" => {
            let payload: GetMcpConfigPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            #[derive(serde::Serialize)]
            #[serde(rename_all = "camelCase")]
            struct McpConfigResponse {
                config_path: String,
                servers: std::collections::HashMap<String, serde_json::Value>,
            }

            let config_path = std::env::var("HOME")
                .map(|home| format!("{home}/.cc-switch/config.json"))
                .unwrap_or_else(|_| "~/.cc-switch/config.json".to_string());
            let servers = core::McpService::get_servers(app_state()?, app_type).map_err(|e| e.to_string())?;
            to_value(McpConfigResponse { config_path, servers })
        }
        "upsert_mcp_server_in_config" => {
            let payload: UpsertMcpServerInConfigPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            let existing = app_state()?
                .db
                .get_all_mcp_servers()
                .map_err(|e| e.to_string())?
                .get(&payload.id)
                .cloned();
            let mut server = existing.unwrap_or_else(|| {
                let mut apps = core::McpApps::default();
                apps.set_enabled_for(&app_type, true);
                core::McpServer {
                    id: payload.id.clone(),
                    name: payload.server.name.clone(),
                    server: payload.server.server.clone(),
                    apps,
                    description: payload.server.description.clone(),
                    homepage: payload.server.homepage.clone(),
                    docs: payload.server.docs.clone(),
                    tags: payload.server.tags.clone(),
                }
            });
            server.server = payload.server.server;
            server.apps.set_enabled_for(&app_type, true);
            core::McpService::upsert_server(app_state()?, server).map_err(|e| e.to_string())?;
            to_value(true)
        }
        "delete_mcp_server_in_config" => {
            let payload: DeleteMcpServerInConfigPayload = parse_payload(payload)?;
            let _app_type = parse_app_type(&payload.app)?;
            core::McpService::delete_server(app_state()?, &payload.id).map_err(|e| e.to_string())?;
            to_value(true)
        }
        "set_mcp_enabled" => {
            let payload: ToggleMcpAppPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            core::McpService::set_enabled(app_state()?, app_type, &payload.server_id, payload.enabled)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "get_mcp_servers" => {
            to_value(core::McpService::get_all_servers(app_state()?).map_err(|e| e.to_string())?)
        }
        "upsert_mcp_server" => {
            let payload: UpsertMcpServerPayload = parse_payload(payload)?;
            core::McpService::upsert_server(app_state()?, payload.server).map_err(|e| e.to_string())?;
            to_value(true)
        }
        "delete_mcp_server" => {
            let payload: IdPayload = parse_payload(payload)?;
            to_value(core::McpService::delete_server(app_state()?, &payload.id).map_err(|e| e.to_string())?)
        }
        "toggle_mcp_app" => {
            let payload: ToggleMcpAppPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            to_value(core::McpService::toggle_app(app_state()?, &payload.server_id, app_type, payload.enabled).map_err(|e| e.to_string())?)
        }
        "import_mcp_from_apps" => {
            let state = app_state()?;
            let total = core::McpService::import_from_claude(state).map_err(|e| e.to_string())?
                + core::McpService::import_from_codex(state).map_err(|e| e.to_string())?
                + core::McpService::import_from_gemini(state).map_err(|e| e.to_string())?
                + core::McpService::import_from_opencode(state).map_err(|e| e.to_string())?;
            to_value(total)
        }

        "get_installed_skills" => to_value(core::SkillService::get_all_installed(&app_state()?.db).map_err(|e| e.to_string())?),
        "get_skill_backups" => to_value(core::SkillService::list_backups().map_err(|e| e.to_string())?),
        "delete_skill_backup" => {
            let payload: IdPayload = parse_payload(payload)?;
            core::SkillService::delete_backup(&payload.id).map_err(|e| e.to_string())?;
            to_value(true)
        }
        "install_skill_unified" => {
            let payload: InstallSkillUnifiedPayload = parse_payload(payload)?;
            let app_type = parse_skill_app_type(&payload.current_app)?;
            with_runtime(async move {
                SKILL_SERVICE
                    .install(&app_state()?.db, &payload.skill, &app_type)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(to_value)
            })
        }
        "uninstall_skill_unified" => {
            let payload: IdPayload = parse_payload(payload)?;
            to_value(core::SkillService::uninstall(&app_state()?.db, &payload.id).map_err(|e| e.to_string())?)
        }
        "restore_skill_backup" => {
            let payload: RestoreSkillBackupPayload = parse_payload(payload)?;
            let app_type = parse_skill_app_type(&payload.current_app)?;
            to_value(
                core::SkillService::restore_from_backup(&app_state()?.db, &payload.backup_id, &app_type)
                    .map_err(|e| e.to_string())?,
            )
        }
        "toggle_skill_app" => {
            let payload: ToggleSkillAppPayload = parse_payload(payload)?;
            let app_type = parse_skill_app_type(&payload.app)?;
            core::SkillService::toggle_app(&app_state()?.db, &payload.id, &app_type, payload.enabled)
                .map_err(|e| e.to_string())?;
            to_value(true)
        }
        "scan_unmanaged_skills" => {
            to_value(core::SkillService::scan_unmanaged(&app_state()?.db).map_err(|e| e.to_string())?)
        }
        "import_skills_from_apps" => {
            let payload: ImportSkillsPayload = parse_payload(payload)?;
            to_value(core::SkillService::import_from_apps(&app_state()?.db, payload.imports).map_err(|e| e.to_string())?)
        }
        "discover_available_skills" => {
            let repos = app_state()?.db.get_skill_repos().map_err(|e| e.to_string())?;
            with_runtime(async move { SKILL_SERVICE.discover_available(repos).await.map_err(|e| e.to_string()).and_then(to_value) })
        }
        "get_skills" => {
            let repos = app_state()?.db.get_skill_repos().map_err(|e| e.to_string())?;
            let db = app_state()?.db.clone();
            with_runtime(async move { SKILL_SERVICE.list_skills(repos, &db).await.map_err(|e| e.to_string()).and_then(to_value) })
        }
        "get_skills_for_app" => {
            let _payload: AppPayload = parse_payload(payload)?;
            let repos = app_state()?.db.get_skill_repos().map_err(|e| e.to_string())?;
            let db = app_state()?.db.clone();
            with_runtime(async move { SKILL_SERVICE.list_skills(repos, &db).await.map_err(|e| e.to_string()).and_then(to_value) })
        }
        "get_skill_repos" => to_value(app_state()?.db.get_skill_repos().map_err(|e| e.to_string())?),
        "add_skill_repo" => {
            let payload: SkillRepoPayload = parse_payload(payload)?;
            app_state()?.db.save_skill_repo(&payload.repo).map_err(|e| e.to_string())?;
            to_value(true)
        }
        "remove_skill_repo" => {
            let payload: SkillRepoOwnerPayload = parse_payload(payload)?;
            app_state()?.db.delete_skill_repo(&payload.owner, &payload.name).map_err(|e| e.to_string())?;
            to_value(true)
        }
        "install_skills_from_zip" => {
            let payload: InstallSkillFromZipPayload = parse_payload(payload)?;
            let app_type = parse_skill_app_type(&payload.current_app)?;
            to_value(
                core::SkillService::install_from_zip(
                    &app_state()?.db,
                    std::path::Path::new(&payload.file_path),
                    &app_type,
                )
                .map_err(|e| e.to_string())?,
            )
        }

        "get_openclaw_live_provider_ids" => with_runtime(async { core::get_openclaw_live_provider_ids().and_then(to_value) }),
        "import_openclaw_providers_from_live" => {
            to_value(core::import_openclaw_providers_from_live(app_state()?).map_err(|e| e.to_string())?)
        }
        "get_openclaw_live_provider" => {
            let payload: OpenClawLiveProviderPayload = parse_payload(payload)?;
            with_runtime(async move { core::get_openclaw_live_provider(payload.provider_id).and_then(to_value) })
        }
        "scan_openclaw_config_health" => with_runtime(async { core::scan_openclaw_config_health().and_then(to_value) }),
        "get_openclaw_default_model" => with_runtime(async { core::get_openclaw_default_model().and_then(to_value) }),
        "set_openclaw_default_model" => {
            let payload: OpenClawDefaultModelPayload = parse_payload(payload)?;
            with_runtime(async move { core::set_openclaw_default_model(payload.model).and_then(to_value) })
        }
        "get_openclaw_model_catalog" => with_runtime(async { core::get_openclaw_model_catalog().and_then(to_value) }),
        "set_openclaw_model_catalog" => {
            let payload: OpenClawModelCatalogPayload = parse_payload(payload)?;
            with_runtime(async move { core::set_openclaw_model_catalog(payload.catalog).and_then(to_value) })
        }
        "get_openclaw_agents_defaults" => with_runtime(async { core::get_openclaw_agents_defaults().and_then(to_value) }),
        "set_openclaw_agents_defaults" => {
            let payload: OpenClawAgentsDefaultsPayload = parse_payload(payload)?;
            with_runtime(async move { core::set_openclaw_agents_defaults(payload.defaults).and_then(to_value) })
        }
        "get_openclaw_env" => with_runtime(async { core::get_openclaw_env().and_then(to_value) }),
        "set_openclaw_env" => {
            let payload: OpenClawEnvPayload = parse_payload(payload)?;
            with_runtime(async move { core::set_openclaw_env(payload.env).and_then(to_value) })
        }
        "get_openclaw_tools" => with_runtime(async { core::get_openclaw_tools().and_then(to_value) }),
        "set_openclaw_tools" => {
            let payload: OpenClawToolsPayload = parse_payload(payload)?;
            with_runtime(async move { core::set_openclaw_tools(payload.tools).and_then(to_value) })
        }
        "import_opencode_providers_from_live" => {
            to_value(core::import_opencode_providers_from_live(app_state()?).map_err(|e| e.to_string())?)
        }
        "get_opencode_live_provider_ids" => {
            with_runtime(async { core::get_opencode_live_provider_ids().and_then(to_value) })
        }

        "read_omo_local_file" => with_runtime(async { core::read_omo_local_file().await.and_then(to_value) }),
        "get_current_omo_provider_id" => {
            let provider = app_state()?
                .db
                .get_current_omo_provider("opencode", "omo")
                .map_err(|e| e.to_string())?;
            to_value(provider.map(|item| item.id).unwrap_or_default())
        }
        "disable_current_omo" => {
            let providers = app_state()?.db.get_all_providers("opencode").map_err(|e| e.to_string())?;
            for (id, provider) in providers {
                if provider.category.as_deref() == Some("omo") {
                    app_state()?
                        .db
                        .clear_omo_provider_current("opencode", &id, "omo")
                        .map_err(|e| e.to_string())?;
                }
            }
            core::OmoService::delete_config_file(&core::OMO_STANDARD).map_err(|e| e.to_string())?;
            to_value(Value::Null)
        }
        "read_omo_slim_local_file" => with_runtime(async { core::read_omo_slim_local_file().await.and_then(to_value) }),
        "get_current_omo_slim_provider_id" => {
            let provider = app_state()?
                .db
                .get_current_omo_provider("opencode", "omo-slim")
                .map_err(|e| e.to_string())?;
            to_value(provider.map(|item| item.id).unwrap_or_default())
        }
        "disable_current_omo_slim" => {
            let providers = app_state()?.db.get_all_providers("opencode").map_err(|e| e.to_string())?;
            for (id, provider) in providers {
                if provider.category.as_deref() == Some("omo-slim") {
                    app_state()?
                        .db
                        .clear_omo_provider_current("opencode", &id, "omo-slim")
                        .map_err(|e| e.to_string())?;
                }
            }
            core::OmoService::delete_config_file(&core::OMO_SLIM).map_err(|e| e.to_string())?;
            to_value(Value::Null)
        }

        "queryProviderUsage" => {
            let payload: UsageQueryPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            with_runtime(async move {
                core::ProviderService::query_usage(app_state()?, app_type, &payload.provider_id)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(to_value)
            })
        }
        "testUsageScript" => {
            let payload: TestUsageScriptPayload = parse_payload(payload)?;
            let app_type = parse_app_type(&payload.app)?;
            with_runtime(async move {
                core::ProviderService::test_usage_script(
                    app_state()?,
                    app_type,
                    &payload.provider_id,
                    &payload.script_code,
                    payload.timeout.unwrap_or(10),
                    payload.api_key.as_deref(),
                    payload.base_url.as_deref(),
                    payload.access_token.as_deref(),
                    payload.user_id.as_deref(),
                    payload.template_type.as_deref(),
                )
                .await
                .map_err(|e| e.to_string())
                .and_then(to_value)
            })
        }
        "get_usage_summary" => {
            let payload: UsageRangePayload = parse_payload(payload)?;
            to_value(app_state()?.db.get_usage_summary(payload.start_date, payload.end_date).map_err(|e| e.to_string())?)
        }
        "get_usage_trends" => {
            let payload: UsageRangePayload = parse_payload(payload)?;
            to_value(app_state()?.db.get_daily_trends(payload.start_date, payload.end_date).map_err(|e| e.to_string())?)
        }
        "get_provider_stats" => to_value(app_state()?.db.get_provider_stats().map_err(|e| e.to_string())?),
        "get_model_stats" => to_value(app_state()?.db.get_model_stats().map_err(|e| e.to_string())?),
        "get_request_logs" => {
            let payload: RequestLogsPayload = parse_payload(payload)?;
            to_value(app_state()?.db.get_request_logs(&payload.filters, payload.page, payload.page_size).map_err(|e| e.to_string())?)
        }
        "get_request_detail" => {
            let payload: RequestDetailPayload = parse_payload(payload)?;
            to_value(app_state()?.db.get_request_detail(&payload.request_id).map_err(|e| e.to_string())?)
        }
        "check_provider_limits" => {
            let payload: ProviderPricingPayload = parse_payload(payload)?;
            to_value(app_state()?.db.check_provider_limits(&payload.provider_id, &payload.app_type).map_err(|e| e.to_string())?)
        }

        "list_sessions" => with_runtime(async { core::list_sessions().await.and_then(to_value) }),
        "get_session_messages" => {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Payload {
                provider_id: String,
                source_path: String,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                core::get_session_messages(payload.provider_id, payload.source_path)
                    .await
                    .and_then(to_value)
            })
        }
        "delete_session" => {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Payload {
                provider_id: String,
                session_id: String,
                source_path: String,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                core::delete_session(payload.provider_id, payload.session_id, payload.source_path)
                    .await
                    .and_then(to_value)
            })
        }
        "launch_session_terminal" => {
            #[derive(Deserialize)]
            struct Payload {
                command: String,
                cwd: Option<String>,
                custom_config: Option<String>,
            }
            let payload: Payload = parse_payload(payload)?;
            with_runtime(async move {
                core::launch_session_terminal(payload.command, payload.cwd, payload.custom_config)
                    .await
                    .and_then(to_value)
            })
        }

        "check_env_conflicts" => {
            #[derive(Deserialize)]
            struct Payload {
                app: String,
            }
            let payload: Payload = parse_payload(payload)?;
            to_value(core::check_env_conflicts(payload.app).map_err(|e| e.to_string())?)
        }
        "delete_env_vars" => {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Payload {
                conflicts: Vec<core::EnvConflict>,
            }
            let payload: Payload = parse_payload(payload)?;
            to_value(core::delete_env_vars(payload.conflicts).map_err(|e| e.to_string())?)
        }
        "restore_env_backup" => {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Payload {
                backup_path: String,
            }
            let payload: Payload = parse_payload(payload)?;
            core::restore_env_backup(payload.backup_path).map_err(|e| e.to_string())?;
            to_value(Value::Null)
        }

        "parse_deeplink" => {
            #[derive(Deserialize)]
            struct Payload {
                url: String,
            }
            let payload: Payload = parse_payload(payload)?;
            to_value(core::parse_deeplink(payload.url).map_err(|e| e.to_string())?)
        }
        "merge_deeplink_config" => {
            #[derive(Deserialize)]
            struct Payload {
                request: core::DeepLinkImportRequest,
            }
            let payload: Payload = parse_payload(payload)?;
            to_value(core::merge_deeplink_config(payload.request).map_err(|e| e.to_string())?)
        }
        "read_workspace_file" => {
            let payload: WorkspaceFilePayload = parse_payload(payload)?;
            with_runtime(async move { core::read_workspace_file(payload.filename).await.and_then(to_value) })
        }
        "write_workspace_file" => {
            let payload: WorkspaceWritePayload = parse_payload(payload)?;
            with_runtime(async move { core::write_workspace_file(payload.filename, payload.content).await.and_then(to_value) })
        }
        "list_daily_memory_files" => with_runtime(async { core::list_daily_memory_files().await.and_then(to_value) }),
        "read_daily_memory_file" => {
            let payload: WorkspaceFilePayload = parse_payload(payload)?;
            with_runtime(async move { core::read_daily_memory_file(payload.filename).await.and_then(to_value) })
        }
        "write_daily_memory_file" => {
            let payload: WorkspaceWritePayload = parse_payload(payload)?;
            with_runtime(async move { core::write_daily_memory_file(payload.filename, payload.content).await.and_then(to_value) })
        }
        "delete_daily_memory_file" => {
            let payload: WorkspaceFilePayload = parse_payload(payload)?;
            with_runtime(async move { core::delete_daily_memory_file(payload.filename).await.and_then(to_value) })
        }
        "search_daily_memory_files" => {
            let payload: WorkspaceSearchPayload = parse_payload(payload)?;
            with_runtime(async move { core::search_daily_memory_files(payload.query).await.and_then(to_value) })
        }

        _ => Err(format!("unsupported method: {method}")),
    }
}

#[no_mangle]
pub extern "C" fn otools_plugin_invoke(
    input_ptr: *const u8,
    input_len: usize,
    output_len: *mut usize,
) -> *mut u8 {
    if input_ptr.is_null() || output_len.is_null() {
        return std::ptr::null_mut();
    }

    let input = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let parsed: Value = match serde_json::from_slice(input) {
        Ok(value) => value,
        Err(error) => {
            return write_response(
                json!({
                    "ok": false,
                    "error": format!("Invalid input: {error}"),
                }),
                output_len,
            )
        }
    };

    let (method, payload) = match parsed {
        Value::Object(map) => (
            map.get("method")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            map.get("payload").cloned().unwrap_or(Value::Null),
        ),
        _ => (String::new(), Value::Null),
    };

    let response = match handle_request(&method, payload) {
        Ok(data) => json!({ "ok": true, "data": data }),
        Err(error) => json!({ "ok": false, "error": error }),
    };

    write_response(response, output_len)
}

fn write_response(response: Value, output_len: *mut usize) -> *mut u8 {
    let mut output =
        serde_json::to_vec(&response).unwrap_or_else(|_| br#"{"ok":false,"error":"serialize failed"}"#.to_vec());
    let len = output.len();
    unsafe {
        *output_len = len;
    }
    let ptr = output.as_mut_ptr();
    std::mem::forget(output);
    ptr
}

#[no_mangle]
pub extern "C" fn otools_plugin_free(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}
