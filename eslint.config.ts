import prettier from 'eslint-config-prettier';
import { includeIgnoreFile } from '@eslint/compat';
import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import { fileURLToPath } from 'node:url';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';

const gitignorePath = fileURLToPath(new URL('./.gitignore', import.meta.url));

export default ts.config(
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs.recommended,
	prettier,
	...svelte.configs.prettier,
	{
		languageOptions: {
			globals: { ...globals.browser, ...globals.node }
		},
		rules: {
			// typescript-eslint strongly recommend that you do not use the no-undef lint rule on TypeScript projects.
			// see: https://typescript-eslint.io/troubleshooting/faqs/eslint/#i-get-errors-from-the-no-undef-rule-about-global-variables-not-being-defined-even-though-there-are-no-typescript-errors
			'no-undef': 'off'
		}
	},
	{
		files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: ['.svelte'],
				parser: ts.parser,
				svelteConfig
			}
		}
	},
	// Ignore generated Paraglide files
	{
		ignores: ['src/paraglide/**']
	}
);

// NOTE: Custom rule lives in apps/frontend/eslint-rules/no-svelte-lifecycle.js
// Register the local rule and enable it for Svelte files so editors and CI pick it up.
import path from 'node:path';
import { createRequire } from 'node:module';
const requireC = createRequire(import.meta.url);
const localRulePath = path.resolve(
	new URL('./eslint-rules/no-svelte-lifecycle.cjs', import.meta.url).pathname
);

// Dynamically attach the local rule to the exported config. This is safe because eslint config runs in Node.
// eslint-config tooling expects a 'rules' map. We'll add an override for Svelte files.
export const rules = {
	// Placeholder to ensure the rules object exists for editors that import this file.
};

export const overrides = [
	{
		files: ['**/*.svelte'],
		plugins: {
			'local-rules': {
				rules: {
					'no-svelte-lifecycle': requireC(localRulePath)
				}
			}
		},
		rules: {
			'local-rules/no-svelte-lifecycle': 'warn'
		}
	}
];
