package client

import (
	"context"
)

// UserserviceGrpcApiAsyncClient provides async gRPC operations
type UserserviceGrpcApiAsyncClient struct {
	client *UserserviceGrpcApi
}

// NewUserserviceGrpcApiAsync creates a new async gRPC client
func NewUserserviceGrpcApiAsync(config ClientConfig) (*UserserviceGrpcApiAsyncClient, error) {
	client, err := NewUserserviceGrpcApi(config)
	if err != nil {
		return nil, err
	}
	return &UserserviceGrpcApiAsyncClient{
		client: client,
	}, nil
}
