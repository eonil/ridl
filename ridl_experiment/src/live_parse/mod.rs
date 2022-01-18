/// Assume persistent vector.
type PVec<T> = Vec<T>;
type PString = std::rc::Rc<String>;
type PMap<K,V> = HashMap<K,V>;
type Ver = usize;



struct ByteDoc {
    ver: Ver,
    bytes: PVec<u8>,
}
type ByteTimeline = Timeline<ByteDoc>;



struct Timeline<Snapshot> {
    transactions: Replacement<Snapshot>,
}
impl Timeline {
    fn points(&self) -> Timepoints {

    }
}



struct MetaBlockDoc {
    meta_blocks: PVec<MetaBlock>,
}
struct MetaBlock {
    meta: Len,
    content: Len,
}

/// We can scan meta-blocks very quickly with keywords, operators and parentheses.
/// Each meta-blocks will be parsed into AST individually.
/// 
/// We can use contents of meta-block as their keys to AST.
/// If meta-block content has not been changed, we can re-use the parsed AST.
/// Dumb flat content comparison is always far faster than re-parsing them.
/// This can be even faster if persistent-vector performs chunk address comparison.
/// In ideal-case, large chunk comparison can even be done with single pointer comparison.
/// 
/// Don't forget to keep parsed AST a little bit longer as they are likely to be re-used.
///
/// We can incrementally scan data-structures and function signatures from the AST.
/// That's enough for name resolutions for most cases.
/// On adding/removing an AST subtree, we can add/remove corresponding names to/from name database.

struct DefDB {
    defs: PMap<DefPath,Def>,
}
impl DefDB {
    fn insert(p:DefPath, def:Def) {}
    fn remove(p:DefPath) {}
}
type DefPath = PString;
enum Def {
    Fn(DefFn),
    Struct(DefStruct),
}
struct DefFn {
    signature: PString,
}
struct DefStruct {
    fields: PVec<DefStructField>,
}
struct DefStructField {
    name: PString,
    ty: DefTyExpr,
}



type Len = usize;
struct Replacement<Content> {
    old: ReplacementPart<Content>,
    new: ReplacementPart<Content>,
}
struct ReplacementPart<Content> {
    range: Range<Len>,
    content: Content,
}

struct ByteDoc {
    bytes: Vec<u8>,
}
impl ByteDoc {
    fn apply(replacement:ByteReplacement) {}
}
type ByteReplacement = Replacement<Vec<u8>>;

struct TokenDoc {
    tokens: Vec<Token>,
}
struct Token {
    bytes: ByteDoc,
    len: Len,
}
enum TokenKind {
    Comment, OpenBrace, CloseBrace, OpenParen, CloseParen, 
}
impl Token {
    fn is
}
enum Token {
    Comment(Len),
    OpenBrace,
    CloseBrace,

}
type TokenReplacement = Replacement<Vec<Token>>;
impl TokenDoc {
    fn apply(x:ByteReplacement) -> TokenReplacement {

    }
}

/// Comment & text doc.
struct CTDoc {
    pieces: Vec<CTPiece>,
}
enum CTPiece {
    Comment(Len),
    Text(Len),
}
type CTReplacement = Replacement;
impl CTDoc {
    fn apply(text:TextReplacement) -> CTReplacement<CTToken> {

    }
}
enum CTReplacement {
    old:
}

struct BlockDoc {
    tokens: Vec<BlockToken>,
}
enum BlockToken {
    Open,
    Close,
    Text(Len),
}

struct DivDoc {

}
enum DivToken {
    SemiColon,
    Text(Len),
}

struct DefDoc {

}
struct DefBlock {
    
}
struct DefItem {
    block: usize,

}
struct DefStmt {

}

struct DefDB {

}
impl DefDB {
    fn delete_defs_in_range(range:Range<Len>) {

    }
}
