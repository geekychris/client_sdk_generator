use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::config::{
    RetryConfig, TelemetryConfig, CachingConfig, TargetLanguage,
    BackoffStrategy, TelemetryProvider
};

/// Feature code generators that create language-specific implementations
/// of retry logic, telemetry, and caching
pub struct FeatureGenerator {
    language: TargetLanguage,
}

impl FeatureGenerator {
    pub fn new(language: TargetLanguage) -> Self {
        Self { language }
    }
    
    /// Generate retry logic implementation
    pub fn generate_retry_code(&self, config: &RetryConfig) -> FeatureCode {
        match self.language {
            TargetLanguage::Java => self.generate_java_retry(config),
            TargetLanguage::Python => self.generate_python_retry(config),
            TargetLanguage::Rust => self.generate_rust_retry(config),
            TargetLanguage::Go => self.generate_go_retry(config),
            TargetLanguage::TypeScript => self.generate_typescript_retry(config),
        }
    }
    
    /// Generate telemetry implementation
    pub fn generate_telemetry_code(&self, config: &TelemetryConfig) -> FeatureCode {
        match self.language {
            TargetLanguage::Java => self.generate_java_telemetry(config),
            TargetLanguage::Python => self.generate_python_telemetry(config),
            TargetLanguage::Rust => self.generate_rust_telemetry(config),
            TargetLanguage::Go => self.generate_go_telemetry(config),
            TargetLanguage::TypeScript => self.generate_typescript_telemetry(config),
        }
    }
    
    /// Generate caching implementation
    pub fn generate_caching_code(&self, config: &CachingConfig) -> FeatureCode {
        match self.language {
            TargetLanguage::Java => self.generate_java_caching(config),
            TargetLanguage::Python => self.generate_python_caching(config),
            TargetLanguage::Rust => self.generate_rust_caching(config),
            TargetLanguage::Go => self.generate_go_caching(config),
            TargetLanguage::TypeScript => self.generate_typescript_caching(config),
        }
    }
    
