use crate::lexer;
use lexer::Token;
use std::process;


#[derive(Debug)]
pub enum Node {
    Ident(String, lexer::SourceLocation),
    Str(String, lexer::SourceLocation),
    Int(usize, lexer::SourceLocation),

    Nop,

    Comparison {
        rexpr: (Box<Node>, lexer::SourceLocation),
        lexpr: (Box<Node>, lexer::SourceLocation),
        op: (lexer::Comparison, lexer::SourceLocation),
    },
    BinaryExpr {
        rexpr: (Box<Node>, lexer::SourceLocation),
        lexpr: (Box<Node>, lexer::SourceLocation),
        op: (lexer::Operator, lexer::SourceLocation),
    },
    Function {
        name: (String, lexer::SourceLocation),
        parameters: (Vec<(String, lexer::Type, lexer::SourceLocation)>, lexer::SourceLocation),
        return_type: (lexer::Type, lexer::SourceLocation),
        body: (Vec<Node>, lexer::SourceLocation)
    },
    Return {
        expr: (Box<Node>, lexer::SourceLocation),
    },
    Call {
        name: (String, lexer::SourceLocation),
        parameters: (Vec<Node>, lexer::SourceLocation),
    },
    Variable {
        name: (String, lexer::SourceLocation),
        var_type: (lexer::Type, lexer::SourceLocation),
        expr: (Box<Node>, lexer::SourceLocation),
    },
    If {
        test: (Box<Node>, lexer::SourceLocation),
        then_body: (Vec<Node>, lexer::SourceLocation),
        else_body: (Vec<Node>, lexer::SourceLocation),
    }
}

fn expected(expected: &str, got: &Token, location: &lexer::SourceLocation) {
    println!("{}:{}: [Expected a {} but got {:?}", location.0, location.1, expected, got);
    process::exit(1);
}

fn parse_expr(expr: &Vec<(Token, lexer::SourceLocation)>) -> Node {
    if expr.len() == 1 {
        // value
        match &expr[0].0 {
            Token::Int(integer) => {
                return Node::Int(integer.clone(), expr[0].1.clone());
            },
            Token::Str(string) => {
                return Node::Str(string.clone(), expr[0].1.clone());
            },
            Token::Ident(ident) => {
                return Node::Ident(ident.clone(), expr[0].1.clone());
            },
            _ => {},
        }
    } else if let (Token::Ident(ident), ident_location) = &expr[0] {
        // Function call
        let mut index = 1;
        if &expr[index].0 != &Token::OpenParen {
            expected("'('", &expr[index].0, &expr[index].1);
        }
        index += 1;
        let mut parameter: Vec<(Token, lexer::SourceLocation)> = Vec::new();
        let parameters_location = expr[index].1.clone();
        let mut parameters: Vec<Node> = Vec::new();
        let mut indentation = 0;
        while (&expr[index].0 != &Token::CloseParen && indentation == 0) || indentation != 0 {
            if &expr[index].0 == &Token::OpenParen {
                indentation += 1;
            } else if &expr[index].0 == &Token::CloseParen {
                indentation -= 1;
            }
            if &expr[index].0 == &Token::Comma {
                parameters.push(parse_expr(&parameter));
                parameter = Vec::new();
            } else {
                parameter.push(expr[index].clone());
            }
            index += 1;
        }
        if parameter.len() != 0 {
            parameters.push(parse_expr(&parameter));
        }
        return Node::Call {
            name: (ident.clone(), ident_location.clone()),
            parameters: (parameters, parameters_location),
        };
    } else {
        // Binary expression
        let mut lexpr: (Vec<(Token, lexer::SourceLocation)>, lexer::SourceLocation) = (Vec::new(), (0, 0));
        let mut rexpr: (Vec<(Token, lexer::SourceLocation)>, lexer::SourceLocation) = (Vec::new(), (0, 0));
        let mut operator = (lexer::Operator::And, (0, 0));
        let mut index = 0;

        // iterate trough right expression
        let mut operator_found = false;
        while operator_found == false {
            match &expr[index].0 {
                Token::Operator(op) => {
                    operator = (op.clone(), expr[index].1);
                    operator_found = true;
                },
                _ => {
                    rexpr.0.push(expr[index].clone());
                },
            }
            index += 1;
        }

        // iterate trough left expression
        while index < expr.len() {
            lexpr.0.push(expr[index].clone());
            index += 1;
        }
        return Node::BinaryExpr {
            lexpr: (Box::new(parse_expr(&lexpr.0)), lexpr.1),
            rexpr: (Box::new(parse_expr(&rexpr.0)), rexpr.1),
            op: operator,
        };
    }
    return Node::Int(0, (0, 0));
}

