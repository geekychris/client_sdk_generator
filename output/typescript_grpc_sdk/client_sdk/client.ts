import { SampleRequest, SampleResponse } from './types';


/**
 * Client configuration options
 */
export interface ClientConfig {
  baseURL: string;
  timeout?: number;
  headers?: Record<string, string>;
}


/**
 * UserService gRPC API TypeScript SDK Client
 * 
 * Generated from UserService gRPC proto files
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
   * Register a new user account
   */
  async registeruser(): Promise<> {
    const path = '/userservice.v1.UserService/RegisterUser';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Authenticate a user and create session
   */
  async loginuser(): Promise<> {
    const path = '/userservice.v1.UserService/LoginUser';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Refresh authentication token
   */
  async refreshtoken(): Promise<> {
    const path = '/userservice.v1.UserService/RefreshToken';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Logout user and invalidate session
   */
  async logoutuser(): Promise<> {
    const path = '/userservice.v1.UserService/LogoutUser';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Get user profile by ID
   */
  async getuser(): Promise<> {
    const path = '/userservice.v1.UserService/GetUser';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Get current authenticated user profile
   */
  async getcurrentuser(): Promise<> {
    const path = '/userservice.v1.UserService/GetCurrentUser';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Update user profile information
   */
  async updateuser(): Promise<> {
    const path = '/userservice.v1.UserService/UpdateUser';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Delete user account
   */
  async deleteuser(): Promise<> {
    const path = '/userservice.v1.UserService/DeleteUser';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * List users with pagination and filtering
   */
  async listusers(): Promise<> {
    const path = '/userservice.v1.UserService/ListUsers';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Change user password
   */
  async changepassword(): Promise<> {
    const path = '/userservice.v1.UserService/ChangePassword';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Reset user password
   */
  async resetpassword(): Promise<> {
    const path = '/userservice.v1.UserService/ResetPassword';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Send email verification
   */
  async sendverificationemail(): Promise<> {
    const path = '/userservice.v1.UserService/SendVerificationEmail';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Verify email address
   */
  async verifyemail(): Promise<> {
    const path = '/userservice.v1.UserService/VerifyEmail';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * List user active sessions
   */
  async listusersessions(): Promise<> {
    const path = '/userservice.v1.UserService/ListUserSessions';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Revoke a user session
   */
  async revokesession(): Promise<> {
    const path = '/userservice.v1.UserService/RevokeSession';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Get user preferences
   */
  async getuserpreferences(): Promise<> {
    const path = '/userservice.v1.UserService/GetUserPreferences';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

  /**
   * Update user preferences
   */
  async updateuserpreferences(): Promise<> {
    const path = '/userservice.v1.UserService/UpdateUserPreferences';

    const params = undefined;

    const body = undefined;

    return this.makeRequest<>('', path, params, body);
  }

}