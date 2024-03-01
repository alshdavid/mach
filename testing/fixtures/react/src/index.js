import * as React from 'react'
import * as ReactDOM from 'react-dom/client'

const rootElement = document.createElement('div')
document.body.appendChild(rootElement)

const root = ReactDOM.createRoot(rootElement)
// root.render(<div>Hello World!</div>)
root.render(React.createElement('div', {}, ['Hello World!']))
