use std::process::exit;
use std::cell::RefCell;

struct Calculator {
    numbers: RefCell<Vec<f64>>,
    operator: RefCell<Vec<u8>>,
    expression: String,
}

impl Calculator {

    pub fn new(expr: String) -> Calculator {
        Calculator {
            numbers: RefCell::new(Vec::new()),
            operator: RefCell::new(Vec::new()),
            expression: expr,
        }
    }

    pub fn run(&self) -> Result<f64, String> {

        let mut ab: u8 = 0;
        let mut lo: usize = 0;
        let mut bk: u32 = 0;
        let expr = &self.expression;
        let bytes = self.expression.as_bytes();
        const PI: f64 = 3.141592653589793;

        let priority = |x: &u8| -> u8 {
            match x {
                b'+' | b'-' => 1,
                b'*' | b'/' | b'%' => 2,
                b'^' => 3,
                _ => exit(0),
            }
        };

        let computing = |x: &u8| -> Result<f64, String> {
            match x {
                b'+' => {
                    let c1 = self.numbers.borrow_mut().pop().unwrap();
                    let c2 = self.numbers.borrow_mut().pop().unwrap();
                    return Ok(c2 + c1);
                }

                b'-' => {
                    let c1 = self.numbers.borrow_mut().pop().unwrap();
                    let c2 = self.numbers.borrow_mut().pop().unwrap();
                    return Ok(c2 - c1);
                }

                b'*' => {
                    let c1 = self.numbers.borrow_mut().pop().unwrap();
                    let c2 = self.numbers.borrow_mut().pop().unwrap();
                    return Ok(c2 * c1);
                }

                b'/' => {
                    let c1 = self.numbers.borrow_mut().pop().unwrap();
                    let c2 = self.numbers.borrow_mut().pop().unwrap();
                    if c1 == 0.0 {
                        return Err("Divide By Zero".to_string());
                    }
                    return Ok(c2 / c1);
                }

                b'%' => {
                    let c1 = self.numbers.borrow_mut().pop().unwrap();
                    let c2 = self.numbers.borrow_mut().pop().unwrap();
                    if c1 == 0.0 {
                        return Err("Divide By Zero".to_string());
                    }
                    return Ok(c2 % c1);
                }

                b'^' => {
                    let c1 = self.numbers.borrow_mut().pop().unwrap();
                    let c2 = self.numbers.borrow_mut().pop().unwrap();
                    return Ok(c2.powf(c1));
                }

                _ => exit(0),
            }
        };

        for (index, &value) in bytes.iter().enumerate() {
            match value {
                b'0' ..= b'9' | b'.' => continue,

                ch @ b'+' | ch @ b'-' | ch @ b'*' | ch @ b'/' | ch @ b'%' | ch @ b'^' => {
                    if ab != b'A' {
                        let num = expr[lo..index].parse::<f64>().unwrap();   //将运算符前的数字取出来
                        self.numbers.borrow_mut().push(num);   //读取的数字进栈
                        ab = b'A';
                    }

                    while self.operator.borrow().len() != 0 && self.operator.borrow().last().unwrap() != &b'(' {
                        let p1 = priority(self.operator.borrow().last().unwrap());
                        let p2 = priority(&ch);
                        if p1 >= p2 {    //优先级比较
                            let res = computing(self.operator.borrow().last().unwrap());    //调用二元运算函数
                            match res {
                                Ok(_) => {
                                    self.numbers.borrow_mut().push(res.unwrap());    //运算结果进栈
                                    self.operator.borrow_mut().pop();    //运算符出栈
                                }
                                Err(_) => return res,
                            }
                        } else {
                            break
                        }
                    }

                    self.operator.borrow_mut().push(ch);     //运算符进栈
                    lo = index + 1;    //移动切片定位
                    ab = b'B';
                    continue
                }

                ch @ b'(' => {
                    if ab != b'A' {
                        self.operator.borrow_mut().push(ch);     //左括号直接进栈
                        lo = index + 1;   //移动切片定位
                        bk = bk + 1;
                        continue
                    }
                    return Err("Expression Error".to_string());
                }

                b')' => {
                    if ab != b'A' {
                        let num = expr[lo..index].parse::<f64>().unwrap();    //将运算符前的数字取出来
                        self.numbers.borrow_mut().push(num);    //读取的数字进栈
                        ab = b'A';
                    }

                    if bk > 0 && ab == b'A' {    //运算符栈中必须有左括号与右括号前必须是数字
                        while self.operator.borrow().last().unwrap() != &b'(' {    //遇到左括号时停止循环
                            let res = computing(&self.operator.borrow_mut().pop().unwrap());    //调用二元运算函数
                            match res {
                                Ok(_) => self.numbers.borrow_mut().push(res.unwrap()),    //运算结果进栈
                                Err(_) => return res,
                            }
                        }

                        self.operator.borrow_mut().pop();     //运算符出栈
                        lo = index + 1;     //移动切片定位
                        bk = bk-1;
                        continue
                    }

                    return Err("Expression Error".to_string());
                }

                b'=' => {
                    if ab != b'A' {
                        let num = expr[lo..index].parse::<f64>().unwrap();    //将运算符前的数字取出来
                        self.numbers.borrow_mut().push(num);     //读取的数字进栈
                        ab = b'A';
                    }

                    if bk > 0 {
                        return Err("Expression Error".to_string());
                    }

                    while self.operator.borrow().len() != 0 {     //直到运算符栈为空停止循环
                        let res = computing(&self.operator.borrow_mut().pop().unwrap());     //调用二元运算函数
                        match res {
                            Ok(_) => self.numbers.borrow_mut().push(res.unwrap()),    //运算结果进栈
                            Err(_) => return res,
                        }
                    }

                    let res = self.numbers.borrow_mut().pop().unwrap();     //清空最后一个数据栈
                    return Ok(res);
                }

                b'A' => {
                    if ab != b'A' {
                        let num = expr[lo..index].parse::<f64>().unwrap();     //将运算符前的数字取出来
                        self.numbers.borrow_mut().push(num);    //读取的数字进栈
                        ab = b'A';
                    }

                    if ab == b'A' {
                        let res = self.numbers.borrow_mut().pop().unwrap();
                        self.numbers.borrow_mut().push(res.abs());    //Abs(X) X的绝对值
                        lo = index + 1;    //移动切片定位
                        continue
                    }

                    return Err("Expression Error".to_string());
                }

                b'S' => {
                    if ab != b'A' {
                        let num = expr[lo..index].parse::<f64>().unwrap();    //将运算符前的数字取出来
                        self.numbers.borrow_mut().push(num);    //读取的数字进栈
                        ab = b'A';
                    }

                    if ab == b'A' {
                        let res = self.numbers.borrow_mut().pop().unwrap();
                        if res >= 0.0 {
                            self.numbers.borrow_mut().push(res.sqrt());    //Sqrt(X) X的平方根
                            lo = index + 1;     //移动切片定位
                            continue
                        }
                        return Err("Expression Error".to_string());
                    }
                    return Err("Expression Error".to_string());
                }

                b'P' => {
                    if ab != b'A' {     //标记符前面必须是字符或者为空
                        self.numbers.borrow_mut().push(PI);    //Pi常量进入数字栈
                        lo = index + 1;    //移动切片定位
                        ab = b'A';
                        continue
                    }
                    return Err("Expression Error".to_string());
                }

                _ => return Err("Operator Error".to_string()),
            }
        }

        Err("Possible Error".to_string())
    }
}

fn main() {
    let test = Calculator::new(String::from("10*(2/1-(6*2+(21.5A+3/5)-8*P/3)*23.5S)+8*2^2%3="));
    println!("= {}", test.run().unwrap());
}