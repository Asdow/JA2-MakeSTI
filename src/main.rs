#![allow(non_snake_case)]
use std::env;
use std::fs;
use std::process;
use std::io;
use std::io::BufRead;
use ini::Ini;

fn main() {
    let conf = readConfig();
    conf.print();

    let animationFileList = readAnimationDataFilenames();
    loop {
        let animationChoice = displayAnimationChoice(&animationFileList);
        let animationFile = &animationFileList[animationChoice];
        let animations = readAnimationDataFromFile(animationFile);
        
        let props = readProps();
        
        loop {
            let choice = displayPropChoice(&props);
            
            if choice == 98
            {
                break
            }
            else if choice == 99
            {
                return
            }
            else
            {
                convertRenderOutputToBMP(&animations, &props[choice], &conf);
                createSTIfiles(&animations, &props[choice], &conf);
            }
        }
    }
}


struct STIconfig {
    OUTPUTDIR: String,
    OFFSET: String,
    CROPSETTINGS: String,
    PIVOT: String,
    DEBUG: bool
}
impl STIconfig {
    fn print(&self) {
        println!("OUTPUTDIR = {}", self.OUTPUTDIR);
        println!("OFFSET = {}", self.OFFSET);
        println!("CROPSETTINGS = {}", self.CROPSETTINGS);
        println!("PIVOT = {}\n", self.PIVOT);
        if self.DEBUG == true
        {
            println!("Debug print active");
        }
    }
}

struct Animation {
    name: String,
    endFrame: u32,
    stiName: String,
    nDirections: u32
}
impl Animation {
    fn print(&self) {
        println!("Animation = {}", self.name);
        println!("Frames = {}", self.endFrame);
        println!("STI filename = {}", self.stiName);
        println!("Directions = {}\n", self.nDirections.to_string());
    }
}

struct PropFile {
    filename: String,
    description: String,
    props: Vec<Prop>
}

struct Prop {
    palette: String,
    prefix: String,
    suffix: String
}
impl Prop {
    fn print(&self) {
        println!("Palette = {}", self.palette);
        println!("Prefix = {}", self.prefix);
        println!("Suffix = {}", self.suffix);
    }
}


fn readConfig() -> STIconfig 
{
    let mut filePath = env::current_dir().unwrap();
    filePath.push("batchSriptData");
    filePath.push("stiConfig.ini");
    // println!("{}", filePath.display());

    let conf = Ini::load_from_file(filePath).unwrap();

    let section = conf.section(Some("STIconf")).unwrap();
    let OUTPUTDIR = section.get("OUTPUTDIR").unwrap().to_string();
    let OFFSET = section.get("OFFSET").unwrap().to_string();
    let CROPSETTINGS = section.get("CROPSETTINGS").unwrap().to_string();
    let PIVOT = section.get("PIVOT").unwrap().to_string();
    let debugstring = section.get("DEBUG_PRINT").unwrap();
    let DEBUG;
    match debugstring {
        "true" => DEBUG = true,
        "True" => DEBUG = true,
        "TRUE" => DEBUG = true,
        _ => DEBUG = false
    }

    return STIconfig {OUTPUTDIR, OFFSET, CROPSETTINGS, PIVOT, DEBUG};
}

fn readProps() -> Vec<PropFile>
{
    let mut filePath = env::current_dir().unwrap();
    filePath.push("batchSriptData");
    filePath.push("PropFiles.txt");

    let file = fs::File::open(filePath).unwrap();
    let f = io::BufReader::new(file);

    let mut propFiles: Vec<PropFile> = Vec::new();
    for line in f.lines() {
        let lineString = line.unwrap();
        if !lineString.contains(";")
        {
            let v: Vec<&str> = lineString
                .trim()
                .split_terminator("::")
                .collect()
            ;
            
            let filename = v[0].trim().to_string();
            let description = v[1].trim().to_string();
            let props = readPropData(&filename);

            propFiles.push(PropFile{filename, description, props});
        }
    }

    return propFiles;
}

