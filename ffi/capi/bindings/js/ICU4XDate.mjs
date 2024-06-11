import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XCalendar } from "./ICU4XCalendar.mjs"
import { ICU4XCalendarError_js_to_rust, ICU4XCalendarError_rust_to_js } from "./ICU4XCalendarError.mjs"
import { ICU4XIsoDate } from "./ICU4XIsoDate.mjs"
import { ICU4XIsoWeekday_js_to_rust, ICU4XIsoWeekday_rust_to_js } from "./ICU4XIsoWeekday.mjs"
import { ICU4XWeekOf } from "./ICU4XWeekOf.mjs"
import { ICU4XWeekRelativeUnit_js_to_rust, ICU4XWeekRelativeUnit_rust_to_js } from "./ICU4XWeekRelativeUnit.mjs"

const ICU4XDate_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDate_destroy(underlying);
});

export class ICU4XDate {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XDate_box_destroy_registry.register(this, underlying);
    }
  }

  static create_from_iso_in_calendar(arg_year, arg_month, arg_day, arg_calendar) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDate_create_from_iso_in_calendar(diplomat_receive_buffer, arg_year, arg_month, arg_day, arg_calendar.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XDate(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XCalendarError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_from_codes_in_calendar(arg_era_code, arg_year, arg_month_code, arg_day, arg_calendar) {
    const buf_arg_era_code = diplomatRuntime.DiplomatBuf.str8(wasm, arg_era_code);
    const buf_arg_month_code = diplomatRuntime.DiplomatBuf.str8(wasm, arg_month_code);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDate_create_from_codes_in_calendar(diplomat_receive_buffer, buf_arg_era_code.ptr, buf_arg_era_code.size, arg_year, buf_arg_month_code.ptr, buf_arg_month_code.size, arg_day, arg_calendar.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XDate(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XCalendarError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_era_code.free();
    buf_arg_month_code.free();
    return diplomat_out;
  }

  to_calendar(arg_calendar) {
    return new ICU4XDate(wasm.ICU4XDate_to_calendar(this.underlying, arg_calendar.underlying), true, []);
  }

  to_iso() {
    return new ICU4XIsoDate(wasm.ICU4XDate_to_iso(this.underlying), true, []);
  }

  day_of_year() {
    return wasm.ICU4XDate_day_of_year(this.underlying);
  }

  day_of_month() {
    return wasm.ICU4XDate_day_of_month(this.underlying);
  }

  day_of_week() {
    return ICU4XIsoWeekday_rust_to_js[wasm.ICU4XDate_day_of_week(this.underlying)];
  }

  week_of_month(arg_first_weekday) {
    return wasm.ICU4XDate_week_of_month(this.underlying, ICU4XIsoWeekday_js_to_rust[arg_first_weekday]);
  }

  week_of_year(arg_calculator) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XDate_week_of_year(diplomat_receive_buffer, this.underlying, arg_calculator.underlying);
      const out = new ICU4XWeekOf(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }

  ordinal_month() {
    return wasm.ICU4XDate_ordinal_month(this.underlying);
  }

  month_code() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XDate_month_code(this.underlying, write);
    });
  }

  year_in_era() {
    return wasm.ICU4XDate_year_in_era(this.underlying);
  }

  era() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XDate_era(this.underlying, write);
    });
  }

  months_in_year() {
    return wasm.ICU4XDate_months_in_year(this.underlying);
  }

  days_in_month() {
    return wasm.ICU4XDate_days_in_month(this.underlying);
  }

  days_in_year() {
    return wasm.ICU4XDate_days_in_year(this.underlying);
  }

  calendar() {
    return new ICU4XCalendar(wasm.ICU4XDate_calendar(this.underlying), true, []);
  }
}
