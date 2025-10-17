/**
 * Sample gRPC response
 */
export interface Sampleresponse {
  /**
   * Response result
   */
  result: string;
}


/**
 * gRPC message class for Sampleresponse
 */
export class SampleresponseMessage {
  public result: string;

  constructor(data: Partial<Sampleresponse> = {}) {
    this.result = data.result || '';
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
  static deserialize(data: Uint8Array): SampleresponseMessage {
    // Implementation would depend on the specific protobuf library
    // This is a placeholder for the actual deserialization logic
    return new SampleresponseMessage();
  }

  /**
   * Convert to plain object
   */
  toObject(): Sampleresponse {
    return {
      result: this.result,
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
  static fromObject(obj: Sampleresponse): SampleresponseMessage {
    return new SampleresponseMessage(obj);
  }

  /**
   * Create from JSON string
   */
  static fromJSON(json: string): SampleresponseMessage {
    return SampleresponseMessage.fromObject(JSON.parse(json));
  }

  /**
   * Validate the message
   */
  validate(): string[] {
    const errors: string[] = [];
    
    if (this.result === undefined || this.result === null) {
      errors.push('result is required');
    }
    
    return errors;
  }

  /**
   * Clone the message
   */
  clone(): SampleresponseMessage {
    return new SampleresponseMessage(this.toObject());
  }

  /**
   * Check equality with another message
   */
  equals(other: SampleresponseMessage): boolean {
    if (this.result !== other.result) {
      return false;
    }
    return true;
  }
}

/**
 * Type guard to check if an object is a valid Sampleresponse
 */
export function isSampleresponse(obj: any): obj is Sampleresponse {
  if (!obj || typeof obj !== 'object') {
    return false;
  }
  
  if (!('result' in obj)) {
    return false;
  }
  
  return true;
}

/**
 * Creates a new Sampleresponse with default values
 */
export function createSampleresponse(result: string): Sampleresponse {
  return {
    result,
  };
}

/**
 * Validates a Sampleresponse object
 */
export function validateSampleresponse(obj: Sampleresponse): string[] {
  const message = new SampleresponseMessage(obj);
  return message.validate();
}

/**
 * gRPC service method request/response helpers
 */
export namespace SampleresponseHelpers {
  /**
   * Create a gRPC request for Sampleresponse
   */
  export function createRequest(data: Sampleresponse): SampleresponseMessage {
    return new SampleresponseMessage(data);
  }

  /**
   * Process a gRPC response for Sampleresponse
   */
  export function processResponse(response: SampleresponseMessage): Sampleresponse {
    return response.toObject();
  }

  /**
   * Create a stream of requests
   */
  export function* createRequestStream(items: Sampleresponse[]): Generator<SampleresponseMessage> {
    for (const item of items) {
      yield new SampleresponseMessage(item);
    }
  }

  /**
   * Process a stream of responses
   */
  export async function* processResponseStream(
    stream: AsyncIterable<SampleresponseMessage>
  ): AsyncGenerator<Sampleresponse> {
    for await (const response of stream) {
      yield response.toObject();
    }
  }
}

export default Sampleresponse;