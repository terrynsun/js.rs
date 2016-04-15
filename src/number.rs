use std::cell::RefCell;
use std::rc::Rc;

use french_press::ScopeManager;
use jsrs_common::backend::Backend;

use jsrs_common::ast::*;
use jsrs_common::ast::BinOp::*;
use jsrs_common::types::coerce::{AsBool,AsNumber};
use jsrs_common::types::js_var::{JsVar, JsType};
use jsrs_common::types::js_var::JsType::*;
use jsrs_common::js_error::{self, JsError};

macro_rules! b { ($e: expr) => { $e.as_bool() } }

macro_rules! n { ($e: expr) => { $e.as_number() } }
macro_rules! ni64 { ($e: expr) => { $e.as_number() as i64 } }
macro_rules! nu64 { ($e: expr) => { $e.as_number() as u64 } }
macro_rules! ni32 { ($e: expr) => { $e.as_number() as i32 } }
macro_rules! nu32 { ($e: expr) => { $e.as_number() as u32 } }


pub fn eval_binop(op: &BinOp, val1: JsVar, val2: JsVar,
                  state: Rc<RefCell<ScopeManager>>) -> js_error::Result<JsType> {
    let v = match *op {
        And => JsBool(b!(val1) && b!(val2)),
        Or  => JsBool(b!(val1) || b!(val2)),

        Ge  => JsBool(b!(val1) >= b!(val2)),
        Gt  => JsBool(b!(val1) >  b!(val2)),
        Le  => JsBool(b!(val1) <= b!(val2)),
        Lt  => JsBool(b!(val1) <  b!(val2)),

        // TODO: equality?
        Neq => JsBool(b!(val1) != b!(val2)),
        Eql => JsBool(b!(val1) == b!(val2)),

        EqlStrict => {
            println!("{}", val1.t);
            let val1_ptr = match state.borrow_mut().load(&val1.binding) {
                Ok((_, ptr)) => ptr,
                Err(e) => {
                    println!("{}", e);
                    return Err(JsError::ReferenceError(format!("{:?} is not defined", val1.binding)));
                }
            };
            let val2_ptr = match state.borrow_mut().load(&val2.binding) {
                Ok((_, ptr)) => ptr,
                Err(_) => return Err(JsError::ReferenceError(format!("{:?} is not defined", val1.binding))),
            };
            println!("{:#?} {:#?}", val1_ptr, val2_ptr);
            JsBool(val1.t == val2.t)
        }
        NeqStrict => JsBool(val1.t != val2.t),

        BitOr  => JsNum((ni64!(val1) | ni64!(val2)) as f64),
        BitXor => JsNum((ni64!(val1) ^ ni64!(val2)) as f64),
        BitAnd => JsNum((ni64!(val1) & ni64!(val2)) as f64),

        // TODO: Rust panics on shift overflow, and I don't want this.
        ShiftLeft          => JsNum(0.0),
        ShiftRight         => JsNum(0.0),
        ShiftRightUnsigned => JsNum(0.0),
        //ShiftLeft          => JsNum((ni32!(val1) << ni32!(val2)) as f64),
        //ShiftRight         => JsNum((ni32!(val1) >> ni32!(val2)) as f64),
        //ShiftRightUnsigned => JsNum((nu32!(val1) >> ni32!(val2)) as f64),

        Minus => JsNum(n!(val1) - n!(val2)),
        Plus  => JsNum(n!(val1) + n!(val2)),
        Slash => JsNum(n!(val1) / n!(val2)),
        Star  => JsNum(n!(val1) * n!(val2)),
        Mod   => JsNum(n!(val1) % n!(val2)),
        Exponent   => JsNum(n!(val1) % n!(val2)),
    };
    Ok(v)
}
