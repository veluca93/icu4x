import { FFIError } from "./diplomat-runtime"
import { ICU4XLocaleParseError } from "./ICU4XLocaleParseError";
import { ICU4XOrdering } from "./ICU4XOrdering";

/**

 * An ICU4X Locale, capable of representing strings like `"en-US"`.

 * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html Rust documentation for `Locale`} for more information.
 */
export class ICU4XLocale {

  /**

   * Construct an {@link ICU4XLocale `ICU4XLocale`} from an locale identifier.

   * This will run the complete locale parsing algorithm. If code size and performance are critical and the locale is of a known shape (such as `aa-BB`) use `create_und`, `set_language`, `set_script`, and `set_region`.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XLocaleParseError}>
   */
  static create_from_string(name: string): ICU4XLocale | never;

  /**

   * Construct a default undefined {@link ICU4XLocale `ICU4XLocale`} "und".

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#associatedconstant.UND Rust documentation for `UND`} for more information.
   */
  static create_und(): ICU4XLocale;

  /**

   * Clones the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html Rust documentation for `Locale`} for more information.
   */
  clone(): ICU4XLocale;

  /**

   * Returns a string representation of the `LanguageIdentifier` part of {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  basename(): string;

  /**

   * Returns a string representation of the unicode extension.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.extensions Rust documentation for `extensions`} for more information.
   */
  get_unicode_extension(bytes: string): string | undefined;

  /**

   * Returns a string representation of {@link ICU4XLocale `ICU4XLocale`} language.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  language(): string;

  /**

   * Set the language part of the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XLocaleParseError}>
   */
  set_language(bytes: string): void | never;

  /**

   * Returns a string representation of {@link ICU4XLocale `ICU4XLocale`} region.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  region(): string | undefined;

  /**

   * Set the region part of the {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XLocaleParseError}>
   */
  set_region(bytes: string): void | never;

  /**

   * Returns a string representation of {@link ICU4XLocale `ICU4XLocale`} script.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#structfield.id Rust documentation for `id`} for more information.
   */
  script(): string | undefined;

  /**

   * Set the script part of the {@link ICU4XLocale `ICU4XLocale`}. Pass an empty string to remove the script.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.try_from_bytes Rust documentation for `try_from_bytes`} for more information.
   * @throws {@link FFIError}<{@link ICU4XLocaleParseError}>
   */
  set_script(bytes: string): void | never;

  /**

   * Best effort locale canonicalizer that doesn't need any data

   * Use ICU4XLocaleCanonicalizer for better control and functionality

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.canonicalize Rust documentation for `canonicalize`} for more information.
   * @throws {@link FFIError}<{@link ICU4XLocaleParseError}>
   */
  static canonicalize(bytes: string): string | never;

  /**

   * Returns a string representation of {@link ICU4XLocale `ICU4XLocale`}.

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.write_to Rust documentation for `write_to`} for more information.
   */
  to_string(): string;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.normalizing_eq Rust documentation for `normalizing_eq`} for more information.
   */
  normalizing_eq(other: string): boolean;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.strict_cmp Rust documentation for `strict_cmp`} for more information.
   */
  strict_cmp(other: string): ICU4XOrdering;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/locale/struct.Locale.html#method.total_cmp Rust documentation for `total_cmp`} for more information.
   */
  total_cmp(other: ICU4XLocale): ICU4XOrdering;
}