fn parse_condition(condition: &Vec<(Token, lexer::SourceLocation)>) -> Node {
    // NOTE: Implement multiple conditions later
    let mut lexpr: Vec<(Token, lexer::SourceLocation)> = Vec::new();
    let lexpr_location = condition[0].1.clone();
    let mut rexpr: Vec<(Token, lexer::SourceLocation)> = Vec::new();
    let mut rexpr_location = (0, 0);
    let mut index = 0;
    let mut operator = lexer::Comparison::Nop;
    let mut op_location = (0, 0);
    while operator == lexer::Comparison::Nop {
        match condition[index].0 {
            Token::Comparison(op) => {
                operator = op;
                op_location = condition[index].1.clone();
            },
            _ => {
                lexpr.push(condition[index].clone());
            },
        }
        index += 1;
    }
    rexpr_location = condition[index].1.clone();
    while index < condition.len() {
        rexpr.push(condition[index].clone());
        index += 1;
    }
    return Node::Comparison {
        rexpr: (Box::new(parse_expr(&rexpr)), rexpr_location),
        lexpr: (Box::new(parse_expr(&lexpr)), lexpr_location),
        op: (operator, op_location),
    };
}

pub fn build_ast(tokens: Vec<(lexer::Token, lexer::SourceLocation)>) -> Vec<Node> {
    let mut index = 0;
    let mut ast: Vec<Node> = Vec::new();
    while index < tokens.len() {
        match &tokens[index].0 {
            Token::Keyword(lexer::Keyword::If) => {
                index += 1;
                let mut condition: Vec<(Token, lexer::SourceLocation)> = Vec::new();
                let condition_location = tokens[index].1;
                while &tokens[index].0 != &Token::OpenBrace {
                    condition.push(tokens[index].clone());
                    index += 1;
                }
                index += 1;

                let mut body: Vec<(Token, lexer::SourceLocation)> = Vec::new();
                let mut indentation = 0;
                while (&tokens[index].0 != &Token::CloseBrace && indentation == 0) || indentation != 0 {
                    if &tokens[index].0 == &Token::OpenBrace {
                        indentation += 1;
                    } else if &tokens[index].0 == &Token::CloseBrace {
                        indentation -= 1;
                    }
                    body.push(tokens[index].clone());
                    index += 1;
                }
                if &tokens[index].0 != &Token::Keyword(lexer::Keyword::Else) {
                    ast.push(Node::If {
                        test: (Box::new(parse_condition(&condition)), condition_location),
                        then_body: (build_ast(body), (0, 0)),
                        else_body: (vec![Node::Nop], (0, 0)),
                    });
                }
            },
            Token::Keyword(lexer::Keyword::Let) => {
                index += 1;
                let ident = match &tokens[index].0 {
                    Token::Ident(identifier) => (identifier.clone(), tokens[index].1.clone()),
                    _ => {
                        expected("identifier", &tokens[index].0, &tokens[index].1);
                        process::exit(1);
                    },
                };
                index += 1;
                if &tokens[index].0 != &Token::Colon {
                    expected("':'", &tokens[index].0, &tokens[index].1);
                }
                index += 1;
                let var_type = match &tokens[index].0 {
                    Token::Type(typeid) => (typeid.clone(), tokens[index].1.clone()),
                    _ => {
                        expected("identifier", &tokens[index].0, &tokens[index].1);
                        process::exit(1);
                    },
                };
                index += 1;
                if &tokens[index].0 != &Token::Equal {
                    expected("'='", &tokens[index].0, &tokens[index].1);
                }
                index += 1;
                let mut expression: Vec<(Token, lexer::SourceLocation)> = Vec::new();
                let expr_location = tokens[index].1.clone();
                while &tokens[index].0 != &Token::Newline {
                    expression.push(tokens[index].clone());
                    index += 1;
                }
                ast.push(Node::Variable {
                    name: ident,
                    var_type,
                    expr: (Box::new(parse_expr(&expression)), expr_location),
                });
            },
            Token::Keyword(lexer::Keyword::Return) => {
                index += 1;
                let mut expression: Vec<(Token, lexer::SourceLocation)> = Vec::new();
                let expr_location = tokens[index].1.clone();
                while &tokens[index].0 != &Token::Newline {
                    expression.push(tokens[index].clone());
                    index += 1;
                }
                ast.push(Node::Return {
                    expr: (Box::new(parse_expr(&expression)), expr_location),
                });
            },
            Token::Keyword(lexer::Keyword::Function) => {
                index += 1;
                let mut function_name: (String, lexer::SourceLocation) = (String::new(), (0, 0));
                let mut function_type: (lexer::Type, lexer::SourceLocation) = (lexer::Type::Void, (0, 0));
                let mut function_parameters: (Vec<(String, lexer::Type, lexer::SourceLocation)>, lexer::SourceLocation)
                    = (Vec::new(), (0, 0));
                match &tokens[index].0 {
                    Token::Ident(ident) => {
                        function_name = (ident.clone(), tokens[index].1.clone());
                    },
                    _ => {
                        expected("identifier", &tokens[index].0, &tokens[index].1);
                    },
                }
                index += 1;
                if &tokens[index].0 != &Token::OpenParen {
                    expected("'('", &tokens[index].0, &tokens[index].1);
                }
                index += 1;
                function_parameters.1 = tokens[index].1.clone();
                while &tokens[index].0 != &Token::CloseParen {
                    match &tokens[index].0 {
                        Token::Type(typeid) => {
                            index += 1;
                            match &tokens[index].0 {
                                Token::Ident(ident) => {
                                    function_parameters.0.push((ident.clone(), typeid.clone(), tokens[index].1.clone()));
                                    if &tokens[index + 1].0 == &Token::Comma {
                                        index += 1;
                                    }
                                },
                                _ => {
                                    expected("type", &tokens[index].0, &tokens[index].1);
                                },
                            }
                        },
                        _ => {
                            expected("type", &tokens[index].0, &tokens[index].1);
                        },
                    }
                    index += 1;
                }
                index += 1;
                if &tokens[index].0 != &Token::Colon {
                    expected("':'", &tokens[index].0, &tokens[index].1);
                }
                index += 1;
                match &tokens[index].0 {
                    Token::Type(typeid) => {
                        function_type = (typeid.clone(), tokens[index].1.clone());
                    },
                    _ => {
                        expected("type", &tokens[index].0, &tokens[index].1);
                    },
                }
                index += 1;
                if &tokens[index].0 != &Token::OpenBrace {
                    expected("'{{'", &tokens[index].0, &tokens[index].1);
                }
                index += 1;
                let mut body_tokens: Vec<(Token, lexer::SourceLocation)> = Vec::new();
                let body_location = tokens[index].1.clone();
                let mut indentation = 0;
                while (&tokens[index].0 != &Token::CloseBrace && indentation == 0) || indentation != 0 {
                    if &tokens[index].0 == &Token::OpenBrace {
                        indentation += 1;
                    } else if &tokens[index].0 == &Token::CloseBrace {
                        indentation -= 1;
                    }
                    body_tokens.push(tokens[index].clone());
                    index += 1;
                }
                ast.push(Node::Function {
                    body: (build_ast(body_tokens), body_location),
                    name: function_name,
                    parameters: function_parameters,
                    return_type: function_type,
                });
            },
            _ => {
                let mut expr: Vec<(Token , lexer::SourceLocation)> = Vec::new();
                while &tokens[index].0 != &Token::Newline {
                    expr.push(tokens[index].clone());
                    index += 1;
                }
                if expr.len() != 0 {
                    ast.push(parse_expr(&expr));
                }
            },
        }
        index += 1;
    }
    return ast;
}


