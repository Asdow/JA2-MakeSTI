#![allow(non_snake_case)]
use std::env;
use std::fs;
use std::process;
use std::io;
use std::io::BufRead;
use std::io::Write;
use ini::Ini;

fn main() {
    let conf = readConfig();
    conf.print();

    let animationFileList = readAnimationDataFilenames();
    let animationChoice = displayAnimationChoice(&animationFileList);
    let animationFile = &animationFileList[animationChoice];
    let animations = readAnimationDataFromFile(animationFile);

    // createInputsForSTIcom();
    let props = readProps();

    loop {
        let choice = displayPropChoice(&props);

        if choice == 99
        {
            break
        }
        else if choice == 0
        {
            // Layered body
        }
        else
        {
            convertOutputToExtractForProps(&animations, &props[choice-1], &conf);
            createSTIfiles();
        }
    }

}


struct STIconfig {
    OUTPUTDIR: String,
    OFFSET: String,
    CROPSETTINGS: String,
    PIVOT: String,
}
impl STIconfig {
    fn print(&self) {
        println!("OUTPUTDIR = {}", self.OUTPUTDIR);
        println!("OFFSET = {}", self.OFFSET);
        println!("CROPSETTINGS = {}", self.CROPSETTINGS);
        println!("PIVOT = {}\n", self.PIVOT);
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


struct PropFile{
    filename: String,
    description: String,
    props: Vec<Prop>
}

struct Prop{
    palette: String,
    number: u32,
    suffix: String
}
impl Prop {
    fn print(&self) {
        println!("Palette = {}", self.palette);
        println!("Prop number = {}", self.number.to_string());
        println!("Suffix = {}", self.suffix);
    }
}


struct STIinput {

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

    return STIconfig {OUTPUTDIR, OFFSET, CROPSETTINGS, PIVOT};
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
                .split_terminator(',')
                .collect()
            ;
            
            let palette = v[0].trim().to_string();
            let number: u32 = v[1].trim().parse().unwrap();
            let suffix = v[2].trim().to_string();
            
            let prop = Prop{ palette, number, suffix};
            // prop.print();
            props.push(prop);
        }
    }
    return props;
}

fn displayPropChoice(props: &Vec<PropFile>) -> usize
{
    println!("Choose\n[0] for making a layered body");

    let mut i = 1;
    for prop in props
    {
        println!("[{}] {}", i, prop.description);
        i += 1;
    }
    i -= 1;
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
            let stiName = v[2].to_string();
            let nDirections: u32 = v[3].parse().unwrap();
            
            let anim = Animation{ name, endFrame, stiName, nDirections };
            // anim.print();
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

fn createInputsForSTIcom(animations: Vec<Animation>)
{

}

fn convertFramesToHexa()
{

}

fn convertOutputToExtractForProps(animations: &Vec<Animation>, propfile: &PropFile, conf: &STIconfig)
{
    let currDir = env::current_dir().unwrap();

    println!("---------------");
    for anim in animations
    {
        let folderName = &anim.name;
        let sourceDir = "output\\".to_string() + folderName;
        println!("{}", sourceDir);

        for prop in &propfile.props
        {
            let nProp = prop.number;
            let inputDir = sourceDir.clone() + "\\Prop" + &nProp.to_string() + "_C*.png";
            let outputDir = "make_script\\extract\\".to_string() + folderName + "\\Prop" + &nProp.to_string();

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
                    &(sourceDir.clone() + "\\Prop" + &nProp.to_string() + "_C2*.png"),
                    &(sourceDir.clone() + "\\Prop" + &nProp.to_string() + "_C4*.png"),
                    &(sourceDir.clone() + "\\Prop" + &nProp.to_string() + "_C6*.png"),
                    &(sourceDir.clone() + "\\Prop" + &nProp.to_string() + "_C8*.png"),
                    "-crop", &conf.CROPSETTINGS, 
                    &("BMP3:".to_string() + &outputDir + "\\0.bmp") 
                ];

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
                
                process::Command::new("make_script\\convert.exe")
                .args( &convertArgs)
                .output().expect("failed to execute convert.exe");
            }
        }
    }
}

fn createSTIfiles()
{

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

        if choice != 99 && choice > i {
           println!("Select between 0...{}", i) ;
           continue
        } 
        
        println!("\n");
        return choice;
    }
}