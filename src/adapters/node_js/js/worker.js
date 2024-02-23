/*
  TODO migrate this to a NAPI module

  Protocol is text separated by the \n character:
    message_ref
    action_type
    payload_as_json

  The message_ref is sent back to the sender to notify them that the
  request has completed.

  The action_type is used to pick the callback to run.

  The payload_as_json is the body of the request formatted as JSON.

  Notes:
    The protocol can be made to be more efficient to serialize/deserialize 
    but JSON is just easy to work with for a demo
  
    I am using a TCP socket per worker for communicating with the parent process.
    This is slightly slower than having multiple Node.js instances talking via
    stdin/stdout however I wanted to preserve the stdout capabilities of the
    plugins - also workers are more memory efficient than multiple Node.js
    instances.
*/
const { Socket } = require('net')

const plugins = {}

async function load_plugin({ plugin_key, specifier }) {
  const module = await import(specifier)
  plugins[plugin_key] = module.default
}

async function run_resolver({ plugin_key, dependency }) {
  /** @type {import('@alshdavid/mach').Resolver} */
  let resolver = plugins[plugin_key]
  const result = await resolver.init.resolve({ dependency })
  if (result === null || result === undefined) {
    return {}
  }
  return result
}

async function run_transformer({ plugin_key, config, file_path, kind, code }) {
  let resolver = plugins[plugin_key]
  
  let updated = false
  const dependencies = []
  const asset = new class MutableAsset {
    file_path = file_path;
    get kind() {
      return kind
    }

    set kind(value) {
      updated = true
      kind = value
    }

    async get_code() {
      return code
    }
    async set_code(/** @type {string} */ value) {
      updated = true
      code = value
    }
    add_dependency(options) {
      updated = true
      dependencies.push(options)
    }
  }()
  const result = await resolver.init.transform({ config, asset })
  if (!updated) {
    return {
      updated: false,
      dependencies: [],
      code: ''
    }
  }
  return {
    updated: true,
    dependencies,
    code,
  }
}

/** @type {Record<string, Function>} */
const actions = {
  load_plugin,
  run_resolver,
  run_transformer,
}

{
const client = new Socket();

let incoming_msg_ref = ''
let incoming_action = ''
let buffer = ''

// When we get a message from the host split on a newline
// character, parse the data and run the callback
client.on('data', async function(data_str) {
  for (const char of data_str.toString()) {
    if (char === '\n' && incoming_msg_ref === '') {
      incoming_msg_ref = buffer
      buffer = ''
      continue
    }
    if (char === '\n' && incoming_msg_ref !== '' && incoming_action === '') {
      incoming_action = buffer
      buffer = ''
      continue
    }
    if (char === '\n' && incoming_action !== '') {
      const data = JSON.parse(buffer)
      const msg_ref = incoming_msg_ref
      const action = incoming_action
      incoming_msg_ref = ''
      incoming_action = ''
      buffer = ''

      setTimeout(async () => {
        const result = await actions[action](data)
        const response = `${msg_ref}\n${JSON.stringify(result)}\n`
        client.write(response)
      }, 0)
      continue
    }
    buffer += char
  }
});

// Close the process if the parent terminates
client.on('end', () => process.exit())
client.on('close', () => process.exit());

// @ts-expect-error
client.connect(__MACH__PORT__, '127.0.0.1');
}

{
  class Resolver {
    init
    constructor(init) {
      this.init = init
    }
  }

  globalThis.Resolver = Resolver

  class Transformer {
    init
    constructor(init) {
      this.init = init
    }
  }

  globalThis.Transformer = Transformer
}
