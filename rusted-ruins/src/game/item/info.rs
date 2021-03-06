use crate::text::ToText;
use common::gamedata::*;
use common::gobj;

const UI_IMG_ID_ITEM_INFO: &str = "!icon-item-info";

#[derive(Default, Debug)]
pub struct ItemInfoText {
    pub item_name: String,
    pub item_kind: String,
    pub desc_text: Vec<(&'static str, String)>,
}

impl ItemInfoText {
    pub fn new(item: &Item) -> ItemInfoText {
        let idx = item.idx;
        let obj = gobj::get_obj(idx);

        let item_name = item.to_text().into_owned();
        let item_kind = misc_txt_format!("item_info_text-item_kind"; item_kind=obj.kind);
        let mut desc_text = Vec::new();

        match obj.kind {
            ItemKind::Potion | ItemKind::Food => {
                let t = misc_txt_format!("item_info_text-nutrition"; nutrition=obj.nutrition);
                desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                if obj.medical_effect != MedicalEffect::None {
                    let t = misc_txt_format!(
                        "item_info_text-medical_effect"; medical_effect=obj.medical_effect);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                }
            }
            ItemKind::MagicDevice => {}
            ItemKind::Weapon(weapon_kind) => {
                if weapon_kind.is_melee() {
                    let t = misc_txt_format!(
                        "item_info_text-melee_weapon"; dice_x=obj.dice_x, dice_n=obj.dice_n);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                } else {
                    let t = misc_txt_format!(
                        "item_info_text-ranged_weapon"; dice_x=obj.dice_x, dice_n=obj.dice_n);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                }
            }
            ItemKind::Armor(_) => {
                let d0 = format!("{:+}", obj.def[Element::Physical]);
                let d1 = format!("{:+}", obj.def[Element::Fire]);
                let d2 = format!("{:+}", obj.def[Element::Cold]);
                let d3 = format!("{:+}", obj.def[Element::Shock]);
                let d4 = format!("{:+}", obj.def[Element::Poison]);
                let d5 = format!("{:+}", obj.def[Element::Spirit]);
                let t = misc_txt_format!(
                    "item_info_text_defence";
                    physical=d0, fire=d1, cold=d2, shock=d3, poison=d4, spirit=d5);
                desc_text.push((UI_IMG_ID_ITEM_INFO, t));
            }
            ItemKind::Tool => {}
            ItemKind::Container => {}
            ItemKind::Special => {}
            ItemKind::Material => {}
            ItemKind::Object => {}
        }

        ItemInfoText {
            item_name,
            item_kind,
            desc_text,
        }
    }
}
