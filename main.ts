import { Plugin } from 'obsidian';
import initWasm, { onload } from './pkg/rust_plugin.js';
import wasmData from './pkg/rust_plugin_bg.wasm?url';

// biome-ignore lint/style/noDefaultExport: <explanation>
export default class SamplePlugin extends Plugin {
    async onload() {
        await this.loadSettings();

        await initWasm(wasmData);

        await this.loadSettings();

        onload(this);
    }

    async loadSettings() {}

    async saveSettings() {}
}
