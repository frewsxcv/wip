use crate::literals::LiteralInteger;
use crate::node::Node;
use crate::utils::CommaSeparated;
use crate::Ending;
use crate::Expression;
use crate::Statement;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ForeachStatement {
    pub foreach: Span,                      // `foreach`
    pub left_parenthesis: Span,             // `(`
    pub iterator: ForeachStatementIterator, // `( *expression* as & $var => $value )`
    pub right_parenthesis: Span,            // `)`
    pub body: ForeachStatementBody,         // `{ ... }`
}

impl Node for ForeachStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.iterator, &mut self.body]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ForeachStatementIterator {
    // `*expression* as &$var`
    Value {
        expression: Expression,  // `*expression*`
        r#as: Span,              // `as`
        ampersand: Option<Span>, // `&`
        value: Expression,       // `$var`
    },
    // `*expression* as &$key => $value`
    KeyAndValue {
        expression: Expression,  // `*expression*`
        r#as: Span,              // `as`
        ampersand: Option<Span>, // `&`
        key: Expression,         // `$key`
        double_arrow: Span,      // `=>`
        value: Expression,       // `$value`
    },
}

impl Node for ForeachStatementIterator {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            ForeachStatementIterator::Value {
                expression, value, ..
            } => {
                vec![expression, value]
            }
            ForeachStatementIterator::KeyAndValue {
                expression,
                key,
                value,
                ..
            } => vec![expression, key, value],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ForeachStatementBody {
    Statement {
        statement: Box<Statement>,
    },
    Block {
        colon: Span,                // `:`
        statements: Vec<Statement>, // `*statements*`
        endforeach: Span,           // `endforeach`
        ending: Ending,             // `;` or `?>`
    },
}

impl Node for ForeachStatementBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            ForeachStatementBody::Statement { statement } => vec![statement.as_mut()],
            ForeachStatementBody::Block { statements, .. } => {
                statements.iter_mut().map(|s| s as &mut dyn Node).collect()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ForStatement {
    pub r#for: Span,                    // `for`
    pub left_parenthesis: Span,         // `(`
    pub iterator: ForStatementIterator, // `*expression*; *expression*; *expression*`
    pub right_parenthesis: Span,        // `)`
    pub body: ForStatementBody,         // `{ ... }`
}

impl Node for ForStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.iterator, &mut self.body]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ForStatementIterator {
    pub initializations: CommaSeparated<Expression>, // `*expression*;`
    pub initializations_semicolon: Span,             // `;`
    pub conditions: CommaSeparated<Expression>,      // `*expression*;`
    pub conditions_semicolon: Span,                  // `;`
    pub r#loop: CommaSeparated<Expression>,          // `*expression*`
}

impl Node for ForStatementIterator {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children = vec![];
        children.extend(
            self.initializations
                .inner
                .iter_mut()
                .map(|x| x as &mut dyn Node),
        );
        children.extend(self.conditions.inner.iter_mut().map(|x| x as &mut dyn Node));
        children.extend(self.r#loop.inner.iter_mut().map(|x| x as &mut dyn Node));
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ForStatementBody {
    Statement {
        statement: Box<Statement>,
    },
    Block {
        colon: Span,                // `:`
        statements: Vec<Statement>, // `*statements*`
        endfor: Span,               // `endfor`
        ending: Ending,             // `;` or `?>`
    },
}

impl Node for ForStatementBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            ForStatementBody::Statement { statement } => vec![statement.as_mut()],
            ForStatementBody::Block { statements, .. } => {
                statements.iter_mut().map(|x| x as &mut dyn Node).collect()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DoWhileStatement {
    pub r#do: Span,              // `do`
    pub body: Box<Statement>,    // `{ ... }`
    pub r#while: Span,           // `while`
    pub left_parenthesis: Span,  // `(`
    pub condition: Expression,   // `( *expression* )`
    pub right_parenthesis: Span, // `)`
    pub semicolon: Span,         // `;`
}

impl Node for DoWhileStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.body.as_mut(), &mut self.condition]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct WhileStatement {
    pub r#while: Span,            // `while`
    pub left_parenthesis: Span,   // `(`
    pub condition: Expression,    // *expression*
    pub right_parenthesis: Span,  // `)`
    pub body: WhileStatementBody, // `{ ... }`
}

impl Node for WhileStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.condition, &mut self.body]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum WhileStatementBody {
    Statement {
        statement: Box<Statement>,
    },
    Block {
        colon: Span,                // `:`
        statements: Vec<Statement>, // `*statements*`
        endwhile: Span,             // `endwhile`
        ending: Ending,             // `;` or `?>`
    },
}

impl Node for WhileStatementBody {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            WhileStatementBody::Statement { statement } => vec![statement.as_mut()],
            WhileStatementBody::Block { statements, .. } => {
                statements.iter_mut().map(|s| s as &mut dyn Node).collect()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Level {
    Literal(LiteralInteger),
    Parenthesized {
        left_parenthesis: Span, // `(`
        level: Box<Level>,
        right_parenthesis: Span, // `)`
    },
}

impl Node for Level {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Level::Literal(literal) => vec![literal],
            Level::Parenthesized { level, .. } => level.children(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BreakStatement {
    pub r#break: Span,        // `break`
    pub level: Option<Level>, // `3`
    pub ending: Ending,       // `;` or `?>`
}

impl Node for BreakStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match &mut self.level {
            Some(level) => vec![level],
            None => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ContinueStatement {
    pub r#continue: Span,     // `continue`
    pub level: Option<Level>, // `2`
    pub ending: Ending,       // `;` or `?>`
}

impl Node for ContinueStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match &mut self.level {
            Some(level) => vec![level],
            None => vec![],
        }
    }
}
