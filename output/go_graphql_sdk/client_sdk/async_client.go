package client

import (
	"context"
)

// GraphqlApiclientAsyncClient provides async GraphQL operations
type GraphqlApiclientAsyncClient struct {
	client *GraphqlApiclient
}

// NewGraphqlApiclientAsync creates a new async GraphQL client
func NewGraphqlApiclientAsync(config ClientConfig) *GraphqlApiclientAsyncClient {
	return &GraphqlApiclientAsyncClient{
		client: NewGraphqlApiclient(config),
	}
}

// QueryResult holds the result of a GraphQL query
type QueryResult struct {
	Data  interface{}
	Error error
}

// QueryAsync executes a GraphQL query asynchronously
func (c *GraphqlApiclientAsyncClient) QueryAsync(ctx context.Context, query string, variables map[string]interface{}) <-chan QueryResult {
	resultChan := make(chan QueryResult, 1)
	
	go func() {
		defer close(resultChan)
		
		result, err := c.client.Query(ctx, query, variables)
		resultChan <- QueryResult{
			Data:  result,
			Error: err,
		}
	}()
	
	return resultChan
}

// MutationResult holds the result of a GraphQL mutation
type MutationResult struct {
	Data  interface{}
	Error error
}

// MutationAsync executes a GraphQL mutation asynchronously
func (c *GraphqlApiclientAsyncClient) MutationAsync(ctx context.Context, mutation string, variables map[string]interface{}) <-chan MutationResult {
	resultChan := make(chan MutationResult, 1)
	
	go func() {
		defer close(resultChan)
		
		result, err := c.client.Mutation(ctx, mutation, variables)
		resultChan <- MutationResult{
			Data:  result,
			Error: err,
		}
	}()
	
	return resultChan
}