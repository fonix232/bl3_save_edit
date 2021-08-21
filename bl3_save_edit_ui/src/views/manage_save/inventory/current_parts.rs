use std::collections::BTreeMap;

use iced::{
    button, scrollable, Align, Button, Color, Column, Container, Element, Length, Row, Scrollable,
    Text,
};

use bl3_save_edit_core::bl3_save::bl3_item::{
    Bl3Item, Bl3Part, MAX_BL3_ITEM_ANOINTMENTS, MAX_BL3_ITEM_PARTS,
};
use bl3_save_edit_core::resources::{ResourceCategorizedParts, ResourcePart, ResourcePartInfo};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiStyleNoBorder};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::inventory::extra_part_info::add_extra_part_info;
use crate::views::manage_save::inventory::inventory_button_style::InventoryButtonStyle;
use crate::views::manage_save::inventory::parts_tab_bar::{parts_tab_bar_button, CurrentPartType};
use crate::views::manage_save::inventory::InventoryInteractionMessage;
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Copy, Clone, Default)]
pub struct CurrentPartTypeIndex {
    pub category_index: usize,
    pub part_index: usize,
}

#[derive(Debug)]
pub struct CurrentCategorizedPart {
    pub category: String,
    pub parts: Vec<CurrentInventoryPart>,
}

impl CurrentCategorizedPart {
    pub fn new(
        category_id: usize,
        category: String,
        part_type: CurrentPartType,
        parts: Vec<Bl3PartWithInfo>,
    ) -> Self {
        let parts = parts
            .into_iter()
            .enumerate()
            .map(|(id, p)| CurrentInventoryPart::new(category_id, id, part_type.clone(), p))
            .collect();

        Self { category, parts }
    }
}

#[derive(Debug)]
pub struct CurrentInventoryPart {
    category_index: usize,
    part_index: usize,
    part_type: CurrentPartType,
    pub part: Bl3PartWithInfo,
    button_state: button::State,
}

impl CurrentInventoryPart {
    pub fn new(
        category_index: usize,
        part_index: usize,
        part_type: CurrentPartType,
        part: Bl3PartWithInfo,
    ) -> Self {
        Self {
            category_index,
            part_index,
            part_type,
            part,
            button_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let part_contents_col = Column::new()
            .push(
                TextMargin::new(
                    self.part
                        .part
                        .short_ident
                        .as_ref()
                        .unwrap_or(&self.part.part.ident),
                    1,
                )
                .0
                .font(JETBRAINS_MONO)
                .size(16),
            )
            .spacing(10);

        let part_contents_col = add_extra_part_info(part_contents_col, &self.part.info);

        let part_contents = Container::new(part_contents_col).align_x(Align::Start);

        Button::new(&mut self.button_state, part_contents)
            .on_press(InteractionMessage::ManageSaveInteraction(
                ManageSaveInteractionMessage::Inventory(match self.part_type {
                    CurrentPartType::Parts => {
                        InventoryInteractionMessage::CurrentPartPressed(CurrentPartTypeIndex {
                            category_index: self.category_index,
                            part_index: self.part_index,
                        })
                    }
                    CurrentPartType::Anointments => {
                        InventoryInteractionMessage::CurrentAnointmentPressed(
                            CurrentPartTypeIndex {
                                category_index: self.category_index,
                                part_index: self.part_index,
                            },
                        )
                    }
                }),
            ))
            .padding(10)
            .width(Length::Fill)
            .style(InventoryButtonStyle { is_active: false })
            .into_element()
    }
}

#[derive(Debug, Clone)]
pub struct InventoryCategorizedParts {
    pub category: String,
    pub parts: Vec<Bl3PartWithInfo>,
}

#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bl3PartWithInfo {
    pub part: Bl3Part,
    info: ResourcePartInfo,
}

#[derive(Debug, Default)]
pub struct CurrentParts {
    pub scrollable_state: scrollable::State,
    pub parts: Vec<CurrentCategorizedPart>,
    pub parts_tab_view: CurrentPartType,
    pub current_parts_tab_button_state: button::State,
    pub current_anointments_tab_button_state: button::State,
}

