use crate::Type;

enum Error {
    ParseError(),
    RuntimeError(RuntimeError)
}

enum RuntimeError {
    TypeError {
        expected: Type,
        current: Type,
    },
}
