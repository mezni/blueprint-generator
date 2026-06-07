/**
 * Custom error classes for structured error handling
 * Enables better error tracking, logging, and recovery
 */

/**
 * Base custom error class
 */
export class CustomError extends Error {
  public readonly timestamp: Date;
  public readonly context?: Record<string, any>;

  constructor(message: string, context?: Record<string, any>) {
    super(message);
    this.name = this.constructor.name;
    this.timestamp = new Date();
    this.context = context;
    Object.setPrototypeOf(this, new.target.prototype);
  }

  toJSON() {
    return {
      name: this.name,
      message: this.message,
      timestamp: this.timestamp.toISOString(),
      context: this.context,
    };
  }
}

/**
 * Authentication error
 * Thrown when auth operations fail (login, logout, token refresh)
 */
export class AuthenticationError extends CustomError {
  public readonly statusCode?: number;

  constructor(message: string, context?: Record<string, any>, statusCode?: number) {
    super(message, context);
    this.statusCode = statusCode;
  }
}

/**
 * API error
 * Thrown when API calls fail
 */
export class ApiError extends CustomError {
  public readonly statusCode?: number;
  public readonly endpoint?: string;

  constructor(message: string, statusCode?: number, endpoint?: string, context?: Record<string, any>) {
    super(message, context);
    this.statusCode = statusCode;
    this.endpoint = endpoint;
  }
}

/**
 * Network error
 * Thrown when network connectivity issues occur
 */
export class NetworkError extends CustomError {
  public readonly isOffline: boolean;

  constructor(message: string, isOffline: boolean = true, context?: Record<string, any>) {
    super(message, context);
    this.isOffline = isOffline;
  }
}

/**
 * Validation error
 * Thrown when input validation fails
 */
export class ValidationError extends CustomError {
  public readonly field?: string;
  public readonly value?: any;

  constructor(message: string, field?: string, value?: any, context?: Record<string, any>) {
    super(message, context);
    this.field = field;
    this.value = value;
  }
}

/**
 * Storage error
 * Thrown when local storage operations fail
 */
export class StorageError extends CustomError {
  public readonly key?: string;

  constructor(message: string, key?: string, context?: Record<string, any>) {
    super(message, context);
    this.key = key;
  }
}

/**
 * Configuration error
 * Thrown when configuration is invalid or missing
 */
export class ConfigurationError extends CustomError {
  public readonly configKey?: string;

  constructor(message: string, configKey?: string, context?: Record<string, any>) {
    super(message, context);
    this.configKey = configKey;
  }
}

/**
 * Type guard to check if error is a custom error
 */
export function isCustomError(error: unknown): error is CustomError {
  return error instanceof CustomError;
}

/**
 * Type guard to check if error is an authentication error
 */
export function isAuthenticationError(error: unknown): error is AuthenticationError {
  return error instanceof AuthenticationError;
}

/**
 * Type guard to check if error is an API error
 */
export function isApiError(error: unknown): error is ApiError {
  return error instanceof ApiError;
}

/**
 * Type guard to check if error is a network error
 */
export function isNetworkError(error: unknown): error is NetworkError {
  return error instanceof NetworkError;
}

/**
 * Type guard to check if error is a validation error
 */
export function isValidationError(error: unknown): error is ValidationError {
  return error instanceof ValidationError;
}
