import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';
import css from 'rollup-plugin-css-only';
import externalGlobals from 'rollup-plugin-external-globals';

const production = !process.env.ROLLUP_WATCH;

function serve() {
  let server;

  function toExit() {
    if (server) server.kill(0);
  }

  return {
    writeBundle() {
      if (server) return;
      server = require('child_process').spawn('npm', ['run', 'start', '--', '--dev'], {
        stdio: ['ignore', 'inherit', 'inherit'],
        shell: true
      });

      process.on('SIGTERM', toExit);
      process.on('exit', toExit);
    }
  };
}

export default [
  // Browser bundle.
  {
    input: 'client/main.ts',
    output: {
      sourcemap: true,
      format: 'iife',
      name: 'app',
      file: 'static/build/bundle.js',
      exports: 'named',
    },
    plugins: [
      svelte({
        exclude: /\.wc\.svelte$/,
        preprocess: sveltePreprocess({
          sourceMap: !production,
          scss: {
            prependData: `@import 'client/global.scss';`
          },
        }),
        compilerOptions: {
          // enable run-time checks when not in production
          dev: !production,
          hydratable: true,
          customElement: false,
        }
      }),
      svelte({
        include: /\.wc\.svelte$/,
        preprocess: sveltePreprocess({
          sourceMap: !production,
          scss: {
            prependData: `@import 'client/global.scss';`
          },
        }),
        compilerOptions: {
          // enable run-time checks when not in production
          dev: !production,
          customElement: true,
        }
      }),
      // we'll extract any component CSS out into
      // a separate file - better for performance
      css({ output: 'bundle.css' }),

      // If you have external dependencies installed from
      // npm, you'll most likely need these plugins. In
      // some cases you'll need additional configuration -
      // consult the documentation for details:
      // https://github.com/rollup/plugins/tree/master/packages/commonjs
      resolve({
        browser: true,
        dedupe: ['svelte']
      }),
      commonjs(),
      typescript({
        rootDir: './client',
        sourceMap: !production,
        inlineSources: !production
      }),

      // In dev mode, call `npm run start` once
      // the bundle has been generated
      !production && serve(),

      // Watch the `static` directory and refresh the
      // browser on changes when not in production
      !production && livereload('static'),

      // If we're building for production (npm run build
      // instead of npm run dev), minify
      production && terser()
    ],
    watch: {
      clearScreen: false
    }
  },
  // SSR Bundle.
  {
    input: 'client/App.svelte',
    output: {
      exports: 'default',
      sourcemap: false,
      format: 'cjs',
      name: 'app',
      file: 'static/build/ssr.js',
    },
    plugins: [
      svelte({
        preprocess: sveltePreprocess({
          scss: {
            prependData: `@import 'client/global.scss';`
          },
        }),
        compilerOptions: {
          generate: 'ssr',
          hydratable: true,
        }
      }),
      css({ output: 'bundle.css' }),
      resolve(),
      commonjs(),
      typescript({
        sourceMap: false,
      }),
      production && terser()
    ],
  },
  // SSR Entrypoint.
  {
    input: 'client/ssrEntry.js',
    output: {
      sourcemap: false,
      format: 'umd',
      name: 'ssrEntry',
      file: 'static/build/ssrEntry.js',
    },
    plugins: [
      externalGlobals({
        fetch: 'fetch',
      }),
      resolve(),
      commonjs(),
      production && terser()
    ],
  },
];
