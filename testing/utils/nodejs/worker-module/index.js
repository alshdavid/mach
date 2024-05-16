import { parentPort } from 'worker_threads';

parentPort.addEventListener('message', async (event) => {
  const { id, action, data } = event.data
  if (action === 'eval') {
    try {
      const result = await eval(data)
      parentPort.postMessage({ id, data: result })
    } catch (error) {
      parentPort.postMessage({ id, error })
    }
  }
})
