Create a Client SDK generator.  This generator should be able to take as input:
openapi spec
graphql schema
grpc idl

The generator should be able to create a client sdk that for any of these in java, python or rust.  And should be extensible to add other languages.  The sdk should be able to also include some other best practices, these include the option to:

add resiliency such as retry logic using best open source solutions for each language.  They should be able to instrument calls with prometheus or other telemetry system and finaly it should be possible to enable client side caches on a per call basis.

The generated sdk should use typed objects such as POJO's for input and results.  Further should support regular calls and async calling patterns.

You can write this tool in whatever language you prefer.  The tools should include tests, detailed documentation on design, how to use and how to extend and include some examples of each input and target SDK programming language.
