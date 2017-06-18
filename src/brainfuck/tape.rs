pub trait Tape<T: Sized> {
    /// Move the focus `delta` positions to the right.
    fn seek(&mut self, delta: i64);

    /// Get a mutable reference to the currently focused element.
    fn get_focus_ref(&mut self) -> &mut T;

    /// Modify the currently focused element.
    fn modify<F>(&mut self, f: FnMut(T)) {
        f(self.get_focus_ref())
    }
}

struct VecTape<T> {
    tape: Vec<T>,
    focus: i64,
}

impl<T> VecTape<T> {
    fn new() -> VecTape<T> {
        VecTape {
            tape: Vec::new(),
            focus: 0,
        }
    }
}

impl<T> Tape<T> for VecTape<T> {
    fn seek(&mut self, delta: i64) {
        self.focus += delta;

        if self.focus < 0 {
            self.focus = 0;
        } else if self.focus > self.tape.len() {
            let new_len = self.tape.len() + 1; // +1 or loop will diverge for empty tape
            while new_len < self.focus {
                new_len *= 2;
            }
            self.tape.resize(new_len);
        }
    }

    fn get_focus_ref(&mut self) -> &mut T {
        self.tape.index_mut(self.focus)
    }
}
