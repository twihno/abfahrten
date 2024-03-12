// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },

  vite: {
    vue: {
      script: {
        propsDestructure: true,
      },
    },
  },

  typescript: {
    typeCheck: true,
    tsConfig: {
      compilerOptions: {
        paths: {
          "@": ["."],
          "@/*": ["./*"],
        },
      },
    },
  },

  css: ["@/styles/global.css"],
  modules: ["@nuxt/fonts", "@vueuse/nuxt", "nuxt-icon"],
});
