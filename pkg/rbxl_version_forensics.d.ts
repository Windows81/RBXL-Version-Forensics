/* tslint:disable */
/* eslint-disable */

export function analyze_rbxl_bytes(data: Uint8Array): string;

export function decode_rbxl(data: Uint8Array): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly analyze_rbxl_bytes: (a: number, b: number) => [number, number];
    readonly decode_rbxl: (a: number, b: number) => [number, number];
    readonly rust_lz4_wasm_shim_calloc: (a: number, b: number) => number;
    readonly rust_lz4_wasm_shim_free: (a: number) => void;
    readonly rust_lz4_wasm_shim_malloc: (a: number) => number;
    readonly rust_lz4_wasm_shim_memcmp: (a: number, b: number, c: number) => number;
    readonly rust_lz4_wasm_shim_memcpy: (a: number, b: number, c: number) => number;
    readonly rust_lz4_wasm_shim_memmove: (a: number, b: number, c: number) => number;
    readonly rust_lz4_wasm_shim_memset: (a: number, b: number, c: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
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
