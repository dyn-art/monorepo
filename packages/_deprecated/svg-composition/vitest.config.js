const { defineConfig, mergeConfig } = require('vitest/config');
const { nodeConfig } = require('@dyn/config/vite/node.config');

module.exports = mergeConfig(nodeConfig, defineConfig({}));
