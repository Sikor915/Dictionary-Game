pub mod json_dictionary;

use std::ops::Index;

trait Dictionary: Index<usize> {
    fn len(&self) -> usize;
}