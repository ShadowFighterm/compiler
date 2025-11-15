#[derive(Debug)]
pub enum TypeChkError {
    ErroneousVarDecl,
    FnCallParamCount,
    FnCallParamType,
    ErroneousReturnType,
    ExpressionTypeMismatch,
    ExpectedBooleanExpression,
    ErroneousBreak,
    NonBooleanCondStmt,
    EmptyExpression,
    AttemptedBoolOpOnNonBools,
    AttemptedBitOpOnNonNumeric,
    AttemptedShiftOnNonInt,
    AttemptedAddOpOnNonNumeric,
    AttemptedExponentiationOfNonNumeric,
    ReturnStmtNotFound,
}
