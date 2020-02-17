use std::fmt::Write;

trait ToAtomStrings {
    /// Convert an atom (or potentially nested Vec of atoms) to a 1D Vec of the atoms' string
    /// representations.
    /// The string vec argument stores the result.
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<String>);
}

impl ToAtomStrings for i32 {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<String>) {
        match builder.last_mut() {
            Some(s) => write!(s, "{}", self),
            None => {
                let mut s = String::new();
                let write_result = write!(s, "{}", self);
                builder.push(s);
                write_result
            }
        }.unwrap();
    }
}

impl ToAtomStrings for bool {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<String>) {
        match builder.last_mut() {
            Some(s) => write!(s, "{}", self),
            None => {
                let mut s = String::new();
                let write_result = write!(s, "{}", self);
                builder.push(s);
                write_result
            }
        }.unwrap();
    }
}

impl<A: ToAtomStrings, B: ToAtomStrings> ToAtomStrings for (A,B) {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<String>) {
        let s = match builder.last_mut() {
            Some(s) => s,
            None => {
                let mut s = String::new();
                builder.push(s);
                // the compiler doesn't know this is safe, but I do
                builder.last_mut().unwrap()
            }
        };
        let (a,b) = self;
        write!("(", s);
        a.convert_to_flat_atom_list(builder);
        write!(",", s);
        b.convert_to_flat_atom_list(builder);
        write!(")", s);
    }
}

/*
impl<A,B> ToAtomStrings for (A,B) {
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<String>) {
        let s = match builder.last() {
            Some(s) => s,
            None => {
                let s = String::new();
                builder.push(s);
                s
            }
        }
    }
}
class Convertible_To_Atom_Strings a where
  convert_to_flat_atom_list :: a -> ST_Val_To_String_Config -> [String]

instance Convertible_To_Atom_Strings Integer where
  convert_to_flat_atom_list x conf = [make_integer_string_for_backend conf x]

instance Convertible_To_Atom_Strings Bool where
  convert_to_flat_atom_list x conf = [make_bool_string_for_backend conf x]

instance (Convertible_To_Atom_Strings a, Convertible_To_Atom_Strings b) =>
  Convertible_To_Atom_Strings (a, b) where
  convert_to_flat_atom_list (x, y) conf =
    [make_tuple_string_for_backend conf
      (head $ convert_to_flat_atom_list x conf)
      (head $ convert_to_flat_atom_list y conf)]

instance (Convertible_To_Atom_Strings a) => Convertible_To_Atom_Strings [a] where
  convert_to_flat_atom_list xs conf = concat $
    map (\x -> convert_to_flat_atom_list x conf) xs

instance Convertible_To_Atom_Strings AST_Atoms where
  convert_to_flat_atom_list (BitA b) conf =
    [make_bool_string_for_backend conf $ b]
  convert_to_flat_atom_list (IntA i) conf =
    [make_integer_string_for_backend conf i]
  convert_to_flat_atom_list (ATupleA x y) conf =
    [make_tuple_string_for_backend conf
      (head $ convert_to_flat_atom_list x conf)
      (head $ convert_to_flat_atom_list y conf)]

*/