fn readPropData(filename: &String) -> Vec<Prop>
{
    let mut filePath = env::current_dir().unwrap();
    filePath.push("batchSriptData");
    filePath.push(filename);

    let file = fs::File::open(filePath).unwrap();
    let f = io::BufReader::new(file);

    let mut props: Vec<Prop> = Vec::new();
    for line in f.lines() {
        let lineString = line.unwrap();
        if !lineString.contains(";")
        {
            let v: Vec<&str> = lineString
                .trim()
                .split_terminator("::")
                .collect()
            ;
            
            let palette = v[0].trim().to_string();
            let prefix = v[1].trim().to_string();
            let suffix = v[2].trim().to_string();
            
            let prop = Prop{ palette, prefix, suffix};
            // prop.print();
            props.push(prop);
        }
    }
    return props;
}

fn displayPropChoice(props: &Vec<PropFile>) -> usize
{
    let mut i = 0;
    for prop in props
    {
        println!("[{}] {}", i, prop.description);
        i += 1;
    }
    i -= 1;
    println!("[98] Choose another animation file");
    println!("[99] Quit");

    return decision(i);
}

fn readAnimationDataFilenames() -> Vec<String>
{
    let mut animFilePath = env::current_dir().unwrap();
    animFilePath.push("batchSriptData");
    animFilePath.push("AnimationFiles.txt");
    // println!("{}", animFilePath.display());

    // Open animation text file, and read only uncommented lines
    let file = fs::File::open(animFilePath).unwrap();
    let f = io::BufReader::new(file);

    let mut animations = Vec::new();
    for line in f.lines() {
        let lineString = line.unwrap();
        if !lineString.contains(";")
        {
            // println!("{}", lineString);
            animations.push(lineString);
        }
    }

    return animations;
}

fn readAnimationDataFromFile(filename: &String) -> Vec<Animation>
{
    let mut filePath = env::current_dir().unwrap();
    filePath.push("batchSriptData");
    filePath.push(filename);

    println!("---------------");
    println!("Selected animations");
    println!("---------------");
    
    // Open animation text file, and read only uncommented lines
    let file = fs::File::open(filePath).unwrap();
    let f = io::BufReader::new(file);

    let mut animations: Vec<Animation> = Vec::new();
    for line in f.lines() {
        let lineString = line.unwrap();
        if !lineString.contains(";")
        {
            let v: Vec<&str> = lineString
                .trim()
                .trim_matches(|c| c == '(' || c == ',' || c == ')')
                .split_terminator(',')
                .collect()
            ;
            
            let name = v[0].trim_matches('"').to_string();
            let endFrame: u32 = v[1].parse().unwrap();
            let stiName = v[2].trim_matches('"').to_string();
            let nDirections: u32 = v[3].parse().unwrap();
            
            let anim = Animation{ name, endFrame, stiName, nDirections };
            println!("{}", &anim.name);

            animations.push(anim);
        }
    }
    println!("");

    return animations;
}

fn displayAnimationChoice(animations: &[String]) -> usize
{
    println!("Choose animation data file");
    let mut i = 0;
    for anim in animations {
        println!("[{}] {}", i, anim);
        i += 1;
    }
    i -=1;
    
    return decision(i);
}


