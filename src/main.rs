use num::*;

use yew::prelude::*;

enum Msg {
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Clear,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

struct Model {
    value: f64,
    current_initd: bool,
    current: f64,
    op:fn(&mut Self) -> f64,
}

impl Model{
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
    fn sqrt(&mut self) -> f64{
        Float::sqrt(self.value)
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::N1 => {
                if self.current_initd {
                    self.current = 1.0;
                    self.current_initd = false;
                } else{
                    self.current = self.current * 10.0 + 1.0;
                }
                true
            },
            Msg::N2 => {
                if self.current_initd {
                    self.current = 2.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 2.0;
                }
                true
            },
            Msg::N3 => {
                if self.current_initd {
                    self.current = 3.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 3.0;
                }
                true
            },
            Msg::N4 => {
                if self.current_initd {
                    self.current = 4.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 4.0;
                }
                true
            },
            Msg::N5 => {
                if self.current_initd {
                    self.current = 5.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 5.0;
                }
                true
            },
            Msg::N6 => {
                if self.current_initd {
                    self.current = 6.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 6.0;
                }
                true
            },
            Msg::N7 => {
                if self.current_initd {
                    self.current = 7.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 7.0;
                }
                true
            },
            Msg::N8 => {
                if self.current_initd {
                    self.current = 8.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 8.0;
                }
                true
            },
            Msg::N9 => {
                if self.current_initd {
                    self.current = 9.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 9.0;
                }
                true
            },
            Msg::N0 => {
                if self.current_initd {
                    self.current = 0.0;
                    self.current_initd = false;

                } else{
                    self.current = self.current * 10.0 + 0.0;
                }
                true
            },
            Msg::Clear => {
                self.op = Self::clear;
                self.current = 0.0;
                self.current_initd = true;
                true
            },
            Msg::Add => {
                if !self.current_initd {
                    self.value = (self.op)(self);
                }
                self.op = Self::add;
                self.current = 0.0;
                self.current_initd = true;
                true
            },
            Msg::Sub => {
                if !self.current_initd {
                    self.value = (self.op)(self);
                }
                self.op = Self::sub;
                self.current = 0.0;
                self.current_initd = true;
                true
            },
            Msg::Mul => {
                if !self.current_initd {
                    self.value = (self.op)(self);
                }
                self.op = Self::mul;
                self.current = 0.0;
                self.current_initd = true;
                true
            },
            Msg::Div => {
                if !self.current_initd {
                    self.value = (self.op)(self);
                }
                self.op = Self::div;
                self.current = 0.0;
                self.current_initd = true;
                true
            },
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
                    <p id="result">{ if self.current_initd {self.value} else {self.current} }</p>
                </div>
                <div id="input">
                    <button class="normal util" onclick={link.callback(|_| Msg::Clear)}>{ "C" }</button>
                    <button class="normal util" onclick={link.callback(|_| Msg::Clear)}>{ "C" }</button>
                    <button class="normal util" onclick={link.callback(|_| Msg::Clear)}>{ "C" }</button>
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
                    <button class="normal op" onclick={link.callback(|_| Msg::Eq)}>{ "=" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
