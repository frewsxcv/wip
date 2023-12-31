use crate::expect_token;
use crate::error::ParseResult;
use crate::internal::attributes;
use crate::internal::constants;
use crate::internal::functions::method;
use crate::internal::functions::Method;
use crate::internal::functions::MethodType;
use crate::internal::identifiers;
use crate::internal::modifiers;
use crate::internal::properties;
use crate::internal::utils;
use crate::state::State;
use crate::peek_token;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::modifiers::VisibilityModifier;
use pxp_ast::traits::TraitBody;
use pxp_ast::traits::TraitMember;
use pxp_ast::traits::TraitStatement;
use pxp_ast::traits::TraitUsage;
use pxp_ast::traits::TraitUsageAdaptation;
use pxp_ast::Statement;
use pxp_token::Token;
use pxp_token::TokenKind;

pub fn usage(state: &mut State) -> ParseResult<TraitUsage> {
    let span = utils::skip(state, TokenKind::Use)?;

    let mut traits = Vec::new();

    while state.stream.current().kind != TokenKind::SemiColon
        && state.stream.current().kind != TokenKind::LeftBrace
    {
        let t = identifiers::full_type_name(state)?;
        traits.push(t);

        if state.stream.current().kind == TokenKind::Comma {
            if state.stream.peek().kind == TokenKind::SemiColon {
                // will fail with unexpected token `,`
                // as `use` doesn't allow for trailing commas.
                utils::skip_semicolon(state)?;
            } else if state.stream.peek().kind == TokenKind::LeftBrace {
                // will fail with unexpected token `{`
                // as `use` doesn't allow for trailing commas.
                utils::skip_left_brace(state)?;
            } else {
                state.stream.next();
            }
        } else {
            break;
        }
    }

    let mut adaptations = Vec::new();
    if state.stream.current().kind == TokenKind::LeftBrace {
        utils::skip_left_brace(state)?;

        while state.stream.current().kind != TokenKind::RightBrace {
            let (r#trait, method): (Option<SimpleIdentifier>, SimpleIdentifier) =
                match state.stream.peek().kind {
                    TokenKind::DoubleColon => {
                        let r#trait = identifiers::full_type_name(state)?;
                        state.stream.next();
                        let method = identifiers::identifier(state)?;
                        (Some(r#trait), method)
                    }
                    _ => (None, identifiers::identifier(state)?),
                };

            expect_token!([
                    TokenKind::As => {
                        match state.stream.current() {
                            Token { kind: TokenKind::Public | TokenKind::Protected | TokenKind::Private, span, .. }=> {
                                let visibility = peek_token!([
                                    TokenKind::Public => VisibilityModifier::Public(*span),
                                    TokenKind::Protected => VisibilityModifier::Protected(*span),
                                    TokenKind::Private => VisibilityModifier::Private(*span),
                                ], state, ["`private`", "`protected`", "`public`"]);

                                state.stream.next();

                                if state.stream.current().kind == TokenKind::SemiColon {
                                    adaptations.push(TraitUsageAdaptation::Visibility {
                                        r#trait,
                                        method,
                                        visibility,
                                    });
                                } else {
                                    let alias: SimpleIdentifier = identifiers::name(state)?;
                                    adaptations.push(TraitUsageAdaptation::Alias {
                                        r#trait,
                                        method,
                                        alias,
                                        visibility: Some(visibility),
                                    });
                                }
                            }
                            _ => {
                                let alias: SimpleIdentifier = identifiers::name(state)?;
                                adaptations.push(TraitUsageAdaptation::Alias {
                                    r#trait,
                                    method,
                                    alias,
                                    visibility: None,
                                });
                            }
                        }
                    },
                    TokenKind::Insteadof => {
                        let mut insteadof = vec![
                            identifiers::full_type_name(state)?
                        ];

                        if state.stream.current().kind == TokenKind::Comma {
                            if state.stream.peek().kind == TokenKind::SemiColon {
                                // will fail with unexpected token `,`
                                // as `insteadof` doesn't allow for trailing commas.
                                utils::skip_semicolon(state)?;
                            }

                            state.stream.next();

                            while state.stream.current().kind != TokenKind::SemiColon {
                                insteadof.push(identifiers::full_type_name(state)?);

                                if state.stream.current().kind == TokenKind::Comma {
                                    if state.stream.peek().kind == TokenKind::SemiColon {
                                        // will fail with unexpected token `,`
                                        // as `insteadof` doesn't allow for trailing commas.
                                        utils::skip_semicolon(state)?;
                                    } else {
                                        state.stream.next();
                                    }
                                } else {
                                    break;
                                }
                            }
                        }

                        adaptations.push(TraitUsageAdaptation::Precedence {
                            r#trait,
                            method,
                            insteadof,
                        });
                    }
                ], state, ["`as`", "`insteadof`"]);

            utils::skip_semicolon(state)?;
        }

        utils::skip_right_brace(state)?;
    } else {
        utils::skip_semicolon(state)?;
    }

    Ok(TraitUsage {
        r#use: span,
        traits,
        adaptations,
    })
}

pub fn parse(state: &mut State) -> ParseResult<Statement> {
    let span = utils::skip(state, TokenKind::Trait)?;
    let name = identifiers::type_identifier(state)?;
    let attributes = state.get_attributes();

    let body = TraitBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace && !state.stream.is_eof() {
                members.push(member(state, &name)?);
            }
            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(Statement::Trait(TraitStatement {
        r#trait: span,
        name,
        attributes,
        body,
    }))
}

fn member(state: &mut State, class_name: &SimpleIdentifier) -> ParseResult<TraitMember> {
    let has_attributes = attributes::gather_attributes(state)?;

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return usage(state).map(TraitMember::TraitUsage);
    }

    if state.stream.current().kind == TokenKind::Var {
        return properties::parse_var(state, Some(class_name)).map(TraitMember::VariableProperty);
    }

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        return constants::classish(state, modifiers::constant_group(modifiers)?)
            .map(TraitMember::Constant);
    }

    if state.stream.current().kind == TokenKind::Function {
        let method = method(
            state,
            MethodType::DependingOnModifiers,
            modifiers::method_group(modifiers)?,
            Some(class_name),
        )?;

        return match method {
            Method::Abstract(method) => Ok(TraitMember::AbstractMethod(method)),
            Method::Concrete(method) => Ok(TraitMember::ConcreteMethod(method)),
            Method::AbstractConstructor(ctor) => Ok(TraitMember::AbstractConstructor(ctor)),
            Method::ConcreteConstructor(ctor) => Ok(TraitMember::ConcreteConstructor(ctor)),
        };
    }

    properties::parse(
        state,
        Some(class_name),
        modifiers::property_group(modifiers)?,
    )
    .map(TraitMember::Property)
}
