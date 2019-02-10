use std::{io,fs,thread};
use std::path::Path;
use std::time::Duration;

fn set_mode(manual: bool) -> io::Result<()> {
    let manual_path = "/sys/devices/platform/applesmc.768/fan1_manual";
    fs::write(manual_path, (manual as u8).to_string())
}

fn set_speed(speed: i32) -> io::Result<()> {
    let output_path = "/sys/devices/platform/applesmc.768/fan1_output";
    fs::write(output_path, speed.to_string())
}

fn get_temp_at(dir: &Path) -> Result<(i32), Box<std::error::Error>> {
    let paths = fs::read_dir(dir)?;
    let mut max_temp: i32 = -127;

    for entry in paths {
        let entry = entry?;
        let name = entry.file_name().into_string();
        if let Ok(n) = name {
            if n.ends_with("_input") && 
                n.starts_with("temp") {
                let path = entry.path();
                let s = fs::read_to_string(path)?;
                let temp: i32 = s.trim().parse()?;
                max_temp = i32::max(max_temp, temp/1000);
            }
        } else {
            let err = name.unwrap_err();
            println!("can't parse name: {:?}", err); 
        }
    }
    Ok(max_temp)
}

fn get_temp() -> Result<(i32), Box<std::error::Error>> {
    let mut max_temp: i32;

    let smc_dir = "/sys/devices/platform/applesmc.768";
    let smc_temp = get_temp_at(Path::new(smc_dir))?;

    max_temp = smc_temp;

    let hwmon_dir = "/sys/devices/platform/coretemp.0/hwmon";
    let hwmons = fs::read_dir(hwmon_dir)?;

    for hwmon in hwmons {
        let temp = get_temp_at(&hwmon?.path())?;
        max_temp = i32::max(max_temp, temp);
    }
    Ok(max_temp)
}
    

fn main() {
    set_mode(true).expect("can't set mode to manual");
    let mut prev_speed = None;
    loop {
        let temp = get_temp();

        if let Ok(temp_val) = temp {
            println!("temp={}", temp_val);
            let speed = match temp_val {
                n if n >= 80 => 6000,
                70...79 => 5000,
                67...69 => 3000,
                n if n <= 65 => 1299,
                _ => -1
            };
            if speed != -1 && prev_speed != Some(speed) {
                println!("setting speed to {}", speed);
                let res = set_speed(speed);
                if let Err(err) = res {
                    println!("setting speed failed: {}", err);
                }
                prev_speed = Some(speed);
            }
        } else {
            let err = temp.unwrap_err();
            println!("getting temps failed: {}", err);
        }
        thread::sleep(Duration::from_secs(5));
    }
}
