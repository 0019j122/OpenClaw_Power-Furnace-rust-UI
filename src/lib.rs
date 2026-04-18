use godot::prelude::*;
use godot::classes::{Control, Label, VBoxContainer, Button, LineEdit, TextEdit};
use godot::classes::control::{LayoutPreset, SizeFlags};

#[derive(GodotClass)]
#[class(base=Control, init)]
struct FurnaceUI {
    base: Base<Control>,
    current_level: String,
}

#[godot_api]
impl FurnaceUI {
    fn _ready(&mut self) {
        godot_print!("Furnace ready");
        self.current_level = "P_1".to_string();
        self.build_ui();
    }

    fn build_ui(&mut self) {
        let level = self.current_level.clone();
        
        {
            let mut root = self.base_mut();
            let children: Vec<_> = root.get_children().iter_shared().collect();
            for child in children {
                root.remove_child(&child);
            }
        }

        match level.as_str() {
            "P_1" => self.setup_p1_login(),
            "B_6" => self.setup_b6_library(),
            "B_7" => self.setup_b7_office(),
            "B_17" => self.setup_b17_furnace(),
            _ => self.setup_p1_login(),
        }
    }

    fn setup_p1_login(&mut self) {
        let mut root = self.base_mut();
        let mut vbox = VBoxContainer::new_alloc();
        vbox.set_anchors_preset(LayoutPreset::FULL_RECT);
        root.add_child(&vbox);

        let mut title = Label::new_alloc();
        title.set_text("Login Screen");
        vbox.add_child(&title);

        let mut btn1 = Button::new_alloc();
        btn1.set_text("Guest");
        vbox.add_child(&btn1);

        let mut btn2 = Button::new_alloc();
        btn2.set_text("Student");
        vbox.add_child(&btn2);

        let mut btn3 = Button::new_alloc();
        btn3.set_text("Admin");
        vbox.add_child(&btn3);
    }

    fn setup_b6_library(&mut self) {
        let mut root = self.base_mut();
        let mut vbox = VBoxContainer::new_alloc();
        vbox.set_anchors_preset(LayoutPreset::FULL_RECT);
        root.add_child(&vbox);

        let mut title = Label::new_alloc();
        title.set_text("Library - Text Synthesis");
        vbox.add_child(&title);

        let mut text = TextEdit::new_alloc();
        text.set_v_size_flags(SizeFlags::EXPAND_FILL);
        vbox.add_child(&text);

        let mut btn = Button::new_alloc();
        btn.set_text("Back to Login");
        vbox.add_child(&btn);
    }

    fn setup_b7_office(&mut self) {
        let mut root = self.base_mut();
        let mut vbox = VBoxContainer::new_alloc();
        vbox.set_anchors_preset(LayoutPreset::FULL_RECT);
        root.add_child(&vbox);

        let mut title = Label::new_alloc();
        title.set_text("Admin Office");
        vbox.add_child(&title);

        let mut label = Label::new_alloc();
        label.set_text("API Key:");
        vbox.add_child(&label);

        let mut edit = LineEdit::new_alloc();
        edit.set_placeholder("Enter API key");
        vbox.add_child(&edit);

        let mut btn = Button::new_alloc();
        btn.set_text("Back to Login");
        vbox.add_child(&btn);
    }

    fn setup_b17_furnace(&mut self) {
        let mut root = self.base_mut();
        let mut vbox = VBoxContainer::new_alloc();
        vbox.set_anchors_preset(LayoutPreset::FULL_RECT);
        root.add_child(&vbox);

        let mut title = Label::new_alloc();
        title.set_text("Furnace Display");
        vbox.add_child(&title);

        let mut status = Label::new_alloc();
        status.set_text("Temperature: 1200C");
        vbox.add_child(&status);

        let mut btn = Button::new_alloc();
        btn.set_text("Back to Login");
        vbox.add_child(&btn);
    }

    fn on_btn_pressed(&mut self, level: String) {
        self.current_level = level;
        self.build_ui();
    }
}