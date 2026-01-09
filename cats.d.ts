/* tslint:disable */
/* eslint-disable */

export function run(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: () => void;
  readonly wasm_bindgen__convert__closures_____invoke__h1bbf50a35efb5d62: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__hc7cbddcd7b8e24b5: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__he65f689ee506eec0: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__closure__destroy__h022c5dbe0146d7aa: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h0d725392e12bfb39: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h23959bc58e1e4dc7: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h2e5b0c74c112b27b: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h33091d4a47b2d821: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hcd2f4914dca0ad89: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h8e2beb3cae15adf9: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h0d99fd9f9469cdbb: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
