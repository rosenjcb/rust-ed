extern crate crossterm;
extern crate log;
extern crate stderrlog;

pub mod clipboard;
mod coord;

mod application;
mod editor;
mod renderer;

mod gridcell;
mod gridrow;

pub mod screen;
mod selection;
