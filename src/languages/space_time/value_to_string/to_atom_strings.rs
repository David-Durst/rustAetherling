use std::fmt::Write;
use std::rc::Rc;
pub trait ToAtomStrings {
    /// Convert an atom (or potentially nested Vec of atoms) to a 1D Vec of the atoms' string
    /// representations.
    /// The string vec argument stores the result.
    ///
    /// Call this with an empty `builder` and `top` as True, it will recur and
    /// update those values
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, top: bool);
}

impl ToAtomStrings for i32 {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, _: bool) {
        match builder.last_mut() {
            Some(s) => write!(Rc::get_mut(s).unwrap(), "{}", self),
            None => {
                let mut s = String::new();
                let write_result = write!(s, "{}", self);
                builder.push(Rc::from(s));
                write_result
            }
        }.unwrap();
    }
}

impl ToAtomStrings for bool {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, _: bool) {
        match builder.last_mut() {
            Some(s) => write!(Rc::get_mut(s).unwrap(), "{}", self),
            None => {
                let mut s = String::new();
                let write_result = write!(s, "{}", self);
                builder.push(Rc::from(s));
                write_result
            }
        }.unwrap();
    }
}

impl<A: ToAtomStrings, B: ToAtomStrings> ToAtomStrings for (A,B) {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, _: bool) {
        // ensure builder isn't empty
        match builder.last_mut() {
            Some(s) => s,
            None => {
                builder.push(Rc::from(String::new()));
                // the compiler doesn't know this is safe, but I do
                builder.last_mut().unwrap()
            }
        };
        let (a,b) = self;
        // now I know builder isn't empty and I'm trusting
        // that tuples are only of atoms.
        write!(Rc::get_mut(builder.last_mut().unwrap()).unwrap(), "(").unwrap();
        a.convert_to_flat_atom_list(builder, false);
        write!(Rc::get_mut(builder.last_mut().unwrap()).unwrap(), ",").unwrap();
        b.convert_to_flat_atom_list(builder, false);
        write!(Rc::get_mut(builder.last_mut().unwrap()).unwrap(), ")").unwrap();
    }
}

impl<A: ToAtomStrings > ToAtomStrings for Vec<A> {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, top: bool) {
        for (idx, elem) in self.iter().enumerate() {
            // if this is the first element, only add a vec if this is the top vector
            // otherwise on first index let parent vector create string
            // always insert string otherwise
            if (idx == 0 && top) || (idx > 0) {
                builder.push(Rc::from(String::new()))
            }
            elem.convert_to_flat_atom_list(builder, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_flat_atom_list_int() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        1.convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("1"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_bool() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        true.convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("true"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_tuple() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        (3, false).convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("(3,false)"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_array() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        vec!(4,2,1,5).convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("4")), Rc::from(String::from("2")),
                                 Rc::from(String::from("1")), Rc::from(String::from("5"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_nested_array() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        vec!(vec!(4,2),vec!(1,5)).convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("4")), Rc::from(String::from("2")),
                                 Rc::from(String::from("1")), Rc::from(String::from("5"))))
    }
}

