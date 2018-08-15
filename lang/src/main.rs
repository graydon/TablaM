#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate lalrpop_util;

// Keep until lalrpop 0.16
lalrpop_mod!(pub tablam);
// pub mod tablam;

mod core;

use std::io::{self, Write, BufRead};
// use core::operators::*;

fn main() -> io::Result<()> {
    // let nums1:Vec<i64> = (0..100).into_iter().collect();
    // let nums2:Vec<i64> = (100..200).into_iter().collect();
    // let nums3:Vec<i64> = (100..200).into_iter().collect();
    // let bools1:Vec<bool> = (100..200).into_iter().map( |x| x > 0).collect();

    // let f1 = Column::from(nums1);
    // let f2 = Column::from(nums2);
    // let f3 = Column::from(nums3);
    // let f4 = Column::from(bools1);
    // let s1 = Column::from(vec!(1));

    // // println!("Sum:  {:?}", sum_pair(num1, num2));
    // println!("Row0:  {:?}", row(&vec!(f1.clone(), f2.clone()), 0));
    // println!("Col1:  {:?}", to_i64(&f1));
    // println!("Col2:  {:?}", to_i64(&f2));
    // println!("Col3:  {:?}", to_i64(&f3));
    // println!("Col1 == Col2:  {:?}", &f1 == &f2);
    // println!("Col2 == Col3:  {:?}", &f2 == &f3);
//  //   println!("Sum Dot:  {:?}", f2.clone() + f1.clone());
//  //   println!("Sum Scalar:  {:?}", f2.clone() + s1.clone());

    // let e = Exp::BinOp(
    //     BinOp::Plus,
    //     Exp::Scalar(Scalar::I32(3)).into(),
    //     Exp::Scalar(Scalar::I64(4)).into()
    //     );
    // println!("e: {:?}", e)

    // let mut buffer = String::new();
    loop {
        use tablam::*;

        // io::stdin().read_to_string(&mut buffer)?;
        let stdin = io::stdin();
        print!("> ");
        std::io::stdout().flush().unwrap();
        for line_r in stdin.lock().lines() {
            let line = line_r.unwrap();
            let parser = StatementParser::new();
            let eparser = ExprParser::new();
            match parser.parse(&line.clone()) {
                Ok(ast) => println!("ok(S): {:?}", ast),
                Err(err) => match eparser.parse(&line.clone()) {
                    Ok(ast) => println!("ok(E): {:?}", ast),
                    Err(err) => println!("error: {:?}", err),
                }
            }
            print!("> ");
            std::io::stdout().flush().unwrap();
        }
    }
}

use core::ast::Exp;
fn name(s: &str) -> Exp {
    Exp::Name(s.into())
}

fn stringlit(s: &str) -> Exp {
    use core::types::{Scalar, encode_str};
    Exp::Scalar(Scalar::UTF8(encode_str(s)))
}

use std::rc::Rc;
fn rc<T>(s: T) -> Rc<T> {
    Rc::new(s)
}

