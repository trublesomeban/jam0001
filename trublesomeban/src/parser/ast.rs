use nexer::lexer;

#[derive(Debug, Clone, Copy)]
pub enum IO<'a> {
    Comment(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub enum AtomNode<'a> {
    Ident(&'a str),
}

#[derive(Debug, Clone)]
pub enum ValueNode {
    Int(i32),
    Float(f32),
    Str(String),
}

#[derive(Debug, Clone)]
pub struct UnaryOperationNode<'a> {
    oper: AtomNode<'a>,
    op: lexer::token::Token,
}

#[derive(Debug, Clone)]
pub struct BinaryOperationNode<'a> {
    lhs: AtomNode<'a>,
    rhs: AtomNode<'a>,
    op: lexer::token::Token,
}

#[derive(Debug, Clone)]
pub struct BindingNode<'a> {
    ident: AtomNode<'a>,
    val: Box<ExprNode<'a>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclarationNode<'a> {
    arg_list: Vec<AtomNode<'a>>,
    expr_list: Vec<ExprNode<'a>>,
}

#[derive(Debug, Clone)]
pub enum ExprNode<'a> {
    BinOp(BinaryOperationNode<'a>),
    UnOp(UnaryOperationNode<'a>),
    Atom(AtomNode<'a>),
    Val(ValueNode),
    Bind(BindingNode<'a>),
    FuncDecl(FunctionDeclarationNode<'a>),
}
