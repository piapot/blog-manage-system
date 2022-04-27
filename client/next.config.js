const path = require("path")
const WindiCSSWebpackPlugin = require("windicss-webpack-plugin")

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  webpack(config) {
    config.resolve.alias["@"] = path.resolve(__dirname)
    config.plugins.push(new WindiCSSWebpackPlugin())
    return config
  }
}

module.exports = nextConfig
