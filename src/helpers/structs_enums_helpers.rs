use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum DialogMode {
    Create,
    Update,
    CreateSubfolder,
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps{
    pub accordion_title: Option<String>,
    pub accordion_description: Option<Element>,
}