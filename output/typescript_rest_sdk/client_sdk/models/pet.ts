/**
 * 
 */
export interface Pet {
  /**
   * Unique identifier for the pet
   */
  id: String;
  /**
   * Name of the pet
   */
  name: String;
  /**
   * Tag associated with the pet
   */
  tag?: String;
  /**
   * Pet status in the store
   */
  status?: String;
  /**
   * URLs of pet photos
   */
  photourls?: String;
}


/**
 * Type guard to check if an object is a valid Pet
 */
export function isPet(obj: any): obj is Pet {
  if (!obj || typeof obj !== 'object') {
    return false;
  }
  
  if (!('id' in obj)) {
    return false;
  }
  if (!('name' in obj)) {
    return false;
  }
  
  return true;
}

/**
 * Creates a new Pet with default values
 */
export function createPet(id: String, name: String, ): Pet {
  return {
    id,
    name,
  };
}

/**
 * Validates a Pet object
 */
export function validatePet(obj: Pet): string[] {
  const errors: string[] = [];
  
  if (obj.id === undefined || obj.id === null) {
    errors.push('id is required');
  }
  if (obj.name === undefined || obj.name === null) {
    errors.push('name is required');
  }
  
  return errors;
}

export default Pet;