#[test]
fn tablam() {
    use tablam::*;
    // It is unclear why I have to import both ColumnExp and * ... ???
    use core::ast::{ColumnExp, *};

    assert!(
        ExprParser::new().parse("12").unwrap()
        == 12i32.into());

    assert!(
        ExprParser::new().parse("12i64").unwrap()
        == 12i64.into());

    assert!(
        ExprParser::new().parse("LLAMA").unwrap()
        == Exp::Constant("LLAMA".into())
        );

    assert!(
        ExprParser::new().parse("true").unwrap()
        == true.into()
        );

    assert!(
        ExprParser::new().parse("false").unwrap()
        == false.into()
        );

    assert!(
        ExprParser::new().parse(r#""hello""#).unwrap()
        == stringlit("hello")
        );

    assert!(ExprParser::new().parse(r#""hello"#).is_err());

    assert!(ExprParser::new().parse("1").unwrap()
            == 1i32.into());

    assert!(ExprParser::new().parse("1 + 2").unwrap()
            == Exp::BinOp(BinOp::Plus, rc(1.into()), rc(2.into())));

    assert!(StatementParser::new().parse("{1; 2; 3;}").unwrap()
            ==
            Stmt::Block(vec!(
                    Stmt::Exp(1.into()),
                    Stmt::Exp(2.into()),
                    Stmt::Exp(3.into()),
                    ))
            );

    assert!(ExprParser::new().parse("do 1; 2; 3 end").unwrap()
            ==
            Exp::Block(
                vec!(Stmt::Exp(1.into()), Stmt::Exp(2.into())),
                rc(3.into())
                )
            );

    assert!(ColumnLiteralParser::new().parse("[name:String; \"hello\" \"world\"]").unwrap()
            ==
            ColumnExp {
                name: Some("name".into()),
                ty: Some(Ty::Star("String".into())),
                es: vec!(stringlit("hello"), stringlit("world")),
            });

    assert!(ColumnLiteralParser::new().parse("[String; \"hello\" \"world\"]").unwrap()
            ==
            ColumnExp {
                name: None,
                ty: Some(Ty::Star("String".into())),
                es: vec!(stringlit("hello"), stringlit("world")),
            });

    assert!(ExprParser::new().parse("List[1 2]").unwrap()
            ==
            Exp::Container(
                Ty::Star("List".into()),
                ColumnExp { name: None, ty: None, es: vec!(1.into(), 2.into()) }
                ));

    assert!(StatementParser::new().parse("let x = [a:Ty; 1 2 3];").unwrap()
            ==
            Stmt::Let(LetKind::Imm, "x".into(), None, Exp::Column(ColumnExp {
                name: Some("a".into()),
                ty: Some(Ty::Star("Ty".into())),
                es: vec![1i32.into(), 2i32.into(), 3i32.into()],
            }).into()));

    assert!(RelationLiteralParser::new().parse(r#"[<id, name; 1 "1"; 2 "2" >]"#).unwrap()
            ==
            RelationExp {
                rel_type: RelType::Row,
                names: vec!("id".into(), "name".into()),
                data: vec!(
                    vec!(1.into(), stringlit("1")),
                    vec!(2.into(), stringlit("2")),
                    )
            });

    assert!(RelationLiteralParser::new().parse(r#"[| id= 1 2; name= "1" "2" |]"#).unwrap()
            ==
            RelationExp {
                rel_type: RelType::Col,
                names: vec!("id".into(), "name".into()),
                data: vec!(
                    vec!(1.into(), 2.into()),
                    vec!(stringlit("1"), stringlit("2")),
                    )
            });

    assert!(RangeLiteralParser::new().parse("(1..llama_world)").unwrap()
            == RangeExp {
                start: rc(1i32.into()),
                end: name("llama_world").into(),
            });

    assert!(StatementParser::new().parse("if true 3 else 4").unwrap()
            ==
            Stmt::IfElse(
                rc(true.into()),
                rc(3i32.into()),
                rc(4i32.into())));

    assert!(RowLiteralParser::new().parse("{hello=1, world=true}").unwrap()
            ==
            RowExp {
                names: Some(vec!["hello".into(), "world".into()]),
                types: vec!(None, None),
                es: vec![1i32.into(), true.into()],
            });

    assert!(RowLiteralParser::new().parse("{hello:Int=1, world=true}").unwrap()
            ==
            RowExp {
                names: Some(vec!["hello".into(), "world".into()]),
                types: vec!(Some(Ty::Star("Int".into())), None),
                es: vec![1i32.into(), true.into()],
            });

    assert!(RowLiteralParser::new().parse("{1, true}").unwrap()
            ==
            RowExp {
                names: None,
                types: vec!(None, None),
                es: vec![1i32.into(), true.into()],
            });

    assert!(StatementParser::new().parse("while true do hello; end").unwrap()
            ==
            Stmt::While(
                rc(true.into()),
                Stmt::Block(
                    vec![Stmt::Exp(name("hello"))],
                ).into()));

    assert!(ColumnLiteralParser::new().parse("[1 2 3 (4+5)]").unwrap()
            ==
            ColumnExp {
                name: None,
                ty: None,
                es: vec![
                    1i32.into(),
                    2i32.into(),
                    3i32.into(),
                    Exp::BinOp(
                        BinOp::Plus,
                        rc(4i32.into()),
                        rc(5i32.into())
                    ),
                ]
            });

    assert!(ExprParser::new().parse("()").unwrap() == Exp::Unit);

    assert!(TypeParser::new().parse("Int").unwrap()
            == Ty::Star("Int".into()));

    assert!(TypeParser::new().parse("Int -> String -> Float").unwrap()
            == Ty::Arrow(vec!(
                    Ty::Star("Int".into()),
                    Ty::Star("String".into()),
                    Ty::Star("Float".into())
                    )));

    assert!(ExprParser::new().parse("a ? # name == \"Max\" # your != mom").unwrap()
            == Exp::QueryFilter(
                Exp::Name("a".into()).into(),
                vec!(
                    FilterExp::RelOp(
                        RelOp::Equals,
                        "name".into(),
                        stringlit("Max").into()),
                    FilterExp::RelOp(
                        RelOp::NotEquals,
                        "your".into(),
                        name("mom").into()),
                    )));

    assert!(ExprParser::new().parse("[1 2 3] # 1").unwrap()
            ==
            Exp::QuerySelect(
                Exp::Column(ColumnExp { name: None, ty: None, es: vec!(1.into(), 2.into(), 3.into()) }).into(),
                rc(1.into())
                ));

    assert!(ExprParser::new().parse("[1 2 3] # [1 2]").unwrap()
            ==
            Exp::QuerySelect(
                Exp::Column(ColumnExp { name: None, ty: None, es: vec!(1.into(), 2.into(), 3.into()) }).into(),
                Exp::Column(ColumnExp { name: None, ty: None, es: vec!(1.into(), 2.into()) }).into(),
                ));

    assert!(ExprParser::new().parse("[1 2 3] # (1..2)").unwrap()
            ==
            Exp::QuerySelect(
                Exp::Column(ColumnExp { name: None, ty: None, es: vec!(1.into(), 2.into(), 3.into()) }).into(),
                Exp::Range(RangeExp { start: rc(1.into()), end: rc(2.into()) }).into(),
                ));

    assert!(FunctionDefinitionParser::new().parse("fun test[a:Int, b:String]: Int = do 1 + 2 end").unwrap()
            ==
            FunDef {
                name: "test".into(),
                params: vec!(
                    ("a".into(), Ty::Star("Int".into())),
                    ("b".into(), Ty::Star("String".into()))
                    ),
                ret_ty: Ty::Star("Int".into()),
                body: Exp::Block(
                    vec!(),
                    rc(Exp::BinOp(
                            BinOp::Plus,
                            rc(1.into()),
                            rc(2.into()))))
            });

    assert!(ExprParser::new().parse("print(a(b), c(d))").unwrap()
            ==
            Exp::Apply(
                name("print").into(),
                vec!(
                    Exp::Apply(name("a").into(), vec!(name("b"))),
                    Exp::Apply(name("c").into(), vec!(name("d"))),
                    )));

    assert!(ExprParser::new().parse("a | b | c").unwrap()
            ==
            Exp::Apply(
                name("c").into(),
                vec!(Exp::Apply(
                        name("b").into(),
                        vec!(name("a").into())))));

    assert!(ExprParser::new().parse("1 + 2 | print").unwrap()
            ==
            Exp::Apply(
                name("print").into(),
                vec!(Exp::BinOp(BinOp::Plus, rc(1.into()), rc(2.into())))));
}
