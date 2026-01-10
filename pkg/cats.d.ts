/* tslint:disable */
/* eslint-disable */

export function run(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: () => void;
  readonly wasm_bindgen__convert__closures_____invoke__h191a8d2f1b5ab73e: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h064db88dcd744927: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__he71d2d4cd1bd3623: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h8f148df042134320: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h3be56cda1f9164f0: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h5d4ed820227c6cd3: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h718c60f2cae19483: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__he0e3970d6c2b5893: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h55a1984c953fe18a: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__ha422922f2c42099a: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hd43ac4cfa4d8fcb0: (a: number, b: number, c: any, d: any) => void;
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
