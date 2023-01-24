window.SIDEBAR_ITEMS = {"fn":[["checked_pow","Raises a value to the power of exp, returning `None` if an overflow occurred."],["ensure_pow","Raises a value to the power of exp, returning `ArithmeticError` if an overflow occurred."]],"struct":[["AccountIdLookup","A lookup implementation returning the `AccountId` from a `MultiAddress`."],["AppendZerosInput","Input that adds infinite number of zero after wrapped input."],["BadOrigin","An error type that indicates that the origin is invalid."],["BlakeTwo256","Blake2-256 Hash implementation."],["ConvertInto","A structure that performs standard conversion using the standard Rust conversion traits."],["Identity","A structure that performs identity conversion."],["IdentityLookup","A lookup implementation returning the input value."],["Keccak256","Keccak-256 Hash implementation."],["LookupError","An error that indicates that a lookup failed."],["ReduceBy","Mutator which reduces a scalar by a particular amount."],["Replace","Morpher to disregard the source value and replace with another."],["SignedExtensionMetadata","Information about a [`SignedExtension`] for the runtime metadata."],["TrailingZeroInput","Input that adds infinite number of zero after wrapped input."]],"trait":[["AccountIdConversion","This type can be converted into and possibly from an AccountId (which itself is generic)."],["AppVerify","Means of signature verification of an application key."],["Applyable","An “executable” piece of information, used by the standard Substrate Executive in order to enact a piece of extrinsic information by marshalling and dispatching to a named function call."],["AtLeast32Bit","A meta trait for arithmetic."],["AtLeast32BitUnsigned","A meta trait for arithmetic.  Same as [`AtLeast32Bit `], but also bounded to be unsigned."],["BlindCheckable","A “checkable” piece of information, used by the standard Substrate Executive in order to check the validity of a piece of extrinsic information, usually by verifying the signature. Implement for pieces of information that don’t require additional context in order to be checked."],["Block","Something which fulfills the abstract idea of a Substrate block. It has types for `Extrinsic` pieces of information as well as a `Header`."],["BlockIdTo","Something that can convert a `BlockId` to a number or a hash."],["BlockNumberProvider","Get current block number"],["Bounded","Numbers which have upper and lower bounds"],["CheckEqual","Something that can be checked for equality and printed out to a debug channel if bad."],["Checkable","Extract the digest type for a block. A “checkable” piece of information, used by the standard Substrate Executive in order to check the validity of a piece of extrinsic information, usually by verifying the signature. Implement for pieces of information that require some additional context `Context` in order to be checked."],["CheckedAdd","Performs addition that returns `None` instead of wrapping around on overflow."],["CheckedConversion","Convenience type to work around the highly unergonomic syntax needed to invoke the functions of overloaded generic traits, in this case `TryFrom` and `TryInto`."],["CheckedDiv","Performs division that returns `None` instead of panicking on division by zero and instead of wrapping around on underflow and overflow."],["CheckedMul","Performs multiplication that returns `None` instead of wrapping around on underflow or overflow."],["CheckedShl","Performs a left shift that returns `None` on shifts larger than the type width."],["CheckedShr","Performs a right shift that returns `None` on shifts larger than the type width."],["CheckedSub","Performs subtraction that returns `None` instead of wrapping around on underflow."],["Clear","Trait for things that can be clear (have no bits set). For numeric types, essentially the same as `Zero`."],["Convert","Extensible conversion trait. Generic over both source and destination types."],["ConvertBack","Extensible conversion trait. Generic over both source and destination types."],["Dispatchable","A lazy call (module function and argument values) that can be executed via its `dispatch` method."],["Ensure",""],["EnsureAdd","Performs addition that returns [`ArithmeticError`] instead of wrapping around on overflow."],["EnsureAddAssign","Performs self addition that returns [`ArithmeticError`] instead of wrapping around on overflow."],["EnsureDiv","Performs division that returns [`ArithmeticError`] instead of wrapping around on overflow."],["EnsureDivAssign","Performs self division that returns [`ArithmeticError`] instead of wrapping around on overflow."],["EnsureFixedPointNumber","Extends [`FixedPointNumber`] with the Ensure family functions."],["EnsureFrom","Similar to [`TryFrom`] but returning an [`ArithmeticError`] error."],["EnsureInto","Similar to [`TryInto`] but returning an [`ArithmeticError`] error."],["EnsureMul","Performs multiplication that returns [`ArithmeticError`] instead of wrapping around on overflow."],["EnsureMulAssign","Performs self multiplication that returns [`ArithmeticError`] instead of wrapping around on overflow."],["EnsureOp","Meta trait that supports all immutable arithmetic `Ensure*` operations"],["EnsureOpAssign","Meta trait that supports all assigned arithmetic `Ensure*` operations"],["EnsureSub","Performs subtraction that returns [`ArithmeticError`] instead of wrapping around on underflow."],["EnsureSubAssign","Performs self subtraction that returns [`ArithmeticError`] instead of wrapping around on underflow."],["Extrinsic","Something that acts like an `Extrinsic`."],["ExtrinsicMetadata","Implementor is an [`Extrinsic`] and provides metadata about this extrinsic."],["GetNodeBlockType","A marker trait for something that knows the type of the node block."],["GetRuntimeBlockType","A marker trait for something that knows the type of the runtime block."],["Hash","Abstraction around hashing"],["Header","Something which fulfills the abstract idea of a Substrate header. It has types for a `Number`, a `Hash` and a `Hashing`. It provides access to an `extrinsics_root`, `state_root` and `parent_hash`, as well as a `digest` and a block `number`."],["IdentifyAccount","Some type that is able to be collapsed into an account ID. It is not possible to recreate the original value from the account ID."],["IntegerSquareRoot","A trait implementing integer square root."],["IsMember","Determine if a `MemberId` is a valid member."],["Lazy","A lazy value."],["Lookup","Means of changing one type into another in a manner dependent on the source type."],["MaybeDisplay","A type that implements Display when in std environment."],["MaybeFromStr","A type that implements FromStr when in std environment."],["MaybeHash","A type that implements Hash when in std environment."],["MaybeSerialize","A type that implements Serialize when in std environment."],["MaybeSerializeDeserialize","A type that implements Serialize, DeserializeOwned and Debug when in std environment."],["Member","A type that can be used in runtime structures."],["Morph","Extensible conversion trait. Generic over only source type, with destination type being associated."],["One","Defines a multiplicative identity element for `Self`."],["OpaqueKeys","Opaque data type that may be destructured into a series of raw byte slices (which represent individual keys)."],["Printable","Trait for things which can be printed from the runtime."],["SaturatedConversion","Convenience type to work around the highly unergonomic syntax needed to invoke the functions of overloaded generic traits, in this case `SaturatedFrom` and `SaturatedInto`."],["Saturating","Saturating arithmetic operations, returning maximum or minimum values instead of overflowing."],["Scale","Multiply and divide by a number that isn’t necessarily the same type. Basically just the same as `Mul` and `Div` except it can be used for all basic numeric types."],["SignedExtension","Means by which a transaction may be extended. This type embodies both the data and the logic that should be additionally associated with the transaction. It should be plain old data."],["SimpleBitOps","A meta trait for all bit ops."],["StaticLookup","Means of changing one type into another in a manner dependent on the source type. This variant is different to `Lookup` in that it doesn’t (can cannot) require any context."],["TryMorph","Extensible conversion trait. Generic over only source type, with destination type being associated."],["UniqueSaturatedFrom","Just like `From` except that if the source value is too big to fit into the destination type then it’ll saturate the destination."],["UniqueSaturatedInto","Just like `Into` except that if the source value is too big to fit into the destination type then it’ll saturate the destination."],["ValidateUnsigned","Provide validation for unsigned extrinsics."],["Verify","Means of signature verification."],["Zero","Defines an additive identity element for `Self`."]],"type":[["DispatchInfoOf","Shortcut to reference the `Info` type of a `Dispatchable`."],["HashFor","Extract the hashing type for a block."],["NumberFor","Extract the number type for a block."],["PostDispatchInfoOf","Shortcut to reference the `PostInfo` type of a `Dispatchable`."]]};