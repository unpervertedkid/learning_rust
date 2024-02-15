struct A;
struct S(A);
struct SGeneric<T>(T);

fn regular_function(_s:S) {}

fn generic_over_type(_s: SGeneric<A>) {}

fn generic_specific_type(_s: SGeneric<i32>) {}

fn generic<T>(_s:SGeneric<T>) {}

fn main() {
    // Using geneneric functions
    regular_function(S(A));
    generic_over_type(SGeneric(A));
    generic_specific_type(SGeneric(6));

    // Explicitly specified type parameter 'char' to 'generic()'
    generic::<char>(SGeneric('a'));

    // Implicitly specified type parameter 'char'  to 'generic'
    generic(SGeneric('c'));
}
