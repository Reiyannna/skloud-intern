// Nuxt 4 config. Local demo only: the API base is the Axum server on 3001.
// Change this one value if the backend ever moves ports or hosts.
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  runtimeConfig: {
    public: {
      apiBase: 'http://localhost:3001',
    },
  },
})
