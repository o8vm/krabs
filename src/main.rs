use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Read, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(short = "w", long = "write")]
    pub write: bool,
    #[structopt(short = "e", long = "emurate")]
    emu: bool,
    #[structopt(
        short = "d",
        long = "debug",
        default_value("4"),
        possible_values(&["1", "2", "3", "4"]),
    )]
    pub debug: usize,
}

type BuildStatus = Option<PathBuf>;
#[derive(Debug, Default)]
struct Builds {
    stage_1st: BuildStatus,
    stage_2nd: BuildStatus,
    stage_3rd: BuildStatus,
    stage_4th: BuildStatus,
}

#[derive(Debug, Clone)]
enum SomeError {
    InvalidBuildNumber,
    NoBIOSBootPartition,
    NoEFISystemPartition,
    NoDiskImage,
    SomeBuildsFailed,
    Stage1stOverSize,
    Stage2ndOverSize,
    Stage3rdOverSize,
    Stage4thOverSize,
}
impl std::fmt::Display for SomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl std::error::Error for SomeError {}

impl Builds {
    fn new(root_target_dir: &Path) -> Self {
        let stages = ["stage_1st", "stage_2nd", "stage_3rd", "stage_4th"];
        let mut pathes: Vec<PathBuf> = Vec::with_capacity(4);
        for stage in stages.iter() {
            pathes.push(
                root_target_dir
                    .join(stage)
                    .join(format!("i586-{}", stage))
                    .join("release")
                    .join(format!("{}.bin", stage)),
            );
        }
        let stage_1st = if pathes[0].exists() {
            Some(pathes[0].to_owned())
        } else {
            Default::default()
        };
        let stage_2nd = if pathes[1].exists() {
            Some(pathes[1].to_owned())
        } else {
            Default::default()
        };
        let stage_3rd = if pathes[2].exists() {
            Some(pathes[2].to_owned())
        } else {
            Default::default()
        };
        let stage_4th = if pathes[3].exists() {
            Some(pathes[3].to_owned())
        } else {
            Default::default()
        };
        Self {
            stage_1st,
            stage_2nd,
            stage_3rd,
            stage_4th,
        }
    }
    fn check(&self, d: usize) -> Result<(), Box<dyn Error>> {
        eprintln!("Checking the first [{}] stage(s) build...", d);
        let mut result = true;
        for n in 1..=d {
            match n {
                1 => result &= self.stage_1st.is_some(),
                2 => result &= self.stage_2nd.is_some(),
                3 => result &= self.stage_3rd.is_some(),
                4 => result &= self.stage_4th.is_some(),
                _ => return Err(SomeError::InvalidBuildNumber.into()),
            }
        }
        match result {
            false => {
                eprintln!("{:?}", self);
                Err(SomeError::SomeBuildsFailed.into())
            }
            true => {
                println!("    Build exists. {:?}", self);
                Ok(())
            }
        }
    }
}

pub fn diskimg_check(path: &Path) -> Result<(u64, u64), Box<dyn Error>> {
    println!("Checking partitions...");
    let mut result = (None, None, None);
    if !path.exists() {
        return Err(SomeError::NoDiskImage.into());
    }
    let cfg = gpt::GptConfig::new().writable(false);
    let disk = cfg.open(path)?;
    for (_, value) in disk.partitions().iter() {
        if value.part_type_guid.guid.as_bytes() == "21686148-6449-6E6F-744E-656564454649".as_bytes()
        {
            result.0 = Some(value.first_lba);
            result.1 = Some(value.last_lba);
        }
        if value.part_type_guid.guid.as_bytes() == "C12A7328-F81F-11D2-BA4B-00A0C93EC93B".as_bytes()
        {
            result.2 = Some(value.first_lba);
        }
    }
    let result = (
        result.0.ok_or(SomeError::NoBIOSBootPartition)?,
        result.1.ok_or(SomeError::NoBIOSBootPartition)?,
        result.2.ok_or(SomeError::NoEFISystemPartition)?,
    );
    println!(
        "    BIOS boot partition: start = {}, end = {}\n    EFI System Partition: start = {}",
        result.0, result.1, result.2
    );
    Ok((result.0, result.1))
}

