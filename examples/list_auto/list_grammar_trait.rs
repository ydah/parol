// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

#![allow(unused_imports)]
use crate::list_grammar::ListGrammar;
use id_tree::Tree;
use log::trace;
use miette::{bail, miette, IntoDiagnostic, Result};
use parol_runtime::lexer::Token;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};

/// Semantic actions trait generated for the user grammar
/// All functions have default implementations.
pub trait ListGrammarTrait<'t> {
    /// Semantic action for non-terminal 'List'
    fn list(&mut self, _arg: &List) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'Num'
    fn num(&mut self, _arg: &Num) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'TrailingComma'
    fn trailing_comma(&mut self, _arg: &TrailingComma<'t>) -> Result<()> {
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
//
// Output Types of productions deduced from the structure of the transformed grammar
//

// -------------------------------------------------------------------------------------------------
//
// Types of non-terminals deduced from the structure of the transformed grammar
//

///
/// Type derived for non-terminal List
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct List {
    pub list_opt: Option<Box<ListOpt>>,
}

///
/// Type derived for non-terminal ListOpt
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ListOpt {
    pub num: Box<Num>,
    pub list_opt_list: Vec<ListOptList>,
}

///
/// Type derived for non-terminal ListOptList
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ListOptList {
    pub num: Box<Num>,
}

///
/// Type derived for non-terminal Num
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Num {
    pub num: u32, /* 0|[1-9][0-9]* */
}

///
/// Type derived for non-terminal TrailingComma
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct TrailingComma<'t> {
    pub trailing_comma_opt: Option<Box<TrailingCommaOpt<'t>>>,
}

///
/// Type derived for non-terminal TrailingCommaOpt
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct TrailingCommaOpt<'t> {
    pub comma: Token<'t>, /* , */
}

// -------------------------------------------------------------------------------------------------

