use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::interpreter::VariableType::Int;
use crate::lexical::{Operator, Token};
use crate::parser::{Leaf, Node};

#[derive(Debug, Clone)]
pub struct Value
{
    name: String,
    value: VariableType,
}

#[derive(Debug, Clone)]
pub struct Array
{
    name: String,
    values: Vec<VariableType>,
}

#[derive(Debug, Clone)]
pub enum VariableType
{
    Int(i32),
    Float(f64),
}

pub enum Variable {
    Value(Value),
    Array(Array),
}


pub struct Interpreter
{
    root: Rc<RefCell<Node>>,
    variables: Vec<Variable>,
}

impl Interpreter
{
    pub fn new(root: Rc<RefCell<Node>>) -> Self
    {
        Interpreter
        {
            root,
            variables: Vec::new(),
        }
    }

    pub fn run(&mut self)
    {}

    fn interpret_node(&mut self, node: &Rc<RefCell<Node>>)
    {
        if let Some(val) = node.borrow().val()
        {
            match val
            {
                Leaf::Declaration(variable_type) =>
                    {
                        // node の左側から変数名を取得
                        if let Some(lhs) = node.borrow().lhs()
                        {
                            let identifier = self.identifier_name(lhs);

                            // 
                        }
                    }
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
        }
    }

    fn statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
    {
        if let Some(val) = node.borrow().val()
        {
            match val
            {
                Leaf::Operator(op) =>
                    {
                        if let Some((lhs, rhs))
                            = node.borrow().get_lhs_and_rhs()
                        {
                            return self.operator(op, lhs, rhs);
                        }
                    }
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
        }

        panic!("未対応のノードです");
    }


    fn operator(&mut self, op: &Operator, lhs: &Rc<RefCell<Node>>, rhs: &Rc<RefCell<Node>>) -> VariableType
    {
        let lhs = self.statement(lhs);
        let rhs = self.statement(rhs);

        match op
        {
            Operator::LogicalOr =>
                {
                    let result = self.logical_or(lhs, rhs);
                    result
                }
            Operator::LogicalAnd =>
                {
                    let result = self.logical_and(lhs, rhs);
                    result
                }
            Operator::Equal =>
                {
                    let result = self.equal(lhs, rhs);
                    result
                }
            Operator::NotEqual =>
                {
                    let result = self.not_equal(lhs, rhs);
                    result
                }
            Operator::LessThan =>
                {
                    let result = self.less_than(lhs, rhs);
                    result
                }
            Operator::GreaterThan =>
                {
                    let result = self.greater_than(lhs, rhs);
                    result
                }
            Operator::LessThanOrEqual =>
                {
                    let result = self.less_than_or_equal(lhs, rhs);
                    result
                }
            Operator::GreaterThanOrEqual =>
                {
                    let result = self.greater_than_or_equal(lhs, rhs);
                    result
                }
            Operator::Plus =>
                {
                    let result = self.add(lhs, rhs);
                    result
                }
            Operator::Minus =>
                {
                    let result = self.sub(lhs, rhs);
                    result
                }
            Operator::Multiply =>
                {
                    let result = self.mul(lhs, rhs);
                    result
                }
            Operator::Divide =>
                {
                    let result = self.div(lhs, rhs);
                    result
                }
            _ => {
                panic!("未対応の演算子です : {:?}", op);
            }
        }
    }

    // 加算演算子　'+'
    fn add(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs + rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    Int(lhs + rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs + rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs + rhs) as i32)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 減算演算子　'-'
    fn sub(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs - rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    Int(lhs - rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs - rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs - rhs) as i32)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 乗算演算子　'*'
    fn mul(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs * rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    Int(lhs * rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs * rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs * rhs) as i32)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 除算演算子　'/'
    fn div(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        // 右辺値が0の場合はエラー
        match rhs
        {
            VariableType::Int(val) if val == 0 =>
                {
                    panic!("0で割ることはできません");
                }
            VariableType::Float(val) if val == 0.0 =>
                {
                    panic!("0で割ることはできません");
                }
            _ => {}
        }

        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs / rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    Int(lhs / rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs / rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs / rhs) as i32)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 同値演算子　'=='
    fn equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs == rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs == rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs == rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs == rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 否定演算子　'!='
    fn not_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 小なり演算子　'<'
    fn less_than(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs < rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs < rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs < rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs < rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 大なり演算子　'>'
    fn greater_than(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs > rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs > rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs > rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs > rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 小なりイコール演算子　'<='
    fn less_than_or_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs <= rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs <= rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs <= rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs <= rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 大なりイコール演算子　'>='
    fn greater_than_or_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs >= rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs >= rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs >= rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs >= rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 論理和　'||'
    fn logical_or(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0 || rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0 || rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0.0 || rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0.0 || rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 論理積　'&&'
    fn logical_and(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0 && rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0 && rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0.0 && rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0.0 && rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    fn identifier_name(&self, node: &Rc<RefCell<Node>>) -> String
    {
        let mut identifier = String::new();
        if let Some(val) = node.borrow().val()
        {
            match val
            {
                Leaf::Identifier(name) => {
                    identifier = name.clone();
                }
                _ => {
                    panic!("識別子ではありません : {:?}", val);
                }
            }
        }
        identifier
    }
}