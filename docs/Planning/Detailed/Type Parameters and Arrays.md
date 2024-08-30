## Target
1. Type parameters
2. Arrays

## Implementation Requirements
- [[Type ID]]s to be changed to recursive enums
	- Enum needs to support arrays
	- Everywhere that uses type IDs needs to support this
	- Builtin types how?
- [[Function ID]]s to be changed to a composite of the new [[Type ID]]s and a type-unique UID, possibly a `usize`
- [[Name Resolution]] needs to be available at [[Function Compilation]] (as all types can no longer be preprocessed)
	- Consider doing all name resolution on-demand instead of having an explicit step. This will solve [[By-Struct Name Resolution]]
- [[Parsing]] needs to support rust-like `<` and `>` syntax, as well as `[]` everywhere where types may be used

