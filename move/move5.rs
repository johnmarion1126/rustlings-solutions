// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    
    let z = &mut x;
    *z += 1000;

    // This works because the last time we will use y is at line 5
    // No overlapping mutable refernce with y and z
    
    assert_eq!(x, 1200);
}