fn write_into_disk(
    disk_path: &Path,
    builds: &Builds,
    bbp: (u64, u64),
    debug: usize,
) -> Result<(), Box<dyn Error>> {
    println!("Burn each stage to disk....");
    let output = fs::OpenOptions::new()
        .write(true)
        .truncate(false)
        .open(disk_path)?;
    let mut writer = BufWriter::new(&output);
    let default_sizes = 512;
    let mut total_sector_size: u64 = 0;

    // write stage_1st into disk
    if let Some(stage_1st) = &builds.stage_1st {
        print!("    stage_1st...");
        if 446 < fs::metadata(stage_1st)?.len() {
            return Err(SomeError::Stage1stOverSize.into());
        }
        let input = fs::File::open(stage_1st)?;
        let buf = BufReader::new(input)
            .bytes()
            .collect::<io::Result<Vec<u8>>>()?;
        writer.seek(SeekFrom::Start(0))?;
        writer.write_all(&buf)?;
        println!("  done.")
    }

    // write stage_2nd into disk, if it exists
    if let Some(stage_2nd) = &builds.stage_2nd {
        if debug < 2 {
            return Ok(());
        }
        print!("    stage_2nd...");
        let stage2_ssize = fs::metadata(stage_2nd)?.len() / default_sizes + 1;
        if bbp.1 < bbp.0 + total_sector_size + stage2_ssize {
            return Err(SomeError::Stage2ndOverSize.into());
        }
        // write stage2 size and bbp start sector as parameter
        writer.seek(SeekFrom::Start(442))?;
        writer.write_all(&(stage2_ssize as u16).to_le_bytes())?;
        writer.seek(SeekFrom::Start(444))?;
        writer.write_all(&(bbp.0 as u16).to_le_bytes())?; // bbp start sector size
                                                          // write stage_2nd.bin
        writer.seek(SeekFrom::Start(bbp.0 * default_sizes))?;
        for result in BufReader::new(fs::File::open(stage_2nd)?).bytes() {
            let byte = result?;
            writer.write_all(&[byte])?;
        }
        total_sector_size += stage2_ssize;
        println!("  done. start = {}", bbp.0 * 512);
    }

    // write stage_3rd into disk, if it exists
    if let Some(stage_3rd) = &builds.stage_3rd {
        if debug < 3 {
            return Ok(());
        }
        print!("    stage_3rd...");
        let stage3rd_ssize = fs::metadata(stage_3rd)?.len() / default_sizes + 1;
        if bbp.1 < bbp.0 + total_sector_size + stage3rd_ssize {
            return Err(SomeError::Stage3rdOverSize.into());
        }

        // write stage3 start sector position
        writer.seek(SeekFrom::Start(bbp.0 * default_sizes))?;
        writer.write_all(&((total_sector_size + bbp.0) as u16).to_le_bytes())?;
        // write stage3 sector size as a parameter
        writer.seek(SeekFrom::Start(bbp.0 * default_sizes + 2))?;
        writer.write_all(&(stage3rd_ssize as u16).to_le_bytes())?;
        // write stage_3rd.bin
        writer.seek(SeekFrom::Start((bbp.0 + total_sector_size) * default_sizes))?;
        for result in BufReader::new(fs::File::open(stage_3rd)?).bytes() {
            let byte = result?;
            writer.write_all(&[byte])?;
        }
        total_sector_size += stage3rd_ssize;
        println!("  done");
    }

    // write stage_4th into disk, it it exists
    if let Some(stage_4th) = &builds.stage_4th {
        if debug < 4 {
            return Ok(());
        }
        print!("    stage_4th...");
        let stage4th_ssize = fs::metadata(stage_4th)?.len() / default_sizes + 1;
        if bbp.1 < bbp.0 + total_sector_size + stage4th_ssize {
            return Err(SomeError::Stage4thOverSize.into());
        }
        // write stage4 sector size as a parameter
        writer.seek(SeekFrom::Start(bbp.0 * default_sizes + 4))?;
        writer.write_all(&(stage4th_ssize as u16).to_le_bytes())?;
        // write stage_4th.bin
        writer.seek(SeekFrom::Start((bbp.0 + total_sector_size) * default_sizes))?;
        for result in BufReader::new(fs::File::open(stage_4th)?).bytes() {
            let byte = result?;
            writer.write_all(&[byte])?;
        }
        println!("  done");
    }
    writer.flush()?;
    Ok(())
}
fn run_emu(disk_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Try qemu...");
    let mut qemu = Command::new("qemu-system-x86_64")
        .arg("--hda")
        .arg(disk_path)
        .arg("-m")
        .arg("1G")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    qemu.wait()?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    // Prepare
    let manifest_dir_path =
        env::var("CARGO_MANIFEST_DIR").expect("Missing CARGO_MANIFEST_DIR environment variable");
    let manifest_dir = Path::new(&manifest_dir_path);
    let current_dir = env::current_dir().expect("Couldn't get current directory");
    let target_dir_rel = manifest_dir.join("target");
    let target_dir = current_dir.join(target_dir_rel);
    let builds = Builds::new(&target_dir);

    // main start
    let args = Cli::from_args();
    let disk_path = &args.path;

    // check builds and patitions.
    if args.debug > 0 {
        // check build status
        builds.check(args.debug)?;
        // check disk status
        let bbp = diskimg_check(disk_path)?;
        // if above all is ok, let's write binaries into the disk
        if args.write {
            write_into_disk(disk_path, &builds, bbp, args.debug)?;
        }
    }

    if args.emu {
        run_emu(disk_path)?;
    }
    Ok(())
}
