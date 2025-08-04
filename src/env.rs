use crate::liblisp::Literal;
use crate::liblisp::{self, builtins_table};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug,Clone)]
pub enum Executable {
	//BytecodeObject,
	Builtin(i64),
	LispClosure(Vec<String>, Literal),
}

impl Executable {
	pub fn evaluate_with(&self,  env: Rc<RefCell<ExecutionEnv>>, params: Vec<Literal>) -> Literal{
		match self {
			Executable::Builtin(num) => {
				liblisp::get_builtins()[*num as usize](env, params)
			},
			Executable::LispClosure(args, body) => {
				let defs = args.into_iter().zip(params).map(|(s,l)| (s.to_string(),l)).collect();
				{
				env.borrow_mut().add_binds(defs);
				}
				ExecutionEnv::evaluate(env.clone(), body.clone())			
			}
			_ => {
				Literal::Atom(format!("null"))
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct ExecutionEnv {
	permission: i64,
	parent: Option<Rc<RefCell<ExecutionEnv>>>,
	defined_vals: HashMap<String,Literal>,
	defined_funcs: HashMap<String,Executable>
}

impl ExecutionEnv {
	pub fn new(permnum: i64, par: Rc<RefCell<ExecutionEnv>>, vals: HashMap<String, Literal>, funcs: HashMap<String, Executable>) -> Self {
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
	pub fn add_function(&mut self,func_name: String,func: Executable) {
		self.defined_funcs.insert(func_name, func);
	}
	pub fn get_parent(&self) -> ExecutionEnv {
		self.parent.as_ref().unwrap().deref().clone().into_inner()
	}
	pub fn add_binds(&mut self, binds: HashMap<String, Literal>) {
		self.defined_vals.extend(binds);
	}
	pub fn lookup_function(&self, val: &liblisp::Literal) -> Result<Executable, Literal> {
		match val {
			Literal::Atom(s) => {
				match self.defined_funcs.get(s) {
					Some(e) => {
						Ok(e.clone())
					}
					None => {
						match self.parent.clone() {
							Some(_) => {
								self.get_parent().lookup_function(val)
							}
							None => {
								Err(Literal::Err(format!("{:?} is not a recognized function name", val), 3))
							}
						}
					}
				}
			},
			_ => {
				Err(Literal::Err(format!("{} is not a recognized function name", val.print()), 3))
			}
		}
	}
	pub fn evaluate(env: Rc<RefCell<ExecutionEnv>>, val: liblisp::Literal) -> liblisp::Literal {
		match val {
			Literal::String(s) => {
				Literal::String(s)
			},
			Literal::Num(_) => {
				val
			},
			Literal::Atom(s) => {
				env.deref().clone().into_inner().defined_vals[&s].clone()
			}
			//For this to work:
			//function definition form /// closure form MUST return defined name of function
			Literal::List(l) => {
				let mut list = l.into_iter();
				let first_entry = list.next();
				let func: Result<Executable, Literal>;
				match first_entry {
					Some(mut name) => {
						match &name {
							Literal::List(_) => {
								name = ExecutionEnv::evaluate(env.clone(), name);
							}
							_ => {

							}
						}
						let ref_to_env = env.deref().clone();
						func = ref_to_env.into_inner().lookup_function(&name);
						match func {
							Ok(exec) => {
								exec.evaluate_with(env.clone(), list.collect())
							}
							Err(e) => {
								e
							}
						}
					}
					None => {
						Literal::Atom(format!("null"))
					}
				}
			}
			Literal::Err(s,c ) => {
				Literal::Err(s, c)
			}
		}
	}
}
