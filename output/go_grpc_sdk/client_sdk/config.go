package client

import (
	"time"
)

// ClientConfig holds the configuration for the UserService gRPC API gRPC client
type ClientConfig struct {
	// BaseURL is the gRPC server address (e.g., "localhost:50051")
	BaseURL string

	// UseTLS determines whether to use TLS for the connection
	UseTLS bool

	// TLSServerName is the server name for TLS verification
	TLSServerName string

	// Timeout for gRPC requests
	Timeout time.Duration

	// UserAgent for gRPC requests
	UserAgent string

	// Retry settings
	MaxRetries int
	RetryDelay time.Duration

	// Telemetry settings
	TelemetryEnabled bool
	MetricsEndpoint  string

	// gRPC-specific settings
	MaxReceiveMessageSize int
	MaxSendMessageSize    int
	KeepAliveTime         time.Duration
	KeepAliveTimeout      time.Duration
	PermitWithoutStream   bool
}

// NewClientConfig creates a new ClientConfig with default values for gRPC
func NewClientConfig(baseURL string) *ClientConfig {
	return &ClientConfig{
		BaseURL:               baseURL,
		UseTLS:                false,
		Timeout:               30 * time.Second,
		UserAgent:             "client-grpc-client/1.0.0",
		MaxRetries:            3,
		RetryDelay:            time.Second,
		TelemetryEnabled:      true,
		MaxReceiveMessageSize: 4 * 1024 * 1024, // 4MB
		MaxSendMessageSize:    4 * 1024 * 1024, // 4MB
		KeepAliveTime:         30 * time.Second,
		KeepAliveTimeout:      5 * time.Second,
		PermitWithoutStream:   true,
	}
}

// WithTLS enables TLS for the gRPC connection
func (c *ClientConfig) WithTLS(serverName string) *ClientConfig {
	c.UseTLS = true
	c.TLSServerName = serverName
	return c
}

// WithTimeout sets the gRPC timeout
func (c *ClientConfig) WithTimeout(timeout time.Duration) *ClientConfig {
	c.Timeout = timeout
	return c
}

// WithUserAgent sets the user agent string
func (c *ClientConfig) WithUserAgent(userAgent string) *ClientConfig {
	c.UserAgent = userAgent
	return c
}

// WithMessageSize sets the maximum message sizes
func (c *ClientConfig) WithMessageSize(maxReceive, maxSend int) *ClientConfig {
	c.MaxReceiveMessageSize = maxReceive
	c.MaxSendMessageSize = maxSend
	return c
}

// WithKeepAlive configures keep-alive settings
func (c *ClientConfig) WithKeepAlive(time, timeout time.Duration, permitWithoutStream bool) *ClientConfig {
	c.KeepAliveTime = time
	c.KeepAliveTimeout = timeout
	c.PermitWithoutStream = permitWithoutStream
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
