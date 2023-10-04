const sveltePreprocess = require('svelte-preprocess');

const preprocessOptions = {
    sourceMap: true,
    scss: {
        prependData: `@import 'client/global.scss';`
    },
};

module.exports = {
    preprocess: sveltePreprocess(preprocessOptions),
    compilerOptions: {
      customElement: true,
      tag: null,
    },

    // Export this to allow rollup.config.js to inherit the same preprocess options.
    preprocessOptions,
}
