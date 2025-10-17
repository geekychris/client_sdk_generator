/**
 * Sample gRPC request
 */
export interface Samplerequest {
  /**
   * Request message
   */
  message: string;
}


/**
 * gRPC message class for Samplerequest
 */
export class SamplerequestMessage {
  public message: string;

  constructor(data: Partial<Samplerequest> = {}) {
    this.message = data.message || '';
  }

  /**
   * Serialize to gRPC wire format
   */
  serialize(): Uint8Array {
    // Implementation would depend on the specific protobuf library
    // This is a placeholder for the actual serialization logic
    return new Uint8Array();
  }

  /**
   * Deserialize from gRPC wire format
   */
  static deserialize(data: Uint8Array): SamplerequestMessage {
    // Implementation would depend on the specific protobuf library
    // This is a placeholder for the actual deserialization logic
    return new SamplerequestMessage();
  }

  /**
   * Convert to plain object
   */
  toObject(): Samplerequest {
    return {
      message: this.message,
    };
  }

  /**
   * Convert to JSON string
   */
  toJSON(): string {
    return JSON.stringify(this.toObject());
  }

  /**
   * Create from plain object
   */
  static fromObject(obj: Samplerequest): SamplerequestMessage {
    return new SamplerequestMessage(obj);
  }

  /**
   * Create from JSON string
   */
  static fromJSON(json: string): SamplerequestMessage {
    return SamplerequestMessage.fromObject(JSON.parse(json));
  }

  /**
   * Validate the message
   */
  validate(): string[] {
    const errors: string[] = [];
    
    if (this.message === undefined || this.message === null) {
      errors.push('message is required');
    }
    
    return errors;
  }

  /**
   * Clone the message
   */
  clone(): SamplerequestMessage {
    return new SamplerequestMessage(this.toObject());
  }

  /**
   * Check equality with another message
   */
  equals(other: SamplerequestMessage): boolean {
    if (this.message !== other.message) {
      return false;
    }
    return true;
  }
}

/**
 * Type guard to check if an object is a valid Samplerequest
 */
export function isSamplerequest(obj: any): obj is Samplerequest {
  if (!obj || typeof obj !== 'object') {
    return false;
  }
  
  if (!('message' in obj)) {
    return false;
  }
  
  return true;
}

/**
 * Creates a new Samplerequest with default values
 */
export function createSamplerequest(message: string): Samplerequest {
  return {
    message,
  };
}

/**
 * Validates a Samplerequest object
 */
export function validateSamplerequest(obj: Samplerequest): string[] {
  const message = new SamplerequestMessage(obj);
  return message.validate();
}

/**
 * gRPC service method request/response helpers
 */
export namespace SamplerequestHelpers {
  /**
   * Create a gRPC request for Samplerequest
   */
  export function createRequest(data: Samplerequest): SamplerequestMessage {
    return new SamplerequestMessage(data);
  }

  /**
   * Process a gRPC response for Samplerequest
   */
  export function processResponse(response: SamplerequestMessage): Samplerequest {
    return response.toObject();
  }

  /**
   * Create a stream of requests
   */
  export function* createRequestStream(items: Samplerequest[]): Generator<SamplerequestMessage> {
    for (const item of items) {
      yield new SamplerequestMessage(item);
    }
  }

  /**
   * Process a stream of responses
   */
  export async function* processResponseStream(
    stream: AsyncIterable<SamplerequestMessage>
  ): AsyncGenerator<Samplerequest> {
    for await (const response of stream) {
      yield response.toObject();
    }
  }
}

export default Samplerequest;