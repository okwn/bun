//! Zig comptime-reflection helpers.
//!
//! Almost everything in `meta.zig` is built on `@typeInfo` / `@TypeOf` /
//! `@hasDecl` / `@Type`, which have **no Rust equivalent** (see PORTING.md
//! §Comptime reflection). In Rust the call sites should use:
//!   - a generic `<T>` directly instead of `@TypeOf(anytype)`
//!   - a trait bound instead of `@hasDecl` duck-typing
//!   - `#[derive(...)]` instead of field iteration
//!   - `core::any::type_name::<T>()` instead of `@typeName`
//!
//! The few items that *do* have a Rust shape are ported below; the rest are
//! stubbed with `// TODO(port):` pointing callers at the idiomatic
//! replacement.

pub mod tagged_union;
pub use tagged_union::TaggedUnion;

// ──────────────────────────────────────────────────────────────────────────
// Type-level reflection helpers — no Rust equivalent
// ──────────────────────────────────────────────────────────────────────────

// TODO(port): `OptionalChild(T)` extracts `U` from `*?U`. Rust has no
// type-level reflection; callers should name the inner type directly or use
// an associated type on a trait. No replacement provided.

// TODO(port): `ReturnOfMaybe(function)` / `MaybeResult(MaybeType)` extract
// the `Ok` payload type from a `bun_sys::Result<T>`-returning fn. In Rust
// the `T` is already named at the call site; no helper needed.

// TODO(port): `ReturnOf(function)` / `ReturnOfType(Type)` extract a fn's
// return type. Rust has no fn-signature reflection; callers must name the
// return type or use an associated type (`FnOnce() -> R` bound gives `R`).

pub fn type_name<T: ?Sized>() -> &'static str {
    type_base_name(core::any::type_name::<T>())
}

/// partially emulates behaviour of @typeName in previous Zig versions,
/// converting "some.namespace.MyType" to "MyType"
#[inline]
pub fn type_base_name(fullname: &'static str) -> &'static str {
    if fullname.contains('(') || fullname.contains('<') {
        return fullname;
    }
    let after_dot = match fullname.rfind('.') {
        None => fullname,
        Some(idx) => &fullname[idx + 1..],
    };
    match after_dot.rfind("::") {
        None => after_dot,
        Some(idx) => &after_dot[idx + 2..],
    }
}

// TODO(port): `banFieldType(Container, T)` — compile-time assertion that no
// field of `Container` has type `T`. No Rust equivalent; would require a
// proc-macro. Callers should drop the check (it was a lint, not load-bearing).

// TODO(port): `Item(T)` — element type of a slice/array/pointer. In Rust the
// element type is always nameable directly (`&[T]` → `T`); no helper needed.

// ──────────────────────────────────────────────────────────────────────────
// ConcatArgs* — build an ArgsTuple for `@call`
// ──────────────────────────────────────────────────────────────────────────

// TODO(port): `CreateUniqueTuple(N, types)` — `@Type` synthesis of a tuple
// struct. Rust tuples `(T0, T1, ...)` are the direct equivalent; no helper
// needed. (This was `fn`-private in Zig anyway.)

// ──────────────────────────────────────────────────────────────────────────
// Layout / copy / eql predicates — become marker-trait bounds
// ──────────────────────────────────────────────────────────────────────────

// TODO(port): `isSimpleCopyType(T)` — recursive "is this trivially
// copyable". In Rust this is exactly the `Copy` bound. Callers: `T: Copy`.

// TODO(port): `isScalar(T)` — `i32|u32|i64|u64|f32|f64|bool|enum`. Callers
// should use a sealed `Scalar` marker trait impl'd for those types, or just
// accept `T: Copy + PartialEq` if that was the intent.

// TODO(port): `isSimpleEqlType(T)` — types where `a == b` is bitwise. In
// Rust: `T: Eq` (or `bytemuck::Pod` for the bitwise guarantee). Callers:
// add the bound.

// ──────────────────────────────────────────────────────────────────────────
// List-container duck-typing
// ──────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ListContainerType {
    ArrayList,
    Vec,
    SmallList,
}

// TODO(port): `Tagged(U, T)` — re-synthesize a `union` with a new tag type
// via `@Type`. Rust enums are always tagged; there is no "retag" operation.
// Callers must define the enum they want directly.

// TODO(port): `SliceChild(T)` — `&[U]` → `U`, else `T`. Same as `Item`;
// callers name `U` directly.

// ──────────────────────────────────────────────────────────────────────────
// useAllFields — exhaustive-field-use lint (ziglang/zig#21879)
// ──────────────────────────────────────────────────────────────────────────

#[inline]
pub fn void_field_type_discard_helper<T>(_data: T) {
    // intentionally empty
}

// ported from: src/meta/meta.zig