impl CurrentParts {
    pub fn view(
        &mut self,
        item: &Bl3Item,
        anointments_list: &[ResourceCategorizedParts],
        all_parts_list: Option<&Vec<ResourceCategorizedParts>>,
    ) -> Container<Message> {
        match self.parts_tab_view {
            CurrentPartType::Parts => {
                let mut categorized_parts: BTreeMap<String, Vec<Bl3PartWithInfo>> = BTreeMap::new();

                if let Some(all_parts_list) = all_parts_list {
                    if let Some(item_parts) = &item.item_parts {
                        item_parts.parts().iter().for_each(|p| {
                            let resource_part: Option<(&String, &ResourcePart)> =
                                //Find extra info about the part
                                all_parts_list.iter().find_map(|cat_resource| {
                                    let part = cat_resource.parts.iter().find(|cat_part| {
                                        part_contains(
                                            p.short_ident.as_ref(),
                                            &p.ident,
                                            &cat_part.name,
                                        )
                                    });

                                    part.map(|part| (&cat_resource.category, part))
                                });

                            if let Some((category, resource_part)) = resource_part {
                                let curr_cat_parts = categorized_parts
                                    .entry(category.to_owned())
                                    .or_insert_with(Vec::new);

                                curr_cat_parts.push(Bl3PartWithInfo {
                                    part: p.to_owned(),
                                    info: resource_part.info.to_owned(),
                                });
                            } else {
                                let curr_cat_parts = categorized_parts
                                    .entry("Unknown Parts".to_owned())
                                    .or_insert_with(Vec::new);

                                curr_cat_parts.push(Bl3PartWithInfo {
                                    part: p.to_owned(),
                                    info: ResourcePartInfo::default(),
                                });
                            }
                        });
                    }
                } else if let Some(item_parts) = &item.item_parts {
                    item_parts.parts().iter().for_each(|p| {
                        let curr_cat_parts = categorized_parts
                            .entry("Unknown Parts".to_owned())
                            .or_insert_with(Vec::new);

                        curr_cat_parts.push(Bl3PartWithInfo {
                            part: p.to_owned(),
                            info: ResourcePartInfo::default(),
                        });
                    })
                }

                let inventory_categorized_parts =
                    categorized_parts.into_iter().map(|(category, mut parts)| {
                        parts.sort();
                        InventoryCategorizedParts { category, parts }
                    });

                self.parts = inventory_categorized_parts
                    .into_iter()
                    .enumerate()
                    .map(|(cat_id, cat_p)| {
                        CurrentCategorizedPart::new(
                            cat_id,
                            cat_p.category,
                            CurrentPartType::Parts,
                            cat_p.parts,
                        )
                    })
                    .collect();
            }
            CurrentPartType::Anointments => {
                let mut categorized_parts: BTreeMap<String, Vec<Bl3PartWithInfo>> = BTreeMap::new();

                if let Some(item_parts) = &item.item_parts {
                    item_parts.generic_parts().iter().for_each(|p| {
                        let curr_cat_parts = categorized_parts
                            .entry("Unknown Parts".to_owned())
                            .or_insert_with(Vec::new);

                        //Find extra info about the anointment
                        let resource_part: Option<&ResourcePart> =
                            anointments_list.iter().find_map(|cat_resource| {
                                let part = cat_resource.parts.iter().find(|cat_part| {
                                    part_contains(p.short_ident.as_ref(), &p.ident, &cat_part.name)
                                });

                                part
                            });

                        curr_cat_parts.push(Bl3PartWithInfo {
                            part: p.to_owned(),
                            info: resource_part
                                .map(|rp| rp.info.to_owned())
                                .unwrap_or_default(),
                        });
                    })
                }

                let inventory_categorized_anointments =
                    categorized_parts.into_iter().map(|(category, mut parts)| {
                        parts.sort();
                        InventoryCategorizedParts { category, parts }
                    });

                self.parts = inventory_categorized_anointments
                    .into_iter()
                    .enumerate()
                    .map(|(cat_id, cat_p)| {
                        CurrentCategorizedPart::new(
                            cat_id,
                            cat_p.category,
                            CurrentPartType::Anointments,
                            cat_p.parts,
                        )
                    })
                    .collect();
            }
        }

        let current_parts_list =
            self.parts
                .iter_mut()
                .fold(Column::new(), |mut curr, cat_parts| {
                    curr = curr.push(
                        Container::new(
                            Text::new(&cat_parts.category)
                                .font(JETBRAINS_MONO_BOLD)
                                .size(17)
                                .color(Color::from_rgb8(242, 203, 5)),
                        )
                        .width(Length::Fill)
                        .style(Bl3UiStyleNoBorder)
                        .padding(10),
                    );

                    for p in cat_parts.parts.iter_mut() {
                        curr = curr.push(p.view())
                    }

                    curr
                });

        let mut title_row = Row::new().align_items(Align::Center);

        title_row = title_row.push(
            Container::new(parts_tab_bar_button(
                &mut self.current_parts_tab_button_state,
                CurrentPartType::Parts,
                &self.parts_tab_view,
                InventoryInteractionMessage::CurrentPartsTabPressed,
                Some(format!(
                    "({}/{})",
                    item.item_parts
                        .as_ref()
                        .map(|ip| ip.parts().len())
                        .unwrap_or(0),
                    MAX_BL3_ITEM_PARTS
                )),
            ))
            .padding(1)
            .width(Length::FillPortion(2)),
        );

        title_row = title_row.push(
            Container::new(parts_tab_bar_button(
                &mut self.current_anointments_tab_button_state,
                CurrentPartType::Anointments,
                &self.parts_tab_view,
                InventoryInteractionMessage::CurrentAnointmentsTabPressed,
                Some(format!(
                    "({}/{})",
                    item.item_parts
                        .as_ref()
                        .map(|ip| ip.generic_parts().len())
                        .unwrap_or(0),
                    MAX_BL3_ITEM_ANOINTMENTS
                )),
            ))
            .padding(1)
            .width(Length::FillPortion(2)),
        );

        let mut current_parts_column = Column::new().push(Container::new(title_row));

        let no_parts_message = Container::new(
            Text::new("This item has no parts.")
                .font(JETBRAINS_MONO)
                .size(17)
                .color(Color::from_rgb8(220, 220, 220)),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Align::Center)
        .align_y(Align::Center)
        .padding(1);

        if let Some(item_parts) = &item.item_parts {
            if !item_parts.parts().is_empty() {
                current_parts_column = current_parts_column.push(
                    Container::new(
                        Scrollable::new(&mut self.scrollable_state)
                            .push(current_parts_list)
                            .height(Length::Fill)
                            .width(Length::Fill),
                    )
                    .padding(1),
                );
            } else {
                current_parts_column = current_parts_column.push(no_parts_message);
            }
        } else {
            current_parts_column = current_parts_column.push(no_parts_message);
        }

        Container::new(current_parts_column)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(Bl3UiStyle)
    }
}

fn part_contains(short_ident: Option<&String>, ident: &str, cat_part_name: &str) -> bool {
    if let Some(short_ident) = short_ident {
        cat_part_name.to_lowercase() == short_ident.to_lowercase()
    } else {
        ident.to_lowercase().contains(&cat_part_name.to_lowercase())
    }
}
