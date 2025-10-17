/**
 * 
 */
export interface Newpet {
  /**
   * Name of the pet
   */
  name: String;
  /**
   * Tag associated with the pet
   */
  tag?: String;
  /**
   * URLs of pet photos
   */
  photourls?: String;
}


/**
 * Type guard to check if an object is a valid Newpet
 */
export function isNewpet(obj: any): obj is Newpet {
  if (!obj || typeof obj !== 'object') {
    return false;
  }
  
  if (!('name' in obj)) {
    return false;
  }
  
  return true;
}

/**
 * Creates a new Newpet with default values
 */
export function createNewpet(name: String, ): Newpet {
  return {
    name,
  };
}

/**
 * Validates a Newpet object
 */
export function validateNewpet(obj: Newpet): string[] {
  const errors: string[] = [];
  
  if (obj.name === undefined || obj.name === null) {
    errors.push('name is required');
  }
  
  return errors;
}

export default Newpet;