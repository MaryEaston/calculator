use num::*;
use rand::Rng;

use yew::prelude::*;

enum Msg {
    N1,N2,N3,N4,N5,N6,N7,N8,N9,N0,

    Clear,

    // 定数
    E,Pi,Rand,

    // 状態
    Point,Second,

    // 単項演算子
    Neg,Pct,Square,Cubic,Exp,Exp10,Rec,Sqrt,Root3,Ln,Log10,Sin,Cos,Tan,Sinh,Cosh,Tanh,

    // 二項演算子
    Add,Sub,Mul,Div,Pow,Root,EE,

    Eq,
}

enum State {
    Point,
    Second,
}

struct Model {
    value: f64,
    current_initd: bool,
    current: f64,
    op:fn(&mut Self) -> f64,
    state: Vec<State>,
}

impl Model{
    fn number(&mut self,num: i64){
        if self.current_initd {
            if self.state.iter().any(|e| match e {State::Point => true, _ => false}) {
                self.current = num as f64 / 10.0;
            }else{
                self.current = num as f64;
            }
            self.current_initd = false;
        } else{
            if self.state.iter().any(|e| match e {State::Point => true, _ => false}) {
                self.current = (self.current * 10.0 + num as f64) / 10.0;
            }else{
                self.current = self.current * 10.0 + num as f64;
            }
        }
    }

    fn constant(&mut self,num: f64){
        self.current = num;
        if self.current_initd {
            self.current_initd = false;
        }
    }

    fn unary_op(&mut self,op: fn(f64) -> f64){
        if self.current_initd{
            self.value = op(self.value);
        }else{
            self.value = op(self.current);
        }
        self.current = 0.0;
        self.current_initd = true;
    }
    fn neg(x: f64) -> f64{
        -x
    }
    fn pct(x: f64) -> f64{
        x / 100.0
    }
    fn square(x: f64) -> f64{
        x.powf(2.0)
    }
    fn cubic(x: f64) -> f64{
        x.powf(3.0)
    }
    fn exp(x: f64) -> f64{
        x.exp()
    }
    fn exp10(x: f64) -> f64{
        let ten = 10.0;
        ten.powf(x)
    }
    fn rec(x: f64) -> f64{
        1.0 / x
    }
    fn sqrt(x: f64) -> f64{
        x.sqrt()
    }
    fn root3(x: f64) -> f64{
        x.powf(1.0 / 3.0)
    }
    fn ln(x: f64) -> f64{
        x.ln()
    }
    fn log10(x: f64) -> f64{
        x.log10()
    }
    fn fact(x: f64) -> f64{
        0.0
    }
    fn sin(x: f64) -> f64{
        x.sin()
    }
    fn cos(x: f64) -> f64{
        x.cos()
    }
    fn tan(x: f64) -> f64{
        x.tan()
    }
    fn sinh(x: f64) -> f64{
        x.sinh()
    }
    fn cosh(x: f64) -> f64{
        x.cosh()
    }
    fn tanh(x: f64) -> f64{
        x.tanh()
    }

    fn binary_op(&mut self,op: fn(&mut Model) -> f64){
        if !self.current_initd {
            self.value = (self.op)(self);
        }
        self.op = op;
        self.current = 0.0;
        self.current_initd = true;
    }
    fn add(&mut self) -> f64{
        self.value + self.current
    }
    fn sub(&mut self) -> f64{
        self.value - self.current
    }
    fn mul(&mut self) -> f64{
        self.value * self.current
    }
    fn div(&mut self) -> f64{
        self.value / self.current
    }
    fn pow(&mut self) -> f64{
        self.value.powf(self.current)
    }
    fn root(&mut self) -> f64{
        self.value.powf(1.0 / self.current)
    }
    fn ee(&mut self) -> f64{
        let ten = 10.0;
        self.value * ten.powf(self.current)
    }