    // Java implementations
    fn generate_java_retry(&self, config: &RetryConfig) -> FeatureCode {
        let dependencies = vec![
            "dev.failsafe:failsafe:3.3.2".to_string(),
            "dev.failsafe:failsafe-okhttp:3.3.2".to_string(),
        ];
        
        let imports = vec![
            "import dev.failsafe.Failsafe;".to_string(),
            "import dev.failsafe.RetryPolicy;".to_string(),
            "import java.time.Duration;".to_string(),
        ];
        
        let backoff_config = match &config.backoff_strategy {
            BackoffStrategy::Fixed { delay_ms } => {
                format!(".withDelay(Duration.ofMillis({}))", delay_ms)
            }
            BackoffStrategy::Exponential { initial_delay_ms, multiplier, max_delay_ms } => {
                format!(
                    ".withBackoff(Duration.ofMillis({}), Duration.ofMillis({}), {})",
                    initial_delay_ms, max_delay_ms, multiplier
                )
            }
            BackoffStrategy::Linear { initial_delay_ms, increment_ms } => {
                format!(
                    ".withDelay(Duration.ofMillis({}), Duration.ofMillis({}))",
                    initial_delay_ms, increment_ms
                )
            }
        };
        
        let code = format!(
            r#"
private static final RetryPolicy<Object> RETRY_POLICY = RetryPolicy.builder()
    .withMaxAttempts({})
    {}
    .handle(ConnectException.class, SocketTimeoutException.class)
    .build();

public <T> T executeWithRetry(Supplier<T> operation) {{
    return Failsafe.with(RETRY_POLICY).get(operation);
}}
"#,
            config.max_attempts, backoff_config
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_java_telemetry(&self, config: &TelemetryConfig) -> FeatureCode {
        let mut dependencies = vec![
            "io.micrometer:micrometer-core:1.12.0".to_string(),
        ];
        
        let mut imports = vec![
            "import io.micrometer.core.instrument.MeterRegistry;".to_string(),
            "import io.micrometer.core.instrument.Timer;".to_string(),
            "import io.micrometer.core.instrument.Counter;".to_string(),
        ];
        
        match &config.provider {
            TelemetryProvider::Prometheus => {
                dependencies.push("io.micrometer:micrometer-registry-prometheus:1.12.0".to_string());
                imports.push("import io.micrometer.prometheus.PrometheusConfig;".to_string());
                imports.push("import io.micrometer.prometheus.PrometheusMeterRegistry;".to_string());
            }
            TelemetryProvider::OpenTelemetry => {
                dependencies.push("io.opentelemetry:opentelemetry-api:1.32.0".to_string());
                imports.push("import io.opentelemetry.api.OpenTelemetry;".to_string());
            }
            _ => {}
        }
        
        let code = r#"
private final MeterRegistry meterRegistry;
private final Timer requestTimer;
private final Counter requestCounter;
private final Counter errorCounter;

public void recordRequest(String method, String path, long durationMs, boolean success) {
    requestTimer.record(Duration.ofMillis(durationMs));
    requestCounter.increment(
        Tags.of(
            Tag.of("method", method),
            Tag.of("path", path)
        )
    );
    
    if (!success) {
        errorCounter.increment(
            Tags.of(
                Tag.of("method", method),
                Tag.of("path", path)
            )
        );
    }
}
"#.to_string();
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_java_caching(&self, config: &CachingConfig) -> FeatureCode {
        let dependencies = vec![
            "com.github.ben-manes.caffeine:caffeine:3.1.8".to_string(),
        ];
        
        let imports = vec![
            "import com.github.benmanes.caffeine.cache.Cache;".to_string(),
            "import com.github.benmanes.caffeine.cache.Caffeine;".to_string(),
            "import java.time.Duration;".to_string(),
        ];
        
        let code = format!(
            r#"
private final Cache<String, Object> cache = Caffeine.newBuilder()
    .maximumSize({})
    .expireAfterWrite(Duration.ofSeconds({}))
    .build();

public <T> Optional<T> getFromCache(String key, Class<T> type) {{
    Object value = cache.getIfPresent(key);
    return value != null ? Optional.of(type.cast(value)) : Optional.empty();
}}

public void putInCache(String key, Object value) {{
    cache.put(key, value);
}}

private String generateCacheKey(String method, Object... params) {{
    return method + ":" + Arrays.hashCode(params);
}}
"#,
            config.max_cache_size, config.default_ttl_seconds
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    // Python implementations
    fn generate_python_retry(&self, config: &RetryConfig) -> FeatureCode {
        let dependencies = vec![
            "tenacity>=8.2.0".to_string(),
        ];
        
        let imports = vec![
            "from tenacity import retry, stop_after_attempt, wait_exponential, retry_if_exception_type".to_string(),
            "import httpx".to_string(),
        ];
        
        let wait_strategy = match &config.backoff_strategy {
            BackoffStrategy::Fixed { delay_ms } => {
                format!("wait_fixed({})", *delay_ms as f64 / 1000.0)
            }
            BackoffStrategy::Exponential { initial_delay_ms, multiplier, max_delay_ms } => {
                format!(
                    "wait_exponential(multiplier={}, min={}, max={})",
                    multiplier,
                    *initial_delay_ms as f64 / 1000.0,
                    *max_delay_ms as f64 / 1000.0
                )
            }
            BackoffStrategy::Linear { initial_delay_ms, increment_ms } => {
                format!(
                    "wait_incrementing(start={}, increment={})",
                    *initial_delay_ms as f64 / 1000.0,
                    *increment_ms as f64 / 1000.0
                )
            }
        };
        
        let code = format!(
            r#"
@retry(
    stop=stop_after_attempt({}),
    wait={},
    retry=retry_if_exception_type((httpx.ConnectError, httpx.TimeoutException))
)
async def execute_with_retry(self, operation):
    return await operation()
"#,
            config.max_attempts, wait_strategy
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_python_telemetry(&self, config: &TelemetryConfig) -> FeatureCode {
        let mut dependencies = vec![
            "prometheus-client>=0.19.0".to_string(),
        ];
        
        let mut imports = vec![
            "from prometheus_client import Counter, Histogram, CollectorRegistry".to_string(),
            "import time".to_string(),
        ];
        
        match &config.provider {
            TelemetryProvider::OpenTelemetry => {
                dependencies.push("opentelemetry-api>=1.21.0".to_string());
                imports.push("from opentelemetry import trace".to_string());
            }
            _ => {}
        }
        
        let code = r#"
class TelemetryHandler:
    def __init__(self):
        self.registry = CollectorRegistry()
        self.request_duration = Histogram(
            'http_request_duration_seconds',
            'Time spent on HTTP requests',
            ['method', 'path'],
            registry=self.registry
        )
        self.request_count = Counter(
            'http_requests_total',
            'Total HTTP requests',
            ['method', 'path', 'status'],
            registry=self.registry
        )
    
    def record_request(self, method: str, path: str, duration: float, status: int):
        self.request_duration.labels(method=method, path=path).observe(duration)
        self.request_count.labels(method=method, path=path, status=str(status)).inc()
"#.to_string();
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_python_caching(&self, config: &CachingConfig) -> FeatureCode {
        let dependencies = vec![
            "cachetools>=5.3.0".to_string(),
        ];
        
        let imports = vec![
            "from cachetools import TTLCache".to_string(),
            "import hashlib".to_string(),
            "import json".to_string(),
        ];
        
        let code = format!(
            r#"
class CacheHandler:
    def __init__(self):
        self.cache = TTLCache(maxsize={}, ttl={})
    
    def get_from_cache(self, key: str):
        return self.cache.get(key)
    
    def put_in_cache(self, key: str, value):
        self.cache[key] = value
    
    def generate_cache_key(self, method: str, **params) -> str:
        cache_data = {{"method": method, "params": params}}
        cache_str = json.dumps(cache_data, sort_keys=True)
        return hashlib.md5(cache_str.encode()).hexdigest()
"#,
            config.max_cache_size, config.default_ttl_seconds
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    // Rust implementations
    fn generate_rust_retry(&self, config: &RetryConfig) -> FeatureCode {
        let dependencies = vec![
            "tokio-retry = \"0.3\"".to_string(),
            "tokio = { version = \"1.0\", features = [\"time\"] }".to_string(),
        ];
        
        let imports = vec![
            "use tokio_retry::{strategy::*, Retry};".to_string(),
            "use std::time::Duration;".to_string(),
        ];
        
        let strategy = match &config.backoff_strategy {
            BackoffStrategy::Fixed { delay_ms } => {
                format!("FixedInterval::from_millis({})", delay_ms)
            }
            BackoffStrategy::Exponential { initial_delay_ms, multiplier, max_delay_ms } => {
                format!(
                    "ExponentialBackoff::from_millis({}).factor({}).max_delay(Duration::from_millis({}))",
                    initial_delay_ms, multiplier, max_delay_ms
                )
            }
            BackoffStrategy::Linear { initial_delay_ms, increment_ms } => {
                format!(
                    "FibonacciBackoff::from_millis({}).max_delay(Duration::from_millis({}))",
                    initial_delay_ms, increment_ms * 10 // Approximation
                )
            }
        };
        
        let code = format!(
            r#"
use tokio_retry::{{strategy::*, Retry}};

pub async fn execute_with_retry<F, T, E>(operation: F) -> Result<T, E>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>> + Send,
    E: std::fmt::Debug,
{{
    let retry_strategy = {}.take({});
    
    Retry::spawn(retry_strategy, || async {{
        operation().await
    }}).await
}}
"#,
            strategy, config.max_attempts
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_rust_telemetry(&self, config: &TelemetryConfig) -> FeatureCode {
        let mut dependencies = vec![
            "prometheus = \"0.13\"".to_string(),
            "lazy_static = \"1.4\"".to_string(),
        ];
        
        let mut imports = vec![
            "use prometheus::{Counter, Histogram, Registry, Opts, HistogramOpts};".to_string(),
            "use lazy_static::lazy_static;".to_string(),
            "use std::time::Instant;".to_string(),
        ];
        
        match &config.provider {
            TelemetryProvider::OpenTelemetry => {
                dependencies.push("opentelemetry = \"0.21\"".to_string());
                imports.push("use opentelemetry::{trace::Tracer, global};".to_string());
            }
            _ => {}
        }
        
        let code = r#"
lazy_static! {
    static ref REQUEST_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
    ).unwrap();
    
    static ref REQUEST_COUNT: Counter = Counter::with_opts(
        Opts::new("http_requests_total", "Total HTTP requests")
    ).unwrap();
}

pub struct TelemetryHandler {
    registry: Registry,
}

impl TelemetryHandler {
    pub fn new() -> Self {
        let registry = Registry::new();
        registry.register(Box::new(REQUEST_DURATION.clone())).unwrap();
        registry.register(Box::new(REQUEST_COUNT.clone())).unwrap();
        
        Self { registry }
    }
    
    pub fn record_request(&self, method: &str, path: &str, duration: f64) {
        REQUEST_DURATION.observe(duration);
        REQUEST_COUNT.inc();
    }
}
"#.to_string();
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_rust_caching(&self, config: &CachingConfig) -> FeatureCode {
        let dependencies = vec![
            "moka = { version = \"0.12\", features = [\"future\"] }".to_string(),
            "tokio = { version = \"1.0\", features = [\"time\"] }".to_string(),
        ];
        
        let imports = vec![
            "use moka::future::Cache;".to_string(),
            "use std::time::Duration;".to_string(),
            "use std::hash::{Hash, Hasher};".to_string(),
            "use std::collections::hash_map::DefaultHasher;".to_string(),
        ];
        
        let code = format!(
            r#"
pub struct CacheHandler {{
    cache: Cache<String, serde_json::Value>,
}}

impl CacheHandler {{
    pub fn new() -> Self {{
        let cache = Cache::builder()
            .max_capacity({})
            .time_to_live(Duration::from_secs({}))
            .build();
        
        Self {{ cache }}
    }}
    
    pub async fn get_from_cache<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {{
        let value = self.cache.get(key).await?;
        serde_json::from_value(value).ok()
    }}
    
    pub async fn put_in_cache<T>(&self, key: String, value: &T)
    where
        T: serde::Serialize,
    {{
        if let Ok(json_value) = serde_json::to_value(value) {{
            self.cache.insert(key, json_value).await;
        }}
    }}
    
    pub fn generate_cache_key(&self, method: &str, params: &impl Hash) -> String {{
        let mut hasher = DefaultHasher::new();
        method.hash(&mut hasher);
        params.hash(&mut hasher);
        format!("{{}}:{{:x}}", method, hasher.finish())
    }}
}}
"#,
            config.max_cache_size, config.default_ttl_seconds
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    // Go implementations
    fn generate_go_retry(&self, config: &RetryConfig) -> FeatureCode {
        let dependencies = vec![
            // No external dependencies - using standard library
        ];
        
        let imports = vec![
            "time".to_string(),
            "math".to_string(),
        ];
        
        let code = format!(
            r#"
type RetryConfig struct {{
    MaxAttempts int
    BaseDelay   time.Duration
    MaxDelay    time.Duration
}}

func (c *Client) executeWithRetry(fn func() error) error {{
    var lastErr error
    
    for attempt := 0; attempt < {}; attempt++ {{
        err := fn()
        if err == nil {{
            return nil
        }}
        
        lastErr = err
        
        if attempt < {} - 1 {{
            delay := time.Duration(math.Pow(2, float64(attempt))) * time.Second
            if delay > 10*time.Second {{
                delay = 10 * time.Second
            }}
            time.Sleep(delay)
        }}
    }}
    
    return lastErr
}}
"#,
            config.max_attempts, config.max_attempts
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_go_telemetry(&self, config: &TelemetryConfig) -> FeatureCode {
        let dependencies = vec![
            "github.com/prometheus/client_golang/prometheus".to_string(),
        ];
        
        let imports = vec![
            "github.com/prometheus/client_golang/prometheus".to_string(),
            "time".to_string(),
        ];
        
        let code = r#"
var (
    requestDuration = prometheus.NewHistogramVec(
        prometheus.HistogramOpts{
            Name: "http_request_duration_seconds",
            Help: "Duration of HTTP requests.",
        },
        []string{"method", "path"},
    )
    
    requestCount = prometheus.NewCounterVec(
        prometheus.CounterOpts{
            Name: "http_requests_total",
            Help: "Total number of HTTP requests.",
        },
        []string{"method", "path", "status"},
    )
)

func init() {
    prometheus.MustRegister(requestDuration)
    prometheus.MustRegister(requestCount)
}

func (c *Client) recordMetrics(method, path string, duration time.Duration, statusCode int) {
    requestDuration.WithLabelValues(method, path).Observe(duration.Seconds())
    requestCount.WithLabelValues(method, path, fmt.Sprintf("%d", statusCode)).Inc()
}
"#.to_string();
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_go_caching(&self, config: &CachingConfig) -> FeatureCode {
        let dependencies = vec![
            "github.com/patrickmn/go-cache".to_string(),
        ];
        
        let imports = vec![
            "github.com/patrickmn/go-cache".to_string(),
            "time".to_string(),
            "crypto/md5".to_string(),
            "fmt".to_string(),
        ];
        
        let code = format!(
            r#"
type CacheHandler struct {{
    cache *cache.Cache
}}

func NewCacheHandler() *CacheHandler {{
    c := cache.New({}*time.Second, 10*time.Minute)
    return &CacheHandler{{cache: c}}
}}

func (ch *CacheHandler) Get(key string) (interface{{}}, bool) {{
    return ch.cache.Get(key)
}}

func (ch *CacheHandler) Set(key string, value interface{{}}) {{
    ch.cache.Set(key, value, cache.DefaultExpiration)
}}

func (ch *CacheHandler) GenerateCacheKey(method string, params ...interface{{}}) string {{
    h := md5.New()
    h.Write([]byte(method))
    for _, param := range params {{
        h.Write([]byte(fmt.Sprintf("%v", param)))
    }}
    return fmt.Sprintf("%x", h.Sum(nil))
}}
"#,
            config.default_ttl_seconds
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    // TypeScript implementations
    fn generate_typescript_retry(&self, config: &RetryConfig) -> FeatureCode {
        let dependencies = vec![
            // No external dependencies - using built-in features
        ];
        
        let imports = vec![
            // No special imports needed
        ];
        
        let code = format!(
            r#"
interface RetryConfig {{
  maxAttempts: number;
  baseDelay: number;
  maxDelay: number;
}}

class RetryHandler {{
  private config: RetryConfig;
  
  constructor(config: RetryConfig = {{ maxAttempts: {}, baseDelay: 1000, maxDelay: 10000 }}) {{
    this.config = config;
  }}
  
  async executeWithRetry<T>(fn: () => Promise<T>): Promise<T> {{
    let lastError: Error;
    
    for (let attempt = 0; attempt < this.config.maxAttempts; attempt++) {{
      try {{
        return await fn();
      }} catch (error) {{
        lastError = error instanceof Error ? error : new Error(String(error));
        
        if (attempt < this.config.maxAttempts - 1) {{
          const delay = Math.min(
            this.config.baseDelay * Math.pow(2, attempt),
            this.config.maxDelay
          );
          await new Promise(resolve => setTimeout(resolve, delay));
        }}
      }}
    }}
    
    throw lastError!;
  }}
}}
"#,
            config.max_attempts
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_typescript_telemetry(&self, config: &TelemetryConfig) -> FeatureCode {
        let dependencies = vec![
            "prom-client".to_string(),
        ];
        
        let imports = vec![
            "import { Counter, Histogram, register } from 'prom-client';".to_string(),
        ];
        
        let code = r#"
class TelemetryHandler {
  private requestDuration: Histogram<string>;
  private requestCount: Counter<string>;
  
  constructor() {
    this.requestDuration = new Histogram({
      name: 'http_request_duration_seconds',
      help: 'Duration of HTTP requests in seconds',
      labelNames: ['method', 'path']
    });
    
    this.requestCount = new Counter({
      name: 'http_requests_total',
      help: 'Total number of HTTP requests',
      labelNames: ['method', 'path', 'status']
    });
    
    register.registerMetric(this.requestDuration);
    register.registerMetric(this.requestCount);
  }
  
  recordRequest(method: string, path: string, duration: number, status: number): void {
    this.requestDuration.labels(method, path).observe(duration);
    this.requestCount.labels(method, path, status.toString()).inc();
  }
}
"#.to_string();
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
    
    fn generate_typescript_caching(&self, config: &CachingConfig) -> FeatureCode {
        let dependencies = vec![
            "node-cache".to_string(),
        ];
        
        let imports = vec![
            "import NodeCache from 'node-cache';".to_string(),
            "import crypto from 'crypto';".to_string(),
        ];
        
        let code = format!(
            r#"
class CacheHandler {{
  private cache: NodeCache;
  
  constructor() {{
    this.cache = new NodeCache({{
      stdTTL: {},
      maxKeys: {},
      checkperiod: 600
    }});
  }}
  
  get<T>(key: string): T | undefined {{
    return this.cache.get<T>(key);
  }}
  
  set<T>(key: string, value: T, ttl?: number): boolean {{
    return this.cache.set(key, value, ttl);
  }}
  
  generateCacheKey(method: string, params: Record<string, any>): string {{
    const data = {{ method, params }};
    const hash = crypto.createHash('md5');
    hash.update(JSON.stringify(data));
    return hash.digest('hex');
  }}
  
  clear(): void {{
    this.cache.flushAll();
  }}
}}
"#,
            config.default_ttl_seconds, config.max_cache_size
        );
        
        FeatureCode {
            dependencies,
            imports,
            code,
            configuration: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCode {
    pub dependencies: Vec<String>,
    pub imports: Vec<String>,
    pub code: String,
    pub configuration: HashMap<String, String>,
}

impl FeatureCode {
    pub fn merge(features: Vec<FeatureCode>) -> FeatureCode {
        let mut dependencies = Vec::new();
        let mut imports = Vec::new();
        let mut code_parts = Vec::new();
        let mut configuration = HashMap::new();
        
        for feature in features {
            dependencies.extend(feature.dependencies);
            imports.extend(feature.imports);
            code_parts.push(feature.code);
            configuration.extend(feature.configuration);
        }
        
        // Remove duplicates
        dependencies.sort();
        dependencies.dedup();
        imports.sort();
        imports.dedup();
        
        FeatureCode {
            dependencies,
            imports,
            code: code_parts.join("\n\n"),
            configuration,
        }
    }
}