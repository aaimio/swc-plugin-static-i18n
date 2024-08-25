# swc-plugin-static-i18n

A [Speedy Web Compiler (SWC)](https://swc.rs/) plugin that replaces (translatable) strings at build time.


## Getting started

> ⚠️ This is still in development and below instructions do not work.

1. Install the plugin:

```sh
pnpm i swc-plugin-static-i18n
```

2. Define your translatable strings:


```js
export const en_GB = {
  "Hello World": "Hello World",
  "Goodbye {{ name }}": "Goodbye {{ name }}",
};

/** @type {{ [key in keyof en_GB]: string }} */
export const nl_NL = {
  "Hello World": "Hallo Wereld",
  "Goodbye {{ name }}": "Tot ziens {{ name }}",
};
```

2. Add configuration to `.swcrc.mjs` or `next.config.js`:

```js
import { nl_NL, en_GB } from "./strings";

export default {
  jsc: {
    plugins: [
      [
        "swc-plugin-static-i18n",
        {
          function_name: "translate",
          strings: (() => {
            switch (process.env.LOCALE) {
              case "nl_NL": {
                return nl_NL;
              }

              default: {
                return en_GB;
              }
            }
          })(),
        },
      ],
    ],
  },
};
```

In this example, the plugin will replace all `translate` calls with the
replacement strings in the specified `strings` parameter, i.e.:

```diff
const SomeComponent = () => {
  return (
    <div className='p-2'>
-      <h1>{translate("Hello World")}</h1>
+      <h1>{translate("Hallo Wereld")}</h1>
    </div>
  )
}
```

## Wishlist

- Add a config option to inline strings if the `translate` function doesn't do anything (other than acting as a point to inject strings)
  - For example:

  ```diff
  const SomeComponent = () => {
    return (
      <div className='p-2'>
  -      <h1>{translate("Hello World")}</h1>
  +      <h1>{"Hallo Wereld"}</h1>
      </div>
    )
  }
  ```

- Expose a `translate` function and `<Trans>` component (similar to `react-i18next`) for [interpolation](https://react.i18next.com/latest/trans-component#interpolation)