    fn clear(&mut self) -> f64{
        self.current
    }
    fn eq(&mut self) -> f64{
        self.value
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0.0,
            current_initd: true,
            current: 0.0,
            op: Self::clear,
            state: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::N1 => {
                self.number(1);
                true
            },
            Msg::N2 => {
                self.number(2);
                true
            },
            Msg::N3 => {
                self.number(3);
                true
            },
            Msg::N4 => {
                self.number(4);
                true
            },
            Msg::N5 => {
                self.number(5);
                true
            },
            Msg::N6 => {
                self.number(6);
                true
            },
            Msg::N7 => {
                self.number(7);
                true
            },
            Msg::N8 => {
                self.number(8);
                true
            },
            Msg::N9 => {
                self.number(9);
                true
            },
            Msg::N0 => {
                self.number(0);
                true
            },

            Msg::Clear => {
                if self.current_initd{
                    self.op = Self::clear;
                    self.value = 0.0;
                }else{
                    self.op = Self::clear;
                    self.current = 0.0;
                    self.current_initd = true;
                }
                true
            },

            // 定数
            Msg::E => {
                self.constant(2.718281828459045);
                true
            }
            Msg::Pi => {
                self.constant(3.141592653589793);
                true
            }
            Msg::Rand => {
                let mut rng = rand::thread_rng();
                self.constant(rng.gen());
                true
            }

            // 状態
            Msg::Point => {
                self.state.push(State::Point);
                true
            }
            Msg::Second => {
                self.state.push(State::Second);
                true
            }

            // 単項演算子
            Msg::Neg =>{
                self.unary_op(Self::neg);
                true
            },
            Msg::Pct => {
                self.unary_op(Self::pct);
                true
            },
            Msg::Square => {
                self.unary_op(Self::square);
                true
            },
            Msg::Cubic => {
                self.unary_op(Self::cubic);
                true
            },
            Msg::Exp => {
                self.unary_op(Self::exp);
                true
            },
            Msg::Exp10 => {
                self.unary_op(Self::exp10);
                true
            },
            Msg::Rec => {
                self.unary_op(Self::rec);
                true
            },
            Msg::Sqrt => {
                self.unary_op(Self::sqrt);
                true
            },
            Msg::Root3 => {
                self.unary_op(Self::root3);
                true
            },
            Msg::Ln => {
                self.unary_op(Self::ln);
                true
            },
            Msg::Log10 => {
                self.unary_op(Self::log10);
                true
            },
            Msg::Sin => {
                self.unary_op(Self::sin);
                true
            }
            Msg::Cos => {
                self.unary_op(Self::cos);
                true
            }
            Msg::Tan => {
                self.unary_op(Self::tan);
                true
            }
            Msg::Sinh => {
                self.unary_op(Self::sinh);
                true
            }
            Msg::Cosh => {
                self.unary_op(Self::cosh);
                true
            }
            Msg::Tanh => {
                self.unary_op(Self::tanh);
                true
            }

            // 二項演算子
            Msg::Add => {
                self.binary_op(Self::add);
                true
            },
            Msg::Sub => {
                self.binary_op(Self::sub);
                true
            },
            Msg::Mul => {
                self.binary_op(Self::mul);
                true
            },
            Msg::Div => {
                self.binary_op(Self::div);
                true
            },
            Msg::Pow => {
                self.binary_op(Self::pow);
                true
            },
            Msg::Root => {
                self.binary_op(Self::root);
                true
            },
            Msg::EE => {
                self.binary_op(Self::ee);
                true
            }

            Msg::Eq => {
                if !self.current_initd {
                    self.value = (self.op)(self);
                }
                self.op = Self::eq;
                self.current = 0.0;
                self.current_initd = true;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div id="app">
                <div id="display">
                    <p class="debug">{"hello value="} {self.value} {" | current="} {self.current}</p>
                    <p id="result">{ if self.current_initd {self.value} else {self.current} }</p>
                </div>
                <div id="function">
                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "(" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ ")" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "mc" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "m+" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "m-" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "mr" }</button>

                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "2" } <sup>{"nd"}</sup></button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Square)}>{ "x" }<sup>{"2"}</sup></button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Cubic)}>{ "x" }<sup>{"3"}</sup></button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Pow)}>{ "x" }<sup>{"y"}</sup></button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Exp)}>{ "e" }<sup>{"x"}</sup></button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Exp10)}>{ "10" }<sup>{"x"}</sup></button>

                    <button class="normal func" onclick={link.callback(|_| Msg::Rec)}>{ "1/x" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Sqrt)}>{ "√x" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Root3)}><sup>{"3"}</sup>{ "√x" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Root)}><sup>{"y"}</sup>{ "√x" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Ln)}>{ "ln" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Log10)}>{ "log" }<sub>{"10"}</sub></button>

                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "x!" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Sin)}>{ "sin" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Cos)}>{ "cos" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Tan)}>{ "tan" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::E)}>{ "e" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::EE)}>{ "EE" }</button>

                    <button class="normal func" onclick={link.callback(|_| Msg::Clear)}>{ "Rad" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Sinh)}>{ "sinh" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Cosh)}>{ "cosh" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Tanh)}>{ "tanh" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Pi)}>{ "π" }</button>
                    <button class="normal func" onclick={link.callback(|_| Msg::Rand)}>{ "Rand" }</button>
                </div>
                <div id="input">
                    <button class="normal util" onclick={link.callback(|_| Msg::Clear)}>{ "C" }</button>
                    <button class="normal util" onclick={link.callback(|_| Msg::Neg)}>{ "±" }</button>
                    <button class="normal util" onclick={link.callback(|_| Msg::Pct)}>{ "%" }</button>
                    <button class="normal op" onclick={link.callback(|_| Msg::Div)}>{ "÷" }</button>

                    <button class="normal number" onclick={link.callback(|_| Msg::N7)}>{ "7" }</button>
                    <button class="normal number" onclick={link.callback(|_| Msg::N8)}>{ "8" }</button>
                    <button class="normal number" onclick={link.callback(|_| Msg::N9)}>{ "9" }</button>
                    <button class="normal op" onclick={link.callback(|_| Msg::Mul)}>{ "×" }</button>

                    <button class="normal number" onclick={link.callback(|_| Msg::N4)}>{ "4" }</button>
                    <button class="normal number" onclick={link.callback(|_| Msg::N5)}>{ "5" }</button>
                    <button class="normal number" onclick={link.callback(|_| Msg::N6)}>{ "6" }</button>
                    <button class="normal op" onclick={link.callback(|_| Msg::Sub)}>{ "-" }</button>

                    <button class="normal number" onclick={link.callback(|_| Msg::N1)}>{ "1" }</button>
                    <button class="normal number" onclick={link.callback(|_| Msg::N2)}>{ "2" }</button>
                    <button class="normal number" onclick={link.callback(|_| Msg::N3)}>{ "3" }</button>
                    <button class="normal op" onclick={link.callback(|_| Msg::Add)}>{ "+" }</button>

                    <button class="wide number" onclick={link.callback(|_| Msg::N0)}>{ "0" }</button>
                    <button class="normal number" onclick={link.callback(|_| Msg::Point)}>{ "." }</button>
                    <button class="normal op" onclick={link.callback(|_| Msg::Eq)}>{ "=" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
