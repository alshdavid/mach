# Node.js Plugins

todo

Mach supports plugins that share the ideas and API of [Parcel Plugins](https://parceljs.org/features/plugins) for cases where bundling must be customized.

## Usage

Import the plugin API from the `@alshdavid/mach` npm package

```javascript
import { Transformer } from '@alshdavid/mach'
```

## Performance

It goes without saying that JavaScript plugins are not as fast plugins written in Rust. While JavaScript itself is fast, there is additional overhead associated with talking to Nodejs from an external process.

To mitigate this, Mach uses low level APIs provided by the operating system to share memory and communicate with a Nodejs child process/workers.

On an M1 MacBook Pro; In addition to the time taken for the plugin code to run, a project with 1000 files will see 100ms added to the build time for each JavaScript plugin added.

This can add up quickly, especially when considering the endless void that is node_modules. It's recommended that performance sensitive projects migrate JavaScript plugins to Rust.