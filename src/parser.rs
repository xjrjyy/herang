extern crate nom;

use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{u32, multispace0, multispace1},
    sequence::{tuple, pair, preceded},
    branch::alt,
    multi::{separated_list0, separated_list1, many0},
    combinator::map, AsChar,
};

pub use crate::value::*;
pub use crate::ast::*;

pub fn value(input: &str) -> IResult<&str, Value> {
    let (input, value) = u32(input)?;
    Ok((input, Value::new(vec![value])))
}

pub fn is_alpha_or_underscore(c: char) -> bool {
    c.is_alpha() || c == '_'
}

pub fn identifier(input: &str) -> IResult<&str, String> {
    map(take_while1(is_alpha_or_underscore), |s: &str| s.to_string())(input)
}

// ast

fn value_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    map(value, |v| Box::new(ValueAST::new(v)) as Box<dyn AST>)(input)
}

fn var_ref_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, var_name) = identifier(input)?;
    let (input, _) = pair(multispace0, tag("["))(input)?;
    let (input, index) = expr_ast(input)?;
    let (input, _) = pair(multispace0, tag("]"))(input)?;

    Ok((input, Box::new(VarRefAST::new(var_name, index))))
}

fn var_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    map(identifier, |v| Box::new(VarAST::new(v)) as Box<dyn AST>)(input)
}

fn func_call_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, func_name) = identifier(input)?;

    let (input, _) = pair(multispace0, tag("("))(input)?;
    let (input, args) = separated_list0(
        tuple((multispace0, tag(","), multispace0)),
        expr_ast,
    )(input)?;
    let (input, _) = pair(multispace0, tag(")"))(input)?;

    Ok((input, Box::new(FuncCallAST::new(func_name, args)) as Box<dyn AST>))
}

pub fn value_expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    if input.starts_with("(") {
        let (input, _) = tag("(")(input)?;
        let (input, expr) = expr_ast(input)?;
        let (input, _) = pair(multispace0, tag(")"))(input)?;
        Ok((input, expr))
    } else {
        let (input, ast) = alt((
            func_call_ast,
            value_ast,
            var_ref_ast,
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

fn plus_minus_expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, left) = or_expr_ast(input)?;

    let result = preceded(
        multispace0,
        alt((tag("+"), tag("-")))
    )(input);

    if result.is_err() {
        return Ok((input, left));
    }

    let (input, sign) = result?;

    let expr_type = match sign {
        "+" => Some(ArithmeticExprType::Add),
        "-" => Some(ArithmeticExprType::Sub),
        _ => None,
    }.unwrap();

    let (input, right) = or_expr_ast(input)?;

    Ok((input, Box::new(ArithmeticExprAST::new(left, right, expr_type))))
}

fn mul_expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, mut expr) = separated_list1(
        tuple((multispace0, tag("*"), multispace0)),
        plus_minus_expr_ast,
    )(input)?;
    expr.reverse();
    let mut ast = expr.pop().unwrap();
    while let Some(right) = expr.pop() {
        ast = Box::new(ArithmeticExprAST::new(ast, right, ArithmeticExprType::Mul));
    }
    Ok((input, ast))
}

pub fn equality_expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, left) = mul_expr_ast(input)?;

    let result = preceded(
        multispace0,
        alt((tag("=="), tag("!="), tag("<="), tag(">="), tag("<"), tag(">")))
    )(input);

    if result.is_err() {
        return Ok((input, left));
    }

    let (input, sign) = result?;

    let expr_type = match sign {
        "==" => Some(EqualityExprType::Eq),
        "!=" => Some(EqualityExprType::Ne),
        "<" => Some(EqualityExprType::Lt),
        ">" => Some(EqualityExprType::Gt),
        "<=" => Some(EqualityExprType::Le),
        ">=" => Some(EqualityExprType::Ge),
        _ => None,
    }.unwrap();

    let (input, right) = mul_expr_ast(input)?;

    Ok((input, Box::new(EqualityExprAST::new(left, right, expr_type))))
}

pub fn expr_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    return alt((
        var_assign_ast,
        var_ref_assign_ast,
        equality_expr_ast,
    ))(input);
}

pub fn var_assign_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, var_name) = identifier(input)?;
    let (input, _) = pair(multispace0, tag("="))(input)?;

    let (input, value) = expr_ast(input)?;
    Ok((input, Box::new(VarAssignAST::new(var_name, value))))
}

pub fn var_ref_assign_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, var_name) = identifier(input)?;
    let (input, _) = pair(multispace0, tag("["))(input)?;
    let (input, index) = expr_ast(input)?;
    let (input, _) = pair(multispace0, tag("]"))(input)?;
    let (input, _) = pair(multispace0, tag("="))(input)?;

    let (input, value) = expr_ast(input)?;
    Ok((input, Box::new(VarRefAssignAST::new(var_name, index, value))))
}

fn var_def_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("def")(input)?;
    let (input, _) = multispace1(input)?;
    
    let (input, var_name) = identifier(input)?;
    Ok((input, Box::new(VarDefAST::new(var_name))))
}

fn func_def_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = tag("$")(input)?;
    
    let (input, func_name) = identifier(input)?;

    let (input, _) = tuple((multispace0, tag("("), multispace0))(input)?;
    let (input, args) = separated_list0(
        tuple((multispace0, tag(","), multispace0)),
        identifier,
    )(input)?;
    let (input, _) = pair(multispace0, tag(")"))(input)?;

    let (input, _) = pair(multispace0, tag("{"))(input)?;
    let (input, body) = block_ast(input)?;
    let (input, _) = pair(multispace0, tag("}"))(input)?;
    Ok((input, Box::new(FuncDefAST::new(func_name, args, body))))
}

pub fn for_in_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = pair(multispace0, tag("@"))(input)?;

    let (input, _) = pair(multispace0, tag("("))(input)?;
    let (input, var_name) = identifier(input)?;
    let (input, _) = pair(multispace0, tag(":"))(input)?;
    let (input, value) = expr_ast(input)?;
    let (input, _) = pair(multispace0, tag(")"))(input)?;

    let (input, _) = pair(multispace0, tag("{"))(input)?;
    let (input, body) = block_ast(input)?;
    let (input, _) = pair(multispace0, tag("}"))(input)?;
    Ok((input, Box::new(ForInAST::new(var_name, value, body))))
}

// ?=(a, b) {}
pub fn if_eq_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = pair(multispace0, tag("?"))(input)?;

    let (input, _) = pair(multispace0, tag("("))(input)?;
    let (input, value) = expr_ast(input)?;
    let (input, _) = pair(multispace0, tag(")"))(input)?;

    let (input, _) = pair(multispace0, tag("{"))(input)?;
    let (input, body) = block_ast(input)?;
    let (input, _) = pair(multispace0, tag("}"))(input)?;
    Ok((input, Box::new(IfAST::new(value, body))))
}

pub fn statement_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;

    let (input, statement) = alt((
        var_def_ast,
        expr_ast,
        func_def_ast,
        if_eq_ast,
        for_in_ast,
    ))(input)?;
    let statement = Box::new(StatementAST::new(statement));
    let (input, _) = pair(multispace0, tag(";"))(input)?;
    Ok((input, statement))
}

pub fn block_ast(input: &str) -> IResult<&str, Box<dyn AST>> {
    let (input, _) = multispace0(input)?;
    let (input, statements) = many0(
        statement_ast,
    )(input)?;
    Ok((input, Box::new(BlockAST::new(statements))))
}
