import { Socket } from 'net'
import { connect } from './engine'

process.stdin.on('data', (data) => {
  return
  const client = new Socket();
  
  connect(client)
  
  // Close the process if the parent terminates
  client.on('end', () => process.exit())
  client.on('close', () => process.exit());

  // client.connect(__MACH__PORT__, '127.0.0.1');
})

process.stdin.on('close', () => process.exit())
process.stdin.on('end', () => process.exit())
