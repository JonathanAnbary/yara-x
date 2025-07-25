use std::mem::size_of;

use crate::compiler::{Expr, IR};
use crate::types::TypeValue;

#[test]
fn expr_size() {
    // Sentinel test for making sure the Expr doesn't grow in future
    // changes.
    #[cfg(target_pointer_width = "32")]
    assert_eq!(size_of::<Expr>(), 24);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(size_of::<Expr>(), 48);
}

#[test]
fn ancestors() {
    let mut ir = IR::new();

    let const_1 = ir.constant(TypeValue::const_integer_from(1));
    let const_2 = ir.constant(TypeValue::const_integer_from(2));
    let const_3 = ir.constant(TypeValue::const_integer_from(3));
    let add = ir.add(vec![const_2, const_3]).unwrap();
    let root = ir.add(vec![const_1, add]).unwrap();

    let mut ancestors = ir.ancestors(const_3);
    assert_eq!(ancestors.next(), Some(add));
    assert_eq!(ancestors.next(), Some(root));
    assert_eq!(ancestors.next(), None);

    let mut ancestors = ir.ancestors(const_1);
    assert_eq!(ancestors.next(), Some(root));
    assert_eq!(ancestors.next(), None);

    let mut ancestors = ir.ancestors(root);
    assert_eq!(ancestors.next(), None);
}

#[test]
fn children() {
    let mut ir = IR::new();

    let const_1 = ir.constant(TypeValue::const_integer_from(1));
    let const_2 = ir.constant(TypeValue::const_integer_from(2));
    let const_3 = ir.constant(TypeValue::const_integer_from(3));
    let add = ir.add(vec![const_2, const_3]).unwrap();
    let root = ir.add(vec![const_1, add]).unwrap();

    let mut children = ir.children(root);

    assert_eq!(children.next(), Some(const_1));
    assert_eq!(children.next(), Some(add));
    assert_eq!(children.next(), None);

    let mut children = ir.children(add);

    assert_eq!(children.next(), Some(const_2));
    assert_eq!(children.next(), Some(const_3));
    assert_eq!(children.next(), None);

    let mut children = ir.children(const_1);
    assert_eq!(children.next(), None);
}

// This test is run only in 64-bits systems because the IR tree shows the hash
// of each node, which will be either 32 or 64 bits long, depending on the
// system.
#[cfg(target_pointer_width = "64")]
#[test]
fn ir() {
    use std::fs;
    use std::io::BufWriter;

    use crate::Compiler;

    let files: Vec<_> = globwalk::glob("src/compiler/ir/tests/testdata/*.in")
        .unwrap()
        .flatten()
        .map(|entry| entry.into_path())
        .collect();

    files.into_iter().for_each(|path| {
        println!("file: {:?}", path);

        let mut mint = goldenfile::Mint::new(".");

        let output_path = if cfg!(feature = "constant-folding") {
            path.with_extension("ir")
        } else {
            path.with_extension("no-folding.ir")
        };

        let source = fs::read_to_string(path).unwrap();

        let output_file = mint.new_goldenfile(&output_path).unwrap();
        let mut compiler = Compiler::new();
        let w = BufWriter::new(output_file);

        compiler
            .hoisting(false)
            .set_ir_writer(w)
            .add_source(source.as_str())
            .unwrap();

        #[cfg(feature = "constant-folding")]
        {
            let cse_output = output_path.with_extension("cse.ir");
            let output_file = mint.new_goldenfile(&cse_output).unwrap();
            let mut compiler = Compiler::new();
            let w = BufWriter::new(output_file);

            compiler
                .hoisting(false)
                .set_ir_writer(w)
                .add_source(source.as_str())
                .unwrap();

            let hoisting_output = output_path.with_extension("hoisting.ir");
            let output_file = mint.new_goldenfile(&hoisting_output).unwrap();
            let mut compiler = Compiler::new();
            let w = BufWriter::new(output_file);

            compiler
                .hoisting(true)
                .set_ir_writer(w)
                .add_source(source.as_str())
                .unwrap();
        }
    });
}
