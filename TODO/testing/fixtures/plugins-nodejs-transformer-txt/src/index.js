import content from './content.txt'

globalThis.content_key = undefined

if (typeof content !== 'undefined') {
  globalThis.content_key = content
}
