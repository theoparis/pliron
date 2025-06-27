//! Test format derive for plain structs, attributes / types.

use pliron::{
    builtin::types::IntegerType,
    location,
    parsable::{self, Parsable, state_stream_from_iterator},
    printable::Printable,
    r#type::TypePtr,
};
use pliron_derive::format;

mod common;
use common::setup_context_dialects;

#[format]
struct IntWrapper {
    inner: TypePtr<IntegerType>,
}

#[test]
fn int_wrapper() {
    let ctx = &mut setup_context_dialects();
    let int_ty = IntegerType::get(ctx, 64, pliron::builtin::types::Signedness::Signed);
    let test_ty = IntWrapper { inner: int_ty };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("{inner=builtin.integer si64}", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = IntWrapper::parser(())
        .parse(state_stream)
        .expect("IntWrapper parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format("`BubbleWrap` `[` $inner `]`")]
struct IntWrapperCustom {
    inner: TypePtr<IntegerType>,
}

#[test]
fn int_wrapper_custom() {
    let ctx = &mut setup_context_dialects();
    let int_ty = IntegerType::get(ctx, 64, pliron::builtin::types::Signedness::Signed);
    let test_ty = IntWrapperCustom { inner: int_ty };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("BubbleWrap[builtin.integer si64]", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = match IntWrapperCustom::parser(()).parse(state_stream) {
        Err(err) => panic!("IntWrapper parser failed: {err}"),
        Ok(res) => res,
    };
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format]
struct DoubleWrap {
    one: TypePtr<IntegerType>,
    two: IntWrapper,
}

#[test]
fn double_wrap() {
    let ctx = &mut setup_context_dialects();
    let int_ty = IntegerType::get(ctx, 64, pliron::builtin::types::Signedness::Signed);
    let test_ty_intermediate = IntWrapper { inner: int_ty };
    let test_ty = DoubleWrap {
        one: int_ty,
        two: test_ty_intermediate,
    };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!(
        "{one=builtin.integer si64,two={inner=builtin.integer si64}}",
        &printed
    );

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = DoubleWrap::parser(())
        .parse(state_stream)
        .expect("DoubleWrap parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format]
enum Enum {
    /// Some comment
    A(TypePtr<IntegerType>),
    B {
        one: TypePtr<IntegerType>,
        two: IntWrapper,
    },
    C,
    /// Some other comment
    #[format("`<` $upper `/` $lower `>`")]
    Op {
        upper: u64,
        lower: u64,
    },
    #[format("`<` opt($a) `>`")]
    WithOpt {
        a: Option<u64>,
    },
    #[format("`<` vec($a, Char(`,`)) `>`")]
    WithVec {
        a: Vec<u64>,
    },
    #[format("`<` opt($0) `;` vec($1, CharSpace(`,`)) `>`")]
    WithOptTuple(Option<u64>, Vec<u64>),
}

#[test]
fn enum_test() {
    let ctx = &mut setup_context_dialects();
    let int_ty = IntegerType::get(ctx, 64, pliron::builtin::types::Signedness::Signed);
    let test_ty = Enum::B {
        one: int_ty,
        two: IntWrapper { inner: int_ty },
    };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!(
        "B{one=builtin.integer si64,two={inner=builtin.integer si64}}",
        &printed
    );

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = Enum::parser(())
        .parse(state_stream)
        .expect("Enum parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);

    let test_ty = Enum::A(int_ty);
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("A(builtin.integer si64)", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = Enum::parser(())
        .parse(state_stream)
        .expect("Enum parser failed");

    assert_eq!(res.disp(ctx).to_string(), printed);

    let test_ty = Enum::C;
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("C", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = Enum::parser(())
        .parse(state_stream)
        .expect("Enum parser failed");

    assert_eq!(res.disp(ctx).to_string(), printed);

    let test_ty = Enum::Op {
        upper: 42,
        lower: 7,
    };
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("Op<42/7>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = Enum::parser(())
        .parse(state_stream)
        .expect("Enum parser failed");

    assert_eq!(res.disp(ctx).to_string(), printed);

    let test_ty = Enum::WithOpt { a: Some(42) };
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("WithOpt<42>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = Enum::parser(())
        .parse(state_stream)
        .expect("Enum parser failed");

    assert_eq!(res.disp(ctx).to_string(), printed);

    let test_ty = Enum::WithVec { a: vec![1, 2, 3] };
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("WithVec<1,2,3>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = Enum::parser(())
        .parse(state_stream)
        .expect("Enum parser failed");

    assert_eq!(res.disp(ctx).to_string(), printed);

    let test_ty = Enum::WithOptTuple(Some(42), vec![1, 2, 3]);
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("WithOptTuple<42;1, 2, 3>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = Enum::parser(())
        .parse(state_stream)
        .expect("Enum parser failed");

    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format]
struct U64Wrapper {
    a: u64,
}

#[test]
fn u64_wrapper() {
    let ctx = &mut setup_context_dialects();
    let test_ty = U64Wrapper { a: 42 };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("{a=42}", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = U64Wrapper::parser(())
        .parse(state_stream)
        .expect("U64Wrapper parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format("$upper `/` $lower")]
struct IntDiv {
    upper: u64,
    lower: u64,
}

#[test]
fn int_div() {
    let ctx = &mut setup_context_dialects();
    let test_ty = IntDiv {
        upper: 42,
        lower: 7,
    };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("42/7", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = IntDiv::parser(())
        .parse(state_stream)
        .expect("IntDiv parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format("`<` opt($a) `>`")]
struct OptionalField {
    /// Some comment
    a: Option<u64>,
}

#[test]
fn optional_field() {
    let ctx = &mut setup_context_dialects();
    let test_ty = OptionalField { a: Some(42) };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<42>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = OptionalField::parser(())
        .parse(state_stream)
        .expect("OptionalField parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);

    let test_ty = OptionalField { a: None };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = OptionalField::parser(())
        .parse(state_stream)
        .expect("OptionalField parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format("`<` vec($a, Char(`,`)) `>`")]
struct VecField {
    a: Vec<u64>,
}

#[test]
fn vec_field() {
    let ctx = &mut setup_context_dialects();
    let test_ty = VecField { a: vec![1, 2, 3] };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<1,2,3>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = VecField::parser(())
        .parse(state_stream)
        .expect("VecField parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);

    // Test empty vector
    let test_ty = VecField { a: vec![] };
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = VecField::parser(())
        .parse(state_stream)
        .expect("VecField parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format("`<` opt($a) `;` vec($b, Char(`,`)) `>`")]
struct OptAndVec {
    a: Option<u64>,
    b: Vec<u64>,
}

#[test]
fn opt_and_vec() {
    let ctx = &mut setup_context_dialects();
    let test_ty = OptAndVec {
        a: Some(42),
        b: vec![1, 2, 3],
    };

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<42;1,2,3>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = OptAndVec::parser(())
        .parse(state_stream)
        .expect("OptAndVec parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);

    // Test empty vector
    let test_ty = OptAndVec { a: None, b: vec![] };
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<;>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = OptAndVec::parser(())
        .parse(state_stream)
        .expect("OptAndVec parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}

#[format("`<` opt($0) `;` vec($1, Char(`,`)) `>`")]
struct OptAndVecTuple(Option<u64>, Vec<u64>);

#[test]
fn opt_and_vec_tuple() {
    let ctx = &mut setup_context_dialects();
    let test_ty = OptAndVecTuple(Some(42), vec![1, 2, 3]);

    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<42;1,2,3>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = OptAndVecTuple::parser(())
        .parse(state_stream)
        .expect("OptAndVecTuple parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);

    // Test empty vector
    let test_ty = OptAndVecTuple(None, vec![]);
    let printed = test_ty.disp(ctx).to_string();
    assert_eq!("<;>", &printed);

    let state_stream = state_stream_from_iterator(
        printed.chars(),
        parsable::State::new(ctx, location::Source::InMemory),
    );
    let (res, _) = OptAndVecTuple::parser(())
        .parse(state_stream)
        .expect("OptAndVecTuple parser failed");
    assert_eq!(res.disp(ctx).to_string(), printed);
}
