import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XLocaleParseError_js_to_rust, ICU4XLocaleParseError_rust_to_js } from "./ICU4XLocaleParseError.mjs"
import { ICU4XOrdering_js_to_rust, ICU4XOrdering_rust_to_js } from "./ICU4XOrdering.mjs"

const ICU4XLocale_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocale_destroy(underlying);
});

export class ICU4XLocale {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XLocale_box_destroy_registry.register(this, underlying);
    }
  }

  static create_from_string(arg_name) {
    const buf_arg_name = diplomatRuntime.DiplomatBuf.str8(wasm, arg_name);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocale_create_from_string(diplomat_receive_buffer, buf_arg_name.ptr, buf_arg_name.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XLocale(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XLocaleParseError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_name.free();
    return diplomat_out;
  }

  static create_und() {
    return new ICU4XLocale(wasm.ICU4XLocale_create_und(), true, []);
  }

  clone() {
    return new ICU4XLocale(wasm.ICU4XLocale_clone(this.underlying), true, []);
  }

  basename() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XLocale_basename(this.underlying, write);
    });
  }

  get_unicode_extension(arg_bytes) {
    const buf_arg_bytes = diplomatRuntime.DiplomatBuf.str8(wasm, arg_bytes);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.ICU4XLocale_get_unicode_extension(this.underlying, buf_arg_bytes.ptr, buf_arg_bytes.size, write) == 1;
        if (!is_ok) return;
      })();
    });
    buf_arg_bytes.free();
    return diplomat_out;
  }

  language() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XLocale_language(this.underlying, write);
    });
  }

  set_language(arg_bytes) {
    const buf_arg_bytes = diplomatRuntime.DiplomatBuf.str8(wasm, arg_bytes);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocale_set_language(diplomat_receive_buffer, this.underlying, buf_arg_bytes.ptr, buf_arg_bytes.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XLocaleParseError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_bytes.free();
    return diplomat_out;
  }

  region() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.ICU4XLocale_region(this.underlying, write) == 1;
        if (!is_ok) return;
      })();
    });
  }

  set_region(arg_bytes) {
    const buf_arg_bytes = diplomatRuntime.DiplomatBuf.str8(wasm, arg_bytes);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocale_set_region(diplomat_receive_buffer, this.underlying, buf_arg_bytes.ptr, buf_arg_bytes.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XLocaleParseError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_bytes.free();
    return diplomat_out;
  }

  script() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.ICU4XLocale_script(this.underlying, write) == 1;
        if (!is_ok) return;
      })();
    });
  }

  set_script(arg_bytes) {
    const buf_arg_bytes = diplomatRuntime.DiplomatBuf.str8(wasm, arg_bytes);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocale_set_script(diplomat_receive_buffer, this.underlying, buf_arg_bytes.ptr, buf_arg_bytes.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XLocaleParseError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_bytes.free();
    return diplomat_out;
  }

  static canonicalize(arg_bytes) {
    const buf_arg_bytes = diplomatRuntime.DiplomatBuf.str8(wasm, arg_bytes);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XLocale_canonicalize(diplomat_receive_buffer, buf_arg_bytes.ptr, buf_arg_bytes.size, write);
        const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
        if (is_ok) {
          const ok_value = {};
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          return ok_value;
        } else {
          const throw_value = ICU4XLocaleParseError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    buf_arg_bytes.free();
    return diplomat_out;
  }

  to_string() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XLocale_to_string(this.underlying, write);
    });
  }

  normalizing_eq(arg_other) {
    const buf_arg_other = diplomatRuntime.DiplomatBuf.str8(wasm, arg_other);
    const diplomat_out = wasm.ICU4XLocale_normalizing_eq(this.underlying, buf_arg_other.ptr, buf_arg_other.size);
    buf_arg_other.free();
    return diplomat_out;
  }

  strict_cmp(arg_other) {
    const buf_arg_other = diplomatRuntime.DiplomatBuf.str8(wasm, arg_other);
    const diplomat_out = ICU4XOrdering_rust_to_js[wasm.ICU4XLocale_strict_cmp(this.underlying, buf_arg_other.ptr, buf_arg_other.size)];
    buf_arg_other.free();
    return diplomat_out;
  }

  total_cmp(arg_other) {
    return ICU4XOrdering_rust_to_js[wasm.ICU4XLocale_total_cmp(this.underlying, arg_other.underlying)];
  }
}
