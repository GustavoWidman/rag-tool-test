# rag-tool-test

A slightly more complex example of using the `rig` library to create a chatbot with multiple tools, dynamic context and a tool for lookup on the vector store. Uses the [Gemini API](https://ai.google.dev/gemini-api/docs/models/gemini) for completions and embeddings.

## Setup

```bash
# Set your Gemini API key
$ export GEMINI_API_KEY=<your-gemini-api-key>

# Run the project
$ cargo run
```

## Usage

The project will run a chatbot that can perform basic math operations, look up words in the vector store, and dynamically add context to the chatbot based on the user's input. It's currently using the `gemini-2.0-flash` model for completions and the `text-embedding-004` model for embeddings, both of which are available in the [Gemini API](https://ai.google.dev/gemini-api/docs/models/gemini).

There are currently four "tests" that the chatbot performs by default:

1. Ask the chatbot to calculate a simple subtraction operation (`5 - 2 = ?`).

2. Ask the chatbot to calculate a multi-step operation involving multiple tools (`(2 + 3) / 10  = ?`).

3. Ask the chatbot to look up a word in the vector store (`What does "glarb-glarb" mean?`).

4. Ask the chatbot to dynamically comprehend and compute a complex operation based on the user's input (`Somebody gave me two flurbos yesterday, and i already had 12 before that, but then, I had to give 10% of it to the government this afternoon, how many flurbos do i have left? And how many USD would I have if I converted what I have right now?`). This last test is a bit more complex and requires the chatbot to use most of the tools at it's disposal.

## Tools

The chatbot uses a set of tools to perform various operations. These tools are defined in the `tools` module and are implemented using the `Tool` trait.

### Add

This tool adds two numbers together, pretty simple, see [tools/add.rs](src/tools/add.rs).

### Subtract

This tool subtracts two numbers, pretty simple as well, see [tools/subtract.rs](src/tools/subtract.rs).

### Multiply

This tool multiplies two numbers, see [tools/multiply.rs](src/tools/multiply.rs).

### Divide

This tool divides two numbers, see [tools/divide.rs](src/tools/divide.rs).

### Lookup

This tool looks up the highest scoring document in the vector store given a query, see [tools/lookup.rs](src/tools/lookup.rs).

## Dynamic Context

Aside from being able to access the vector store by using the `lookup` tool, the chatbot will also have context added to it dynamically based on the user's input, this is done using the `dynamic_context` method on the `AgentBuilder`, which effectively works by taking the latest user prompt and using that to query the vector store instead of the static context. As of now, only one document is added to the dynamic context at a time, but this can be easily changed by modifying the arguments passed to the `dynamic_context` method.

## License

This project is licensed under the MIT License.
