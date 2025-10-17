import { Pet, NewPet, Category, Error } from './types';


/**
 * Client configuration options
 */
export interface ClientConfig {
  baseURL: string;
  timeout?: number;
  headers?: Record<string, string>;
}


/**
 * Pet Store API TypeScript SDK Client
 * 
 * A sample API that uses a pet store as an example
 */
export class  {
  private baseURL: string;
  private timeout: number;
  private defaultHeaders: Record<string, string>;

  constructor(config: ClientConfig) {
    this.baseURL = config.baseURL.replace(/\/+$/, '');
    this.timeout = config.timeout || 30000;
    this.defaultHeaders = {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      'User-Agent': 'client/1.0.0 (TypeScript)',
      ...config.headers,
    };
  }


  /**
   * Make an HTTP request with retry logic and authentication
   */
  private async makeRequest<T>(
    method: string,
    path: string,
    params?: Record<string, any>,
    body?: any
  ): Promise<T> {
    const url = new URL(`${this.baseURL}${path}`);
    
    // Add query parameters
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          url.searchParams.append(key, String(value));
        }
      });
    }

    const headers = { ...this.defaultHeaders };


    const requestOptions: RequestInit = {
      method,
      headers,
      ...(body && { body: JSON.stringify(body) }),
    };

        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), this.timeout);
        
        const response = await fetch(url.toString(), {
          ...requestOptions,
          signal: controller.signal,
        });
        
        clearTimeout(timeoutId);


        if (!response.ok) {
          const errorText = await response.text();
          throw new Error(`HTTP ${response.status}: ${errorText}`);
        }

        // Handle empty responses
        const contentType = response.headers.get('content-type');
        if (!contentType || !contentType.includes('application/json')) {
          return undefined as T;
        }

        return await response.json();

  }

  /**
   * Returns a list of all pets in the store
   * @param  Maximum number of pets to return
   * @param  Filter pets by tag
   */
  async listpets(?: string, ?: string): Promise<> {
    const path = '/pets';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Creates a new pet in the store
   */
  async createpet(): Promise<> {
    const path = '/pets';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Returns a single pet by its ID
   * @param  ID of the pet to retrieve
   */
  async getpet(: string): Promise<> {
    const path = '/pets/{petId}';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Updates an existing pet
   * @param  ID of the pet to update
   */
  async updatepet(: string): Promise<> {
    const path = '/pets/{petId}';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Deletes a pet from the store
   * @param  ID of the pet to delete
   */
  async deletepet(: string): Promise<> {
    const path = '/pets/{petId}';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

}