fn convertRenderOutputToBMP(animations: &Vec<Animation>, propfile: &PropFile, conf: &STIconfig)
{
    let currDir = env::current_dir().unwrap();

    println!("---------------");
    println!("Converting rendered animations");
    println!("---------------");
    for anim in animations
    {
        let folderName = &anim.name;
        let sourceDir = "output\\".to_string() + folderName;
        println!("{}", &anim.name);

        for prop in &propfile.props
        {
            let prefix = &prop.prefix;
            let inputDir = sourceDir.clone() + "\\" + &prefix + "_C*.png";
            let outputDir = "make_script\\extract\\".to_string() + folderName + "\\" + &prefix;

            let mut extractPath = currDir.clone();
            extractPath.push(&outputDir);
            let dirExists: bool = extractPath.is_dir();
            if dirExists
            {
                fs::remove_dir_all(&extractPath).unwrap();
            }
            fs::create_dir_all(&extractPath).unwrap();


                
            // Change convert.exe argument depending on how many view directions the animation has
            if anim.nDirections == 4
            {
                let convertArgs = [ 
                    &(sourceDir.clone() + "\\" + &prefix + "_C2*.png"),
                    &(sourceDir.clone() + "\\" + &prefix + "_C4*.png"),
                    &(sourceDir.clone() + "\\" + &prefix + "_C6*.png"),
                    &(sourceDir.clone() + "\\" + &prefix + "_C8*.png"),
                    "-crop", &conf.CROPSETTINGS, 
                    &("BMP3:".to_string() + &outputDir + "\\0.bmp") 
                ];

                if conf.DEBUG == true
                {
                    println!("Calling convert.exe with arguments:");
                    for i in 0..3
                    {
                        println!("{}", &convertArgs[i]);
                    }
                    println!("{} {}", &convertArgs[4], &convertArgs[5]);
                    println!("{}", &convertArgs[6]);
                    println!("");
                }
    
                // Crop and convert rendered images to use correct header type 
                process::Command::new("make_script\\convert.exe")
                    .args( &convertArgs)
                    .output().expect("failed to execute convert.exe");
            }
            else
            {
                let convertArgs = [ 
                    &inputDir, 
                    "-crop", &conf.CROPSETTINGS, 
                    &("BMP3:".to_string() + &outputDir + "\\0.bmp") 
                ];
   
                if conf.DEBUG == true
                {
                    println!("Calling convert.exe with arguments:");
                    println!("{}", &convertArgs[0]);
                    println!("{} {}", &convertArgs[1], &convertArgs[2]);
                    println!("{}", &convertArgs[3]);
                    println!("");
                }

                process::Command::new("make_script\\convert.exe")
                    .args( &convertArgs)
                    .output().expect("failed to execute convert.exe");
            }
        }
    }
}

fn createSTIfiles(animations: &Vec<Animation>, propfile: &PropFile, conf: &STIconfig)
{

    for anim in animations
    {
        for prop in &propfile.props
        {
            // Construct path to .sti file
            let mut outputFile = "".to_string() + &conf.OUTPUTDIR;
            outputFile.push_str(&anim.stiName);
            outputFile.push_str(&prop.suffix);
            outputFile.push_str(".sti");

            // Construct path to .bmp image files
            let mut inputFile = String::from("make_script\\extract\\");
            inputFile.push_str(&anim.name);
            inputFile.push_str("\\");
            inputFile.push_str(&prop.prefix);
            inputFile.push_str("\\0-%d.bmp");
            
            // Construct path to palette file
            let mut palette = String::from("make_script\\Palettes\\");
            palette.push_str(&prop.palette);

            // Calculate frame range and keyframes
            let mut frameRange = String::from("0-"); 
            frameRange.push_str( &(anim.endFrame * &anim.nDirections - 1).to_string());

            let mut keyframes = format!("{:#X}", &anim.endFrame);
            for _i in 1..anim.endFrame
            {
                keyframes.push_str(" 0x0");
            }

            let offset = format!("{}", &conf.OFFSET);

            // Call sticom with the arguments and create the sti file
            if conf.DEBUG == true
            {
                println!("\nCalling sticom.exe with arguments:");
                println!("-o {}", &outputFile);
                println!("-i {}", &inputFile);
                println!("-r {}", &frameRange);
                println!("-p {}", &palette);
                println!("-offset {}", &offset);
                println!("-k {}", &keyframes);
                println!("-F -M TRIM");
                println!("-p {}", &conf.PIVOT);
                println!("");
            }

            let com = process::Command::new("make_script\\sticom.exe")
                .args(&[
                    "new",
                    "-o", &outputFile,
                    "-i", &inputFile,
                    "-r", &frameRange,
                    "-p", &palette,
                    "--offset", &offset,
                    "-k", &keyframes,
                    "-F", "-M", "TRIM",
                    "-P", &conf.PIVOT
                ])
                .status().unwrap();

            if conf.DEBUG == true
            {
                println!("{}", com);
            }
        }
    }
}


fn decision(i: usize) -> usize
{
    loop {
        let mut choice = String::new();
        println!("\nChoice: ");
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        let choice: usize = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_error) => {
                println!("Select between 0...{}", i);
                continue
            },
        };

        match choice {
            choice if choice == 99 => break choice,
            choice if choice == 98 => break choice,
            choice if choice > i => {
                println!("Select between 0...{}", i);
                continue
            },
            _ => break choice
        }
    }
}