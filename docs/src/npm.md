# Npm

Jscore can work seaminglessly with packages from `node_modules/` with the help of [esbuild](https://esbuild.github.io/).
This feature not exclusive to `npm`. It works perfectly fine with `pnpm` or `yarn` because all of them use the same `node_modules/`
directory.

Directly trying to use a package similar to how you would in node wouldn't work in jscore.
To use a package in jscore, you **need to prefix** `esbuild:` to the name of the package.
This tells jscore to use esbuild to quickly convert the package into a standalone JavaScript
file which it can easily load.

**Example:**

Here is an example on how to use lodash.

```js 
import lodash from 'esbuild:lodash';

const arr = [1, 2, 3, 4, 5];
console.log(lodash.chunk(arr, 2));
console.log(lodash.sum(arr));
```

## Technical Details

The packages that are bundled by esbuild are stored in the `.cache/jscore` directory to avoid bundling every time
ewwii reloads jscore, which happens very frequently. Once in a while, its a good idea to clear the cache directory
and let jscore repopulate it.
