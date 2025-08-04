use std::{cell::RefCell, rc::Rc, collections::HashMap, ops::{Add, Div, Mul, Sub}};

use crate::{env::{Executable, ExecutionEnv}};

#[derive(Clone, Debug)]
pub enum Literal {
  String(String),
  Num(f64),
  Atom(String),
  List(Vec<Literal>),
  Err(String, i64),
}

impl Literal {
	pub fn print(&self) -> String{
		match self {
			Literal::String(s) => {
				s.to_string()
			}
			Literal::Num(n) => {
				format!("{}",n)
			}
			Literal::Atom(s) => {
				s.to_string()
			}
			Literal::Err(s, c) => {
				format!("{} with code {1}", s, c)
			}
			Literal::List(l) => {
				l.iter().map(|v| v.print()).fold(format!("("), |acc,s| acc+&s) + ")"
			}
		}
	}
}

impl std::fmt::Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { write!(f, "{}",self.print()); Ok(())}
}

impl Add for Literal {
	type Output = Literal;
	fn add(self, rhs: Self) -> Self::Output {
		if let Literal::Num(a) = self {
			if let Literal::Num(b) = rhs {
				return Literal::Num(a + b)
			}
			else {
				return Literal::Err(format!("Type Error: Addition between non-numeric types not support"), -4)
			}
		}
		else {
			return Literal::Err(format!("Type Error: Addition between non-numeric types not supported"), -4)
		}
	}
}

impl Mul for Literal {
	type Output = Literal;
	fn mul(self, rhs: Self) -> Self::Output {
		if let Literal::Num(a) = self {
			if let Literal::Num(b) = rhs {
				return Literal::Num(a * b)
			}
			else {
				return Literal::Err(format!("Type Error: Addition between non-numeric types not support"), -4)
			}
		}
		else {
			return Literal::Err(format!("Type Error: Addition between non-numeric types not supported"), -4)
		}
	}
}

impl Sub for Literal {
	type Output = Literal;
	fn sub(self, rhs: Self) -> Self::Output {
		if let Literal::Num(a) = self {
			if let Literal::Num(b) = rhs {
				return Literal::Num(a - b)
			}
			else {
				return Literal::Err(format!("Type Error: Addition between non-numeric types not support"), -4)
			}
		}
		else {
			return Literal::Err(format!("Type Error: Addition between non-numeric types not supported"), -4)
		}
	}
}

impl Div for Literal {
	type Output = Literal;
	fn div(self, rhs: Self) -> Self::Output {
		if let Literal::Num(a) = self {
			if let Literal::Num(b) = rhs {
				return Literal::Num(a / b)
			}
			else {
				return Literal::Err(format!("Type Error: Addition between non-numeric types not support"), -4)
			}
		}
		else {
			return Literal::Err(format!("Type Error: Addition between non-numeric types not supported"), -4)
		}
	}
}

type LispBuiltin = fn(Rc<RefCell<ExecutionEnv>>, Vec<Literal>) -> Literal;

pub fn builtins_table() -> HashMap<String, Executable> {
	HashMap::from([
		(format!("progn"), Executable::Builtin(0)),
		(format!("+"), Executable::Builtin(1)),
		(format!("-"), Executable::Builtin(2)),
		(format!("*"), Executable::Builtin(3)),
		(format!("/"), Executable::Builtin(4)),
		(format!("defn"), Executable::Builtin(5))
		])
}

pub fn get_builtins() -> Vec<LispBuiltin> {
	vec![builtin_progn,
		builtin_add,
		builtin_sub,
		builtin_mul,
		builtin_div,
		builtin_fn,
	]
}

pub fn builtin_progn(env: Rc<RefCell<ExecutionEnv>>, params: Vec<Literal>) -> Literal {
	params.into_iter().map(|val| ExecutionEnv::evaluate(env.clone(),val)).last().or_else(|| Some(Literal::Err(format!("Form progn requires at least one argument"), -1))).unwrap()
}

pub fn builtin_add(env: Rc<RefCell<ExecutionEnv>>, params: Vec<Literal>) -> Literal {
	params.into_iter().fold(Literal::Num(0.0), |acc, x| acc + ExecutionEnv::evaluate(env.clone(),x))
}

pub fn builtin_sub(env: Rc<RefCell<ExecutionEnv>>, params: Vec<Literal>) -> Literal {
	if params.len() != 2 {
		return Literal::Err(format!("Input Error: Operation '-' requires 2 arguments"), -5)
	}
	let mut params = params.into_iter().map(|v| ExecutionEnv::evaluate(env.clone(),v));
	params.next().unwrap() - params.next().unwrap()
}

pub fn builtin_div(env: Rc<RefCell<ExecutionEnv>>, params: Vec<Literal>) -> Literal {
	if params.len() != 2 {
		return Literal::Err(format!("Input Error: Operation '/' requires 2 arguments"), -5)
	}
	let mut params = params.into_iter().map(|v| ExecutionEnv::evaluate(env.clone(),v));
	params.next().unwrap() / params.next().unwrap()
}

pub fn builtin_mul(env: Rc<RefCell<ExecutionEnv>>, params: Vec<Literal>) -> Literal {
	params.into_iter().fold(Literal::Num(1.0), |acc, x| acc * ExecutionEnv::evaluate(env.clone(),x))
}

pub fn builtin_fn(env: Rc<RefCell<ExecutionEnv>>, params: Vec<Literal>) -> Literal {
	if params.len() != 3 {
		return Literal::Err(format!("Input Error: form 'fn' requires 3 arguments"), -5)
	}
	let mut param_entries = params.into_iter();
	let name = param_entries.next().unwrap();
	let args = param_entries.next().unwrap();
	let arg_names = match args {
		Literal::List(l) => {
			l.into_iter().map(|a| a.print()).collect()
		}
		_ => {
			return Literal::Err(format!("Input Error: form 'fn' requires list of argument names"), -5)
		}
	};
	let body = param_entries.next().unwrap();
	env.borrow_mut().add_function(name.print(), Executable::LispClosure(arg_names, body));
	name
}