use crate::{
    core::{render::RendererProxy, state::StateInfo},
    puzzle::PartInfo,
};

use super::registry::StateRegistry;

/// A StateOption represents one of the states available for a given StepAction type.
/// You can use it to get information on the states, such as their type name through
/// display_name(), their state identifier through step_type_id(), or their snapshot identifier
/// through snapshot_type_id ; you can also see if it the default state for this step type, and
/// select it if you wish to use it through the select() method.
/// If you select multiple options, the highest index in the list will be used; if you select none,
/// the default will be used.
pub struct StateOption {
    info: StateInfo,
    selected: bool,
}

impl StateOption {
    pub fn display_name(&self) -> &str {
        self.info.display_name
    }

    pub fn is_default(&self) -> bool {
        self.info.is_default
    }

    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn type_id(&self) -> &'static str {
        self.info.type_id
    }
    pub fn snapshot_id(&self) -> &'static str {
        self.info.snapshot_type_id
    }
}

pub struct StateSelector {
    options: Vec<StateOption>,
}

impl StateSelector {
    pub fn options_mut(&mut self) -> &mut [StateOption] {
        &mut self.options
    }
    pub(crate) fn from_step_id(step_id: &str, registry: &StateRegistry) -> Option<Self> {
        let states = registry.get(step_id)?;
        Some(Self {
            options: states
                .clone()
                .into_iter()
                .map(|x| StateOption {
                    info: x,
                    selected: false,
                })
                .collect(),
        })
    }
    pub(crate) fn resolve_selection(&self) -> Option<StateInfo> {
        // Find highest index that's selected
        for option in self.options.iter().rev() {
            if option.selected {
                return Some(option.info.clone());
            }
        }

        // Fall back to default
        self.options
            .iter()
            .find(|opt| opt.is_default())
            .map(|opt| opt.info.clone())
    }
}

pub struct RendererOption {
    pub type_name: &'static str,
    is_default: bool,
    selected: bool,
}

impl RendererOption {
    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    pub fn is_default(&self) -> bool {
        self.is_default
    }
}

pub struct RendererSelector<'renderer_list> {
    options: Vec<RendererOption>,
    renderers: &'renderer_list [Box<dyn RendererProxy>],
}

impl<'renderer_list> RendererSelector<'renderer_list> {
    pub(crate) fn from_renderers(
        renderers: &'renderer_list [Box<dyn RendererProxy>],
    ) -> Option<Self> {
        let list = renderers.iter();
        let mut options = Vec::new();
        for (index, renderer) in list.enumerate() {
            options.push(RendererOption {
                type_name: renderer.renderer_name(),
                is_default: index == 0,
                selected: false,
            })
        }
        Some(Self { options, renderers })
    }
    pub fn options_mut(&mut self) -> &mut [RendererOption] {
        &mut self.options
    }

    pub(crate) fn resolve_selection(&self) -> Option<Box<dyn RendererProxy>> {
        // Find highest index that's selected
        let mut idx = None;
        let mut default = None;
        for (index, option) in self.options.iter().enumerate() {
            if option.selected {
                idx = Some(index);
            }
            if option.is_default {
                default = Some(index);
            }
        }
        assert!(
            self.options.len() == self.renderers.len(),
            "Mismatched options and renderers vectors length... Something is very wrong."
        );
        match idx {
            None => match default {
                None => None,
                Some(idx) => Some(self.renderers[idx].clone_boxed()),
            },
            Some(idx) => Some(self.renderers[idx].clone_boxed()),
        }
    }
}

#[derive(Clone)]
pub(crate) struct PartInfoSelector {
    id: String,
    display_name: String,
    description: Option<String>,
    step_type_id: &'static str,
}

pub struct PartOption {
    info: PartInfoSelector,
    selected: bool,
    is_default: bool,
}

impl PartOption {
    pub fn display_name(&self) -> &str {
        &self.info.display_name
    }
    pub fn id(&self) -> &str {
        &self.info.id
    }

    pub fn is_default(&self) -> bool {
        self.is_default
    }

    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn step_type_id(&self) -> &'static str {
        self.info.step_type_id
    }
    pub fn description(&self) -> Option<&str> {
        self.info.description.as_deref()
    }
}

pub struct PartSelector<'a> {
    options: Vec<PartOption>,
    source: &'a [PartInfo],
}
//FIXME : We're copying things we don't need to. Whatever for now
impl<'a> PartSelector<'a> {
    pub fn options_mut(&mut self) -> &mut [PartOption] {
        self.options.as_mut()
    }

    pub(crate) fn from_parts(parts: &'a [PartInfo]) -> Self {
        let mut options = Vec::new();
        for (idx, part) in parts.iter().enumerate() {
            options.push(PartOption {
                info: PartInfoSelector {
                    id: part.id.clone(),
                    display_name: part.display_name.clone(),
                    description: part.description.clone(),
                    step_type_id: part.step_type_id,
                },
                selected: false,
                is_default: idx == 0,
            })
        }
        Self {
            options,
            source: parts,
        }
    }
    pub(crate) fn resolve_selection(&self) -> Option<&PartInfo> {
        // Find highest index that's selected
        let mut id = None;
        let mut default = None;
        for (idx, option) in self.options.iter().enumerate() {
            if option.selected {
                id = Some(idx);
            }
            if option.is_default {
                default = Some(idx);
            }
        }
        match id {
            None => match default {
                None => None,
                Some(id) => Some(&self.source[id]),
            },
            Some(id) => Some(&self.source[id]),
        }
    }
}
