import builtins from "builtin-modules";
import { type PluginOption, defineConfig } from "vite";

const setOutDir = (mode: string) => {
	switch (mode) {
		case "development":
			return "./test_vault/.obsidian/plugins/sample-plugin";
		case "production":
			return "./build";
	}
};

// biome-ignore lint/style/noDefaultExport: <explanation>
export default defineConfig(({ mode }) => {
	return {
		plugins: [],
		build: {
			ssrEmitAssets: true,
			lib: {
				entry: "main",
				formats: ["cjs"],
			},
			rollupOptions: {
				output: {
					entryFileNames: "main.js",
					assetFileNames: "styles.css",
				},
				external: [
					"obsidian",
					"electron",
					"@codemirror/autocomplete",
					"@codemirror/collab",
					"@codemirror/commands",
					"@codemirror/language",
					"@codemirror/lint",
					"@codemirror/search",
					"@codemirror/state",
					"@codemirror/view",
					"@lezer/common",
					"@lezer/highlight",
					"@lezer/lr",
					...builtins,
				],
			},
			outDir: setOutDir(mode),
			emptyOutDir: false,
			sourcemap: mode === "production" ? false : "inline",
		},
	};
});
