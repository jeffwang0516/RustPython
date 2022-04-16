// File automatically generated by ast/asdl_rs.py.
pub use crate::constant::*;
pub use crate::location::Location;

type Ident = String;

#[derive(Debug, PartialEq)]
pub struct Located<T, U = ()> {
    pub location: Location,
    pub custom: U,
    pub node: T,
}

impl<T> Located<T> {
    pub fn new(location: Location, node: T) -> Self {
        Self {
            location,
            custom: (),
            node,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mod<U = ()> {
    Module {
        body: Vec<Stmt<U>>,
        type_ignores: Vec<TypeIgnore>,
    },
    Interactive {
        body: Vec<Stmt<U>>,
    },
    Expression {
        body: Box<Expr<U>>,
    },
    FunctionType {
        argtypes: Vec<Expr<U>>,
        returns: Box<Expr<U>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum StmtKind<U = ()> {
    FunctionDef {
        name: Ident,
        args: Box<Arguments<U>>,
        body: Vec<Stmt<U>>,
        decorator_list: Vec<Expr<U>>,
        returns: Option<Box<Expr<U>>>,
        type_comment: Option<String>,
    },
    AsyncFunctionDef {
        name: Ident,
        args: Box<Arguments<U>>,
        body: Vec<Stmt<U>>,
        decorator_list: Vec<Expr<U>>,
        returns: Option<Box<Expr<U>>>,
        type_comment: Option<String>,
    },
    ClassDef {
        name: Ident,
        bases: Vec<Expr<U>>,
        keywords: Vec<Keyword<U>>,
        body: Vec<Stmt<U>>,
        decorator_list: Vec<Expr<U>>,
    },
    Return {
        value: Option<Box<Expr<U>>>,
    },
    Delete {
        targets: Vec<Expr<U>>,
    },
    Assign {
        targets: Vec<Expr<U>>,
        value: Box<Expr<U>>,
        type_comment: Option<String>,
    },
    AugAssign {
        target: Box<Expr<U>>,
        op: Operator,
        value: Box<Expr<U>>,
    },
    AnnAssign {
        target: Box<Expr<U>>,
        annotation: Box<Expr<U>>,
        value: Option<Box<Expr<U>>>,
        simple: bool,
    },
    For {
        target: Box<Expr<U>>,
        iter: Box<Expr<U>>,
        body: Vec<Stmt<U>>,
        orelse: Vec<Stmt<U>>,
        type_comment: Option<String>,
    },
    AsyncFor {
        target: Box<Expr<U>>,
        iter: Box<Expr<U>>,
        body: Vec<Stmt<U>>,
        orelse: Vec<Stmt<U>>,
        type_comment: Option<String>,
    },
    While {
        test: Box<Expr<U>>,
        body: Vec<Stmt<U>>,
        orelse: Vec<Stmt<U>>,
    },
    If {
        test: Box<Expr<U>>,
        body: Vec<Stmt<U>>,
        orelse: Vec<Stmt<U>>,
    },
    With {
        items: Vec<Withitem<U>>,
        body: Vec<Stmt<U>>,
        type_comment: Option<String>,
    },
    AsyncWith {
        items: Vec<Withitem<U>>,
        body: Vec<Stmt<U>>,
        type_comment: Option<String>,
    },
    Raise {
        exc: Option<Box<Expr<U>>>,
        cause: Option<Box<Expr<U>>>,
    },
    Try {
        body: Vec<Stmt<U>>,
        handlers: Vec<Excepthandler<U>>,
        orelse: Vec<Stmt<U>>,
        finalbody: Vec<Stmt<U>>,
    },
    Assert {
        test: Box<Expr<U>>,
        msg: Option<Box<Expr<U>>>,
    },
    Import {
        names: Vec<Alias>,
    },
    ImportFrom {
        module: Option<Ident>,
        names: Vec<Alias>,
        level: usize,
    },
    Global {
        names: Vec<Ident>,
    },
    Nonlocal {
        names: Vec<Ident>,
    },
    Expr {
        value: Box<Expr<U>>,
    },
    Pass,
    Break,
    Continue,
}
pub type Stmt<U = ()> = Located<StmtKind<U>, U>;

#[derive(Debug, PartialEq)]
pub enum ExprKind<U = ()> {
    BoolOp {
        op: Boolop,
        values: Vec<Expr<U>>,
    },
    NamedExpr {
        target: Box<Expr<U>>,
        value: Box<Expr<U>>,
    },
    BinOp {
        left: Box<Expr<U>>,
        op: Operator,
        right: Box<Expr<U>>,
    },
    UnaryOp {
        op: Unaryop,
        operand: Box<Expr<U>>,
    },
    Lambda {
        args: Box<Arguments<U>>,
        body: Box<Expr<U>>,
    },
    IfExp {
        test: Box<Expr<U>>,
        body: Box<Expr<U>>,
        orelse: Box<Expr<U>>,
    },
    Dict {
        keys: Vec<Option<Box<Expr<U>>>>,
        values: Vec<Expr<U>>,
    },
    Set {
        elts: Vec<Expr<U>>,
    },
    ListComp {
        elt: Box<Expr<U>>,
        generators: Vec<Comprehension<U>>,
    },
    SetComp {
        elt: Box<Expr<U>>,
        generators: Vec<Comprehension<U>>,
    },
    DictComp {
        key: Box<Expr<U>>,
        value: Box<Expr<U>>,
        generators: Vec<Comprehension<U>>,
    },
    GeneratorExp {
        elt: Box<Expr<U>>,
        generators: Vec<Comprehension<U>>,
    },
    Await {
        value: Box<Expr<U>>,
    },
    Yield {
        value: Option<Box<Expr<U>>>,
    },
    YieldFrom {
        value: Box<Expr<U>>,
    },
    Compare {
        left: Box<Expr<U>>,
        ops: Vec<Cmpop>,
        comparators: Vec<Expr<U>>,
    },
    Call {
        func: Box<Expr<U>>,
        args: Vec<Expr<U>>,
        keywords: Vec<Keyword<U>>,
    },
    FormattedValue {
        value: Box<Expr<U>>,
        conversion: Option<ConversionFlag>,
        format_spec: Option<Box<Expr<U>>>,
    },
    JoinedStr {
        values: Vec<Expr<U>>,
    },
    Constant {
        value: Constant,
        kind: Option<String>,
    },
    Attribute {
        value: Box<Expr<U>>,
        attr: Ident,
        ctx: ExprContext,
    },
    Subscript {
        value: Box<Expr<U>>,
        slice: Box<Expr<U>>,
        ctx: ExprContext,
    },
    Starred {
        value: Box<Expr<U>>,
        ctx: ExprContext,
    },
    Name {
        id: Ident,
        ctx: ExprContext,
    },
    List {
        elts: Vec<Expr<U>>,
        ctx: ExprContext,
    },
    Tuple {
        elts: Vec<Expr<U>>,
        ctx: ExprContext,
    },
    Slice {
        lower: Option<Box<Expr<U>>>,
        upper: Option<Box<Expr<U>>>,
        step: Option<Box<Expr<U>>>,
    },
}
pub type Expr<U = ()> = Located<ExprKind<U>, U>;

#[derive(Debug, PartialEq)]
pub enum ExprContext {
    Load,
    Store,
    Del,
}

#[derive(Debug, PartialEq)]
pub enum Boolop {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

#[derive(Debug, PartialEq)]
pub enum Unaryop {
    Invert,
    Not,
    UAdd,
    USub,
}

#[derive(Debug, PartialEq)]
pub enum Cmpop {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

#[derive(Debug, PartialEq)]
pub struct Comprehension<U = ()> {
    pub target: Box<Expr<U>>,
    pub iter: Box<Expr<U>>,
    pub ifs: Vec<Expr<U>>,
    pub is_async: bool,
}

#[derive(Debug, PartialEq)]
pub enum ExcepthandlerKind<U = ()> {
    ExceptHandler {
        type_: Option<Box<Expr<U>>>,
        name: Option<Ident>,
        body: Vec<Stmt<U>>,
    },
}
pub type Excepthandler<U = ()> = Located<ExcepthandlerKind<U>, U>;

#[derive(Debug, PartialEq)]
pub struct Arguments<U = ()> {
    pub posonlyargs: Vec<Arg<U>>,
    pub args: Vec<Arg<U>>,
    pub vararg: Option<Box<Arg<U>>>,
    pub kwonlyargs: Vec<Arg<U>>,
    pub kw_defaults: Vec<Option<Box<Expr<U>>>>,
    pub kwarg: Option<Box<Arg<U>>>,
    pub defaults: Vec<Expr<U>>,
}

#[derive(Debug, PartialEq)]
pub struct ArgData<U = ()> {
    pub arg: Ident,
    pub annotation: Option<Box<Expr<U>>>,
    pub type_comment: Option<String>,
}
pub type Arg<U = ()> = Located<ArgData<U>, U>;

#[derive(Debug, PartialEq)]
pub struct KeywordData<U = ()> {
    pub arg: Option<Ident>,
    pub value: Box<Expr<U>>,
}
pub type Keyword<U = ()> = Located<KeywordData<U>, U>;

#[derive(Debug, PartialEq)]
pub struct Alias {
    pub name: Ident,
    pub asname: Option<Ident>,
}

#[derive(Debug, PartialEq)]
pub struct Withitem<U = ()> {
    pub context_expr: Box<Expr<U>>,
    pub optional_vars: Option<Box<Expr<U>>>,
}

#[derive(Debug, PartialEq)]
pub enum TypeIgnore {
    TypeIgnore { lineno: usize, tag: String },
}

#[cfg(feature = "fold")]
pub mod fold {
    use super::*;
    use crate::fold_helpers::Foldable;
    pub trait Fold<U> {
        type TargetU;
        type Error;
        fn map_user(&mut self, user: U) -> Result<Self::TargetU, Self::Error>;
        fn fold_mod(&mut self, node: Mod<U>) -> Result<Mod<Self::TargetU>, Self::Error> {
            fold_mod(self, node)
        }
        fn fold_stmt(&mut self, node: Stmt<U>) -> Result<Stmt<Self::TargetU>, Self::Error> {
            fold_stmt(self, node)
        }
        fn fold_expr(&mut self, node: Expr<U>) -> Result<Expr<Self::TargetU>, Self::Error> {
            fold_expr(self, node)
        }
        fn fold_expr_context(&mut self, node: ExprContext) -> Result<ExprContext, Self::Error> {
            fold_expr_context(self, node)
        }
        fn fold_boolop(&mut self, node: Boolop) -> Result<Boolop, Self::Error> {
            fold_boolop(self, node)
        }
        fn fold_operator(&mut self, node: Operator) -> Result<Operator, Self::Error> {
            fold_operator(self, node)
        }
        fn fold_unaryop(&mut self, node: Unaryop) -> Result<Unaryop, Self::Error> {
            fold_unaryop(self, node)
        }
        fn fold_cmpop(&mut self, node: Cmpop) -> Result<Cmpop, Self::Error> {
            fold_cmpop(self, node)
        }
        fn fold_comprehension(
            &mut self,
            node: Comprehension<U>,
        ) -> Result<Comprehension<Self::TargetU>, Self::Error> {
            fold_comprehension(self, node)
        }
        fn fold_excepthandler(
            &mut self,
            node: Excepthandler<U>,
        ) -> Result<Excepthandler<Self::TargetU>, Self::Error> {
            fold_excepthandler(self, node)
        }
        fn fold_arguments(
            &mut self,
            node: Arguments<U>,
        ) -> Result<Arguments<Self::TargetU>, Self::Error> {
            fold_arguments(self, node)
        }
        fn fold_arg(&mut self, node: Arg<U>) -> Result<Arg<Self::TargetU>, Self::Error> {
            fold_arg(self, node)
        }
        fn fold_keyword(
            &mut self,
            node: Keyword<U>,
        ) -> Result<Keyword<Self::TargetU>, Self::Error> {
            fold_keyword(self, node)
        }
        fn fold_alias(&mut self, node: Alias) -> Result<Alias, Self::Error> {
            fold_alias(self, node)
        }
        fn fold_withitem(
            &mut self,
            node: Withitem<U>,
        ) -> Result<Withitem<Self::TargetU>, Self::Error> {
            fold_withitem(self, node)
        }
        fn fold_type_ignore(&mut self, node: TypeIgnore) -> Result<TypeIgnore, Self::Error> {
            fold_type_ignore(self, node)
        }
    }
    fn fold_located<U, F: Fold<U> + ?Sized, T, MT>(
        folder: &mut F,
        node: Located<T, U>,
        f: impl FnOnce(&mut F, T) -> Result<MT, F::Error>,
    ) -> Result<Located<MT, F::TargetU>, F::Error> {
        Ok(Located {
            custom: folder.map_user(node.custom)?,
            location: node.location,
            node: f(folder, node.node)?,
        })
    }
    impl<T, U> Foldable<T, U> for Mod<T> {
        type Mapped = Mod<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_mod(self)
        }
    }
    pub fn fold_mod<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Mod<U>,
    ) -> Result<Mod<F::TargetU>, F::Error> {
        match node {
            Mod::Module { body, type_ignores } => Ok(Mod::Module {
                body: Foldable::fold(body, folder)?,
                type_ignores: Foldable::fold(type_ignores, folder)?,
            }),
            Mod::Interactive { body } => Ok(Mod::Interactive {
                body: Foldable::fold(body, folder)?,
            }),
            Mod::Expression { body } => Ok(Mod::Expression {
                body: Foldable::fold(body, folder)?,
            }),
            Mod::FunctionType { argtypes, returns } => Ok(Mod::FunctionType {
                argtypes: Foldable::fold(argtypes, folder)?,
                returns: Foldable::fold(returns, folder)?,
            }),
        }
    }
    impl<T, U> Foldable<T, U> for Stmt<T> {
        type Mapped = Stmt<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_stmt(self)
        }
    }
    pub fn fold_stmt<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Stmt<U>,
    ) -> Result<Stmt<F::TargetU>, F::Error> {
        fold_located(folder, node, |folder, node| match node {
            StmtKind::FunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
            } => Ok(StmtKind::FunctionDef {
                name: Foldable::fold(name, folder)?,
                args: Foldable::fold(args, folder)?,
                body: Foldable::fold(body, folder)?,
                decorator_list: Foldable::fold(decorator_list, folder)?,
                returns: Foldable::fold(returns, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            }),
            StmtKind::AsyncFunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
            } => Ok(StmtKind::AsyncFunctionDef {
                name: Foldable::fold(name, folder)?,
                args: Foldable::fold(args, folder)?,
                body: Foldable::fold(body, folder)?,
                decorator_list: Foldable::fold(decorator_list, folder)?,
                returns: Foldable::fold(returns, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            }),
            StmtKind::ClassDef {
                name,
                bases,
                keywords,
                body,
                decorator_list,
            } => Ok(StmtKind::ClassDef {
                name: Foldable::fold(name, folder)?,
                bases: Foldable::fold(bases, folder)?,
                keywords: Foldable::fold(keywords, folder)?,
                body: Foldable::fold(body, folder)?,
                decorator_list: Foldable::fold(decorator_list, folder)?,
            }),
            StmtKind::Return { value } => Ok(StmtKind::Return {
                value: Foldable::fold(value, folder)?,
            }),
            StmtKind::Delete { targets } => Ok(StmtKind::Delete {
                targets: Foldable::fold(targets, folder)?,
            }),
            StmtKind::Assign {
                targets,
                value,
                type_comment,
            } => Ok(StmtKind::Assign {
                targets: Foldable::fold(targets, folder)?,
                value: Foldable::fold(value, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            }),
            StmtKind::AugAssign { target, op, value } => Ok(StmtKind::AugAssign {
                target: Foldable::fold(target, folder)?,
                op: Foldable::fold(op, folder)?,
                value: Foldable::fold(value, folder)?,
            }),
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                simple,
            } => Ok(StmtKind::AnnAssign {
                target: Foldable::fold(target, folder)?,
                annotation: Foldable::fold(annotation, folder)?,
                value: Foldable::fold(value, folder)?,
                simple: Foldable::fold(simple, folder)?,
            }),
            StmtKind::For {
                target,
                iter,
                body,
                orelse,
                type_comment,
            } => Ok(StmtKind::For {
                target: Foldable::fold(target, folder)?,
                iter: Foldable::fold(iter, folder)?,
                body: Foldable::fold(body, folder)?,
                orelse: Foldable::fold(orelse, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            }),
            StmtKind::AsyncFor {
                target,
                iter,
                body,
                orelse,
                type_comment,
            } => Ok(StmtKind::AsyncFor {
                target: Foldable::fold(target, folder)?,
                iter: Foldable::fold(iter, folder)?,
                body: Foldable::fold(body, folder)?,
                orelse: Foldable::fold(orelse, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            }),
            StmtKind::While { test, body, orelse } => Ok(StmtKind::While {
                test: Foldable::fold(test, folder)?,
                body: Foldable::fold(body, folder)?,
                orelse: Foldable::fold(orelse, folder)?,
            }),
            StmtKind::If { test, body, orelse } => Ok(StmtKind::If {
                test: Foldable::fold(test, folder)?,
                body: Foldable::fold(body, folder)?,
                orelse: Foldable::fold(orelse, folder)?,
            }),
            StmtKind::With {
                items,
                body,
                type_comment,
            } => Ok(StmtKind::With {
                items: Foldable::fold(items, folder)?,
                body: Foldable::fold(body, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            }),
            StmtKind::AsyncWith {
                items,
                body,
                type_comment,
            } => Ok(StmtKind::AsyncWith {
                items: Foldable::fold(items, folder)?,
                body: Foldable::fold(body, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            }),
            StmtKind::Raise { exc, cause } => Ok(StmtKind::Raise {
                exc: Foldable::fold(exc, folder)?,
                cause: Foldable::fold(cause, folder)?,
            }),
            StmtKind::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => Ok(StmtKind::Try {
                body: Foldable::fold(body, folder)?,
                handlers: Foldable::fold(handlers, folder)?,
                orelse: Foldable::fold(orelse, folder)?,
                finalbody: Foldable::fold(finalbody, folder)?,
            }),
            StmtKind::Assert { test, msg } => Ok(StmtKind::Assert {
                test: Foldable::fold(test, folder)?,
                msg: Foldable::fold(msg, folder)?,
            }),
            StmtKind::Import { names } => Ok(StmtKind::Import {
                names: Foldable::fold(names, folder)?,
            }),
            StmtKind::ImportFrom {
                module,
                names,
                level,
            } => Ok(StmtKind::ImportFrom {
                module: Foldable::fold(module, folder)?,
                names: Foldable::fold(names, folder)?,
                level: Foldable::fold(level, folder)?,
            }),
            StmtKind::Global { names } => Ok(StmtKind::Global {
                names: Foldable::fold(names, folder)?,
            }),
            StmtKind::Nonlocal { names } => Ok(StmtKind::Nonlocal {
                names: Foldable::fold(names, folder)?,
            }),
            StmtKind::Expr { value } => Ok(StmtKind::Expr {
                value: Foldable::fold(value, folder)?,
            }),
            StmtKind::Pass {} => Ok(StmtKind::Pass {}),
            StmtKind::Break {} => Ok(StmtKind::Break {}),
            StmtKind::Continue {} => Ok(StmtKind::Continue {}),
        })
    }
    impl<T, U> Foldable<T, U> for Expr<T> {
        type Mapped = Expr<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_expr(self)
        }
    }
    pub fn fold_expr<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Expr<U>,
    ) -> Result<Expr<F::TargetU>, F::Error> {
        fold_located(folder, node, |folder, node| match node {
            ExprKind::BoolOp { op, values } => Ok(ExprKind::BoolOp {
                op: Foldable::fold(op, folder)?,
                values: Foldable::fold(values, folder)?,
            }),
            ExprKind::NamedExpr { target, value } => Ok(ExprKind::NamedExpr {
                target: Foldable::fold(target, folder)?,
                value: Foldable::fold(value, folder)?,
            }),
            ExprKind::BinOp { left, op, right } => Ok(ExprKind::BinOp {
                left: Foldable::fold(left, folder)?,
                op: Foldable::fold(op, folder)?,
                right: Foldable::fold(right, folder)?,
            }),
            ExprKind::UnaryOp { op, operand } => Ok(ExprKind::UnaryOp {
                op: Foldable::fold(op, folder)?,
                operand: Foldable::fold(operand, folder)?,
            }),
            ExprKind::Lambda { args, body } => Ok(ExprKind::Lambda {
                args: Foldable::fold(args, folder)?,
                body: Foldable::fold(body, folder)?,
            }),
            ExprKind::IfExp { test, body, orelse } => Ok(ExprKind::IfExp {
                test: Foldable::fold(test, folder)?,
                body: Foldable::fold(body, folder)?,
                orelse: Foldable::fold(orelse, folder)?,
            }),
            ExprKind::Dict { keys, values } => Ok(ExprKind::Dict {
                keys: Foldable::fold(keys, folder)?,
                values: Foldable::fold(values, folder)?,
            }),
            ExprKind::Set { elts } => Ok(ExprKind::Set {
                elts: Foldable::fold(elts, folder)?,
            }),
            ExprKind::ListComp { elt, generators } => Ok(ExprKind::ListComp {
                elt: Foldable::fold(elt, folder)?,
                generators: Foldable::fold(generators, folder)?,
            }),
            ExprKind::SetComp { elt, generators } => Ok(ExprKind::SetComp {
                elt: Foldable::fold(elt, folder)?,
                generators: Foldable::fold(generators, folder)?,
            }),
            ExprKind::DictComp {
                key,
                value,
                generators,
            } => Ok(ExprKind::DictComp {
                key: Foldable::fold(key, folder)?,
                value: Foldable::fold(value, folder)?,
                generators: Foldable::fold(generators, folder)?,
            }),
            ExprKind::GeneratorExp { elt, generators } => Ok(ExprKind::GeneratorExp {
                elt: Foldable::fold(elt, folder)?,
                generators: Foldable::fold(generators, folder)?,
            }),
            ExprKind::Await { value } => Ok(ExprKind::Await {
                value: Foldable::fold(value, folder)?,
            }),
            ExprKind::Yield { value } => Ok(ExprKind::Yield {
                value: Foldable::fold(value, folder)?,
            }),
            ExprKind::YieldFrom { value } => Ok(ExprKind::YieldFrom {
                value: Foldable::fold(value, folder)?,
            }),
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => Ok(ExprKind::Compare {
                left: Foldable::fold(left, folder)?,
                ops: Foldable::fold(ops, folder)?,
                comparators: Foldable::fold(comparators, folder)?,
            }),
            ExprKind::Call {
                func,
                args,
                keywords,
            } => Ok(ExprKind::Call {
                func: Foldable::fold(func, folder)?,
                args: Foldable::fold(args, folder)?,
                keywords: Foldable::fold(keywords, folder)?,
            }),
            ExprKind::FormattedValue {
                value,
                conversion,
                format_spec,
            } => Ok(ExprKind::FormattedValue {
                value: Foldable::fold(value, folder)?,
                conversion: Foldable::fold(conversion, folder)?,
                format_spec: Foldable::fold(format_spec, folder)?,
            }),
            ExprKind::JoinedStr { values } => Ok(ExprKind::JoinedStr {
                values: Foldable::fold(values, folder)?,
            }),
            ExprKind::Constant { value, kind } => Ok(ExprKind::Constant {
                value: Foldable::fold(value, folder)?,
                kind: Foldable::fold(kind, folder)?,
            }),
            ExprKind::Attribute { value, attr, ctx } => Ok(ExprKind::Attribute {
                value: Foldable::fold(value, folder)?,
                attr: Foldable::fold(attr, folder)?,
                ctx: Foldable::fold(ctx, folder)?,
            }),
            ExprKind::Subscript { value, slice, ctx } => Ok(ExprKind::Subscript {
                value: Foldable::fold(value, folder)?,
                slice: Foldable::fold(slice, folder)?,
                ctx: Foldable::fold(ctx, folder)?,
            }),
            ExprKind::Starred { value, ctx } => Ok(ExprKind::Starred {
                value: Foldable::fold(value, folder)?,
                ctx: Foldable::fold(ctx, folder)?,
            }),
            ExprKind::Name { id, ctx } => Ok(ExprKind::Name {
                id: Foldable::fold(id, folder)?,
                ctx: Foldable::fold(ctx, folder)?,
            }),
            ExprKind::List { elts, ctx } => Ok(ExprKind::List {
                elts: Foldable::fold(elts, folder)?,
                ctx: Foldable::fold(ctx, folder)?,
            }),
            ExprKind::Tuple { elts, ctx } => Ok(ExprKind::Tuple {
                elts: Foldable::fold(elts, folder)?,
                ctx: Foldable::fold(ctx, folder)?,
            }),
            ExprKind::Slice { lower, upper, step } => Ok(ExprKind::Slice {
                lower: Foldable::fold(lower, folder)?,
                upper: Foldable::fold(upper, folder)?,
                step: Foldable::fold(step, folder)?,
            }),
        })
    }
    impl<T, U> Foldable<T, U> for ExprContext {
        type Mapped = ExprContext;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_expr_context(self)
        }
    }
    pub fn fold_expr_context<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: ExprContext,
    ) -> Result<ExprContext, F::Error> {
        match node {
            ExprContext::Load {} => Ok(ExprContext::Load {}),
            ExprContext::Store {} => Ok(ExprContext::Store {}),
            ExprContext::Del {} => Ok(ExprContext::Del {}),
        }
    }
    impl<T, U> Foldable<T, U> for Boolop {
        type Mapped = Boolop;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_boolop(self)
        }
    }
    pub fn fold_boolop<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Boolop,
    ) -> Result<Boolop, F::Error> {
        match node {
            Boolop::And {} => Ok(Boolop::And {}),
            Boolop::Or {} => Ok(Boolop::Or {}),
        }
    }
    impl<T, U> Foldable<T, U> for Operator {
        type Mapped = Operator;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_operator(self)
        }
    }
    pub fn fold_operator<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Operator,
    ) -> Result<Operator, F::Error> {
        match node {
            Operator::Add {} => Ok(Operator::Add {}),
            Operator::Sub {} => Ok(Operator::Sub {}),
            Operator::Mult {} => Ok(Operator::Mult {}),
            Operator::MatMult {} => Ok(Operator::MatMult {}),
            Operator::Div {} => Ok(Operator::Div {}),
            Operator::Mod {} => Ok(Operator::Mod {}),
            Operator::Pow {} => Ok(Operator::Pow {}),
            Operator::LShift {} => Ok(Operator::LShift {}),
            Operator::RShift {} => Ok(Operator::RShift {}),
            Operator::BitOr {} => Ok(Operator::BitOr {}),
            Operator::BitXor {} => Ok(Operator::BitXor {}),
            Operator::BitAnd {} => Ok(Operator::BitAnd {}),
            Operator::FloorDiv {} => Ok(Operator::FloorDiv {}),
        }
    }
    impl<T, U> Foldable<T, U> for Unaryop {
        type Mapped = Unaryop;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_unaryop(self)
        }
    }
    pub fn fold_unaryop<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Unaryop,
    ) -> Result<Unaryop, F::Error> {
        match node {
            Unaryop::Invert {} => Ok(Unaryop::Invert {}),
            Unaryop::Not {} => Ok(Unaryop::Not {}),
            Unaryop::UAdd {} => Ok(Unaryop::UAdd {}),
            Unaryop::USub {} => Ok(Unaryop::USub {}),
        }
    }
    impl<T, U> Foldable<T, U> for Cmpop {
        type Mapped = Cmpop;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_cmpop(self)
        }
    }
    pub fn fold_cmpop<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Cmpop,
    ) -> Result<Cmpop, F::Error> {
        match node {
            Cmpop::Eq {} => Ok(Cmpop::Eq {}),
            Cmpop::NotEq {} => Ok(Cmpop::NotEq {}),
            Cmpop::Lt {} => Ok(Cmpop::Lt {}),
            Cmpop::LtE {} => Ok(Cmpop::LtE {}),
            Cmpop::Gt {} => Ok(Cmpop::Gt {}),
            Cmpop::GtE {} => Ok(Cmpop::GtE {}),
            Cmpop::Is {} => Ok(Cmpop::Is {}),
            Cmpop::IsNot {} => Ok(Cmpop::IsNot {}),
            Cmpop::In {} => Ok(Cmpop::In {}),
            Cmpop::NotIn {} => Ok(Cmpop::NotIn {}),
        }
    }
    impl<T, U> Foldable<T, U> for Comprehension<T> {
        type Mapped = Comprehension<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_comprehension(self)
        }
    }
    pub fn fold_comprehension<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Comprehension<U>,
    ) -> Result<Comprehension<F::TargetU>, F::Error> {
        let Comprehension {
            target,
            iter,
            ifs,
            is_async,
        } = node;
        Ok(Comprehension {
            target: Foldable::fold(target, folder)?,
            iter: Foldable::fold(iter, folder)?,
            ifs: Foldable::fold(ifs, folder)?,
            is_async: Foldable::fold(is_async, folder)?,
        })
    }
    impl<T, U> Foldable<T, U> for Excepthandler<T> {
        type Mapped = Excepthandler<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_excepthandler(self)
        }
    }
    pub fn fold_excepthandler<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Excepthandler<U>,
    ) -> Result<Excepthandler<F::TargetU>, F::Error> {
        fold_located(folder, node, |folder, node| match node {
            ExcepthandlerKind::ExceptHandler { type_, name, body } => {
                Ok(ExcepthandlerKind::ExceptHandler {
                    type_: Foldable::fold(type_, folder)?,
                    name: Foldable::fold(name, folder)?,
                    body: Foldable::fold(body, folder)?,
                })
            }
        })
    }
    impl<T, U> Foldable<T, U> for Arguments<T> {
        type Mapped = Arguments<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_arguments(self)
        }
    }
    pub fn fold_arguments<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Arguments<U>,
    ) -> Result<Arguments<F::TargetU>, F::Error> {
        let Arguments {
            posonlyargs,
            args,
            vararg,
            kwonlyargs,
            kw_defaults,
            kwarg,
            defaults,
        } = node;
        Ok(Arguments {
            posonlyargs: Foldable::fold(posonlyargs, folder)?,
            args: Foldable::fold(args, folder)?,
            vararg: Foldable::fold(vararg, folder)?,
            kwonlyargs: Foldable::fold(kwonlyargs, folder)?,
            kw_defaults: Foldable::fold(kw_defaults, folder)?,
            kwarg: Foldable::fold(kwarg, folder)?,
            defaults: Foldable::fold(defaults, folder)?,
        })
    }
    impl<T, U> Foldable<T, U> for Arg<T> {
        type Mapped = Arg<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_arg(self)
        }
    }
    pub fn fold_arg<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Arg<U>,
    ) -> Result<Arg<F::TargetU>, F::Error> {
        fold_located(folder, node, |folder, node| {
            let ArgData {
                arg,
                annotation,
                type_comment,
            } = node;
            Ok(ArgData {
                arg: Foldable::fold(arg, folder)?,
                annotation: Foldable::fold(annotation, folder)?,
                type_comment: Foldable::fold(type_comment, folder)?,
            })
        })
    }
    impl<T, U> Foldable<T, U> for Keyword<T> {
        type Mapped = Keyword<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_keyword(self)
        }
    }
    pub fn fold_keyword<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Keyword<U>,
    ) -> Result<Keyword<F::TargetU>, F::Error> {
        fold_located(folder, node, |folder, node| {
            let KeywordData { arg, value } = node;
            Ok(KeywordData {
                arg: Foldable::fold(arg, folder)?,
                value: Foldable::fold(value, folder)?,
            })
        })
    }
    impl<T, U> Foldable<T, U> for Alias {
        type Mapped = Alias;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_alias(self)
        }
    }
    pub fn fold_alias<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Alias,
    ) -> Result<Alias, F::Error> {
        let Alias { name, asname } = node;
        Ok(Alias {
            name: Foldable::fold(name, folder)?,
            asname: Foldable::fold(asname, folder)?,
        })
    }
    impl<T, U> Foldable<T, U> for Withitem<T> {
        type Mapped = Withitem<U>;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_withitem(self)
        }
    }
    pub fn fold_withitem<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: Withitem<U>,
    ) -> Result<Withitem<F::TargetU>, F::Error> {
        let Withitem {
            context_expr,
            optional_vars,
        } = node;
        Ok(Withitem {
            context_expr: Foldable::fold(context_expr, folder)?,
            optional_vars: Foldable::fold(optional_vars, folder)?,
        })
    }
    impl<T, U> Foldable<T, U> for TypeIgnore {
        type Mapped = TypeIgnore;
        fn fold<F: Fold<T, TargetU = U> + ?Sized>(
            self,
            folder: &mut F,
        ) -> Result<Self::Mapped, F::Error> {
            folder.fold_type_ignore(self)
        }
    }
    pub fn fold_type_ignore<U, F: Fold<U> + ?Sized>(
        #[allow(unused)] folder: &mut F,
        node: TypeIgnore,
    ) -> Result<TypeIgnore, F::Error> {
        match node {
            TypeIgnore::TypeIgnore { lineno, tag } => Ok(TypeIgnore::TypeIgnore {
                lineno: Foldable::fold(lineno, folder)?,
                tag: Foldable::fold(tag, folder)?,
            }),
        }
    }
}
