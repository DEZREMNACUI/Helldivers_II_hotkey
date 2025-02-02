use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, Key, KeyboardControllable};
use std::{collections::HashSet, thread, time::Duration};

/// 模拟按键序列：保持一个键按下的同时按下其他键
fn simulate_key_sequence(hold_key: Key, sequence_keys: &[Key]) {
    let mut enigo = Enigo::new();

    // 确保之前的按键都已释放
    // enigo.key_up(hold_key);
    thread::sleep(Duration::from_millis(100));

    // 按下并保持第一个键
    enigo.key_down(hold_key);
    thread::sleep(Duration::from_millis(100));

    // 依次点击序列中的其他键
    for &key in sequence_keys {
        enigo.key_click(key);
        thread::sleep(Duration::from_millis(100));
    }

    // 释放第一个键
    enigo.key_up(hold_key);
    thread::sleep(Duration::from_millis(100));
}

/// 定义一个热键配置结构
struct HotkeyConfig {
    trigger_keys: Vec<Keycode>,
    hold_key: Key,
    sequence_keys: Vec<Key>,
    description: String,
}

/// 监听多个组合键
fn listen_hotkeys(configs: Vec<HotkeyConfig>) {
    let device_state = DeviceState::new();
    let mut pressed_keys: HashSet<Keycode> = HashSet::new();

    // 创建目标按键集合
    let mut target_keys: HashSet<Keycode> = HashSet::new();
    target_keys.insert(Keycode::Escape); // 添加 ESC 用于退出
                                         // 收集所有需要监听的按键
    for config in &configs {
        target_keys.extend(config.trigger_keys.iter().cloned());
    }

    println!("开始监听键盘输入...");
    println!("按 ESC 键退出程序");

    // 打印所有热键配置
    for config in &configs {
        println!(
            "热键配置 - {}: {:?}",
            config.description, config.trigger_keys
        );
    }

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let keys_set: HashSet<_> = keys.iter().collect();

        // 检查每个热键配置
        for config in &configs {
            let all_trigger_keys_pressed = config.trigger_keys.iter().all(|k| keys_set.contains(k));

            if all_trigger_keys_pressed {
                // 确保这是新的按键组合（避免重复触发）
                if !pressed_keys.contains(&config.trigger_keys[0]) {
                    println!("触发: {}", config.description);
                    simulate_key_sequence(config.hold_key, &config.sequence_keys);
                    pressed_keys.insert(config.trigger_keys[0]);
                }
            }
        }

        // 只保留目标按键的状态
        pressed_keys.retain(|key| keys.contains(key) && target_keys.contains(key));

        // 检查是否按下 ESC 键退出
        // if keys.contains(&Keycode::Escape) {
        //     println!("程序退出！");
        //     break;
        // }

        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let hotkey_configs = vec![
        // 飞鹰空袭
        HotkeyConfig {
            trigger_keys: vec![Keycode::LControl, Keycode::Key1],
            hold_key: Key::Space,
            sequence_keys: vec![
                Key::UpArrow,
                Key::RightArrow,
                Key::DownArrow,
                Key::RightArrow,
            ],
            description: String::from("飞鹰空袭 (Ctrl+1)"),
        },
        // 飞鹰集束炸弹
        HotkeyConfig {
            trigger_keys: vec![Keycode::LControl, Keycode::Key2],
            hold_key: Key::Space,
            sequence_keys: vec![
                Key::UpArrow,
                Key::RightArrow,
                Key::DownArrow,
                Key::DownArrow,
                Key::RightArrow,
            ],
            description: String::from("飞鹰集束炸弹 (Ctrl+2)"),
        },
        // 飞鹰500kg炸弹
        HotkeyConfig {
            trigger_keys: vec![Keycode::LControl, Keycode::Key3],
            hold_key: Key::Space,
            sequence_keys: vec![
                Key::UpArrow,
                Key::RightArrow,
                Key::DownArrow,
                Key::DownArrow,
                Key::DownArrow,
            ],
            description: String::from("飞鹰500kg炸弹 (Ctrl+3)"),
        },
        // 飞鹰机枪扫射
        HotkeyConfig {
            trigger_keys: vec![Keycode::LControl, Keycode::Key4],
            hold_key: Key::Space,
            sequence_keys: vec![Key::UpArrow, Key::RightArrow, Key::RightArrow],
            description: String::from("飞鹰机枪扫射 (Ctrl+4)"),
        },
        // 飞鹰凝固汽油弹
        HotkeyConfig {
            trigger_keys: vec![Keycode::LControl, Keycode::Q],
            hold_key: Key::Space,
            sequence_keys: vec![Key::UpArrow, Key::RightArrow, Key::DownArrow, Key::UpArrow],
            description: String::from("飞鹰凝固汽油弹 (Ctrl+4)"),
        },
        // 飞鹰110MM火箭巢
        HotkeyConfig {
            trigger_keys: vec![Keycode::LControl, Keycode::E],
            hold_key: Key::Space,
            sequence_keys: vec![Key::UpArrow, Key::RightArrow, Key::UpArrow, Key::LeftArrow],
            description: String::from("飞鹰110MM火箭巢 (Ctrl+E)"),
        },

        // 加特林炮台
        HotkeyConfig {
            trigger_keys: vec![Keycode::LShift, Keycode::Key1],
            hold_key: Key::Space,
            sequence_keys: vec![
                Key::DownArrow,
                Key::UpArrow,
                Key::RightArrow,
                Key::LeftArrow,
            ],
            description: String::from("加特林炮台 (Shift+1)"),
        },
        // 自动哨戒炮
        HotkeyConfig {
            trigger_keys: vec![Keycode::LShift, Keycode::Key2],
            hold_key: Key::Space,
            sequence_keys: vec![
                Key::DownArrow,
                Key::UpArrow,
                Key::RightArrow,
                Key::UpArrow,
                Key::LeftArrow,
                Key::UpArrow,
            ],
            description: String::from("自动哨戒炮 (Shift+2)"),
        },
        // 火箭哨戒炮
        HotkeyConfig {
            trigger_keys: vec![Keycode::LShift, Keycode::Key3],
            hold_key: Key::Space,
            sequence_keys: vec![
                Key::DownArrow,
                Key::UpArrow,
                Key::RightArrow,
                Key::RightArrow,
                Key::LeftArrow,
            ],
            description: String::from("火箭哨戒炮 (Shift+3)"),
        },
        // 迫击炮
        HotkeyConfig {
            trigger_keys: vec![Keycode::LShift, Keycode::Q],
            hold_key: Key::Space,
            sequence_keys: vec![
                Key::DownArrow,
                Key::UpArrow,
                Key::RightArrow,
                Key::RightArrow,
                Key::DownArrow,
            ],
            description: String::from("迫击炮 (Shift+Q)"),
        },
    ];

    listen_hotkeys(hotkey_configs);
}
