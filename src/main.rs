// Date:   Thu Jun 27 18:24:00 2024
// Mail:   lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{default::Default, io::{BufRead, BufReader, BufWriter, Write}};
use std::path::Path;
use std::fs::File;

use clap::Parser;
use regex::{Regex, RegexSet};

// this crate
use make_epub::{
    common::*, utils::num_to_zh,
    NCX_chapter_entry, NCX_header, NCX_volume_entry, 
    OPF_header, OPF_item, OPF_itemref,
    chapter_header, chapter_line, volume,
    err::Error,
};

#[derive(Parser, Default)]
#[command(about)]
struct Args {
    // #[arg(short, long)]
    #[arg(short, long, default_value_t = String::from(""))]
    author: String,
    
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value_t = String::from(""))]
    out: String,

    #[arg(short, long, default_value_t = String::from(""))]
    name: String,

    #[arg(short, long, default_value_t = String::from(""))]
    cover: String,

    #[arg(short, long, default_value_t = String::from(""))]
    slim: String,

    #[arg(long, value_delimiter = ',')]
    extra_volume: Vec<String>,

    #[arg(long, value_delimiter = ',')]
    extra_chapter: Vec<String>,

    #[arg(long, default_value_t = false)]
    disable_default_match: bool,

    #[arg(short = 'I', default_value_t = false)]
    interactive: bool,
}

struct Chapter {
    seq: usize,
    title: String,
    lines: Vec<String>
}

enum VC {
    Chapter((usize, String)),
    Volume((usize, String))
}

struct Toc {
    table: Vec<VC>,
    chap_count: usize,
    vol_count: usize
}

struct Maker {
    input: String,
    name: String,
    author: String,
    cover: String,
    slim: String,

    root_dir: String,
    cover_ext: String,
    toc: Toc,
    chapter_reset: Vec<Regex>,
    volume_reset: Vec<Regex>,
}

impl Chapter {
    fn new(seq: usize, title: &str) -> Self {
        Self {
            seq,
            title: String::from(title),
            lines: Default::default()
        }
    }

    #[inline]
    fn push_line(&mut self, line: String) {
        self.lines.push(line);
    }
}

impl Toc {
    fn new() -> Self {
        Self {
            table: Vec::new(),
            chap_count: 0,
            vol_count: 0
        }
    }

    fn add_chapter(&mut self, title: &str) {
        self.chap_count += 1;
        self.table.push(VC::Chapter((self.chap_count, String::from(title))));
    }

    fn add_volume(&mut self, title: &str) {
        self.vol_count += 1;
        self.table.push(VC::Volume((self.vol_count, String::from(title))));
    }
}

impl From<Args> for Maker {
    fn from(mut args: Args) -> Self {
        if args.interactive {
            interact(&mut args).unwrap();
        }

        let Args {
            author, input, mut name, mut out, cover, slim,
            mut extra_volume, mut extra_chapter, 
            disable_default_match,
            ..
        } = args;

        if input.is_empty() {
            panic!("Input cannot be empty");
        }
        // check if files exist.
        [&input, &cover, &slim].iter().for_each(|path| {
            if !path.is_empty() && !Path::new(path.trim()).exists() {
                panic!("File {} does not exist", path);
            }
        });

        if name.is_empty() {
            name = String::from(
                Path::new(&input).file_stem()
                    .expect(&format!("Input {} has no stem", input))
                    .to_str()
                    .unwrap()
            );
            assert!(!name.is_empty());
        }
        if out.is_empty() {
            out = name.clone();
        }

        let root_dir = format!("{}.epub", out);
        let cover_ext = String::from(Path::new(&cover)
            .extension()
            .expect(&format!("Cover {} has no extension", cover))
            .to_str()
            .unwrap()
        );

        if !disable_default_match {
            extra_chapter.push(String::from(patterns::DEFAULT_CHAPTER_PAT));
            extra_volume.push(String::from(patterns::DEFAULT_VOLUME_PAT));
        }

        let make_reset = |pats: &Vec<String>| -> Vec<Regex> {
            pats.iter()
                .map(|pat| match Regex::new(pat) {
                    Ok(re) => re,
                    Err(e) => panic!("Invalid regex pattern {}: {}", pat, e)
                })
                .collect::<Vec<Regex>>()
        };

        Self {
            author, input, name, cover, slim,
            root_dir,
            cover_ext,
            toc: Toc::new(),
            chapter_reset: make_reset(&extra_chapter),
            volume_reset: make_reset(&extra_volume),
        }
    }
}

