#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod mario;
mod falco;
mod custom;
mod koopa;
mod fox;
mod ganon;
mod captain;
mod ivysaur;
mod simonandrichter;
mod donkey;
mod diddy;
mod dedede;
mod cloud;

#[skyline::main(name = "super_turbo_mode")]
pub fn main() {
    mario::install();
    falco::install();
    koopa::install();
    custom::install();
    fox::install();
    ganon::install();
    captain::install();
    ivysaur::install();
    simonandrichter::install();
    donkey::install();
    dedede::install();
    //diddy::install();
}