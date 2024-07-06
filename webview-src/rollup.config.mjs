import { nodeResolve } from '@rollup/plugin-node-resolve'
import typescript from '@rollup/plugin-typescript'
import terser from '@rollup/plugin-terser'

export default {
  input: './webview-src/index.mts',
  output: {
    dir: './webview-dist',
    entryFileNames: '[name].js',
    exports: 'auto',
  },
  plugins: [
    nodeResolve(),
    terser(),
    typescript({
      tsconfig: './webview-src/tsconfig.json',
      moduleResolution: 'NodeNext',
    }),
  ],
}
