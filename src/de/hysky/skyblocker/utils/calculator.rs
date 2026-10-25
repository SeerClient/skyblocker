//! Ported from de.hysky.skyblocker.utils.Calculator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: private static final Pattern NUMBER_PATTERN = Pattern.compile("([_,\\d]+\\.?[_,\\d]*)([sekmbtq]?)");
pub static NUMBER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("([_,\\d]+\\.?[_,\\d]*)([sekmbtq]?)"));
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
pub static MAGNITUDE_VALUES: LazyLock<std::collections::HashMap<String, i64>> = LazyLock::new(|| {
    std::collections::HashMap::from([
        ("s".to_string(), 64i64),
        ("e".to_string(), 160i64),
        ("k".to_string(), 1_000i64),
        ("m".to_string(), 1_000_000i64),
        ("b".to_string(), 1_000_000_000i64),
        ("t".to_string(), 1_000_000_000_000i64),
        ("q".to_string(), 1_000_000_000_000_000i64),
    ])
});
//   java: private static final java.util.function.Function<String, @Nullable Operator> OPERATOR_MAP = StringRepresentable.createNameLookup(Operator.values(), op -> op.op);
/// java: private static final java.util.function.Function<String, @Nullable Operator> OPERATOR_MAP = StringRepresentable.createNameLookup(Operator.values(), op -> op.op);
pub static OPERATOR_OPERATOR_MAP: compat::MainThread<Arc<dyn Fn(String) -> Option<Operator>>> = compat::MainThread::new(|| {
    let lookup = mcrust::net::minecraft::util::string_representable::create_name_lookup(&Operator::VALUES, |op: &Operator| op.fields().op.clone());
    Arc::new(move |name: String| lookup(&name).copied())
});
//   java: private static final java.util.function.Function<String, @Nullable Function> FUNCTION_MAP = StringRepresentable.createNameLookup(Function.values(), func -> func.name);
/// java: private static final java.util.function.Function<String, @Nullable Function> FUNCTION_MAP = StringRepresentable.createNameLookup(Function.values(), func -> func.name);
pub static FUNCTION_FUNCTION_MAP: compat::MainThread<Arc<dyn Fn(String) -> Option<Function>>> = compat::MainThread::new(|| {
    let lookup = mcrust::net::minecraft::util::string_representable::create_name_lookup(&Function::VALUES, |func: &Function| func.fields().name.clone());
    Arc::new(move |name: String| lookup(&name).copied())
});
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
    pub fn lex(mut input: String) -> Vec<compat::Shared<dyn AbstractToken>> {
        let mut tokens: Vec<compat::Shared<dyn AbstractToken>> = Vec::new();
        input = (input.replace(" ", "")).to_lowercase().replace("x", "*");
        let mut i: i32 = 0;
        while i < compat::str_len(&input) {
            let __token = match compat::char_at(&input, i) {
                '+' | '-' | '*' | '/' | '%' | '^' => 'yield_2: {
                    let mut op: String = compat::char_at(&input, i).to_string();
                    let mut operator: Option<Operator> = (OPERATOR_OPERATOR_MAP.get_mut())(op.clone());
                    let Some(mut operator) = operator.clone() else {
                        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidOperatorError", vec![op.clone()]);
                    };
                    if !tokens.is_empty() && compat::read(&tokens[tokens.len() - 1], |__t| __t.token_type()) == TokenType::Operator {
                        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.duplicateOperatorError", Vec::new());
                    }
                    break 'yield_2 (compat::shared(OperatorToken::new(operator)) as compat::Shared<dyn AbstractToken>);
                }
                '(' => 'yield_3: {
                    if !tokens.is_empty() {
                        let mut last_type: TokenType = compat::read(&tokens[tokens.len() - 1], |__t| __t.token_type());
                        if last_type == TokenType::RParenthesis || last_type == TokenType::Number {
                            tokens.push((compat::shared(OperatorToken::new(Operator::Mult)) as compat::Shared<dyn AbstractToken>));
                        }
                    }
                    break 'yield_3 (compat::shared(Token::new(TokenType::LParenthesis, "(".to_string())) as compat::Shared<dyn AbstractToken>);
                }
                ')' => (compat::shared(Token::new(TokenType::RParenthesis, ")".to_string())) as compat::Shared<dyn AbstractToken>),
                '_' | ',' => {
                    CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidNumberError2", Vec::new());
                }
                '.' => {
                    CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidNumberError3", Vec::new());
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => 'yield_4: {
                    let mut number_matcher: compat::regex::Matcher = NUMBER_PATTERN.matcher(&compat::substring(&input, i));
                    if !number_matcher.find() {
                        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidCharacterError", vec![compat::substring(&input, i)]);
                    }
                    let mut end: i32 = number_matcher.end(0);
                    let mut number: String = compat::substring2(&input, i, i.wrapping_add(end));
                    i = i.wrapping_add(end.wrapping_sub(1));
                    break 'yield_4 (compat::shared(Token::new(TokenType::Number, number.clone())) as compat::Shared<dyn AbstractToken>);
                }
                'p' => 'yield_5: {
                    if !tokens.is_empty() && compat::read(&tokens[tokens.len() - 1], |__t| __t.token_type()) == TokenType::Number {
                        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidEquation", Vec::new());
                    }
                    if compat::substring2(&input, i, (compat::str_len(&input)).min(i.wrapping_add(5))) == "purse" {
                        i = i.wrapping_add(4);
                    }
                    break 'yield_5 (compat::shared(Token::new(TokenType::Number, ((Utils::get_purse()) as i64).to_string())) as compat::Shared<dyn AbstractToken>);
                }
                _ => 'yield_6: {
                    let mut func: String = (compat::regex::split_limit(&(compat::substring(&input, i)), "[ (]", 2))[(0) as usize].clone();
                    let mut function: Option<Function> = (FUNCTION_FUNCTION_MAP.get_mut())(func.clone());
                    let Some(mut function) = function.clone() else {
                        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidOperatorError", vec![func.clone()]);
                    };
                    if !tokens.is_empty() {
                        let mut last_type: TokenType = compat::read(&tokens[tokens.len() - 1], |__t| __t.token_type());
                        if last_type == TokenType::RParenthesis || last_type == TokenType::Number {
                            tokens.push((compat::shared(OperatorToken::new(Operator::Mult)) as compat::Shared<dyn AbstractToken>));
                        }
                    }
                    i = i.wrapping_add((compat::str_len(&func)).wrapping_sub(1));
                    break 'yield_6 (compat::shared(FunctionToken::new(function.clone())) as compat::Shared<dyn AbstractToken>);
                }
            };
            tokens.push(__token);
            i += 1;
        }
        return tokens;
    }

    /// java: private static List<AbstractToken<?>> shunt(List<AbstractToken<?>> tokens) throws CalculatorException
    pub fn shunt(tokens: &Vec<compat::Shared<dyn AbstractToken>>) -> Vec<compat::Shared<dyn AbstractToken>> {
        let mut operator_stack: std::collections::VecDeque<compat::Shared<dyn AbstractToken>> = std::collections::VecDeque::new();
        let mut output_queue: Vec<compat::Shared<dyn AbstractToken>> = Vec::new();
        for shunting_token in tokens.iter() {
            match compat::read(shunting_token, |__t| __t.token_type()) {
                TokenType::Number => {
                    output_queue.push(shunting_token.clone());
                }
                TokenType::Operator => {
                    let mut op: Operator = compat::read(shunting_token, |__t| token_value::<Operator>(__t));
                    let mut precedence: i32 = Calculator::get_precedence(op);
                    while !operator_stack.is_empty() {
                        let mut left_token: compat::Shared<dyn AbstractToken> = operator_stack.front().cloned().unwrap();
                        if compat::read(&left_token, |__t| __t.token_type()) == TokenType::LParenthesis {
                            break;
                        }
                        debug_assert!(compat::read(&left_token, |__t| __t.token_type()) == TokenType::Operator);
                        let mut left_precedence: i32 = Calculator::get_precedence(compat::read(&left_token, |__t| token_value::<Operator>(__t)));
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
                            CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.unbalancedParenthesisError", Vec::new());
                        }
                        let mut left_token: compat::Shared<dyn AbstractToken> = operator_stack.pop_front().expect("NoSuchElementException");
                        if compat::read(&left_token, |__t| __t.token_type()) == TokenType::LParenthesis {
                            break;
                        }
                        output_queue.push(left_token.clone());
                    }
                    if !operator_stack.is_empty() && compat::read(&operator_stack.front().cloned().unwrap(), |__t| __t.token_type()) == TokenType::Function {
                        output_queue.push(operator_stack.pop_front().expect("NoSuchElementException"));
                    }
                }
                _ => {}
            }
        }
        while !operator_stack.is_empty() {
            let mut left_token: compat::Shared<dyn AbstractToken> = operator_stack.pop_front().expect("NoSuchElementException");
            if compat::read(&left_token, |__t| __t.token_type()) == TokenType::LParenthesis {
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
    pub fn evaluate(tokens: &Vec<compat::Shared<dyn AbstractToken>>) -> f64 {
        let mut values: std::collections::VecDeque<f64> = std::collections::VecDeque::new();
        for token in tokens.iter() {
            match compat::read(token, |__t| __t.token_type()) {
                TokenType::Number => {
                    values.push_front(Calculator::calculate_value(&compat::read(token, |__t| token_value::<String>(__t))));
                }
                TokenType::Operator => {
                    if (values.len() as i32) < 2 {
                        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.missingValueError", Vec::new());
                    }
                    let mut right: f64 = values.pop_front().unwrap();
                    let mut left: f64 = values.pop_front().unwrap();
                    values.push_front(match compat::read(token, |__t| token_value::<Operator>(__t)) {
                        Operator::Add => (left + right),
                        Operator::Sub => (left - right),
                        Operator::Mult => (left * right),
                        Operator::Div => 'yield_3: {
                            if right == (0 as f64) {
                                CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.divisionByZeroError", vec![compat::fstr(left)]);
                            }
                            break 'yield_3 (left / right);
                        }
                        Operator::Mod => 'yield_4: {
                            if right == (0 as f64) {
                                CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.moduloByZeroError", vec![compat::fstr(left)]);
                            }
                            break 'yield_4 (left % right);
                        }
                        Operator::Pow => left.powf(right),
                        _ => unreachable!("switch expression: no case matched"),
                    });
                }
                TokenType::Function => {
                    if values.is_empty() {
                        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.missingValueError", Vec::new());
                    }
                    // java: values.push(((Function) token.value).function.apply(values.pollFirst()));
                    let function: Function = compat::read(token, |__t| token_value::<Function>(__t));
                    let value: f64 = values.pop_front().unwrap();
                    values.push_front((function.fields().function)(value));
                }
                TokenType::LParenthesis | TokenType::RParenthesis => {
                    CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.unbalancedParenthesisError", Vec::new());
                }
                _ => {}
            }
        }
        if values.is_empty() {
            CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.emptyEquationError", Vec::new());
        }
        return values.pop_front().expect("NoSuchElementException");
    }

    /// java: private static double calculateValue(String value) throws CalculatorException
    pub fn calculate_value(value: &str) -> f64 {
        let mut number_matcher: compat::regex::Matcher = NUMBER_PATTERN.matcher(&value.to_lowercase());
        if !number_matcher.matches() {
            CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidNumberError", vec![value.to_lowercase()]);
        }
        let mut number: f64 = (compat::regex::replace_all(&number_matcher.group(1).unwrap(), "[_,]", "")).trim().parse::<f64>().expect("NumberFormatException");
        let mut magnitude: String = number_matcher.group(2).unwrap();
        if !magnitude.is_empty() {
            if !(MAGNITUDE_VALUES.contains_key(&magnitude)) {
                CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidMagnitudeError", vec![magnitude.clone()]);
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

/// java: the virtual methods of AbstractToken (implemented by AbstractTokenBase and every subclass). Every list of
/// the class holds `AbstractToken<?>`: the value's type is erased and read back with Java's cast (`token_value`).
pub trait AbstractToken {
    /// java: public final TokenType type
    fn token_type(&self) -> TokenType;
    /// java: private final T value (erased)
    fn token_value(&self) -> &dyn std::any::Any;
    fn as_any(&self) -> &dyn std::any::Any;

}

/// `(T) token.value` - Java's checked cast of the erased value.
pub fn token_value<T: Clone + 'static>(token: &dyn AbstractToken) -> T {
    token.token_value().downcast_ref::<T>().cloned().expect("ClassCastException: AbstractToken.value")
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

impl AbstractToken for Token {
    fn token_type(&self) -> TokenType {
        self.base.r#type
    }
    fn token_value(&self) -> &dyn std::any::Any {
        &self.base.value
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

impl AbstractToken for OperatorToken {
    fn token_type(&self) -> TokenType {
        self.base.r#type
    }
    fn token_value(&self) -> &dyn std::any::Any {
        &self.base.value
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
        // java: this(op, false);
        this = OperatorFields::new_2(op, false);
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
    pub base: AbstractTokenBase<Function>,
}

impl std::ops::Deref for FunctionToken {
    type Target = AbstractTokenBase<Function>;
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
    pub fn new(mut value: Function) -> Self {
        let mut this = Self {
            base: AbstractTokenBase::<Function>::new(TokenType::Function, value.clone()),
        };
        this
    }

}

impl AbstractToken for FunctionToken {
    fn token_type(&self) -> TokenType {
        self.base.r#type
    }
    fn token_value(&self) -> &dyn std::any::Any {
        &self.base.value
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
        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError", vec![compat::fstr(val), "sqrt".to_string()]);
    }
    return val.sqrt();
})),
            FunctionFields::new("log".to_string(), Arc::new(move |val| {
    if val <= (0 as f64) {
        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError", vec![compat::fstr(val), "log".to_string()]);
    }
    return val.log10();
})),
            FunctionFields::new("lg".to_string(), Arc::new(move |val| {
    if val <= (0 as f64) {
        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError", vec![compat::fstr(val), "lg".to_string()]);
    }
    return (val.ln() / (2 as f64).ln());
})),
            FunctionFields::new("ln".to_string(), Arc::new(move |val| {
    if val <= (0 as f64) {
        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError", vec![compat::fstr(val), "ln".to_string()]);
    }
    return val.ln();
})),
            FunctionFields::new("factorial".to_string(), Arc::new(move |val| {
    if val < (0 as f64) || val > (170 as f64) {
        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError", vec![compat::fstr(val), "factorial".to_string()]);
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
        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError", vec![compat::fstr(val), "asin".to_string()]);
    }
    return val.asin().to_degrees();
})),
            FunctionFields::new("acos".to_string(), Arc::new(move |val| {
    if val < (-1 as f64) || val > (1 as f64) {
        CalculatorException::throw("skyblocker.config.uiAndVisuals.inputCalculator.invalidFunctionInputError", vec![compat::fstr(val), "acos".to_string()]);
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
            FunctionFields::new("round".to_string(), Arc::new(move |__a0| compat::java_round_f64(__a0) as f64)),
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
            name: name.clone(),
            function: function.clone(),
        };
        this
    }

}

/// java: @FunctionalInterface private interface CalculatorFunction
/// a functional interface: `apply(double val)` - a lambda / method reference is this closure type
pub type CalculatorFunction = Arc<dyn Fn(f64) -> f64 + Send + Sync>;

/// java: public static class CalculatorException extends Exception - `getMessage()` (a translation key) and `args`
/// (its translation arguments). Thrown as a panic carrying this value (`std::panic::panic_any`), so a `catch` reads
/// both back with `CalculatorException::of`.
#[derive(Clone, Debug)]
pub struct CalculatorException {
    pub message: String,
    pub args: Vec<String>,
}

impl CalculatorException {
    /// java: throw new CalculatorException(message, args...)
    pub fn throw(message: &str, args: Vec<String>) -> ! {
        std::panic::panic_any(CalculatorException { message: message.to_string(), args })
    }

    /// The exception a `catch (CalculatorException e)` receives from a caught panic; `None` for any other panic.
    pub fn of(payload: &(dyn std::any::Any + Send)) -> Option<&CalculatorException> {
        payload.downcast_ref::<CalculatorException>()
    }

    /// java: Component.translatable(e.getMessage(), e.args)
    pub fn to_component(&self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        mcrust::net::minecraft::network::chat::component::translatable_args(
            &self.message,
            self.args.iter().map(|a| mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(a.clone())).collect(),
        )
    }
}

impl std::fmt::Display for CalculatorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CalculatorException: {}", self.message)
    }
}
