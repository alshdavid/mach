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
import { Socket } from 'net'
import { Transformer } from './public/transformer'
import { Resolver } from './public/resolver'
import { RequestLoadPlugin, load_plugin } from './actions/load_plugin'
import { RequestRunTransformer, run_transformer } from './actions/run_transformer'
import { RequestRunResolver, run_resolver } from './actions/run_resolver'

type ActionType = (
  ['load_plugin', RequestLoadPlugin] | 
  ['run_transformer', RequestRunTransformer] | 
  ['run_resolver', RequestRunResolver]
)

const transformers = new Map<string, Transformer>()
const resolvers = new Map<string, Resolver>()

export const connect = (client: Socket) => {
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
        const msg: ActionType = [incoming_action as any, JSON.parse(buffer)]
        const msg_ref = incoming_msg_ref
        incoming_msg_ref = ''
        incoming_action = ''
        buffer = ''

        setTimeout(async () => {
          let result: any
          if (msg[0] === 'load_plugin') {
            result = await load_plugin(
              transformers,
              resolvers,
              msg[1],
            );
          }
          else if (msg[0] === 'run_transformer') {
            const transformer = transformers.get(msg[1].plugin_key)
            if (!transformer) throw new Error('Cannot find transformer')
            result = await run_transformer(
              transformer,
              msg[1],
            );
          } 
          else if (msg[0] === 'run_resolver') {
            const resolver = resolvers.get(msg[1].plugin_key)
            if (!resolver) throw new Error('Cannot find transformer')
            result = await run_resolver(
              resolver,
              msg[1],
            ); 
          } 
          else {
            throw new Error('Invalid action')
          }

          const response = `${msg_ref}\n${JSON.stringify(result)}\n`
          client.write(response)
        }, 0)
        continue
      }
      buffer += char
    }
  });
}
