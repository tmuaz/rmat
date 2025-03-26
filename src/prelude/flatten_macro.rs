// TODO: Make the flatten macro for Mat<R,C> which will give you [f32;R*C]
// Why a macro? because const generics aren't in stable rust and I can't create a type called
// "type Flattened = [f32;R*C]" but I can actually initialize one
