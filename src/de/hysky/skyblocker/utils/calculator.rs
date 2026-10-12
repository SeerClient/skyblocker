//! Ported from de.hysky.skyblocker.utils.Calculator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: private static final Pattern NUMBER_PATTERN = Pattern.compile("([_,\\d]+\\.?[_,\\d]*)([sekmbtq]?)");
pub static NUMBER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("([_,\\d]+\\.?[_,\\d]*)([sekmbtq]?)"));
// TODO(fabric2seer): static call Object2LongMap.ofEntries(..)
//   java: private static final Object2LongMap<String> MAGNITUDE_VALUES = Object2LongMap.ofEntries(
//   java: 			Object2LongMap.entry("s", 64L),
//   java: 			Object2LongMap.entry("e", 160L),
//   java: 			Object2LongMap.entry("k", 1_000L),
//   java: 			Object2LongMap.entry("m", 1_000_000L),
//   java: 			Object2LongMap.entry("b", 1_000_000_000L),
//   java: 			Object2LongMap.entry("t", 1_000_000_000_000L),
//   java: 			Object2LongMap.entry("q", 1_000_000_000_000_000L)
//   java: 	);
/// java: private static final Object2LongMap<String> MAGNITUDE_VALUES = Object2LongMap.ofEntries( Object2LongMap.entry("s", 64L), Object2LongMap.entry("e", 160L), Object2LongMap.entry("k", 1_000L), Object2Long
pub static MAGNITUDE_VALUES: LazyLock<std::collections::HashMap<String, i64>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `MAGNITUDE_VALUES` - see the TODO above"));
// TODO(fabric2seer): field `op` of a T
//   java: private static final java.util.function.Function<String, @Nullable Operator> OPERATOR_MAP = StringRepresentable.createNameLookup(Operator.values(), op -> op.op);
/// java: private static final java.util.function.Function<String, @Nullable Operator> OPERATOR_MAP = StringRepresentable.createNameLookup(Operator.values(), op -> op.op);
pub static OPERATOR_OPERATOR_MAP: compat::MainThread<Arc<dyn Fn(String) -> Option<Operator>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `OPERATOR_MAP` - see the TODO above"));
// TODO(fabric2seer): field `name` of a T
//   java: private static final java.util.function.Function<String, @Nullable Function> FUNCTION_MAP = StringRepresentable.createNameLookup(Function.values(), func -> func.name);
/// java: private static final java.util.function.Function<String, @Nullable Function> FUNCTION_MAP = StringRepresentable.createNameLookup(Function.values(), func -> func.name);
pub static FUNCTION_FUNCTION_MAP: compat::MainThread<Arc<dyn Fn(String) -> Option<Arc<dyn Fn(compat::Object) -> compat::Object>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FUNCTION_MAP` - see the TODO above"));
/// java: @Serial private static final long serialVersionUID = -4480904461688998159L;
pub const CALCULATOR_EXCEPTION_SERIAL_VERSION_UID: i64 = -4480904461688998159;

/// java: public class Calculator
#[derive(Clone)]
pub struct Calculator {}

impl Calculator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static List<AbstractToken<?>> lex(String input) throws CalculatorException
    pub fn lex(mut input: String) -> Vec<compat::Shared<dyn AbstractToken<compat::Object>>> {
        let mut tokens: Vec<compat::Shared<dyn AbstractToken<compat::Object>>> = Vec::new();
        input = (input.replace(" ", "")).to_lowercase().replace("x", "*");
        let mut i: i32 = 0;
        while i < compat::str_len(&input) {
            tokens.push(match compat::char_at(&input, i) {
                '+' | '-' | '*' | '/' | '%' | '^' => 'yield_2: {
                    let mut op: String = compat::char_at(&input, i).to_string();
                    let mut operator: Option<Operator> = (OPERATOR_OPERATOR_MAP.get_mut())(op.clone());
                    let Some(mut operator) = operator.clone() else {
                        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidOperatorError");
                    };
                    if !tokens.is_empty() && compat::read(&tokens[tokens.len() - 1], |__t| __t.abstract_token_base().r#type) == TokenType::Operator {
                        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.duplicateOperatorError");
                    }
                    break 'yield_2 (compat::shared(OperatorToken::new(operator)) as compat::Shared<dyn AbstractToken<compat::Object>>);
                }
                '(' => 'yield_3: {
                    if !tokens.is_empty() {
                        let mut last_type: TokenType = compat::read(&tokens[tokens.len() - 1], |__t| __t.abstract_token_base().r#type);
                        if last_type == TokenType::RParenthesis || last_type == TokenType::Number {
                            tokens.push((compat::shared(OperatorToken::new(Operator::Mult)) as compat::Shared<dyn AbstractToken<compat::Object>>));
                        }
                    }
                    break 'yield_3 (compat::shared(Token::new(TokenType::LParenthesis, "(".to_string())) as compat::Shared<dyn AbstractToken<compat::Object>>);
                }
                ')' => (compat::shared(Token::new(TokenType::RParenthesis, ")".to_string())) as compat::Shared<dyn AbstractToken<compat::Object>>),
                '_' | ',' => {
                    panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidNumberError2");
                }
                '.' => {
                    panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidNumberError3");
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => 'yield_4: {
                    let mut number_matcher: compat::regex::Matcher = NUMBER_PATTERN.matcher(&compat::substring(&input, i));
                    if !number_matcher.find() {
                        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidCharacterError");
                    }
                    let mut end: i32 = number_matcher.end(0);
                    let mut number: String = compat::substring2(&input, i, i.wrapping_add(end));
                    i = i.wrapping_add(end.wrapping_sub(1));
                    break 'yield_4 (compat::shared(Token::new(TokenType::Number, number.clone())) as compat::Shared<dyn AbstractToken<compat::Object>>);
                }
                'p' => 'yield_5: {
                    if !tokens.is_empty() && compat::read(&tokens[tokens.len() - 1], |__t| __t.abstract_token_base().r#type) == TokenType::Number {
                        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidEquation");
                    }
                    if compat::substring2(&input, i, (compat::str_len(&input)).min(i.wrapping_add(5))) == "purse" {
                        i = i.wrapping_add(4);
                    }
                    break 'yield_5 (compat::shared(Token::new(TokenType::Number, ((Utils::get_purse()) as i64).to_string())) as compat::Shared<dyn AbstractToken<compat::Object>>);
                }
                _ => 'yield_6: {
                    let mut func: String = (compat::regex::split_limit(&(compat::substring(&input, i)), "[ (]", 2))[(0) as usize].clone();
                    let mut function: Option<Arc<dyn Fn(compat::Object) -> compat::Object>> = (FUNCTION_FUNCTION_MAP.get_mut())(func.clone());
                    let Some(mut function) = function.clone() else {
                        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidOperatorError");
                    };
                    if !tokens.is_empty() {
                        let mut last_type: TokenType = compat::read(&tokens[tokens.len() - 1], |__t| __t.abstract_token_base().r#type);
                        if last_type == TokenType::RParenthesis || last_type == TokenType::Number {
                            tokens.push((compat::shared(OperatorToken::new(Operator::Mult)) as compat::Shared<dyn AbstractToken<compat::Object>>));
                        }
                    }
                    i = i.wrapping_add((compat::str_len(&func)).wrapping_sub(1));
                    break 'yield_6 (compat::shared(FunctionToken::new(function.clone())) as compat::Shared<dyn AbstractToken<compat::Object>>);
                }
            });
            i += 1;
        }
        return tokens;
    }

    /// java: private static List<AbstractToken<?>> shunt(List<AbstractToken<?>> tokens) throws CalculatorException
    pub fn shunt(tokens: &Vec<compat::Shared<dyn AbstractToken<compat::Object>>>) -> Vec<compat::Shared<dyn AbstractToken<compat::Object>>> {
        let mut operator_stack: std::collections::VecDeque<compat::Shared<dyn AbstractToken<compat::Object>>> = std::collections::VecDeque::new();
        let mut output_queue: Vec<compat::Shared<dyn AbstractToken<compat::Object>>> = Vec::new();
        for shunting_token in tokens.iter() {
            match compat::read(shunting_token, |__t| __t.abstract_token_base().r#type) {
                TokenType::Number => {
                    output_queue.push(shunting_token.clone());
                }
                TokenType::Operator => {
                    let mut op: Operator = compat::read(shunting_token, |__t| __t.abstract_token_base().value.clone());
                    let mut precedence: i32 = Calculator::get_precedence(op);
                    while !operator_stack.is_empty() {
                        let mut left_token: compat::Shared<dyn AbstractToken<compat::Object>> = operator_stack.front().cloned().unwrap();
                        if compat::read(&left_token, |__t| __t.abstract_token_base().r#type) == TokenType::LParenthesis {
                            break;
                        }
                        debug_assert!(compat::read(&left_token, |__t| __t.abstract_token_base().r#type) == TokenType::Operator);
                        let mut left_precedence: i32 = Calculator::get_precedence(compat::read(&left_token, |__t| __t.abstract_token_base().value.clone()));
                        if left_precedence > precedence || left_precedence == precedence && !(op.fields().right_associative) {
                            output_queue.push(operator_stack.pop_front().expect("NoSuchElementException"));
                            continue;
                        }
                        break;
                    }
                    operator_stack.push_front(shunting_token.clone());
                }
                TokenType::Function | TokenType::LParenthesis => {
                    operator_stack.push_front(shunting_token.clone());
                }
                TokenType::RParenthesis => {
                    loop {
                        if operator_stack.is_empty() {
                            panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.unbalancedParenthesisError");
                        }
                        let mut left_token: compat::Shared<dyn AbstractToken<compat::Object>> = operator_stack.pop_front().expect("NoSuchElementException");
                        if compat::read(&left_token, |__t| __t.abstract_token_base().r#type) == TokenType::LParenthesis {
                            break;
                        }
                        output_queue.push(left_token.clone());
                    }
                    if !operator_stack.is_empty() && compat::read(&operator_stack.front().cloned().unwrap(), |__t| __t.abstract_token_base().r#type) == TokenType::Function {
                        output_queue.push(operator_stack.pop_front().expect("NoSuchElementException"));
                    }
                }
                _ => {}
            }
        }
        while !operator_stack.is_empty() {
            let mut left_token: compat::Shared<dyn AbstractToken<compat::Object>> = operator_stack.pop_front().expect("NoSuchElementException");
            if compat::read(&left_token, |__t| __t.abstract_token_base().r#type) == TokenType::LParenthesis {
                continue;
            }
            output_queue.push(left_token.clone());
        }
        return output_queue.iter().cloned().collect::<Vec<_>>();
    }

    /// java: private static int getPrecedence(Operator operator)
    pub fn get_precedence(mut operator: Operator) -> i32 {
        return match operator {
            Operator::Add | Operator::Sub => 0,
            Operator::Mult | Operator::Div | Operator::Mod => 1,
            Operator::Pow => 2,
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: private static double evaluate(List<AbstractToken<?>> tokens) throws CalculatorException
    pub fn evaluate(tokens: &Vec<compat::Shared<dyn AbstractToken<compat::Object>>>) -> f64 {
        let mut values: std::collections::VecDeque<f64> = std::collections::VecDeque::new();
        for token in tokens.iter() {
            match compat::read(token, |__t| __t.abstract_token_base().r#type) {
                TokenType::Number => {
                    values.push_front(Calculator::calculate_value(&compat::read(token, |__t| __t.abstract_token_base().value.clone())));
                }
                TokenType::Operator => {
                    if (values.len() as i32) < 2 {
                        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.missingValueError");
                    }
                    let mut right: f64 = values.pop_front().unwrap();
                    let mut left: f64 = values.pop_front().unwrap();
                    values.push_front(match compat::read(token, |__t| __t.abstract_token_base().value.clone()) {
                        Operator::Add => (left + right),
                        Operator::Sub => (left - right),
                        Operator::Mult => (left * right),
                        Operator::Div => 'yield_3: {
                            if right == (0 as f64) {
                                panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.divisionByZeroError");
                            }
                            // TODO(fabric2seer): call to `yield(..)` (not a method of this class)
                            //   java: yield (left / right);
                        }
                        Operator::Mod => 'yield_4: {
                            if right == (0 as f64) {
                                panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.moduloByZeroError");
                            }
                            // TODO(fabric2seer): call to `yield(..)` (not a method of this class)
                            //   java: yield (left % right);
                        }
                        Operator::Pow => left.powf(right),
                        _ => unreachable!("switch expression: no case matched"),
                    });
                }
                TokenType::Function => {
                    if values.is_empty() {
                        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.missingValueError");
                    }
                    // TODO(fabric2seer): field `function` of a Function
                    //   java: values.push(((Function) token.value).function.apply(values.pollFirst()));
                }
                TokenType::LParenthesis | TokenType::RParenthesis => {
                    panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.unbalancedParenthesisError");
                }
                _ => {}
            }
        }
        if values.is_empty() {
            panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.emptyEquationError");
        }
        return values.pop_front().expect("NoSuchElementException");
    }

    /// java: private static double calculateValue(String value) throws CalculatorException
    pub fn calculate_value(value: &str) -> f64 {
        let mut number_matcher: compat::regex::Matcher = NUMBER_PATTERN.matcher(&value.to_lowercase());
        if !number_matcher.matches() {
            panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidNumberError");
        }
        let mut number: f64 = (compat::regex::replace_all(&number_matcher.group(1).unwrap(), "[_,]", "")).trim().parse::<f64>().expect("NumberFormatException");
        let mut magnitude: String = number_matcher.group(2).unwrap();
        if !magnitude.is_empty() {
            if !(MAGNITUDE_VALUES.contains_key(&magnitude)) {
                panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidMagnitudeError");
            }
            number = (number * (MAGNITUDE_VALUES.get(&magnitude).copied().unwrap_or_default() as f64));
        }
        return number;
    }

    /// java: public static double calculate(String equation) throws CalculatorException
    pub fn calculate(mut equation: String) -> f64 {
        if equation.starts_with(".") {
            equation = format!("0{}", equation);
        }
        return Calculator::evaluate(&Calculator::shunt(&Calculator::lex(equation.clone())));
    }

}

/// java: public enum TokenType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenType {
    Number,
    Operator,
    Function,
    LParenthesis,
    RParenthesis,
}

impl TokenType {
    pub const VALUES: [TokenType; 5] = [TokenType::Number, TokenType::Operator, TokenType::Function, TokenType::LParenthesis, TokenType::RParenthesis];

    /// java: values()
    pub fn values() -> Vec<TokenType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TokenType::Number => "NUMBER",
            TokenType::Operator => "OPERATOR",
            TokenType::Function => "FUNCTION",
            TokenType::LParenthesis => "L_PARENTHESIS",
            TokenType::RParenthesis => "R_PARENTHESIS",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TokenType {
        match name {
            "NUMBER" => TokenType::Number,
            "OPERATOR" => TokenType::Operator,
            "FUNCTION" => TokenType::Function,
            "L_PARENTHESIS" => TokenType::LParenthesis,
            "R_PARENTHESIS" => TokenType::RParenthesis,
            _ => panic!("IllegalArgumentException: No enum constant TokenType.{name}"),
        }
    }

}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: public abstract static class AbstractToken<T>
#[derive(Clone)]
pub struct AbstractTokenBase<T> {
    /// java: public final TokenType type;
    pub r#type: TokenType,
    /// java: private final T value;
    pub value: T,
}

impl<T: Clone + 'static> AbstractTokenBase<T> {
    /// java: public AbstractToken(TokenType type, T value)
    pub fn new(mut r#type: TokenType, mut value: T) -> Self {
        let __init_type = r#type;
        let __init_value = value.clone();
        let mut this = Self {
            r#type: __init_type,
            value: __init_value,
        };
        this
    }

}

/// java: the virtual methods of AbstractToken (implemented by AbstractTokenBase and every subclass)
pub trait AbstractToken<T> {
    fn abstract_token_base(&self) -> &AbstractTokenBase<T>;
    fn abstract_token_base_mut(&mut self) -> &mut AbstractTokenBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

}

/// java: public static class Token extends AbstractToken<String>
#[derive(Clone)]
pub struct Token {
    /// java: extends AbstractToken
    pub base: AbstractTokenBase<String>,
}

impl std::ops::Deref for Token {
    type Target = AbstractTokenBase<String>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Token {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Token {
    /// java: public Token(TokenType type, String value)
    pub fn new(mut r#type: TokenType, mut value: String) -> Self {
        let mut this = Self {
            base: AbstractTokenBase::<String>::new(r#type, value.clone()),
        };
        this
    }

}

impl AbstractToken<String> for Token {
    fn abstract_token_base(&self) -> &AbstractTokenBase<String> {
        &self.base
    }
    fn abstract_token_base_mut(&mut self) -> &mut AbstractTokenBase<String> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class OperatorToken extends AbstractToken<Operator>
#[derive(Clone)]
pub struct OperatorToken {
    /// java: extends AbstractToken
    pub base: AbstractTokenBase<Operator>,
}

impl std::ops::Deref for OperatorToken {
    type Target = AbstractTokenBase<Operator>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OperatorToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OperatorToken {
    /// java: public OperatorToken(Operator operator)
    pub fn new(mut operator: Operator) -> Self {
        let mut this = Self {
            base: AbstractTokenBase::<Operator>::new(TokenType::Operator, operator),
        };
        this
    }

}

impl AbstractToken<crate::de::hysky::skyblocker::utils::calculator::Operator> for OperatorToken {
    fn abstract_token_base(&self) -> &AbstractTokenBase<crate::de::hysky::skyblocker::utils::calculator::Operator> {
        &self.base
    }
    fn abstract_token_base_mut(&mut self) -> &mut AbstractTokenBase<crate::de::hysky::skyblocker::utils::calculator::Operator> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public enum Operator implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Pow,
}

impl Operator {
    pub const VALUES: [Operator; 6] = [Operator::Add, Operator::Sub, Operator::Mult, Operator::Div, Operator::Mod, Operator::Pow];

    /// java: values()
    pub fn values() -> Vec<Operator> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Operator::Add => "ADD",
            Operator::Sub => "SUB",
            Operator::Mult => "MULT",
            Operator::Div => "DIV",
            Operator::Mod => "MOD",
            Operator::Pow => "POW",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Operator {
        match name {
            "ADD" => Operator::Add,
            "SUB" => Operator::Sub,
            "MULT" => Operator::Mult,
            "DIV" => Operator::Div,
            "MOD" => Operator::Mod,
            "POW" => Operator::Pow,
            _ => panic!("IllegalArgumentException: No enum constant Operator.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static OperatorFields {
        static FIELDS: LazyLock<Vec<OperatorFields>> = LazyLock::new(|| vec![
            OperatorFields::new("+".to_string()),
            OperatorFields::new("-".to_string()),
            OperatorFields::new("*".to_string()),
            OperatorFields::new("/".to_string()),
            OperatorFields::new("%".to_string()),
            OperatorFields::new_2("^".to_string(), true),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().op.clone();
    }

}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Operator constant
pub struct OperatorFields {
    pub op: String,
    pub right_associative: bool,
}

impl OperatorFields {
    fn new(mut op: String) -> Self {
        let mut this = Self {
            op: Default::default(),
            right_associative: false,
        };
        // TODO(fabric2seer): this statement
        //   java: this(op, false);
        this
    }

    fn new_2(mut op: String, mut right_associative: bool) -> Self {
        let mut this = Self {
            op: Default::default(),
            right_associative: false,
        };
        this.op = op.clone();
        this.right_associative = right_associative;
        this
    }

}

/// java: public static class FunctionToken extends AbstractToken<Function>
#[derive(Clone)]
pub struct FunctionToken {
    /// java: extends AbstractToken
    pub base: AbstractTokenBase<Arc<dyn Fn(compat::Object) -> compat::Object>>,
}

impl std::ops::Deref for FunctionToken {
    type Target = AbstractTokenBase<Arc<dyn Fn(compat::Object) -> compat::Object>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FunctionToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FunctionToken {
    /// java: public FunctionToken(Function value)
    pub fn new(mut value: Arc<dyn Fn(compat::Object) -> compat::Object>) -> Self {
        let mut this = Self {
            base: AbstractTokenBase::<Arc<dyn Fn(compat::Object) -> compat::Object>>::new(TokenType::Function, value.clone()),
        };
        this
    }

}

impl AbstractToken<Arc<dyn Fn(compat::Object) -> compat::Object>> for FunctionToken {
    fn abstract_token_base(&self) -> &AbstractTokenBase<Arc<dyn Fn(compat::Object) -> compat::Object>> {
        &self.base
    }
    fn abstract_token_base_mut(&mut self) -> &mut AbstractTokenBase<Arc<dyn Fn(compat::Object) -> compat::Object>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public enum Function implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Function {
    Sqrt,
    Log,
    Lg,
    Ln,
    Factorial,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Abs,
    Floor,
    Ceil,
    Round,
}

impl Function {
    pub const VALUES: [Function; 18] = [Function::Sqrt, Function::Log, Function::Lg, Function::Ln, Function::Factorial, Function::Sin, Function::Cos, Function::Tan, Function::Asin, Function::Acos, Function::Atan, Function::Sinh, Function::Cosh, Function::Tanh, Function::Abs, Function::Floor, Function::Ceil, Function::Round];

    /// java: values()
    pub fn values() -> Vec<Function> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Function::Sqrt => "SQRT",
            Function::Log => "LOG",
            Function::Lg => "LG",
            Function::Ln => "LN",
            Function::Factorial => "FACTORIAL",
            Function::Sin => "SIN",
            Function::Cos => "COS",
            Function::Tan => "TAN",
            Function::Asin => "ASIN",
            Function::Acos => "ACOS",
            Function::Atan => "ATAN",
            Function::Sinh => "SINH",
            Function::Cosh => "COSH",
            Function::Tanh => "TANH",
            Function::Abs => "ABS",
            Function::Floor => "FLOOR",
            Function::Ceil => "CEIL",
            Function::Round => "ROUND",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Function {
        match name {
            "SQRT" => Function::Sqrt,
            "LOG" => Function::Log,
            "LG" => Function::Lg,
            "LN" => Function::Ln,
            "FACTORIAL" => Function::Factorial,
            "SIN" => Function::Sin,
            "COS" => Function::Cos,
            "TAN" => Function::Tan,
            "ASIN" => Function::Asin,
            "ACOS" => Function::Acos,
            "ATAN" => Function::Atan,
            "SINH" => Function::Sinh,
            "COSH" => Function::Cosh,
            "TANH" => Function::Tanh,
            "ABS" => Function::Abs,
            "FLOOR" => Function::Floor,
            "CEIL" => Function::Ceil,
            "ROUND" => Function::Round,
            _ => panic!("IllegalArgumentException: No enum constant Function.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static FunctionFields {
        static FIELDS: LazyLock<Vec<FunctionFields>> = LazyLock::new(|| vec![
            FunctionFields::new("sqrt".to_string(), Arc::new(move |val| {
    if val < (0 as f64) {
        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError");
    }
    return val.sqrt();
})),
            FunctionFields::new("log".to_string(), Arc::new(move |val| {
    if val <= (0 as f64) {
        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError");
    }
    return val.log10();
})),
            FunctionFields::new("lg".to_string(), Arc::new(move |val| {
    if val <= (0 as f64) {
        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError");
    }
    return (val.ln() / (2 as f64).ln());
})),
            FunctionFields::new("ln".to_string(), Arc::new(move |val| {
    if val <= (0 as f64) {
        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError");
    }
    return val.ln();
})),
            FunctionFields::new("factorial".to_string(), Arc::new(move |val| {
    if val < (0 as f64) || val > (170 as f64) {
        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError");
    }
    let mut result: f64 = ((1) as f64) as f64;
    for i in 2..=((val) as i32) {
        result = (result * (i as f64));
    }
    return result;
})),
            FunctionFields::new("sin".to_string(), Arc::new(move |val| val.to_radians().sin())),
            FunctionFields::new("cos".to_string(), Arc::new(move |val| val.to_radians().cos())),
            FunctionFields::new("tan".to_string(), Arc::new(move |val| val.to_radians().tan())),
            FunctionFields::new("asin".to_string(), Arc::new(move |val| {
    if val < (-1 as f64) || val > (1 as f64) {
        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError");
    }
    return val.asin().to_degrees();
})),
            FunctionFields::new("acos".to_string(), Arc::new(move |val| {
    if val < (-1 as f64) || val > (1 as f64) {
        panic!("CalculatorException: {}", "skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError");
    }
    return val.acos().to_degrees();
})),
            FunctionFields::new("atan".to_string(), Arc::new(move |val| val.atan().to_degrees())),
            FunctionFields::new("sinh".to_string(), Arc::new(move |__a0| __a0.sinh())),
            FunctionFields::new("cosh".to_string(), Arc::new(move |__a0| __a0.cosh())),
            FunctionFields::new("tanh".to_string(), Arc::new(move |val| {
    if val > (20 as f64) {
        return 1.0;
    } else if val < (-20 as f64) {
        return -1.0;
    }
    return val.tanh();
})),
            FunctionFields::new("abs".to_string(), Arc::new(move |__a0| __a0.abs())),
            FunctionFields::new("floor".to_string(), Arc::new(move |__a0| __a0.floor())),
            FunctionFields::new("ceil".to_string(), Arc::new(move |__a0| __a0.ceil())),
            FunctionFields::new("round".to_string(), Arc::new(move |__a0| compat::java_round_f64(__a0))),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Function constant
pub struct FunctionFields {
    pub name: String,
    pub function: CalculatorFunction,
}

impl FunctionFields {
    fn new(mut name: String, function: CalculatorFunction) -> Self {
        let mut this = Self {
            name: Default::default(),
            function: Default::default(),
        };
        this.name = name.clone();
        this.function = function.clone();
        this
    }

}

/// java: @FunctionalInterface private interface CalculatorFunction
/// a functional interface: `apply(double val)` - a lambda / method reference is this closure type
pub type CalculatorFunction = Arc<dyn Fn(f64) -> f64>;

/// java: public static class CalculatorException extends Exception
pub struct CalculatorException {
    /// java: extends java.lang.Exception
    pub base: crate::unresolved::Exception,
    /// java: public final Object[] args;
    pub args: Vec<compat::Object>,
}

impl std::ops::Deref for CalculatorException {
    type Target = crate::unresolved::Exception;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CalculatorException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CalculatorException {
    /// java: public CalculatorException(@Translatable String message, Object... args)
    pub fn new(mut message: String, mut args: Vec<compat::Object>) -> Self {
        let __init_args = args.clone();
        // TODO(fabric2seer): `new Exception(..)`: the port has no `Exception::new`
        //   java: super(message);
        let mut this: Self = todo!("constructor");
        this
    }

}
