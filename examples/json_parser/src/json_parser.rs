// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::id_tree::Tree;
use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{
    DFATransition, LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, UserActionsTrait,
};
use parol_runtime::ParolError;
use parol_runtime::{TokenStream, Tokenizer};
use std::cell::RefCell;
use std::path::Path;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 17] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r###"\{"###,
    /*  6 */ r###"\}"###,
    /*  7 */ r###","###,
    /*  8 */ r###":"###,
    /*  9 */ r###"\["###,
    /* 10 */ r###"\]"###,
    /* 11 */ r###"true"###,
    /* 12 */ r###"false"###,
    /* 13 */ r###"null"###,
    /* 14 */
    r###"\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}"###,
    /* 15 */ r###"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][-+]?(?:0|[1-9][0-9]*)?)?"###,
    /* 16 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 17] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "LBrace",
    /*  6 */ "RBrace",
    /*  7 */ "Comma",
    /*  8 */ "Colon",
    /*  9 */ "LBracket",
    /* 10 */ "RBracket",
    /* 11 */ "True",
    /* 12 */ "False",
    /* 13 */ "Null",
    /* 14 */ "String",
    /* 15 */ "Number",
    /* 16 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 11]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* LBrace */
        6,  /* RBrace */
        7,  /* Comma */
        8,  /* Colon */
        9,  /* LBracket */
        10, /* RBracket */
        11, /* True */
        12, /* False */
        13, /* Null */
        14, /* String */
        15, /* Number */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 11] = &[
    /*  0 */ "Array",
    /*  1 */ "ArrayList",
    /*  2 */ "ArraySuffix",
    /*  3 */ "Json",
    /*  4 */ "Number",
    /*  5 */ "Object",
    /*  6 */ "ObjectList",
    /*  7 */ "ObjectSuffix",
    /*  8 */ "Pair",
    /*  9 */ "String",
    /* 10 */ "Value",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 11] = &[
    /* 0 - "Array" */
    LookaheadDFA {
        states: &[Some(7)],
        transitions: &[],
        k: 0,
    },
    /* 1 - "ArrayList" */
    LookaheadDFA {
        states: &[None, Some(10), Some(11)],
        transitions: &[DFATransition(0, 7, 1), DFATransition(0, 10, 2)],
        k: 1,
    },
    /* 2 - "ArraySuffix" */
    LookaheadDFA {
        states: &[None, Some(8), Some(9)],
        transitions: &[
            DFATransition(0, 5, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 1),
            DFATransition(0, 12, 1),
            DFATransition(0, 13, 1),
            DFATransition(0, 14, 1),
            DFATransition(0, 15, 1),
        ],
        k: 1,
    },
    /* 3 - "Json" */
    LookaheadDFA {
        states: &[Some(0)],
        transitions: &[],
        k: 0,
    },
    /* 4 - "Number" */
    LookaheadDFA {
        states: &[Some(20)],
        transitions: &[],
        k: 0,
    },
    /* 5 - "Object" */
    LookaheadDFA {
        states: &[Some(1)],
        transitions: &[],
        k: 0,
    },
    /* 6 - "ObjectList" */
    LookaheadDFA {
        states: &[None, Some(4), Some(5)],
        transitions: &[DFATransition(0, 6, 2), DFATransition(0, 7, 1)],
        k: 1,
    },
    /* 7 - "ObjectSuffix" */
    LookaheadDFA {
        states: &[None, Some(2), Some(3)],
        transitions: &[DFATransition(0, 6, 2), DFATransition(0, 14, 1)],
        k: 1,
    },
    /* 8 - "Pair" */
    LookaheadDFA {
        states: &[Some(6)],
        transitions: &[],
        k: 0,
    },
    /* 9 - "String" */
    LookaheadDFA {
        states: &[Some(19)],
        transitions: &[],
        k: 0,
    },
    /* 10 - "Value" */
    LookaheadDFA {
        states: &[
            None,
            Some(12),
            Some(13),
            Some(14),
            Some(15),
            Some(16),
            Some(17),
            Some(18),
        ],
        transitions: &[
            DFATransition(0, 5, 3),
            DFATransition(0, 9, 4),
            DFATransition(0, 11, 5),
            DFATransition(0, 12, 6),
            DFATransition(0, 13, 7),
            DFATransition(0, 14, 1),
            DFATransition(0, 15, 2),
        ],
        k: 1,
    },
];

pub const PRODUCTIONS: &[Production; 21] = &[
    // 0 - Json: Value;
    Production {
        lhs: 3,
        production: &[ParseType::N(10)],
    },
    // 1 - Object: "\{" ObjectSuffix;
    Production {
        lhs: 5,
        production: &[ParseType::N(7), ParseType::T(5)],
    },
    // 2 - ObjectSuffix: Pair ObjectList /* Vec */ "\}";
    Production {
        lhs: 7,
        production: &[ParseType::T(6), ParseType::N(6), ParseType::N(8)],
    },
    // 3 - ObjectSuffix: "\}";
    Production {
        lhs: 7,
        production: &[ParseType::T(6)],
    },
    // 4 - ObjectList: "," Pair ObjectList;
    Production {
        lhs: 6,
        production: &[ParseType::N(6), ParseType::N(8), ParseType::T(7)],
    },
    // 5 - ObjectList: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 6 - Pair: String ":" Value;
    Production {
        lhs: 8,
        production: &[ParseType::N(10), ParseType::T(8), ParseType::N(9)],
    },
    // 7 - Array: "\[" ArraySuffix;
    Production {
        lhs: 0,
        production: &[ParseType::N(2), ParseType::T(9)],
    },
    // 8 - ArraySuffix: Value ArrayList /* Vec */ "\]";
    Production {
        lhs: 2,
        production: &[ParseType::T(10), ParseType::N(1), ParseType::N(10)],
    },
    // 9 - ArraySuffix: "\]";
    Production {
        lhs: 2,
        production: &[ParseType::T(10)],
    },
    // 10 - ArrayList: "," Value ArrayList;
    Production {
        lhs: 1,
        production: &[ParseType::N(1), ParseType::N(10), ParseType::T(7)],
    },
    // 11 - ArrayList: ;
    Production {
        lhs: 1,
        production: &[],
    },
    // 12 - Value: String;
    Production {
        lhs: 10,
        production: &[ParseType::N(9)],
    },
    // 13 - Value: Number;
    Production {
        lhs: 10,
        production: &[ParseType::N(4)],
    },
    // 14 - Value: Object;
    Production {
        lhs: 10,
        production: &[ParseType::N(5)],
    },
    // 15 - Value: Array;
    Production {
        lhs: 10,
        production: &[ParseType::N(0)],
    },
    // 16 - Value: "true";
    Production {
        lhs: 10,
        production: &[ParseType::T(11)],
    },
    // 17 - Value: "false";
    Production {
        lhs: 10,
        production: &[ParseType::T(12)],
    },
    // 18 - Value: "null";
    Production {
        lhs: 10,
        production: &[ParseType::T(13)],
    },
    // 19 - String: "\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}";
    Production {
        lhs: 9,
        production: &[ParseType::T(14)],
    },
    // 20 - Number: "-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][-+]?(?:0|[1-9][0-9]*)?)?";
    Production {
        lhs: 4,
        production: &[ParseType::T(15)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
    )]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut dyn UserActionsTrait<'t>,
) -> Result<Tree<ParseTreeType<'t>>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        3,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream =
        RefCell::new(TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap());
    let result = llk_parser.parse(token_stream, user_actions);
    match result {
        Ok(()) => Ok(llk_parser.parse_tree),
        Err(e) => Err(e),
    }
}
