/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    serverActions: false,
  },
  webpack: config => {
    config.experiments.asyncWebAssembly = true
    return config
  }
}

module.exports = nextConfig
