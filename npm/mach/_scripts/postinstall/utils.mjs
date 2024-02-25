import * as http from "node:https";

function toArrayBuffer(buffer) {
  const arrayBuffer = new ArrayBuffer(buffer.length);
  const view = new Uint8Array(arrayBuffer);
  for (let i = 0; i < buffer.length; ++i) {
    view[i] = buffer[i];
  }
  return arrayBuffer;
}

/** @returns {Promise<ArrayBuffer>} */
export async function http_get(url) {
  function download_file_legacy(target_url) {
    let buffer = undefined
    
    function _download_file(req_url_str) {
      const req_url = new URL(req_url_str)
      const req_options = { host: req_url.host, path: req_url.path, method: 'GET', headers: {'User-Agent': 'node-fetch'} }

      return new Promise(res => {
        const request = http.get(req_options, async (response) => {
          const redirect = response.headers.location;
  
          if (redirect) {
            res(await _download_file(redirect))
          } else {
            response.on('end', () => res(toArrayBuffer(buffer)))
            response.on('data', (chunk) => {
              if (!buffer) {
                buffer = chunk;
              } else {
                buffer = Buffer.concat([buffer, chunk]);
              }
            })
          }
        });
        request.end()
      })
    }
      
    return _download_file(target_url)
  }

  return globalThis.fetch 
    ? await fetch(url).then(res => res.arrayBuffer()) 
    : await download_file_legacy(url)
}

export async function http_get_json(url) {
  function download_file_legacy(target_url) {
    let buffer = undefined
    
    function _download_file(req_url_str) {
      const req_url = new URL(req_url_str)
      const req_options = { host: req_url.host, path: req_url.path, method: 'GET', headers: {'User-Agent': 'node-fetch'} }

      return new Promise(res => {
        const request = http.get(req_options, async (response) => {
          const redirect = response.headers.location;
  
          if (redirect) {
            res(await _download_file(redirect))
          } else {
            response.on('end', () => res(toArrayBuffer(buffer)))
            response.on('data', (chunk) => {
              if (!buffer) {
                buffer = chunk;
              } else {
                buffer = Buffer.concat([buffer, chunk]);
              }
            })
          }
        });
        request.end()
      })
    }
      
    return _download_file(target_url)
  }

  const enc = new TextDecoder("utf-8");
  const buff = globalThis.fetch 
    ? await fetch(url).then(res => res.arrayBuffer()) 
    : await download_file_legacy(url)
  const str = enc.decode(buff)
  return JSON.parse(str)
}