///
/// Deduced ASTType of expanded grammar
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ASTType<'t> {
    List(List),
    ListOpt(Option<Box<ListOpt>>),
    ListOptList(Vec<ListOptList>),
    Num(Num),
    TrailingComma(TrailingComma<'t>),
    TrailingCommaOpt(Option<Box<TrailingCommaOpt<'t>>>),
}

/// Auto-implemented adapter grammar
///
/// The lifetime parameter `'t` refers to the lifetime of the scanned text.
/// The lifetime parameter `'u` refers to the lifetime of user grammar object.
///
#[allow(dead_code)]
pub struct ListGrammarAuto<'t, 'u>
where
    't: 'u,
{
    // Mutable reference of the actual user grammar to be able to call the semantic actions on it
    user_grammar: &'u mut dyn ListGrammarTrait<'t>,
    // Stack to construct the AST on it
    item_stack: Vec<ASTType<'t>>,
}

///
/// The `ListGrammarAuto` impl is automatically generated for the
/// given grammar.
///
impl<'t, 'u> ListGrammarAuto<'t, 'u> {
    pub fn new(user_grammar: &'u mut dyn ListGrammarTrait<'t>) -> Self {
        Self {
            user_grammar,
            item_stack: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, item: ASTType<'t>, context: &str) {
        trace!("push    {}: {:?}", context, item);
        self.item_stack.push(item)
    }

    #[allow(dead_code)]
    fn pop(&mut self, context: &str) -> Option<ASTType<'t>> {
        if !self.item_stack.is_empty() {
            let item = self.item_stack.pop();
            if let Some(ref item) = item {
                trace!("pop     {}: {:?}", context, item);
            }
            item
        } else {
            None
        }
    }

    #[allow(dead_code)]
    // Use this function for debugging purposes:
    // trace!("{}", self.trace_item_stack(context));
    fn trace_item_stack(&self, context: &str) -> std::string::String {
        format!(
            "Item stack at {}:\n{}",
            context,
            self.item_stack
                .iter()
                .rev()
                .map(|s| format!("  {:?}", s))
                .collect::<Vec<std::string::String>>()
                .join("\n")
        )
    }

    /// Semantic action for production 0:
    ///
    /// List: ListOpt /* Option */ TrailingComma^ /* Clipped */;
    ///
    #[named]
    fn list(
        &mut self,
        _list_opt: &ParseTreeStackEntry<'t>,
        _trailing_comma: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        // Ignore clipped member 'trailing_comma'
        self.pop(context);
        let list_opt = if let Some(ASTType::ListOpt(list_opt)) = self.pop(context) {
            list_opt
        } else {
            bail!("{}: Expecting ASTType::ListOpt", context);
        };
        let list_built = ListBuilder::default()
            .list_opt(list_opt)
            // Ignore clipped member 'trailing_comma'
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.list(&list_built)?;
        self.push(ASTType::List(list_built), context);
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// ListOpt: Num ListOptList /* Vec */; // Option<T>::Some
    ///
    #[named]
    fn list_opt_0(
        &mut self,
        _num: &ParseTreeStackEntry<'t>,
        _list_opt_list: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let list_opt_list = if let Some(ASTType::ListOptList(mut list_opt_list)) = self.pop(context)
        {
            list_opt_list.reverse();
            list_opt_list
        } else {
            bail!("{}: Expecting ASTType::ListOptList", context);
        };
        let num = if let Some(ASTType::Num(num)) = self.pop(context) {
            num
        } else {
            bail!("{}: Expecting ASTType::Num", context);
        };
        let list_opt_0_built = ListOptBuilder::default()
            .num(Box::new(num))
            .list_opt_list(list_opt_list)
            .build()
            .into_diagnostic()?;
        self.push(ASTType::ListOpt(Some(Box::new(list_opt_0_built))), context);
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// ListOptList: ","^ /* Clipped */ Num ListOptList; // Vec<T>::Push
    ///
    #[named]
    fn list_opt_list_0(
        &mut self,
        _comma: &ParseTreeStackEntry<'t>,
        _num: &ParseTreeStackEntry<'t>,
        _list_opt_list: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let mut list_opt_list = if let Some(ASTType::ListOptList(list_opt_list)) = self.pop(context)
        {
            list_opt_list
        } else {
            bail!("{}: Expecting ASTType::ListOptList", context);
        };
        let num = if let Some(ASTType::Num(num)) = self.pop(context) {
            num
        } else {
            bail!("{}: Expecting ASTType::Num", context);
        };
        let list_opt_list_0_built = ListOptListBuilder::default()
            .num(Box::new(num))
            // Ignore clipped member 'comma'
            .build()
            .into_diagnostic()?;
        // Add an element to the vector
        list_opt_list.push(list_opt_list_0_built);
        self.push(ASTType::ListOptList(list_opt_list), context);
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// ListOptList: ; // Vec<T>::New
    ///
    #[named]
    fn list_opt_list_1(&mut self, _parse_tree: &Tree<ParseTreeType<'t>>) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let list_opt_list_1_built = Vec::new();
        self.push(ASTType::ListOptList(list_opt_list_1_built), context);
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// ListOpt: ; // Option<T>::None
    ///
    #[named]
    fn list_opt_1(&mut self, _parse_tree: &Tree<ParseTreeType<'t>>) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        self.push(ASTType::ListOpt(None), context);
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// Num: "0|[1-9][0-9]*" /* : u32 */;
    ///
    #[named]
    fn num(
        &mut self,
        num: &ParseTreeStackEntry<'t>,
        parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let num = num
            .token(parse_tree)?
            .symbol
            .parse::<u32>()
            .into_diagnostic()?;
        let num_built = NumBuilder::default().num(num).build().into_diagnostic()?;
        // Calling user action here
        self.user_grammar.num(&num_built)?;
        self.push(ASTType::Num(num_built), context);
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// TrailingComma: TrailingCommaOpt /* Option */;
    ///
    #[named]
    fn trailing_comma(
        &mut self,
        _trailing_comma_opt: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let trailing_comma_opt =
            if let Some(ASTType::TrailingCommaOpt(trailing_comma_opt)) = self.pop(context) {
                trailing_comma_opt
            } else {
                bail!("{}: Expecting ASTType::TrailingCommaOpt", context);
            };
        let trailing_comma_built = TrailingCommaBuilder::default()
            .trailing_comma_opt(trailing_comma_opt)
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.trailing_comma(&trailing_comma_built)?;
        self.push(ASTType::TrailingComma(trailing_comma_built), context);
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// TrailingCommaOpt: ","; // Option<T>::Some
    ///
    #[named]
    fn trailing_comma_opt_0(
        &mut self,
        comma: &ParseTreeStackEntry<'t>,
        parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let comma = comma.token(parse_tree)?.clone();
        let trailing_comma_opt_0_built = TrailingCommaOptBuilder::default()
            .comma(comma)
            .build()
            .into_diagnostic()?;
        self.push(
            ASTType::TrailingCommaOpt(Some(Box::new(trailing_comma_opt_0_built))),
            context,
        );
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// TrailingCommaOpt: ; // Option<T>::None
    ///
    #[named]
    fn trailing_comma_opt_1(&mut self, _parse_tree: &Tree<ParseTreeType<'t>>) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        self.push(ASTType::TrailingCommaOpt(None), context);
        Ok(())
    }
}

impl<'t> UserActionsTrait<'t> for ListGrammarAuto<'t, '_> {
    ///
    /// This function is implemented automatically for the user's item ListGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry<'t>],
        parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        match prod_num {
            0 => self.list(&children[0], &children[1], parse_tree),
            1 => self.list_opt_0(&children[0], &children[1], parse_tree),
            2 => self.list_opt_list_0(&children[0], &children[1], &children[2], parse_tree),
            3 => self.list_opt_list_1(parse_tree),
            4 => self.list_opt_1(parse_tree),
            5 => self.num(&children[0], parse_tree),
            6 => self.trailing_comma(&children[0], parse_tree),
            7 => self.trailing_comma_opt_0(&children[0], parse_tree),
            8 => self.trailing_comma_opt_1(parse_tree),
            _ => Err(miette!("Unhandled production number: {}", prod_num)),
        }
    }
}
