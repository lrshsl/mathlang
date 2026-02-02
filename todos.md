# Todos

## Current

- Fix math expr parsing `(a <op> b) -> a`

## Sub-commits

- separate tests in `code_generator/src/lib.rs`
- order of instructions (add = 1, sub = 2, ..)
- grammar in doc comments
- grammar file

## Commits

- `parse_expr_list`
- `Vec<Instruction> -> [Instruction; N]`
    - no need to truncate/pad in `graph_canvas/src/graph_shader_pipeline.rs`
- add test `.mth` files

## Branches

- parameters and `varref`s
- More instructions (non-mathematical)
    - Draw shapes
- Graph analysis tools


## Maybe

- syntax for `pow`?

