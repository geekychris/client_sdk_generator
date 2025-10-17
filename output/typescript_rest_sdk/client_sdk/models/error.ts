/**
 * 
 */
export interface Error {
  /**
   * Error code
   */
  code: String;
  /**
   * Error message
   */
  message: String;
  /**
   * Additional error details
   */
  details?: String;
}


/**
 * Type guard to check if an object is a valid Error
 */
export function isError(obj: any): obj is Error {
  if (!obj || typeof obj !== 'object') {
    return false;
  }
  
  if (!('code' in obj)) {
    return false;
  }
  if (!('message' in obj)) {
    return false;
  }
  
  return true;
}

/**
 * Creates a new Error with default values
 */
export function createError(code: String, message: String, ): Error {
  return {
    code,
    message,
  };
}

/**
 * Validates a Error object
 */
export function validateError(obj: Error): string[] {
  const errors: string[] = [];
  
  if (obj.code === undefined || obj.code === null) {
    errors.push('code is required');
  }
  if (obj.message === undefined || obj.message === null) {
    errors.push('message is required');
  }
  
  return errors;
}

export default Error;