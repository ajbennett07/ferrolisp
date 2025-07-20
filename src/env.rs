use crate::liblisp::Literal;
use crate::liblisp::{self, builtins_table};
use std::collections::HashMap;

#[derive(Debug,Clone,Copy)]
pub enum Executable {
	BytecodeObject,
	Builtin(i64),
	LispClosure,
}

impl Executable {
	pub fn evaluate_with(&self,  env: &mut ExecutionEnv, params: Vec<Literal>) -> Literal{
		match self {
			Executable::Builtin(num) => {
				liblisp::get_builtins()[*num as usize](env, params)
			},
			Executable::LispClosure => {
			    Literal::Err(format!("Feature not support just yet"), 0)
			}
			_ => {
				Literal::Atom(format!("null"))
			}
		}
	}
}

pub struct ExecutionEnv {
	permission: i64,
	parent: Option<Box<ExecutionEnv>>,
	defined_vals: HashMap<String,Literal>,
	defined_funcs: HashMap<String,Executable>
}

impl ExecutionEnv {
	pub fn new(permnum: i64, par: Box<ExecutionEnv>, vals: HashMap<String, Literal>, funcs: HashMap<String, Executable>) -> Self {
		Self {
			permission: permnum,
			parent: Some(par),
			defined_vals: vals,
			defined_funcs: funcs
		}
	}
	pub fn root() -> Self {
		Self {
			permission: 0,
			parent: None,
			defined_vals: HashMap::new(),
			defined_funcs: builtins_table()
		}
	}
	pub fn lookup_function(&self, val: &liblisp::Literal) -> Result<Executable, Literal> {
		match val {
			Literal::Atom(s) => {
				Ok(self.defined_funcs[s])
			},
			_ => {
				Err(Literal::Err(format!("{:?} is not a recognized function name", val), 3))
			}
		}
	}
	pub fn evaluate(&mut self, val: liblisp::Literal) -> liblisp::Literal {
		match val {
			Literal::String(s) => {
				Literal::String(s)
			},
			Literal::Num(n) => {
				val
			},
			Literal::Atom(s) => {
				self.defined_vals[&s].clone()
			}
			//For this to work:
			//function definition form /// closure form MUST return defined name of function
			Literal::List(l) => {
				let mut list = l.into_iter();
				let first_entry = list.next();
				let func: Result<Executable, Literal>;
				match first_entry {
					Some(lit) => {
						func = self.lookup_function(&lit);
					}
					None => {
						return Literal::Atom(format!("null"))
					}
				}
				match func {
					Ok(e) => {
						return e.evaluate_with(self, list.collect())
					},
					Err(e) => {
						return e
					}
				}
				
			}
			Literal::Err(s,c ) => {
				Literal::Err(s, c)
			}
		}
	}
}
