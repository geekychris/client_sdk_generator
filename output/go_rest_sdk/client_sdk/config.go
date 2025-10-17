package client

import (
	"time"
)

// ClientConfig holds the configuration for the Pet Store API client
type ClientConfig struct {
	// BaseURL is the base URL for the API
	BaseURL string
	
	// Timeout for HTTP requests
	Timeout time.Duration
	
	// UserAgent for HTTP requests
	UserAgent string
	
	
	// Retry settings
	MaxRetries int
	RetryDelay time.Duration
	
	
	// Telemetry settings
	TelemetryEnabled bool
	MetricsEndpoint  string
	
	// Custom headers to include with every request
	DefaultHeaders map[string]string
}

// NewClientConfig creates a new ClientConfig with default values
func NewClientConfig(baseURL string) *ClientConfig {
	return &ClientConfig{
		BaseURL:   baseURL,
		Timeout:   30 * time.Second,
		UserAgent: "client-client/1.0.0",
		MaxRetries: 3,
		RetryDelay: time.Second,
		TelemetryEnabled: true,
		DefaultHeaders: make(map[string]string),
	}
}


// WithTimeout sets the HTTP timeout
func (c *ClientConfig) WithTimeout(timeout time.Duration) *ClientConfig {
	c.Timeout = timeout
	return c
}

// WithUserAgent sets the user agent string
func (c *ClientConfig) WithUserAgent(userAgent string) *ClientConfig {
	c.UserAgent = userAgent
	return c
}

// WithHeader adds a default header
func (c *ClientConfig) WithHeader(key, value string) *ClientConfig {
	if c.DefaultHeaders == nil {
		c.DefaultHeaders = make(map[string]string)
	}
	c.DefaultHeaders[key] = value
	return c
}

// WithRetry configures retry settings
func (c *ClientConfig) WithRetry(maxRetries int, delay time.Duration) *ClientConfig {
	c.MaxRetries = maxRetries
	c.RetryDelay = delay
	return c
}


// WithTelemetry configures telemetry settings
func (c *ClientConfig) WithTelemetry(enabled bool, endpoint string) *ClientConfig {
	c.TelemetryEnabled = enabled
	c.MetricsEndpoint = endpoint
	return c
}
