//! The `error` module contains the `Error` enum that contains all error types used by this crate.
//!
//! The `Error` enum implements constructors for its struct variants, because those are ugly to construct.
//!
//! The module also contains some helper functions starting with `expect_` that check for a condition and return `Err(_)` if the condition is not fulfilled.
//! They are meant as shortcuts to not write the same error checking code everywhere.

use token::PartialToken;
use value::{value_type::ValueType, TupleType};

use crate::value::Value;

mod display;

/// Errors used in this crate.
#[derive(Debug, PartialEq)]
pub enum EvalexprError {
    /// An operator was called with a wrong amount of arguments.
    WrongOperatorArgumentAmount {
        /// The expected amount of arguments.
        expected: usize,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// A function was called with a wrong amount of arguments.
    WrongFunctionArgumentAmount {
        /// The expected amount of arguments.
        expected: usize,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// A string value was expected.
    ExpectedString {
        /// The actual value.
        actual: Value,
    },

    /// An integer value was expected.
    ExpectedInt {
        /// The actual value.
        actual: Value,
    },

    /// A float value was expected.
    ExpectedFloat {
        /// The actual value.
        actual: Value,
    },

    /// A numeric value was expected.
    /// Numeric values are the variants `Value::Int` and `Value::Float`.
    ExpectedNumber {
        /// The actual value.
        actual: Value,
    },

    /// A numeric or string value was expected.
    /// Numeric values are the variants `Value::Int` and `Value::Float`.
    ExpectedNumberOrString {
        /// The actual value.
        actual: Value,
    },

    /// A boolean value was expected.
    ExpectedBoolean {
        /// The actual value.
        actual: Value,
    },

    /// A tuple value was expected.
    ExpectedTuple {
        /// The actual value.
        actual: Value,
    },

    /// An empty value was expected.
    ExpectedEmpty {
        /// The actual value.
        actual: Value,
    },

    /// Tried to append a child to a leaf node.
    /// Leaf nodes cannot have children.
    AppendedToLeafNode,

    /// Tried to append a child to a node such that the precedence of the child is not higher.
    /// This error should never occur.
    /// If it does, please file a bug report.
    PrecedenceViolation,

    /// A `VariableIdentifier` operation did not find its value in the context.
    VariableIdentifierNotFound(String),

    /// A `FunctionIdentifier` operation did not find its value in the context.
    FunctionIdentifierNotFound(String),

    /// A value has the wrong type.
    /// Only use this if there is no other error that describes the expected and provided types in more detail.
    TypeError {
        /// The expected types.
        expected: TupleType,
        /// The actual value.
        actual: Value,
    },

    /// An opening brace without a matching closing brace was found.
    UnmatchedLBrace,

    /// A closing brace without a matching opening brace was found.
    UnmatchedRBrace,

    /// A `PartialToken` is unmatched, such that it cannot be combined into a full `Token`.
    /// This happens if for example a single `=` is found, surrounded by whitespace.
    /// It is not a token, but it is part of the string representation of some tokens.
    UnmatchedPartialToken {
        /// The unmatched partial token.
        first: PartialToken,
        /// The token that follows the unmatched partial token and that cannot be matched to the partial token, or `None`, if `first` is the last partial token in the stream.
        second: Option<PartialToken>,
    },

    /// An addition operation performed by Rust failed.
    AdditionError {
        /// The first argument of the addition.
        augend: Value,
        /// The second argument of the addition.
        addend: Value,
    },

    /// A subtraction operation performed by Rust failed.
    SubtractionError {
        /// The first argument of the subtraction.
        minuend: Value,
        /// The second argument of the subtraction.
        subtrahend: Value,
    },

    /// A negation operation performed by Rust failed.
    NegationError {
        /// The argument of the negation.
        argument: Value,
    },

    /// A multiplication operation performed by Rust failed.
    MultiplicationError {
        /// The first argument of the multiplication.
        multiplicand: Value,
        /// The second argument of the multiplication.
        multiplier: Value,
    },

    /// A division operation performed by Rust failed.
    DivisionError {
        /// The first argument of the division.
        dividend: Value,
        /// The second argument of the division.
        divisor: Value,
    },

    /// A modulation operation performed by Rust failed.
    ModulationError {
        /// The first argument of the modulation.
        dividend: Value,
        /// The second argument of the modulation.
        divisor: Value,
    },

    /// A regular expression could not be parsed
    InvalidRegex {
        /// The invalid regular expression
        regex: String,
        /// Failure message from the regex engine
        message: String,
    },

    /// A modification was attempted on a `Context` that does not allow modifications.
    ContextNotManipulable,

    /// An escape sequence within a string literal is illegal.
    IllegalEscapeSequence(String),

    /// A custom error explained by its message.
    CustomMessage(String),
}

impl EvalexprError {
    pub(crate) fn wrong_operator_argument_amount(actual: usize, expected: usize) -> Self {
        EvalexprError::WrongOperatorArgumentAmount { actual, expected }
    }

    pub(crate) fn wrong_function_argument_amount(actual: usize, expected: usize) -> Self {
        EvalexprError::WrongFunctionArgumentAmount { actual, expected }
    }

    /// Constructs `Error::TypeError{actual, expected}`.
    pub fn type_error(actual: Value, expected: TupleType) -> Self {
        EvalexprError::TypeError { actual, expected }
    }

    /// Constructs `Error::ExpectedString{actual}`.
    pub fn expected_string(actual: Value) -> Self {
        EvalexprError::ExpectedString { actual }
    }

    /// Constructs `Error::ExpectedInt{actual}`.
    pub fn expected_int(actual: Value) -> Self {
        EvalexprError::ExpectedInt { actual }
    }

    /// Constructs `Error::ExpectedFloat{actual}`.
    pub fn expected_float(actual: Value) -> Self {
        EvalexprError::ExpectedFloat { actual }
    }

    /// Constructs `Error::ExpectedNumber{actual}`.
    pub fn expected_number(actual: Value) -> Self {
        EvalexprError::ExpectedNumber { actual }
    }

    /// Constructs `Error::ExpectedNumberOrString{actual}`.
    pub fn expected_number_or_string(actual: Value) -> Self {
        EvalexprError::ExpectedNumberOrString { actual }
    }

    /// Constructs `Error::ExpectedBoolean{actual}`.
    pub fn expected_boolean(actual: Value) -> Self {
        EvalexprError::ExpectedBoolean { actual }
    }

    /// Constructs `Error::ExpectedTuple{actual}`.
    pub fn expected_tuple(actual: Value) -> Self {
        EvalexprError::ExpectedTuple { actual }
    }

    /// Constructs `Error::ExpectedEmpty{actual}`.
    pub fn expected_empty(actual: Value) -> Self {
        EvalexprError::ExpectedEmpty { actual }
    }

    /// Constructs an error that expresses that the type of `expected` was expected, but `actual` was found.
    pub(crate) fn expected_type(expected: &Value, actual: Value) -> Self {
        match ValueType::from(expected) {
            ValueType::String => Self::expected_string(actual),
            ValueType::Int => Self::expected_int(actual),
            ValueType::Float => Self::expected_float(actual),
            ValueType::Boolean => Self::expected_boolean(actual),
            ValueType::Tuple => Self::expected_tuple(actual),
            ValueType::Empty => Self::expected_empty(actual),
        }
    }

    pub(crate) fn unmatched_partial_token(
        first: PartialToken,
        second: Option<PartialToken>,
    ) -> Self {
        EvalexprError::UnmatchedPartialToken { first, second }
    }

    pub(crate) fn addition_error(augend: Value, addend: Value) -> Self {
        EvalexprError::AdditionError { augend, addend }
    }

    pub(crate) fn subtraction_error(minuend: Value, subtrahend: Value) -> Self {
        EvalexprError::SubtractionError {
            minuend,
            subtrahend,
        }
    }

    pub(crate) fn negation_error(argument: Value) -> Self {
        EvalexprError::NegationError { argument }
    }

    pub(crate) fn multiplication_error(multiplicand: Value, multiplier: Value) -> Self {
        EvalexprError::MultiplicationError {
            multiplicand,
            multiplier,
        }
    }

    pub(crate) fn division_error(dividend: Value, divisor: Value) -> Self {
        EvalexprError::DivisionError { dividend, divisor }
    }

    pub(crate) fn modulation_error(dividend: Value, divisor: Value) -> Self {
        EvalexprError::ModulationError { dividend, divisor }
    }

    /// Constructs `EvalexprError::InvalidRegex(regex)`
    pub fn invalid_regex(regex: String, message: String) -> Self {
        EvalexprError::InvalidRegex { regex, message }
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongOperatorArgumentAmount)` otherwise.
pub(crate) fn expect_operator_argument_amount(
    actual: usize,
    expected: usize,
) -> EvalexprResult<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(EvalexprError::wrong_operator_argument_amount(
            actual, expected,
        ))
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongFunctionArgumentAmount)` otherwise.
pub fn expect_function_argument_amount(actual: usize, expected: usize) -> EvalexprResult<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(EvalexprError::wrong_function_argument_amount(
            actual, expected,
        ))
    }
}

/// Returns `Ok(&str)` if the given value is a `Value::String`, or `Err(Error::ExpectedString)` otherwise.
pub fn expect_string(actual: &Value) -> EvalexprResult<&str> {
    match actual {
        Value::String(string) => Ok(string),
        _ => Err(EvalexprError::expected_string(actual.clone())),
    }
}

/// Returns `Ok(())` if the given value is numeric.
/// Numeric types are `Value::Int` and `Value::Float`.
/// Otherwise, `Err(Error::ExpectedNumber)` is returned.
pub fn expect_number(actual: &Value) -> EvalexprResult<()> {
    match actual {
        Value::Float(_) | Value::Int(_) => Ok(()),
        _ => Err(EvalexprError::expected_number(actual.clone())),
    }
}

/// Returns `Ok(())` if the given value is a string or a numeric
pub fn expect_number_or_string(actual: &Value) -> EvalexprResult<()> {
    match actual {
        Value::String(_) | Value::Float(_) | Value::Int(_) => Ok(()),
        _ => Err(EvalexprError::expected_number_or_string(actual.clone())),
    }
}

/// Returns `Ok(bool)` if the given value is a `Value::Boolean`, or `Err(Error::ExpectedBoolean)` otherwise.
pub fn expect_boolean(actual: &Value) -> EvalexprResult<bool> {
    match actual {
        Value::Boolean(boolean) => Ok(*boolean),
        _ => Err(EvalexprError::expected_boolean(actual.clone())),
    }
}

/// Returns `Ok(&[Value])` if the given value is a `Value::Tuple`, or `Err(Error::ExpectedTuple)` otherwise.
pub fn expect_tuple(actual: &Value) -> EvalexprResult<&TupleType> {
    match actual {
        Value::Tuple(tuple) => Ok(tuple),
        _ => Err(EvalexprError::expected_tuple(actual.clone())),
    }
}

impl std::error::Error for EvalexprError {}

/// Standard result type used by this crate.
pub type EvalexprResult<T> = Result<T, EvalexprError>;
