import { Pet, NewPet, Category, Error } from './types';
import { , ClientConfig } from './client';

/**
 * Pet Store API TypeScript SDK Async Client
 * 
 * A sample API that uses a pet store as an example
 * 
 * Provides Promise-based async operations (TypeScript/JavaScript is naturally async)
 */
export class Async {
  private client: ;

  constructor(config: ClientConfig) {
    this.client = new (config);
  }


  /**
   * Returns a list of all pets in the store (Async)
   * @param  Maximum number of pets to return
   * @param  Filter pets by tag
   */
  async listpetsAsync(?: string, ?: string): Promise<> {
    // In TypeScript/JavaScript, all operations are naturally async
    return this.client.listpets(, );
  }

  /**
   * Creates a new pet in the store (Async)
   */
  async createpetAsync(): Promise<> {
    // In TypeScript/JavaScript, all operations are naturally async
    return this.client.createpet();
  }

  /**
   * Returns a single pet by its ID (Async)
   * @param  ID of the pet to retrieve
   */
  async getpetAsync(: string): Promise<> {
    // In TypeScript/JavaScript, all operations are naturally async
    return this.client.getpet();
  }

  /**
   * Updates an existing pet (Async)
   * @param  ID of the pet to update
   */
  async updatepetAsync(: string): Promise<> {
    // In TypeScript/JavaScript, all operations are naturally async
    return this.client.updatepet();
  }

  /**
   * Deletes a pet from the store (Async)
   * @param  ID of the pet to delete
   */
  async deletepetAsync(: string): Promise<> {
    // In TypeScript/JavaScript, all operations are naturally async
    return this.client.deletepet();
  }

}