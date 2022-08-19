extern crate nom;
use std::vec;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{u8, multispace0, alpha1},
    sequence::tuple,
    branch::alt,
    multi::{separated_list0, separated_list1, many0},
    combinator::map,
};

pub use crate::value::*;
pub use crate::ast::*;

pub fn value(input: &str) -> IResult<&str, Value> {
    let (input, value) = u8(input)?;
    Ok((input, Value::new(vec![value])))
}

pub fn var_name(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(input)
}

// ast

fn value_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    map(value, |v| Box::new(ValueAST::new(v)) as Box<dyn AST>)(input)
}

fn var_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    map(var_name, |v| Box::new(VarAST::new(v)) as Box<dyn AST>)(input)
}

fn func_call_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, func_name) = var_name(input)?;

    let (input, _) = tuple((multispace0, tag("(")))(input)?;
    let (input, args) = separated_list0(
        tuple((
            multispace0,
            tag(","),
            multispace0,
        )),
        expr_ast,
    )(input)?;
    let (input, _) = tuple((multispace0, tag(")")))(input)?;

    Ok((input, Box::new(FuncCallAST::new(func_name, args)) as Box<dyn AST>))
}

pub fn value_expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    if input.starts_with("(") {
        let (input, _) = tag("(")(input)?;
        let (input, expr) = expr_ast(input)?;
        let (input, _) = tuple((multispace0, tag(")")))(input)?;
        Ok((input, expr))
    } else {
        let (input, ast) = alt((
            func_call_ast,
            value_ast,
            var_ast,
        ))(input)?;
        Ok((input, Box::new(ExprAST::new(ast))))
    }
}

fn or_expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, mut expr) = separated_list1(
        tuple((multispace0, tag("|"), multispace0)),
        value_expr_ast,
    )(input)?;
    expr.reverse();
    let mut ast = expr.pop().unwrap();
    while let Some(right) = expr.pop() {
        ast = Box::new(OrExprAST::new(ast, right));
    }
    Ok((input, ast))
}

pub fn expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    return alt((
        var_ref_assign_ast,
        var_assign_ast,
        or_expr_ast,
    ))(input);
}

pub fn var_assign_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, var_name) = var_name(input)?;
    let (input, _) = tuple((multispace0, tag("="), multispace0))(input)?;

    let (input, value) = expr_ast(input)?;
    Ok((input, Box::new(VarAssignAST::new(var_name, value))))
}

pub fn var_ref_assign_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, var_name) = var_name(input)?;
    let (input, _) = tuple((multispace0, tag("[")))(input)?;
    let (input, index) = expr_ast(input)?;
    let (input, _) = tuple((multispace0, tag("]")))(input)?;
    let (input, _) = tuple((multispace0, tag("="), multispace0))(input)?;

    let (input, value) = expr_ast(input)?;
    Ok((input, Box::new(VarRefAssignAST::new(var_name, index, value))))
}

fn func_def_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = tag("$")(input)?;
    
    let (input, func_name) = var_name(input)?;

    let (input, _) = tuple((multispace0, tag("("), multispace0))(input)?;
    let (input, args) = separated_list0(
        tuple((
            multispace0,
            tag(","),
            multispace0,
        )),
        var_name
    )(input)?;
    let (input, _) = tuple((multispace0, tag(")")))(input)?;

    let (input, _) = tuple((multispace0, tag("{")))(input)?;
    let (input, body) = block_ast(input)?;
    let (input, _) = tuple((multispace0, tag("}")))(input)?;
    Ok((input, Box::new(FuncDefAST::new(func_name, args, body))))
}

pub fn statement_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, statement) = alt((
        expr_ast,
        func_def_ast,
    ))(input)?;
    let (input, _) = tuple((multispace0, tag(";")))(input)?;
    Ok((input, statement))
}

pub fn block_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;
    let (input, statements) = many0(
        statement_ast,
    )(input)?;
    Ok((input, Box::new(BlockAST::new(statements))))
}
