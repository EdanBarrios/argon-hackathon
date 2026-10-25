Targeted inputs for the two mutants that survived the generated inputs
(backend/harness/mutants.py): `utf8_two_byte_bound` (\u escapes at the UTF-8
length boundaries U+0080, U+0800, U+10000) and `hex_lower_f_rejected` (lowercase
hex digits, especially `f`, in \u escapes). Kept separate from gen_inputs.py so
the generated input sets, and every number quoted from them, stay unchanged.
Run: backend/harness/adversarial/run.sh <rust_binary> <label> mutant_gaps
