package client

import (
	"context"
	"crypto/tls"
	"fmt"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/keepalive"
	"google.golang.org/grpc/metadata"
)

// UserserviceGrpcApi gRPC Client
//
// Generated from UserService gRPC proto files
type UserserviceGrpcApi struct {
	conn   *grpc.ClientConn
	config ClientConfig
}

// ClientConfig holds gRPC client configuration
type ClientConfig struct {
	ServerAddress     string
	UseTLS            bool
	TLSServerName     string
	Timeout           time.Duration
	MaxMessageSize    int
	KeepAliveInterval time.Duration
	KeepAliveTimeout  time.Duration
}

// NewUserserviceGrpcApi creates a new instance of UserserviceGrpcApi
func NewUserserviceGrpcApi(config ClientConfig) (*UserserviceGrpcApi, error) {
	var opts []grpc.DialOption

	// Configure TLS/credentials
	if config.UseTLS {
		creds := credentials.NewTLS(&tls.Config{
			ServerName: config.TLSServerName,
		})
		opts = append(opts, grpc.WithTransportCredentials(creds))
	} else {
		opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))
	}

	// Configure keep-alive
	if config.KeepAliveInterval > 0 {
		opts = append(opts, grpc.WithKeepaliveParams(keepalive.ClientParameters{
			Time:                config.KeepAliveInterval,
			Timeout:             config.KeepAliveTimeout,
			PermitWithoutStream: true,
		}))
	}

	// Configure message size
	if config.MaxMessageSize > 0 {
		opts = append(opts, grpc.WithDefaultCallOptions(
			grpc.MaxCallRecvMsgSize(config.MaxMessageSize),
			grpc.MaxCallSendMsgSize(config.MaxMessageSize),
		))
	}

	// Set default timeout if not provided
	if config.Timeout == 0 {
		config.Timeout = 30 * time.Second
	}

	// Establish connection
	conn, err := grpc.Dial(config.ServerAddress, opts...)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to gRPC server: %w", err)
	}

	client := &UserserviceGrpcApi{
		conn:   conn,
		config: config,
	}

	return client, nil
}

// Close closes the gRPC connection
func (c *UserserviceGrpcApi) Close() error {
	if c.conn != nil {
		return c.conn.Close()
	}
	return nil
}

// createContext creates a context with authentication metadata
func (c *UserserviceGrpcApi) createContext(ctx context.Context) context.Context {

	// Set timeout
	ctx, _ = context.WithTimeout(ctx, c.config.Timeout)

	return ctx
}

// GetConnection returns the underlying gRPC connection
func (c *UserserviceGrpcApi) GetConnection() *grpc.ClientConn {
	return c.conn
}
