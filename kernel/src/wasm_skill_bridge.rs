extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn wasm_skill_bridge_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn wasm_skill_bridge_exit() {
    // Cleanup logic for the module
}

pub struct WasmSkillBridge {
    skills: Vec<String>,
}

impl WasmSkillBridge {
    pub fn new() -> Self {
        WasmSkillBridge {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn remove_skill(&mut self, index: usize) -> Option<String> {
        if index < self.skills.len() {
            Some(self.skills.remove(index))
        } else {
            None
        }
    }

    pub fn get_skill(&self, index: usize) -> Option<&String> {
        self.skills.get(index)
    }

    pub fn list_skills(&self) -> &[String] {
        &self.skills
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }
}

#[no_mangle]
pub extern "C" fn wasm_skill_bridge_add_skill(bridge: *mut WasmSkillBridge, skill: *const u8, len: usize) {
    if let Some(bridge) = unsafe { bridge.as_mut() } {
        let skill_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(skill, len)) };
        bridge.add_skill(String::from(skill_str));
    }
}

#[no_mangle]
pub extern "C" fn wasm_skill_bridge_remove_skill(bridge: *mut WasmSkillBridge, index: usize) -> bool {
    if let Some(bridge) = unsafe { bridge.as_mut() } {
        bridge.remove_skill(index).is_some()
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn wasm_skill_bridge_get_skill(bridge: *const WasmSkillBridge, index: usize, out: *mut u8, len: usize) -> bool {
    if let Some(bridge) = unsafe { bridge.as_ref() } {
        if let Some(skill) = bridge.get_skill(index) {
            let skill_bytes = skill.as_bytes();
            if skill_bytes.len() <= len {
                unsafe { core::ptr::copy_nonoverlapping(skill_bytes.as_ptr(), out, skill_bytes.len()) };
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn wasm_skill_bridge_list_skills(bridge: *const WasmSkillBridge, out: *mut *const u8, len: usize) -> usize {
    if let Some(bridge) = unsafe { bridge.as_ref() } {
        let skills = bridge.list_skills();
        let count = core::cmp::min(skills.len(), len);
        for i in 0..count {
            unsafe { *out.add(i) = skills[i].as_ptr() };
        }
        count
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn wasm_skill_bridge_skill_count(bridge: *const WasmSkillBridge) -> usize {
    if let Some(bridge) = unsafe { bridge.as_ref() } {
        bridge.skill_count()
    } else {
        0
    }
}