impl Maker {
    pub fn init(&self) -> Result<(), Error> {
        let Self {
            root_dir,
            ..
        } = self;

        let mut exists = false;
        if let Err(e) = std::fs::create_dir(root_dir) {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    exists = true;
                },
                _ => return Err(Error::IOError(e))
            }
        }

        if exists {
            print!("{} exists, remove it? [y/n] ", root_dir);
            std::io::stdout().flush()?;
            let ans = {
                let mut ans = String::new();
                std::io::stdin().read_line(&mut ans)?;
                ans
            };
            match ans.trim().to_lowercase().as_str() {
                "y" => {
                    let root_dir = Path::new(root_dir);
                    if root_dir.is_dir() {
                        std::fs::remove_dir_all(root_dir)?;
                    } else {
                        std::fs::remove_file(root_dir)?;
                    }
                },
                "n" => std::process::exit(0),
                a => panic!("Invalid answer <{}>", a)
            }
        }

        println!("Created root {}", root_dir);

        // create necessary directories
        for sub_dir in SUBDIRS.iter() {
            std::fs::create_dir_all(format!("{}/{}", root_dir, sub_dir))?;
        }

        // copy cover and slim to specific locations
        for (src, dst) in [
            (&self.cover, format!("{}/iTunesArtwork", root_dir)),
            (&self.slim, format!("{}/OPS/images/cover.{}", root_dir, self.cover_ext))
        ]
        .iter() {
            std::fs::copy(src, dst)?;
        }

        let coverpage_html = coverpage(&self.cover_ext);

        for (path, content) in [
            (format!("{}/mimetype", root_dir), MIMETYPE),
            (format!("{}/META-INF/container.xml", root_dir), CONTAINER),
            (format!("{}/OPS/css/main.css", root_dir), MAIN_CSS),
            (format!("{}/OPS/css/cover.css", root_dir), COVER_CSS),
            (format!("{}/OPS/html/coverpage.html", root_dir), &coverpage_html)
        ]
        .iter() {
            let file = File::create(path)?;
            BufWriter::new(file).write(content.as_bytes())?;
        }
        Ok(())
    }

    pub fn make(&mut self) -> Result<(), Error> {
        let reader = BufReader::new(File::open(&self.input)?);

        let skip_re = RegexSet::new(patterns::SKIP_PATS).unwrap();

        let mut chapter: Option<Chapter> = None;

        'reading: for (line_num, line) in reader.lines().enumerate() {
            let line = line?;

            let vol_caps = self.volume_reset.iter()
                .find_map(|re| re.captures(&line));

            if let Some(vol_caps) = vol_caps {
                self.toc.add_volume(&vol_caps["vol_name"]);
                self.make_volume(&vol_caps["vol_name"])?;
                continue 'reading;
            }

            let chap_caps = self.chapter_reset.iter()
                .find_map(|re| re.captures(&line));

            if let Some(chap_caps) = chap_caps {
                let chap_name = &chap_caps["chap_name"];
                self.toc.add_chapter(chap_name);

                let new_chap = Chapter::new(self.toc.chap_count, chap_name);

                if let Some(old_chap) = chapter.replace(new_chap) {
                    self.make_chapter(old_chap)?;
                }
                continue 'reading;
            }

            if !skip_re.is_match(&line) {
                chapter
                    .as_mut()
                    .expect(&format!("Line {} has no chapter", line_num))
                    .push_line(line);
            }
        }
        Ok(())
    }

    fn make_volume(&mut self, title: &str) -> Result<(), Error> {
        let vol_seq = self.toc.vol_count;
        let vol_file = File::create(&format!("{}/OPS/html/volume{}.html", 
                &self.root_dir, vol_seq))?;
        let mut writer = BufWriter::new(vol_file);
        let zh_seq = num_to_zh(vol_seq);
        println!("\n第{}卷 {}", zh_seq, title);
        writer.write_fmt(volume!(&zh_seq, title))?;
        Ok(())
    }

    fn make_chapter(&self, chapter: Chapter) -> Result<(), Error> {
        let chap_seq = chapter.seq;
        let chap_file = File::create(
            &format!("{}/OPS/html/chapter{}.html", &self.root_dir, chap_seq))?;
        let mut writer = BufWriter::new(chap_file);

        let title = format!("第{}章 {}", num_to_zh(chap_seq), chapter.title);
        println!("{}", title);
        writer.write_fmt(chapter_header!(title))?;
        for line in chapter.lines.iter() {
            writer.write_fmt(chapter_line!(line.as_str()))?;
        }
        writer.write(CHAPTER_END.as_bytes())?;

        Ok(())
    }

    pub fn finish(&self) -> Result<(), Error> {
        let ncx = File::create(&format!("{}/OPS/toc.ncx", &self.root_dir))?;
        let opf = File::create(&format!("{}/OPS/content.opf", &self.root_dir))?;

        let mut ncx = BufWriter::new(ncx);
        let mut opf = BufWriter::new(opf);

        ncx.write_fmt(NCX_header!(&self.name))?;
        for (order, entry) in self.toc.table.iter().enumerate() {
            match entry {
                VC::Volume((seq, title)) => 
                    ncx.write_fmt(NCX_volume_entry!(title, order+1, seq))?,
                VC::Chapter((seq, title)) => 
                    ncx.write_fmt(NCX_chapter_entry!(title, order+1, seq, num_to_zh(*seq)))?,
            }
        }
        ncx.write(ncx_end(self.toc.vol_count > 0).as_bytes())?;

        opf.write_fmt(OPF_header!(&self.name, 
                &self.author, &self.cover_ext))?;

        for entry in self.toc.table.iter() {
            match entry {
                VC::Volume((seq, _)) => 
                    opf.write_fmt(OPF_item!(Volume, seq))?,
                VC::Chapter((seq, _)) => 
                    opf.write_fmt(OPF_item!(Chapter, seq))?,
            }
        }
        opf.write(OPF_MID.as_bytes())?;
        
        for seq in 1..(self.toc.vol_count+1) {
            opf.write_fmt(OPF_itemref!(Volume, seq))?;
        }
        for seq in 1..(self.toc.chap_count+1) {
            opf.write_fmt(OPF_itemref!(Chapter, seq))?;
        }
        opf.write(OPF_END.as_bytes())?;

        Ok(())
    }
}

fn interact(args: &mut Args) -> Result<(), Error> {
    std::io::stdout().flush()?;
    macro_rules! ask {
        ($field: ident) => {
            print!("{}: ", stringify!($field));
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut args.$field)?;
            args.$field = String::from(args.$field.as_str().trim());
        };
        ($field: ident, $($rest: ident),*) => {
            ask!($field);
            ask!($($rest),*);
        }
    }
    ask!(name, author, out, cover, slim);
    Ok(())
}

fn main() {
    let args = Args::parse();
    // if book.name.is_empty() {
    //     let path = Path::new(&book.input);
    //     book.name = String::from(path.file_stem().expect(
    //         format!("Input {} has no stem", book.input).as_ref()
    //     ).to_str().unwrap());
    //     assert!(!book.name.is_empty());
    // }

    let mut maker = Maker::from(args);
    if let Err(e) = maker.init() {
        panic!("{:?}", e);
    }
    if let Err(e) = maker.make() {
        panic!("{:?}", e);
    }
    if let Err(e) = maker.finish() {
        panic!("{:?}", e);
    }
}

