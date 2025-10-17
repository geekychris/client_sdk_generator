package client

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"time"
)

// GraphQL API Go SDK Client
//
// Generated from GraphQL schema
type GraphqlApiclient struct {
	httpClient *http.Client
	baseURL    string
}


// ClientConfig holds client configuration
type ClientConfig struct {
	BaseURL    string
	HTTPClient *http.Client
	Timeout    time.Duration
}

// NewGraphqlApiclient creates a new instance of GraphqlApiclient
func NewGraphqlApiclient(config ClientConfig) *GraphqlApiclient {
	if config.HTTPClient == nil {
		config.HTTPClient = &http.Client{
			Timeout: config.Timeout,
		}
		if config.Timeout == 0 {
			config.HTTPClient.Timeout = 30 * time.Second
		}
	}

	client := &GraphqlApiclient{
		httpClient: config.HTTPClient,
		baseURL:    strings.TrimRight(config.BaseURL, "/"),
	}



	return client
}


// makeRequest performs an HTTP request with authentication and retry logic
func (c *GraphqlApiclient) makeRequest(ctx context.Context, method, path string, body interface{}, params map[string]string) (*http.Response, error) {
	url := fmt.Sprintf("%s%s", c.baseURL, path)
	
	// Add query parameters
	if len(params) > 0 {
		u, err := url.Parse(url)
		if err != nil {
			return nil, fmt.Errorf("invalid URL: %w", err)
		}
		
		q := u.Query()
		for key, value := range params {
			q.Add(key, value)
		}
		u.RawQuery = q.Encode()
		url = u.String()
	}

	var reqBody io.Reader
	if body != nil {
		jsonData, err := json.Marshal(body)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal request body: %w", err)
		}
		reqBody = bytes.NewBuffer(jsonData)
	}


		req, err := http.NewRequestWithContext(ctx, method, url, reqBody)
		if err != nil {
			return nil, fmt.Errorf("failed to create request: %w", err)
		}

		// Set headers
		req.Header.Set("Content-Type", "application/json")
		req.Header.Set("Accept", "application/json")
		req.Header.Set("User-Agent", "client/1.0.0 (Go)")


		resp, err := c.httpClient.Do(req)
		if err != nil {
			return nil, fmt.Errorf("request failed: %w", err)
		}


		return resp, nil

}

// Sample Sample GraphQL query
func (c *GraphQL APIClient) Sample(ctx context.Context, query string) (, error) {
	path := "/graphql"

	var params map[string]string

	resp, err := c.makeRequest(ctx, "Post", path, nil, params)
	if err != nil {
		var zero 
		return zero, err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		var zero 
		return zero, fmt.Errorf("HTTP %d: %s", resp.StatusCode, string(body))
	}

	var result 
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		var zero 
		return zero, fmt.Errorf("failed to decode response: %w", err)
	}
	return result, nil
	
	return nil
}

