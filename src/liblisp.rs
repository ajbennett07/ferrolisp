use std::{collections::HashMap, ops::{Add, Mul, Sub, Div}};

use crate::{env::{Executable, ExecutionEnv}};

#[derive(Clone, Debug)]
pub enum Literal {
  String(String),
  Num(f64),
  Atom(String),
  List(Vec<Literal>),
  Err(String, i64),
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

type LispBuiltin = fn(&mut ExecutionEnv, Vec<Literal>) -> Literal;

pub fn builtins_table() -> HashMap<String, Executable> {
	HashMap::from([
		(format!("progn"), Executable::Builtin(0)),
		(format!("+"), Executable::Builtin(1)),
		(format!("-"), Executable::Builtin(2)),
		(format!("*"), Executable::Builtin(3)),
		(format!("/"), Executable::Builtin(4)),
		])
}

pub fn get_builtins() -> Vec<LispBuiltin> {
	vec![builtin_progn,
		builtin_add,
		builtin_sub,
		builtin_mul,
		builtin_div,
	]
}

pub fn builtin_progn(env: &mut ExecutionEnv, params: Vec<Literal>) -> Literal {
	params.into_iter().map(|val| env.evaluate(val)).last().or_else(|| Some(Literal::Err(format!("Form progn requires at least one argument"), -1))).unwrap()
}

pub fn builtin_add(env: &mut ExecutionEnv, params: Vec<Literal>) -> Literal {
	params.into_iter().fold(Literal::Num(0.0), |acc, x| acc + x)
}

pub fn builtin_sub(env: &mut ExecutionEnv, params: Vec<Literal>) -> Literal {
	if params.len() != 2 {
		return Literal::Err(format!("Input Error: Operation '-' requires 2 arguments"), -5)
	}
	let mut params = params.into_iter();
	params.next().unwrap() - params.next().unwrap()
}

pub fn builtin_div(env: &mut ExecutionEnv, params: Vec<Literal>) -> Literal {
	if params.len() != 2 {
		return Literal::Err(format!("Input Error: Operation '/' requires 2 arguments"), -5)
	}
	let mut params = params.into_iter();
	params.next().unwrap() / params.next().unwrap()
}

pub fn builtin_mul(env: &mut ExecutionEnv, params: Vec<Literal>) -> Literal {
	params.into_iter().fold(Literal::Num(0.0), |acc, x| acc * x)
}

pub fn builtin_fn(env: &mut ExecutionEnv, params: Vec<Literal>) -> Literal {
	Literal::Err(format!("TODO"), 0)
}