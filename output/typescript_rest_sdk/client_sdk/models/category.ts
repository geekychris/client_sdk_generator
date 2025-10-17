/**
 * 
 */
export interface Category {
  /**
   * Category identifier
   */
  id?: String;
  /**
   * Category name
   */
  name?: String;
}


/**
 * Type guard to check if an object is a valid Category
 */
export function isCategory(obj: any): obj is Category {
  if (!obj || typeof obj !== 'object') {
    return false;
  }
  
  
  return true;
}

/**
 * Creates a new Category with default values
 */
export function createCategory(): Category {
  return {
  };
}

/**
 * Validates a Category object
 */
export function validateCategory(obj: Category): string[] {
  const errors: string[] = [];
  
  
  return errors;
}

export default Category;