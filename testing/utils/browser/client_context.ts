import * as http from 'node:http'
import * as puppeteer from 'puppeteer-core'
import * as path from 'node:path'
import * as fs from 'node:fs'
import * as fsAsync from 'node:fs/promises'

export type ClientContextOptions = {
  serve_path: string
}

export class ClientContext {
  #server

  constructor(server: http.Server) {
    this.#server = server
  }

  static async new(options: ClientContextOptions): Promise<ClientContext> {
    const server = http.createServer(serve_static(options.serve_path))
    await new Promise<void>((res) => server.listen(0, '0.0.0.0', () => res()))
    return new ClientContext(server)
  }

  address() {
    // @ts-expect-error
    return `http://localhost:${this.#server.address().port}`
  }

  async close() {
    await new Promise<void>((res) => this.#server.close(() => res()))
  }
}

function serve_static(client_path: string) {
  return async function (req: http.IncomingMessage, res: http.ServerResponse) {
    let resource_url = path.join(client_path, ...req.url!.split('/'))
    if (req.url === '/') {
      resource_url = path.join(resource_url, 'index.html')
    }

    if (
      !fs.existsSync(resource_url) ||
      fs.statSync(resource_url).isDirectory()
    ) {
      res.statusCode = 404
      res.end('Not Found')
      return
    }

    res.statusCode = 200
    const mime = mime_types[path.extname(resource_url)] || 'text/plain'
    res.setHeader('Content-Type', mime)
    res.setHeader('Cache-Control', 'max-age=0, private, must-revalidate')
    res.write(await fsAsync.readFile(resource_url))
    res.end()
  }
}

const mime_types: Record<string, string> = {
  ['.aac']: 'audio/aac',
  ['.abw']: 'application/x-abiword',
  ['.apng']: 'image/apng',
  ['.arc']: 'application/x-freearc',
  ['.avif']: 'image/avif',
  ['.avi']: 'video/x-msvideo',
  ['.azw']: 'application/vnd.amazon.ebook',
  ['.bin']: 'application/octet-stream',
  ['.bmp']: 'image/bmp',
  ['.bz']: 'application/x-bzip',
  ['.bz2']: 'application/x-bzip2',
  ['.cda']: 'application/x-cdf',
  ['.csh']: 'application/x-csh',
  ['.css']: 'text/css',
  ['.csv']: 'text/csv',
  ['.doc']: 'application/msword',
  ['.docx']:
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  ['.eot']: 'application/vnd.ms-fontobject',
  ['.epub']: 'application/epub+zip',
  ['.gz']: 'application/gzip',
  ['.gif']: 'image/gif',
  ['.htm']: 'text/html',
  ['.html']: 'text/html',
  ['.ico']: 'image/vnd.microsoft.icon',
  ['.ics']: 'text/calendar',
  ['.jar']: 'application/java-archive',
  ['.jpeg .jpg']: 'image/jpeg',
  ['.js']: 'text/javascript',
  ['.json']: 'application/json',
  ['.jsonld']: 'application/ld+json',
  ['.mid']: 'audio/x-midi',
  ['.midi']: 'audio/x-midi',
  ['.mjs']: 'text/javascript',
  ['.mp3']: 'audio/mpeg',
  ['.mp4']: 'video/mp4',
  ['.mpeg']: 'video/mpeg',
  ['.mpkg']: 'application/vnd.apple.installer+xml',
  ['.odp']: 'application/vnd.oasis.opendocument.presentation',
  ['.ods']: 'application/vnd.oasis.opendocument.spreadsheet',
  ['.odt']: 'application/vnd.oasis.opendocument.text',
  ['.oga']: 'audio/ogg',
  ['.ogv']: 'video/ogg',
  ['.ogx']: 'application/ogg',
  ['.opus']: 'audio/opus',
  ['.otf']: 'font/otf',
  ['.png']: 'image/png',
  ['.pdf']: 'application/pdf',
  ['.php']: 'application/x-httpd-php',
  ['.ppt']: 'application/vnd.ms-powerpoint',
  ['.pptx']:
    'application/vnd.openxmlformats-officedocument.presentationml.presentation',
  ['.rar']: 'application/vnd.rar',
  ['.rtf']: 'application/rtf',
  ['.sh']: 'application/x-sh',
  ['.svg']: 'image/svg+xml',
  ['.tar']: 'application/x-tar',
  ['.tif']: 'image/tiff',
  ['.tiff']: 'image/tiff',
  ['.ts']: 'video/mp2t',
  ['.ttf']: 'font/ttf',
  ['.txt']: 'text/plain',
  ['.vsd']: 'application/vnd.visio',
  ['.wav']: 'audio/wav',
  ['.weba']: 'audio/webm',
  ['.webm']: 'video/webm',
  ['.webp']: 'image/webp',
  ['.woff']: 'font/woff',
  ['.woff2']: 'font/woff2',
  ['.xhtml']: 'application/xhtml+xml',
  ['.xls']: 'application/vnd.ms-excel',
  ['.xlsx']:
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  ['.xml']: 'application/xml',
  ['.xul']: 'application/vnd.mozilla.xul+xml',
  ['.zip']: 'application/zip',
  ['.3gp']: 'video/3gpp',
  ['.3g2']: 'video/3gpp2',
  ['.7z']: 'application/x-7z-compressed',
}
